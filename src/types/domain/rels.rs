//! Relationship Specific Stuff
//!
use std::fmt::Write;

use sarzak::{
    mc::{FormatSnafu, Result},
    sarzak::types::{Binary, Cardinality, Conditionality, External, Object, Referrer},
    v2::domain::Domain,
    woog::{
        store::ObjectStore as WoogStore,
        types::{Ownership, BORROWED},
    },
};
use snafu::prelude::*;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        find_store, get_referents_sorted, get_referrers_sorted, inner_object_is_enum,
        render::{RenderIdent, RenderType},
    },
    options::GraceConfig,
};

pub(crate) fn generate_binary_referrer_rels(
    buffer: &mut Buffer,
    config: &GraceConfig,
    module: &str,
    obj: &Object,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    // Generate binary relationship navigation for the referrer side.
    for referrer in get_referrers_sorted!(obj, domain.sarzak()) {
        let binary = referrer.r6_binary(domain.sarzak())[0];
        let referent = binary.r5_referent(domain.sarzak())[0];
        let r_obj = referent.r16_object(domain.sarzak())[0];
        let cond = referrer.r11_conditionality(domain.sarzak())[0];

        let module = if config.is_imported(&r_obj.id) {
            config.get_imported(&r_obj.id).unwrap().domain.as_str()
        } else {
            module
        };

        // Grab a reference to the store so that we can use it to exhume
        // things.
        let store = find_store(module, woog, domain);

        // Cardinality does not matter from the referrer, because it's always
        // one. This is because of the normalized, table-nature of the store,
        // and more importantly the method.
        match cond {
            Conditionality::Unconditional(_) => {
                forward(buffer, obj, referrer, binary, store, r_obj, woog, domain)?
            }
            Conditionality::Conditional(_) => {
                forward_conditional(buffer, obj, referrer, binary, store, r_obj, woog, domain)?
            }
        }
    }

    Ok(())
}

pub(crate) fn generate_binary_referent_rels(
    buffer: &mut Buffer,
    config: &GraceConfig,
    module: &str,
    obj: &Object,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    // Generate binary relationship navigation for the referent side.
    for referent in get_referents_sorted!(obj, domain.sarzak()) {
        let binary = referent.r5_binary(domain.sarzak())[0];
        let referrer = binary.r6_referrer(domain.sarzak())[0];
        let r_obj = referrer.r17_object(domain.sarzak())[0];
        let my_cond = referent.r12_conditionality(domain.sarzak())[0];
        let other_cond = referrer.r11_conditionality(domain.sarzak())[0];

        // The non-formalizing side will only ever be one, unless it's in an associative
        // relationship. We do however need to check the cardinality of the formalizing side.
        let card = referrer.r9_cardinality(domain.sarzak())[0];

        let module = if config.is_imported(&r_obj.id) {
            config.get_imported(&r_obj.id).unwrap().domain.as_str()
        } else {
            module
        };

        // Grab a reference to the store so that we can use it to exhume
        // things.
        let store = find_store(module, woog, domain);

        match card {
            Cardinality::One(_) => match my_cond {
                Conditionality::Unconditional(_) => {
                    backward_one(buffer, obj, r_obj, binary, &store, referrer, &woog, &domain)?
                }
                Conditionality::Conditional(_) => match other_cond {
                    Conditionality::Unconditional(_) => backward_one_conditional(
                        buffer, obj, r_obj, binary, &store, referrer, &woog, &domain,
                    )?,
                    Conditionality::Conditional(_) => backward_one_biconditional(
                        buffer, obj, r_obj, binary, &store, referrer, &woog, &domain,
                    )?,
                },
            },
            // It's interesting that there are only really two possibilities, and
            // that neither of them depend on the conditionality of the this side.
            Cardinality::Many(_) => match other_cond {
                Conditionality::Unconditional(_) => {
                    backward_1_m(buffer, obj, r_obj, binary, store, referrer, woog, domain)?
                }
                Conditionality::Conditional(_) => {
                    backward_1_mc(buffer, obj, r_obj, binary, store, referrer, woog, domain)?
                }
            },
        }
    }

    Ok(())
}

pub(crate) fn generate_assoc_referrer_rels(
    buffer: &mut Buffer,
    config: &GraceConfig,
    module: &str,
    obj: &Object,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    // Generate associative relationship navigation for the referrer side.
    for assoc_referrer in obj.r26_associative_referrer(domain.sarzak()) {
        let assoc = assoc_referrer.r21_associative(domain.sarzak())[0];

        let one = assoc.r23_associative_referent(domain.sarzak())[0];
        let one_obj = one.r25_object(domain.sarzak())[0];

        let other = assoc.r22_associative_referent(domain.sarzak())[0];
        let other_obj = other.r25_object(domain.sarzak())[0];

        let module = if config.is_imported(&one_obj.id) {
            config.get_imported(&one_obj.id).unwrap().domain.as_str()
        } else {
            module
        };

        // Grab a reference to the store so that we can use it to exhume
        // things.
        let store = find_store(module, woog, domain);

        forward_assoc(
            buffer,
            obj,
            &assoc_referrer.one_referential_attribute,
            assoc.number,
            store,
            one_obj,
            woog,
            domain,
        )?;

        let module = if config.is_imported(&other_obj.id) {
            config.get_imported(&one_obj.id).unwrap().domain.as_str()
        } else {
            module
        };

        // Grab a reference to the store so that we can use it to exhume
        // things.
        let store = find_store(module, woog, domain);
        forward_assoc(
            buffer,
            obj,
            &assoc_referrer.other_referential_attribute,
            assoc.number,
            store,
            other_obj,
            woog,
            domain,
        )?;
    }

    Ok(())
}

pub(crate) fn generate_assoc_referent_rels(
    buffer: &mut Buffer,
    config: &GraceConfig,
    module: &str,
    obj: &Object,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    // Generate associative relationship navigation for the referent side.
    for assoc_referent in obj.r25_associative_referent(domain.sarzak()) {
        let r23 = assoc_referent.r23c_associative(domain.sarzak());
        let (assoc, referrer, referential_attribute) = if r23.is_empty() {
            let assoc = assoc_referent.r22c_associative(domain.sarzak())[0];
            let referrer = assoc.r21_associative_referrer(domain.sarzak())[0];
            (assoc, referrer, &referrer.other_referential_attribute)
        } else {
            let assoc = r23[0];
            let referrer = assoc.r21_associative_referrer(domain.sarzak())[0];
            (assoc, referrer, &referrer.one_referential_attribute)
        };
        let card = assoc_referent.r88_cardinality(domain.sarzak())[0];
        let cond = assoc_referent.r77_conditionality(domain.sarzak())[0];
        let r_obj = referrer.r26_object(domain.sarzak())[0];

        let module = if config.is_imported(&r_obj.id) {
            config.get_imported(&r_obj.id).unwrap().domain.as_str()
        } else {
            module
        };

        // Grab a reference to the store so that we can use it to exhume
        // things.
        let store = find_store(module, woog, domain);

        match card {
            Cardinality::One(_) => match cond {
                Conditionality::Conditional(_) => backward_assoc_one_conditional(
                    buffer,
                    obj,
                    r_obj,
                    assoc.number,
                    store,
                    referential_attribute,
                    woog,
                    domain,
                )?,
                Conditionality::Unconditional(_) => backward_assoc_one(
                    buffer,
                    obj,
                    r_obj,
                    assoc.number,
                    store,
                    referential_attribute,
                    woog,
                    domain,
                )?,
            },
            Cardinality::Many(_) => backward_assoc_many(
                buffer,
                obj,
                r_obj,
                assoc.number,
                store,
                referential_attribute,
                woog,
                domain,
            )?,
        }
    }

    Ok(())
}

pub(crate) fn generate_subtype_rels(
    buffer: &mut Buffer,
    config: &GraceConfig,
    module: &str,
    obj: &Object,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    // Generate navigation methods for subtype to supertype navigation.
    // There should only be one of these, if any.
    for subtype in obj.r15c_subtype(domain.sarzak()) {
        let isa = subtype.r27_isa(domain.sarzak())[0];
        let supertype = isa.r13_supertype(domain.sarzak())[0];
        let s_obj = supertype.r14_object(domain.sarzak())[0];

        // 🚧 This whole bit should be refactored into find_store.
        let module = if config.is_imported(&s_obj.id) {
            config.get_imported(&s_obj.id).unwrap().domain.as_str()
        } else {
            module
        };

        // Grab a reference to the store so that we can use it to exhume
        // things.
        let store = find_store(module, woog, domain);

        subtype_to_supertype(buffer, obj, s_obj, isa.number, store, woog, domain)?;
    }

    Ok(())
}

fn forward(
    buffer: &mut Buffer,
    obj: &Object,
    referrer: &Referrer,
    binary: &Binary,
    store: &External,
    r_obj: &Object,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    buffer.block(
        DirectiveKind::IgnoreOrig,
        format!(
            "{}-struct-impl-nav-forward-to-{}",
            obj.as_ident(),
            referrer.referential_attribute.as_ident()
        ),
        |buffer| {
            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-*)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                binary.number,
            );
            emit!(
                buffer,
                "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                binary.number,
                r_obj.as_ident(),
                store.name,
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
            );
            emit!(
                buffer,
                "vec![store.exhume_{}(&self.{}).unwrap()]",
                r_obj.as_ident(),
                referrer.referential_attribute.as_ident()
            );
            emit!(buffer, "}}");

            Ok(())
        },
    )
}

fn forward_conditional(
    buffer: &mut Buffer,
    obj: &Object,
    referrer: &Referrer,
    binary: &Binary,
    store: &External,
    r_obj: &Object,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    buffer.block(
        DirectiveKind::IgnoreOrig,
        format!(
            "{}-struct-impl-nav-forward-cond-to-{}",
            obj.as_ident(),
            referrer.referential_attribute.as_ident()
        ),
        |buffer| {
            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-*c)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                binary.number,
            );
            emit!(
                buffer,
                "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                binary.number,
                r_obj.as_ident(),
                store.name,
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
            );
            emit!(
                buffer,
                "match self.{} {{",
                referrer.referential_attribute.as_ident()
            );
            emit!(
                buffer,
                "Some(ref {}) => vec![store.exhume_{}({}).unwrap()],",
                referrer.referential_attribute.as_ident(),
                r_obj.as_ident(),
                referrer.referential_attribute.as_ident()
            );
            emit!(buffer, "None => Vec::new(),");
            emit!(buffer, "}}");
            emit!(buffer, "}}");

            Ok(())
        },
    )
}

fn backward_one(
    buffer: &mut Buffer,
    obj: &Object,
    r_obj: &Object,
    binary: &Binary,
    store: &External,
    referrer: &Referrer,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    buffer.block(
        DirectiveKind::IgnoreOrig,
        format!(
            "{}-struct-impl-nav-backward-one-to-{}",
            obj.as_ident(),
            r_obj.as_ident()
        ),
        |buffer| {
            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-1)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                binary.number
            );
            emit!(
                buffer,
                "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                binary.number,
                r_obj.as_ident(),
                store.name,
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
            );
            emit!(buffer, "vec![store.iter_{}()", r_obj.as_ident());
            emit!(
                buffer,
                ".find(|{}| {}.{} == self.id).unwrap()]",
                r_obj.as_ident(),
                r_obj.as_ident(),
                referrer.referential_attribute.as_ident()
            );
            emit!(buffer, "}}");

            Ok(())
        },
    )
}

fn backward_one_conditional(
    buffer: &mut Buffer,
    obj: &Object,
    r_obj: &Object,
    binary: &Binary,
    store: &External,
    referrer: &Referrer,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    buffer.block(
        DirectiveKind::IgnoreOrig,
        format!(
            "{}-struct-impl-nav-backward-cond-to-{}",
            obj.as_ident(),
            r_obj.as_ident()
        ),
        |buffer| {
            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-1c)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                binary.number
            );
            emit!(
                buffer,
                "pub fn r{}c_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                binary.number,
                r_obj.as_ident(),
                store.name,
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
            );
            emit!(
                buffer,
                "let {} = store.iter_{}()",
                r_obj.as_ident(),
                r_obj.as_ident()
            );
            emit!(
                buffer,
                ".find(|{}| {}.{} == self.id);",
                r_obj.as_ident(),
                r_obj.as_ident(),
                referrer.referential_attribute.as_ident()
            );
            emit!(buffer, "match {} {{", r_obj.as_ident());
            emit!(
                buffer,
                "Some(ref {}) => vec![{}],",
                r_obj.as_ident(),
                r_obj.as_ident()
            );
            emit!(buffer, "None => Vec::new(),");
            emit!(buffer, "}}");
            emit!(buffer, "}}");

            Ok(())
        },
    )
}

fn backward_one_biconditional(
    buffer: &mut Buffer,
    obj: &Object,
    r_obj: &Object,
    binary: &Binary,
    store: &External,
    referrer: &Referrer,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    buffer.block(
        DirectiveKind::IgnoreOrig,
        format!(
            "{}-struct-impl-nav-backward-one-bi-cond-to-{}",
            obj.as_ident(),
            r_obj.as_ident()
        ),
        |buffer| {
            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1c-1c)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                binary.number
            );
            emit!(
                buffer,
                "pub fn r{}c_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                binary.number,
                r_obj.as_ident(),
                store.name,
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
            );
            emit!(
                buffer,
                "let {} = store.iter_{}()",
                r_obj.as_ident(),
                r_obj.as_ident()
            );
            emit!(
                buffer,
                ".find(|{}| {}.{} == Some(self.id));",
                r_obj.as_ident(),
                r_obj.as_ident(),
                referrer.referential_attribute.as_ident()
            );
            emit!(buffer, "match {} {{", r_obj.as_ident());
            emit!(
                buffer,
                "Some(ref {}) => vec![{}],",
                r_obj.as_ident(),
                r_obj.as_ident()
            );
            emit!(buffer, "None => Vec::new(),");
            emit!(buffer, "}}");
            emit!(buffer, "}}");

            Ok(())
        },
    )
}

fn backward_1_m(
    buffer: &mut Buffer,
    obj: &Object,
    r_obj: &Object,
    binary: &Binary,
    store: &External,
    referrer: &Referrer,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    buffer.block(
        DirectiveKind::IgnoreOrig,
        format!(
            "{}-struct-impl-nav-backward-1_M-to-{}",
            obj.as_ident(),
            r_obj.as_ident()
        ),
        |buffer| {
            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-M)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                binary.number
            );
            emit!(
                buffer,
                "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                binary.number,
                r_obj.as_ident(),
                store.name,
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
            );
            emit!(buffer, "store.iter_{}()", r_obj.as_ident());
            emit!(
                buffer,
                ".filter_map(|{}| if {}.{} == self.id {{ Some({}) }} else {{ None }})",
                r_obj.as_ident(),
                r_obj.as_ident(),
                referrer.referential_attribute.as_ident(),
                r_obj.as_ident(),
            );
            emit!(buffer, ".collect()");
            emit!(buffer, "}}");

            Ok(())
        },
    )
}

fn backward_1_mc(
    buffer: &mut Buffer,
    obj: &Object,
    r_obj: &Object,
    binary: &Binary,
    store: &External,
    referrer: &Referrer,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    buffer.block(
        DirectiveKind::IgnoreOrig,
        format!(
            "{}-struct-impl-nav-backward-1_Mc-to-{}",
            obj.as_ident(),
            r_obj.as_ident()
        ),
        |buffer| {
            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-Mc)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                binary.number
            );
            emit!(
                buffer,
                "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                binary.number,
                r_obj.as_ident(),
                store.name,
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
            );
            emit!(buffer, "store.iter_{}()", r_obj.as_ident());
            emit!(
                buffer,
                ".filter_map(|{}| if {}.{} == Some(self.id) {{ Some({}) }} else {{ None }})",
                r_obj.as_ident(),
                r_obj.as_ident(),
                referrer.referential_attribute.as_ident(),
                r_obj.as_ident(),
            );
            emit!(buffer, ".collect()");
            emit!(buffer, "}}");

            Ok(())
        },
    )
}

fn forward_assoc(
    buffer: &mut Buffer,
    obj: &Object,
    referential_attribute: &String,
    number: i64,
    store: &External,
    r_obj: &Object,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    buffer.block(
        DirectiveKind::IgnoreOrig,
        format!(
            "{}-struct-impl-nav-forward-assoc-to-{}",
            obj.as_ident(),
            referential_attribute.as_ident()
        ),
        |buffer| {
            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-*)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                number,
            );
            emit!(
                buffer,
                "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                number,
                r_obj.as_ident(),
                store.name,
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
            );
            emit!(
                buffer,
                "vec![store.exhume_{}(&self.{}).unwrap()]",
                r_obj.as_ident(),
                referential_attribute.as_ident()
            );
            emit!(buffer, "}}");

            Ok(())
        },
    )
}

fn backward_assoc_one(
    buffer: &mut Buffer,
    obj: &Object,
    r_obj: &Object,
    number: i64,
    store: &External,
    referential_attribute: &String,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    buffer.block(
        DirectiveKind::IgnoreOrig,
        format!(
            "{}-struct-impl-nav-backward-assoc-one-to-{}",
            obj.as_ident(),
            r_obj.as_ident()
        ),
        |buffer| {
            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-1)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                number
            );
            emit!(
                buffer,
                "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                number,
                r_obj.as_ident(),
                store.name,
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
            );
            emit!(buffer, "vec![store.iter_{}()", r_obj.as_ident());
            emit!(
                buffer,
                ".find(|{}| {}.{} == self.id).unwrap()]",
                r_obj.as_ident(),
                r_obj.as_ident(),
                referential_attribute.as_ident()
            );
            emit!(buffer, "}}");

            Ok(())
        },
    )
}

fn backward_assoc_one_conditional(
    buffer: &mut Buffer,
    obj: &Object,
    r_obj: &Object,
    number: i64,
    store: &External,
    referential_attribute: &String,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    buffer.block(
        DirectiveKind::IgnoreOrig,
        format!(
            "{}-struct-impl-nav-backward-assoc-one-cond-to-{}",
            obj.as_ident(),
            r_obj.as_ident()
        ),
        |buffer| {
            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-1c)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                number
            );
            emit!(
                buffer,
                "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                number,
                r_obj.as_ident(),
                store.name,
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
            );
            emit!(
                buffer,
                "let {} = store.iter_{}()",
                r_obj.as_ident(),
                r_obj.as_ident()
            );
            emit!(
                buffer,
                ".find(|{}| {}.{} == self.id);",
                r_obj.as_ident(),
                r_obj.as_ident(),
                referential_attribute.as_ident()
            );
            emit!(buffer, "match {} {{", r_obj.as_ident());
            emit!(
                buffer,
                "Some(ref {}) => vec![{}],",
                r_obj.as_ident(),
                r_obj.as_ident()
            );
            emit!(buffer, "None => Vec::new(),");
            emit!(buffer, "}}");
            emit!(buffer, "}}");

            Ok(())
        },
    )
}

fn backward_assoc_many(
    buffer: &mut Buffer,
    obj: &Object,
    r_obj: &Object,
    number: i64,
    store: &External,
    referential_attribute: &String,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    buffer.block(
        DirectiveKind::IgnoreOrig,
        format!(
            "{}-struct-impl-nav-backward-assoc_many-to-{}",
            obj.as_ident(),
            r_obj.as_ident()
        ),
        |buffer| {
            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-M)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                number
            );
            emit!(
                buffer,
                "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                number,
                r_obj.as_ident(),
                store.name,
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
            );
            emit!(buffer, "store.iter_{}()", r_obj.as_ident());
            emit!(
                buffer,
                ".filter_map(|{}| if {}.{} == self.id {{ Some({}) }} else {{ None }})",
                r_obj.as_ident(),
                r_obj.as_ident(),
                referential_attribute.as_ident(),
                r_obj.as_ident(),
            );
            emit!(buffer, ".collect()");
            emit!(buffer, "}}");

            Ok(())
        },
    )
}

fn subtype_to_supertype(
    buffer: &mut Buffer,
    obj: &Object,
    s_obj: &Object,
    number: i64,
    store: &External,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    buffer.block(
        DirectiveKind::IgnoreOrig,
        format!(
            "{}-impl-nav-subtype-to-supertype-{}",
            obj.as_ident(),
            s_obj.as_ident()
        ),
        |buffer| {
            emit!(
                buffer,
                "// Navigate to [`{}`] across R{}(isa)",
                s_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                number
            );
            emit!(
                buffer,
                "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                number,
                s_obj.as_ident(),
                store.name,
                s_obj.as_type(&Ownership::new_borrowed(), woog, domain)
            );
            if inner_object_is_enum(obj, domain) {
                emit!(
                    buffer,
                    "vec![store.exhume_{}(&self.id()).unwrap()]",
                    s_obj.as_ident()
                );
            } else {
                emit!(
                    buffer,
                    "vec![store.exhume_{}(&self.id).unwrap()]",
                    s_obj.as_ident()
                );
            }
            emit!(buffer, "}}");

            Ok(())
        },
    )
}

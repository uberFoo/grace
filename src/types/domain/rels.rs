//! Relationship Specific Stuff
//!
use std::fmt::Write;

use sarzak::{
    mc::{FormatSnafu, Result},
    sarzak::types::{Binary, Cardinality, Conditionality, External, Object, Referrer},
    v2::domain::Domain,
    woog::{store::ObjectStore as WoogStore, types::Ownership},
};
use snafu::prelude::*;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        find_store, get_assoc_referent_from_referrer_sorted, get_binary_referents_sorted,
        get_binary_referrers_sorted, get_subtypes_sorted, local_object_is_enum,
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
    for referrer in get_binary_referrers_sorted!(obj, domain.sarzak()) {
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
            Conditionality::Unconditional(_) => forward(
                buffer, obj, referrer, binary, store, r_obj, config, woog, domain,
            )?,
            Conditionality::Conditional(_) => forward_conditional(
                buffer, obj, referrer, binary, store, r_obj, config, woog, domain,
            )?,
        }
    }

    Ok(())
}

pub(crate) fn generate_binary_referent_rels(
    buffer: &mut Buffer,
    config: &GraceConfig,
    module: &str,
    obj: &Object,
    id: &str,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    // Generate binary relationship navigation for the referent side.
    for referent in get_binary_referents_sorted!(obj, domain.sarzak()) {
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
                Conditionality::Unconditional(_) => backward_one(
                    buffer, obj, r_obj, id, binary, store, referrer, config, woog, domain,
                )?,
                Conditionality::Conditional(_) => match other_cond {
                    Conditionality::Unconditional(_) => backward_one_conditional(
                        buffer, obj, r_obj, id, binary, store, referrer, config, woog, domain,
                    )?,
                    Conditionality::Conditional(_) => backward_one_biconditional(
                        buffer, obj, r_obj, id, binary, &store, referrer, config, woog, domain,
                    )?,
                },
            },
            // It's interesting that there are only really two possibilities, and
            // that neither of them depend on the conditionality of this side.
            Cardinality::Many(_) => match other_cond {
                Conditionality::Unconditional(_) => backward_1_m(
                    buffer, obj, r_obj, id, binary, store, referrer, config, woog, domain,
                )?,
                Conditionality::Conditional(_) => backward_1_mc(
                    buffer, obj, r_obj, id, binary, store, referrer, config, woog, domain,
                )?,
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
        let referents = get_assoc_referent_from_referrer_sorted!(assoc_referrer, domain.sarzak());

        for referent in referents {
            let an_ass = referent.r22_an_associative_referent(domain.sarzak())[0];
            let assoc_obj = referent.r25_object(domain.sarzak())[0];

            let module = if config.is_imported(&assoc_obj.id) {
                config.get_imported(&assoc_obj.id).unwrap().domain.as_str()
            } else {
                module
            };

            // Grab a reference to the store so that we can use it to exhume
            // things.
            let store = find_store(module, woog, domain);
            forward_assoc(
                buffer,
                obj,
                &an_ass.referential_attribute,
                assoc.number,
                store,
                assoc_obj,
                config,
                woog,
                domain,
            )?;
        }
    }

    Ok(())
}

pub(crate) fn generate_assoc_referent_rels(
    buffer: &mut Buffer,
    config: &GraceConfig,
    module: &str,
    obj: &Object,
    id: &str,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    // Generate associative relationship navigation for the referent side.
    for assoc_referent in obj.r25_associative_referent(domain.sarzak()) {
        let an_ass = assoc_referent.r22_an_associative_referent(domain.sarzak())[0];
        let assoc = an_ass.r22_associative(domain.sarzak())[0];
        let referrer = assoc.r21_associative_referrer(domain.sarzak())[0];
        let referential_attribute = &an_ass.referential_attribute;

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
                    id,
                    assoc.number,
                    store,
                    referential_attribute,
                    config,
                    woog,
                    domain,
                )?,
                Conditionality::Unconditional(_) => backward_assoc_one(
                    buffer,
                    obj,
                    r_obj,
                    id,
                    assoc.number,
                    store,
                    referential_attribute,
                    config,
                    woog,
                    domain,
                )?,
            },
            Cardinality::Many(_) => backward_assoc_many(
                buffer,
                obj,
                r_obj,
                id,
                assoc.number,
                store,
                &referential_attribute,
                config,
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
    for subtype in get_subtypes_sorted!(obj, domain.sarzak()) {
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

        subtype_to_supertype(buffer, obj, s_obj, isa.number, store, config, woog, domain)?;
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
    config: &GraceConfig,
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
            let is_uber = config.get_uber_store() && !config.is_imported(&r_obj.id);

            if is_uber {
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<Arc<RwLock<{}>>> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            } else {
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            }

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
    config: &GraceConfig,
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
            let is_uber = config.get_uber_store() && !config.is_imported(&r_obj.id);

            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-*c)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                binary.number,
            );

            if is_uber {
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<Arc<RwLock<{}>>> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            } else {
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            }

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
    id: &str,
    binary: &Binary,
    store: &External,
    referrer: &Referrer,
    config: &GraceConfig,
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
            let is_uber = config.get_uber_store() && !config.is_imported(&r_obj.id);

            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-1)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                binary.number
            );

            if is_uber {
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<Arc<RwLock<{}>>> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            } else {
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            }

            emit!(buffer, "vec![store.iter_{}()", r_obj.as_ident());

            if is_uber {
                emit!(
                    buffer,
                    ".find(|{}| {}.read().unwrap().{} == self.{}).unwrap()]",
                    r_obj.as_ident(),
                    r_obj.as_ident(),
                    referrer.referential_attribute.as_ident(),
                    id
                );
            } else {
                emit!(
                    buffer,
                    ".find(|{}| {}.{} == self.{}).unwrap()]",
                    r_obj.as_ident(),
                    r_obj.as_ident(),
                    referrer.referential_attribute.as_ident(),
                    id
                );
            }

            emit!(buffer, "}}");

            Ok(())
        },
    )
}

fn backward_one_conditional(
    buffer: &mut Buffer,
    obj: &Object,
    r_obj: &Object,
    id: &str,
    binary: &Binary,
    store: &External,
    referrer: &Referrer,
    config: &GraceConfig,
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
            let is_uber = config.get_uber_store() && !config.is_imported(&r_obj.id);

            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-1c)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                binary.number
            );

            if is_uber {
                emit!(
                    buffer,
                    "pub fn r{}c_{}<'a>(&'a self, store: &'a {}) -> Vec<Arc<RwLock<{}>>> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            } else {
                emit!(
                    buffer,
                    "pub fn r{}c_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            }

            emit!(
                buffer,
                "let {} = store.iter_{}()",
                r_obj.as_ident(),
                r_obj.as_ident()
            );

            if is_uber {
                emit!(
                    buffer,
                    ".find(|{}| {}.read().unwrap().{} == self.{});",
                    r_obj.as_ident(),
                    r_obj.as_ident(),
                    referrer.referential_attribute.as_ident(),
                    id
                );
            } else {
                emit!(
                    buffer,
                    ".find(|{}| {}.{} == self.{});",
                    r_obj.as_ident(),
                    r_obj.as_ident(),
                    referrer.referential_attribute.as_ident(),
                    id
                );
            }

            emit!(buffer, "match {} {{", r_obj.as_ident());

            if is_uber {
                emit!(
                    buffer,
                    "Some(ref {}) => vec![{}.clone()],",
                    r_obj.as_ident(),
                    r_obj.as_ident()
                );
            } else {
                emit!(
                    buffer,
                    "Some(ref {}) => vec![{}],",
                    r_obj.as_ident(),
                    r_obj.as_ident()
                );
            }

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
    id: &str,
    binary: &Binary,
    store: &External,
    referrer: &Referrer,
    config: &GraceConfig,
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
            let is_uber = config.get_uber_store() && !config.is_imported(&r_obj.id);

            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1c-1c)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                binary.number
            );

            if is_uber {
                emit!(
                    buffer,
                    "pub fn r{}c_{}<'a>(&'a self, store: &'a {}) -> Vec<Arc<RwLock<{}>>> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            } else {
                emit!(
                    buffer,
                    "pub fn r{}c_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            }

            emit!(
                buffer,
                "let {} = store.iter_{}()",
                r_obj.as_ident(),
                r_obj.as_ident()
            );

            if is_uber {
                emit!(
                    buffer,
                    ".find(|{}| {}.read().unwrap().{} == Some(self.{}));",
                    r_obj.as_ident(),
                    r_obj.as_ident(),
                    referrer.referential_attribute.as_ident(),
                    id
                );
            } else {
                emit!(
                    buffer,
                    ".find(|{}| {}.{} == Some(self.{}));",
                    r_obj.as_ident(),
                    r_obj.as_ident(),
                    referrer.referential_attribute.as_ident(),
                    id
                );
            }

            emit!(buffer, "match {} {{", r_obj.as_ident());

            if is_uber {
                emit!(
                    buffer,
                    "Some(ref {}) => vec![{}.clone()],",
                    r_obj.as_ident(),
                    r_obj.as_ident()
                );
            } else {
                emit!(
                    buffer,
                    "Some(ref {}) => vec![{}],",
                    r_obj.as_ident(),
                    r_obj.as_ident()
                );
            }

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
    id: &str,
    binary: &Binary,
    store: &External,
    referrer: &Referrer,
    config: &GraceConfig,
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
            let is_uber = config.get_uber_store() && !config.is_imported(&r_obj.id);

            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-M)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                binary.number
            );

            if is_uber {
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<Arc<RwLock<{}>>> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            } else {
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            }

            emit!(buffer, "store.iter_{}()", r_obj.as_ident());
            emit!(buffer, ".filter_map(|{}| {{", r_obj.as_ident(),);

            if is_uber {
                emit!(
                    buffer,
                    "if {}.read().unwrap().{} == self.{} {{ Some({}) }} else {{ None }}",
                    r_obj.as_ident(),
                    referrer.referential_attribute.as_ident(),
                    id,
                    r_obj.as_ident(),
                );
            } else {
                emit!(
                    buffer,
                    "if {}.{} == self.{} {{ Some({}) }} else {{ None }}",
                    r_obj.as_ident(),
                    referrer.referential_attribute.as_ident(),
                    id,
                    r_obj.as_ident(),
                );
            }
            emit!(buffer, "}})");
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
    id: &str,
    binary: &Binary,
    store: &External,
    referrer: &Referrer,
    config: &GraceConfig,
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
            let is_uber = config.get_uber_store() && !config.is_imported(&r_obj.id);

            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-Mc)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                binary.number
            );

            if is_uber {
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<Arc<RwLock<{}>>> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            } else {
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            }

            emit!(buffer, "store.iter_{}()", r_obj.as_ident());

            if is_uber {
            emit!(
                buffer,
                ".filter_map(|{}| if {}.read().unwrap().{} == Some(self.{}) {{ Some({}) }} else {{ None }})",
                r_obj.as_ident(),
                r_obj.as_ident(),
                referrer.referential_attribute.as_ident(),
                id,
                r_obj.as_ident(),
            );
        } else {
            emit!(
                buffer,
                ".filter_map(|{}| if {}.{} == Some(self.{}) {{ Some({}) }} else {{ None }})",
                r_obj.as_ident(),
                r_obj.as_ident(),
                referrer.referential_attribute.as_ident(),
                id,
                r_obj.as_ident(),
            );
        }

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
    config: &GraceConfig,
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
            let is_uber = config.get_uber_store() && !config.is_imported(&r_obj.id);

            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-*)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                number,
            );

            if is_uber {
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<Arc<RwLock<{}>>> {{",
                    number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            } else {
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            }

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
    id: &str,
    number: i64,
    store: &External,
    referential_attribute: &String,
    config: &GraceConfig,
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
            let is_uber = config.get_uber_store() && !config.is_imported(&r_obj.id);

            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-1)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                number
            );

            if is_uber {
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<Arc<RwLock<{}>>> {{",
                    number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            } else {
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            }

            emit!(buffer, "vec![store.iter_{}()", r_obj.as_ident());

            let lhs = if is_uber {
                format!(
                    "{}.read().unwrap().{}",
                    r_obj.as_ident(),
                    referential_attribute.as_ident()
                )
            } else {
                format!(
                    "{}.read().unwrap().{}",
                    r_obj.as_ident(),
                    referential_attribute.as_ident()
                )
            };

            emit!(
                buffer,
                ".find(|{}| {} == self.{}).unwrap()]",
                r_obj.as_ident(),
                lhs,
                id
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
    id: &str,
    number: i64,
    store: &External,
    referential_attribute: &String,
    config: &GraceConfig,
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
            let is_uber = config.get_uber_store() && !config.is_imported(&r_obj.id);

            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-1c)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                number
            );

            if is_uber {
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<Arc<RwLock<{}>>> {{",
                    number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            } else {
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            }

            emit!(
                buffer,
                "let {} = store.iter_{}()",
                r_obj.as_ident(),
                r_obj.as_ident()
            );
            emit!(
                buffer,
                ".find(|{}| {}.{} == self.{});",
                r_obj.as_ident(),
                r_obj.as_ident(),
                referential_attribute.as_ident(),
                id
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
    id: &str,
    number: i64,
    store: &External,
    referential_attribute: &String,
    config: &GraceConfig,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    buffer.block(
        DirectiveKind::IgnoreOrig,
        format!(
            "{}-struct-impl-nav-backward-assoc-many-to-{}",
            obj.as_ident(),
            r_obj.as_ident()
        ),
        |buffer| {
            let is_uber = config.get_uber_store() && !config.is_imported(&r_obj.id);

            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-M)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                number
            );

            if is_uber {
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<Arc<RwLock<{}>>> {{",
                    number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            } else {
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            }

            emit!(buffer, "store.iter_{}()", r_obj.as_ident());

            let lhs = if is_uber {
                format!(
                    "{}.read().unwrap().{}",
                    r_obj.as_ident(),
                    referential_attribute.as_ident()
                )
            } else {
                format!("{}.{}", r_obj.as_ident(), referential_attribute.as_ident())
            };

            emit!(
                buffer,
                ".filter_map(|{}| if {} == self.{} {{ Some({}) }} else {{ None }})",
                r_obj.as_ident(),
                lhs,
                id,
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
    config: &GraceConfig,
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
            let is_uber = config.get_uber_store() && !config.is_imported(&s_obj.id);

            emit!(
                buffer,
                "// Navigate to [`{}`] across R{}(isa)",
                s_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                number
            );

            if is_uber {
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<Arc<RwLock<{}>>> {{",
                    number,
                    s_obj.as_ident(),
                    store.name,
                    s_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            } else {
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    number,
                    s_obj.as_ident(),
                    store.name,
                    s_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            }

            if local_object_is_enum(obj, config, domain) {
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

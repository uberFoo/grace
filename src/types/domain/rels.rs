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
        local_object_is_hybrid,
        render::{RenderIdent, RenderType},
    },
    options::{GraceConfig, UberStoreOptions},
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
    let obj_ident = r_obj.as_ident();

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
            let is_uber = config.is_uber_store() && !config.is_imported(&r_obj.id);

            if is_uber {
                let store_type = get_value_wrapper(is_uber, config, r_obj, woog, domain);
                if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap() {
                    emit!(
                        buffer,
                        "pub async fn r{}_{obj_ident}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        binary.number,
                        store.name
                    );
                } else {
                    emit!(
                        buffer,
                        "pub fn r{}_{obj_ident}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        binary.number,
                        store.name
                    );
                }
                emit!(
                    buffer,
                    "span!(\"r{}_{obj_ident}\");",
                    binary.number,
                );
            } else {
                emit!(
                    buffer,
                    "pub fn r{}_{obj_ident}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    binary.number,
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
            }

            if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap() {
                emit!(
                    buffer,
                    "vec![store.exhume_{obj_ident}(&self.{}).await.unwrap()]",
                    referrer.referential_attribute.as_ident()
                );
            } else {
                emit!(
                    buffer,
                    "vec![store.exhume_{obj_ident}(&self.{}).unwrap()]",
                    referrer.referential_attribute.as_ident()
                );
            }
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
            let is_uber = config.is_uber_store() && !config.is_imported(&r_obj.id);

            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-*c)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                binary.number,
            );

            if is_uber {
                let store_type = get_value_wrapper(is_uber, config, r_obj, woog, domain);
                if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap() {
                    emit!(
                        buffer,
                        "pub async fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        binary.number,
                        r_obj.as_ident(),
                        store.name
                    );
                } else {
                    emit!(
                        buffer,
                        "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        binary.number,
                        r_obj.as_ident(),
                        store.name
                    );
                }
                emit!(
                    buffer,
                    "span!(\"r{}_{}\");",
                    binary.number,
                    r_obj.as_ident()
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

            if is_uber {
                if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap() {
                    emit!(
                        buffer,
                        "Some(ref {}) => vec![store.exhume_{}({}).await.unwrap()],",
                        referrer.referential_attribute.as_ident(),
                        r_obj.as_ident(),
                        referrer.referential_attribute.as_ident()
                    );
                } else {
                    emit!(
                        buffer,
                        "Some(ref {}) => vec![store.exhume_{}({}).unwrap()],",
                        referrer.referential_attribute.as_ident(),
                        r_obj.as_ident(),
                        referrer.referential_attribute.as_ident()
                    );
                }
            } else {
                emit!(
                    buffer,
                    "Some(ref {}) => vec![store.exhume_{}({}).unwrap()],",
                    referrer.referential_attribute.as_ident(),
                    r_obj.as_ident(),
                    referrer.referential_attribute.as_ident()
                );
            }

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
    let obj_ident = r_obj.as_ident();

    buffer.block(
        DirectiveKind::IgnoreOrig,
        format!(
            "{}-struct-impl-nav-backward-one-to-{obj_ident}",
            obj.as_ident(),
            ),
        |buffer| {
            let is_uber = config.is_uber_store() && !config.is_imported(&r_obj.id);
            let rhs = {
                let cond = referrer.r11_conditionality(domain.sarzak())[0];
                if let Conditionality::Conditional(_) = cond {
                    format!("Some(self.{id})")
                } else {
                    format!("self.{id}")
                }
            };

            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-1)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                binary.number
            );

            if is_uber {
                let store_type = get_value_wrapper(is_uber, config, r_obj, woog, domain);
                let (read, _write) = get_uber_read_write(config);
                if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap() {
                    emit!(
                        buffer,
                        "pub async fn r{}_{obj_ident}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        binary.number,
                        store.name
                    );
                    emit!(
                        buffer,
                        "span!(\"r{}_{obj_ident}\");",
                        binary.number,
                    );

                    emit!(buffer, "let mut result = Vec::new();");
                    emit!(
                        buffer,
                        "for {obj_ident} in store.iter_{obj_ident}().await {{"
                    );
                    emit!(
                        buffer,
                        "if {obj_ident}.read().await.{} == {rhs} {{",
                        referrer.referential_attribute.as_ident()
                    );
                    emit!(buffer, "result.push({obj_ident});");
                    emit!(buffer, "}}");
                    emit!(buffer, "}}");
                    emit!(buffer, "result");
                } else {
                    emit!(
                        buffer,
                        "pub fn r{}_{obj_ident}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        binary.number,
                        store.name
                    );
                    emit!(
                        buffer,
                        "span!(\"r{}_{obj_ident}\");",
                        binary.number,
                    );
                    emit!(buffer, "vec![store.iter_{obj_ident}()");
                    emit!(
                        buffer,
                        ".find(|{0}| {0}{read}.{1} == {rhs}).unwrap()]",
                        obj_ident,
                        referrer.referential_attribute.as_ident(),
                    );
            }

            } else {
                emit!(
                    buffer,
                    "pub fn r{}_{obj_ident}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    binary.number,
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
                emit!(buffer, "vec![store.iter_{}()", r_obj.as_ident());
                emit!(
                    buffer,
                    ".find(|{0}| {0}.{1} == {rhs}).unwrap()]",
                    obj_ident,
                    referrer.referential_attribute.as_ident(),
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
    let obj_ident = r_obj.as_ident();

    buffer.block(
        DirectiveKind::IgnoreOrig,
        format!(
            "{}-struct-impl-nav-backward-cond-to-{obj_ident}",
            obj.as_ident(),
            ),
        |buffer| {
            let is_uber = config.is_uber_store() && !config.is_imported(&r_obj.id);

            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-1c)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                binary.number
            );

            if is_uber {
                let store_type = get_value_wrapper(is_uber, config, r_obj, woog, domain);
                if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap() {
                    emit!(
                        buffer,
                        "pub async fn r{}c_{obj_ident}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        binary.number,
                        store.name
                    );
                    emit!(
                        buffer,
                        "span!(\"r{}_{obj_ident}\");",
                        binary.number,
                    );
                    emit!(buffer, "let mut result = Vec::new();");
                    emit!(buffer, "for {obj_ident} in store.iter_{obj_ident}().await {{");
                    emit!(
                        buffer,
                        "if {obj_ident}.read().await.{} == self.{id} {{",
                        referrer.referential_attribute.as_ident()
                    );
                    emit!(buffer, "result.push({obj_ident})");
                    emit!(buffer, "}}");
                    emit!(buffer, "}}");
                    emit!(buffer, "result");
                } else {
                    emit!(
                        buffer,
                        "pub fn r{}c_{obj_ident}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        binary.number,
                        store.name
                    );
                    emit!(
                        buffer,
                        "span!(\"r{}_{obj_ident}\");",
                        binary.number,
                    );
                    emit!(
                        buffer,
                        "let {obj_ident} = store.iter_{obj_ident}()"
                    );
                    let (read, _write) = get_uber_read_write(config);
                    emit!(
                        buffer,
                        ".find(|{obj_ident}| {obj_ident}{read}.{} == self.{id});",
                        referrer.referential_attribute.as_ident(),
                    );
                    emit!(buffer, "match {obj_ident} {{");
                    emit!(
                        buffer,
                        "Some(ref {obj_ident}) => vec![{obj_ident}.clone()],"
                    );
                    emit!(buffer, "None => Vec::new(),");
                    emit!(buffer, "}}");
                }
            } else {
                emit!(
                    buffer,
                    "pub fn r{}c_{obj_ident}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    binary.number,
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
                emit!(
                    buffer,
                    "let {obj_ident} = store.iter_{obj_ident}()"
                );
                emit!(
                    buffer,
                    ".find(|{obj_ident}| {obj_ident}.{} == self.{id});",
                    referrer.referential_attribute.as_ident(),
                );
                emit!(buffer, "match {obj_ident} {{");
                emit!(
                    buffer,
                    "Some(ref {obj_ident}) => vec![{obj_ident}],"
                );
                emit!(buffer, "None => Vec::new(),");
                emit!(buffer, "}}");
            }

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
    let obj_ident = r_obj.as_ident();

    buffer.block(
        DirectiveKind::IgnoreOrig,
        format!(
            "{}-struct-impl-nav-backward-one-bi-cond-to-{obj_ident}",
            obj.as_ident(),
        ),
        |buffer| {
            let is_uber = config.is_uber_store() && !config.is_imported(&r_obj.id);

            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1c-1c)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                binary.number
            );

            if is_uber {
                let store_type = get_value_wrapper(is_uber, config, r_obj, woog, domain);
                let (read, _write) = get_uber_read_write(config);
                if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap() {
                    emit!(
                        buffer,
                        "pub async fn r{}c_{obj_ident}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        binary.number,
                        store.name
                    );
                    emit!(
                        buffer,
                        "span!(\"r{}_{obj_ident}\");",
                        binary.number,
                    );
                    emit!(buffer, "let mut result = Vec::new();");
                    emit!(
                        buffer,
                        "for {obj_ident} in store.iter_{obj_ident}().await {{"
                    );
                    emit!(
                        buffer,
                        "if {obj_ident}.read().await.{} == Some(self.{id}) {{",
                        referrer.referential_attribute.as_ident(),
                    );
                    emit!(buffer, "result.push({obj_ident}.clone());");
                    emit!(buffer, "}}");
                    emit!(buffer, "}}");
                    emit!(buffer, "result");
                } else {
                    emit!(
                        buffer,
                        "pub fn r{}c_{obj_ident}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        binary.number,
                        store.name
                    );
                    emit!(
                        buffer,
                        "span!(\"r{}_{obj_ident}\");",
                        binary.number,
                    );
                    emit!(
                        buffer,
                        "let {obj_ident} = store.iter_{obj_ident}()"
                    );
                    emit!(
                        buffer,
                        ".find(|{obj_ident}| {obj_ident}{read}.{} == Some(self.{id}));",
                        referrer.referential_attribute.as_ident(),
                    );
                    emit!(buffer, "match {obj_ident} {{");
                    emit!(
                        buffer,
                        "Some(ref {obj_ident}) => vec![{obj_ident}.clone()],"
                    );
                    emit!(buffer, "None => Vec::new(),");
                    emit!(buffer, "}}");
                }

            } else {
                emit!(
                    buffer,
                    "pub fn r{}c_{obj_ident}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    binary.number,
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
                emit!(
                    buffer,
                    "let {obj_ident} = store.iter_{obj_ident}()"
                );
                emit!(
                    buffer,
                    ".find(|{obj_ident}| {obj_ident}.{} == Some(self.{id}));",
                    referrer.referential_attribute.as_ident(),
                );
                emit!(buffer, "match {} {{", r_obj.as_ident());
                emit!(
                    buffer,
                    "Some(ref {obj_ident}) => vec![{obj_ident}],"
                );
                emit!(buffer, "None => Vec::new(),");
                emit!(buffer, "}}");
            }


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
    let obj_ident = r_obj.as_ident();

    buffer.block(
        DirectiveKind::IgnoreOrig,
        format!(
            "{}-struct-impl-nav-backward-1_M-to-{obj_ident}",
            obj.as_ident(),
            ),
        |buffer| {
            let is_uber = config.is_uber_store() && !config.is_imported(&r_obj.id);

            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-M)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                binary.number
            );

            if is_uber {
                let store_type = get_value_wrapper(is_uber, config, r_obj, woog, domain);
                let (read, _write) = get_uber_read_write(config);
                if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap() {
                    emit!(
                        buffer,
                        "pub async fn r{}_{obj_ident}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        binary.number,
                        store.name
                    );
                    emit!(buffer, "span!(\"r{}_{obj_ident}\");", binary.number,);
                    emit!(buffer, "let mut result = Vec::new();");
                    emit!(
                        buffer,
                        "for {obj_ident} in store.iter_{obj_ident}().await {{"
                    );
                    emit!(
                        buffer,
                        "if {obj_ident}.read().await.{} == self.{id} {{",
                        referrer.referential_attribute.as_ident()
                    );
                    emit!(buffer, "result.push({obj_ident})");
                    emit!(buffer, "}}");
                    emit!(buffer, "}}");
                    emit!(buffer, "result");
                } else {
                    emit!(
                        buffer,
                        "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        binary.number,
                        r_obj.as_ident(),
                        store.name
                    );
                    emit!(
                        buffer,
                        "span!(\"r{}_{}\");",
                        binary.number,
                        r_obj.as_ident()
                    );
                    emit!(buffer, "store.iter_{obj_ident}()");
                    emit!(buffer, ".filter(|{obj_ident}| {{");
                    emit!(
                        buffer,
                        "{obj_ident}{read}.{} == self.{id}",
                        referrer.referential_attribute.as_ident(),
                    );
                    emit!(buffer, "}})");
                    emit!(buffer, ".collect()");
                }
            } else {
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
                emit!(buffer, "store.iter_{}()", r_obj.as_ident());
                emit!(buffer, ".filter(|{}| {{", r_obj.as_ident(),);
                emit!(
                    buffer,
                    "{}.{} == self.{}",
                    r_obj.as_ident(),
                    referrer.referential_attribute.as_ident(),
                    id,
                );
                emit!(buffer, "}})");
                emit!(buffer, ".collect()");
            }

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
    let obj_ident = r_obj.as_ident();
    let ref_ident = referrer.referential_attribute.as_ident();

    buffer.block(
        DirectiveKind::IgnoreOrig,
        format!(
            "{}-struct-impl-nav-backward-1_Mc-to-{obj_ident}",
            obj.as_ident(),
            ),
        |buffer| {
            let is_uber = config.is_uber_store() && !config.is_imported(&r_obj.id);

            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-Mc)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                binary.number
            );

            if is_uber {
                if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap() {
                } else {
                }

            } else {
            }

            if is_uber {
                let store_type = get_value_wrapper(is_uber, config, r_obj, woog, domain);
                let (read, _write) = get_uber_read_write(config);

                if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap() {
                    emit!(
                        buffer,
                        "pub async fn r{}_{obj_ident}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        binary.number,
                        store.name
                    );
                    emit!(
                        buffer,
                        "use futures::stream::{{self, StreamExt}};"
                    );
                    emit!(
                        buffer,
                        "span!(\"r{}_{obj_ident}\");",
                        binary.number,
                    );
                    emit!(
                        buffer,
                        "stream::iter(store.iter_{obj_ident}().await.collect::<Vec<Arc<RwLock<{}>>>>())",
                        r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    );
                    emit!(
                        buffer,
                        ".filter(|{obj_ident}: Arc<RwLock<{}>>| async move {{",
                        r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    );
                    emit!(buffer, "{obj_ident}.read().await.{ref_ident} == Some(self.{id}).collect().await}})",
                    );
                } else {
                    emit!(
                        buffer,
                        "pub fn r{}_{obj_ident}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        binary.number,
                        store.name
                    );
                    emit!(
                        buffer,
                        "span!(\"r{}_{obj_ident}\");",
                        binary.number,
                    );
                    emit!(buffer, "store.iter_{}()", r_obj.as_ident());
                    emit!(
                        buffer,
                        ".filter(|{obj_ident}| {obj_ident}{read}.{ref_ident} == Some(self.{id})).collect()",
                    );
                }
            } else {
                emit!(
                    buffer,
                    "pub fn r{}_{obj_ident}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    binary.number,
                    store.name,
                    r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
                emit!(buffer, "store.iter_{}()", r_obj.as_ident());
                emit!(
                    buffer,
                    ".filter(|{obj_ident}| {obj_ident}.{ref_ident} == Some(self.{id})).collect()",
                );
            }
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
            let is_uber = config.is_uber_store() && !config.is_imported(&r_obj.id);

            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-*)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                number,
            );

            if is_uber {
                let store_type = get_value_wrapper(is_uber, config, r_obj, woog, domain);
                if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap() {
                    emit!(
                        buffer,
                        "pub async fn r{number}_{}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        r_obj.as_ident(),
                        store.name
                    );
                } else {
                    emit!(
                        buffer,
                        "pub fn r{number}_{}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        r_obj.as_ident(),
                        store.name
                    );
                }

                emit!(buffer, "span!(\"\"r{number}_{});\"\"", r_obj.as_ident());
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
            let is_uber = config.is_uber_store() && !config.is_imported(&r_obj.id);

            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-1)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                number
            );

            if is_uber {
                let store_type = get_value_wrapper(is_uber, config, r_obj, woog, domain);
                if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap() {
                    emit!(
                        buffer,
                        "pub async fn r{number}_{}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        r_obj.as_ident(),
                        store.name
                    );
                } else {
                    emit!(
                        buffer,
                        "pub fn r{number}_{}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        r_obj.as_ident(),
                        store.name
                    );
                }

                emit!(buffer, "span!(\"r{number}_{});\"", r_obj.as_ident());
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
                let (read, _write) = get_uber_read_write(config);
                format!(
                    "{}{read}.{}",
                    r_obj.as_ident(),
                    referential_attribute.as_ident()
                )
            } else {
                format!("{}.{}", r_obj.as_ident(), referential_attribute.as_ident())
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
            let is_uber = config.is_uber_store() && !config.is_imported(&r_obj.id);

            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-1c)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                number
            );

            if is_uber {
                let store_type = get_value_wrapper(is_uber, config, r_obj, woog, domain);
                if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap() {
                    emit!(
                        buffer,
                        "pub async fn r{number}_{}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        r_obj.as_ident(),
                        store.name
                    );
                } else {
                    emit!(
                        buffer,
                        "pub fn r{number}_{}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        r_obj.as_ident(),
                        store.name
                    );
                }

                emit!(buffer, "span!(\"r{number}_{});\"", r_obj.as_ident());
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
            let is_uber = config.is_uber_store() && !config.is_imported(&r_obj.id);

            emit!(
                buffer,
                "/// Navigate to [`{}`] across R{}(1-M)",
                r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                number
            );

            if is_uber {
                let store_type = get_value_wrapper(is_uber, config, r_obj, woog, domain);
                if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap() {
                    emit!(
                        buffer,
                        "pub async fn r{number}_{}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        r_obj.as_ident(),
                        store.name
                    );
                } else {
                    emit!(
                        buffer,
                        "pub fn r{number}_{}<'a>(&'a self, store: &'a {}) -> Vec<{store_type}> {{",
                        r_obj.as_ident(),
                        store.name
                    );
                }

                emit!(buffer, "span!(\"r{number}_{});\"", r_obj.as_ident());
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
                let (read, _write) = get_uber_read_write(config);
                format!(
                    "{}{read}.{}",
                    r_obj.as_ident(),
                    referential_attribute.as_ident()
                )
            } else {
                format!("{}.{}", r_obj.as_ident(), referential_attribute.as_ident())
            };

            emit!(
                buffer,
                ".filter(|{}| {lhs} == self.{id})",
                r_obj.as_ident()
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
    let obj_ident = obj.as_ident();
    let obj_type = obj.as_type(&Ownership::new_borrowed(), woog, domain);
    let s_obj_ident = s_obj.as_ident();
    let s_obj_type = s_obj.as_type(&Ownership::new_borrowed(), woog, domain);
    let store_name = &store.name;

    buffer.block(
        DirectiveKind::IgnoreOrig,
        format!(
            "{obj_ident}-impl-nav-subtype-to-supertype-{s_obj_ident}"
        ),
        |buffer| {
            let is_uber = config.is_uber_store() && !config.is_imported(&s_obj.id);
            let is_hybrid = local_object_is_hybrid(s_obj, config, domain);

            emit!(
                buffer,
                "// Navigate to [`{s_obj_type}`] across R{number}(isa)"
            );

            if is_uber {
                let store_type = get_value_wrapper(is_uber, config, s_obj, woog, domain);
                if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap() {
                    emit!(
                        buffer,
                        "pub async fn r{number}_{s_obj_ident}<'a>(&'a self, store: &'a {store_name}) -> Vec<{store_type}> {{"
                    );
                } else {
                    emit!(
                        buffer,
                        "pub fn r{number}_{s_obj_ident}<'a>(&'a self, store: &'a {store_name}) -> Vec<{store_type}> {{"
                    );
                }

                emit!(buffer, "span!(\"r{number}_{s_obj_ident}\");");
            } else {
                emit!(
                    buffer,
                    "pub fn r{number}_{s_obj_ident}<'a>(&'a self, store: &'a {store_name}) -> Vec<&{s_obj_type}> {{"
                );
            }

            let id = if local_object_is_enum(obj, config, domain) {
                "id()"
            } else {
                "id"
            };
            // I wish I'd left myself a note about why hybrid is special...
            // Oh! It's got the `subtype` attribute/field.
            if is_hybrid {
                if is_uber {
                    let (read, _write) = get_uber_read_write(config);
                    if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap() {
                        emit!(buffer, "let mut result = Vec::new();");
                        emit!(
                            buffer,
                            "for {s_obj_ident} in store.iter_{s_obj_ident}().await {{"
                        );
                        emit!(
                            buffer,
                            "if let {s_obj_type}Enum::{obj_type}(id) = {s_obj_ident}{read}.subtype {{",
                          );
                        emit!(buffer, "result.push({s_obj_ident}.clone());");
                        emit!(buffer, "}}");
                        emit!(buffer, "}}");
                        emit!(buffer, "result");
                    } else {
                        emit!(buffer, "vec![store.iter_{s_obj_ident}().find(|{s_obj_ident}| {{");
                        emit!(buffer, "if let {s_obj_type}Enum::{obj_type}(id) = {s_obj_ident}{read}.subtype {{");
                        emit!(buffer, "id == self.{id} }} else {{ false }} }}).unwrap()]");
                    }
                } else {
                    emit!(buffer, "vec![store.iter_{s_obj_ident}().find(|{s_obj_ident}| {{");
                    emit!(buffer, "if let {s_obj_type}Enum::{obj_type}(id) = {s_obj_ident}.subtype {{");
                    emit!(buffer, "id == self.{id} }} else {{ false }} }}).unwrap()]");
                }
            } else {
                if is_uber {
                    if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap() {
                        emit!(
                            buffer,
                            "vec![store.exhume_{}(&self.{id}).await.unwrap()]",
                            s_obj.as_ident()
                        );
                    } else {
                        emit!(
                            buffer,
                            "vec![store.exhume_{}(&self.{id}).unwrap()]",
                            s_obj.as_ident()
                        );
                    }
                } else {
                    emit!(
                        buffer,
                        "vec![store.exhume_{}(&self.{id}).unwrap()]",
                        s_obj.as_ident()
                    );
                }
            }
            emit!(buffer, "}}");

            Ok(())
        },
    )
}

fn get_uber_read_write(config: &GraceConfig) -> (&str, &str) {
    use UberStoreOptions::*;
    let write = match config.get_uber_store().unwrap() {
        Disabled => unreachable!(),
        AsyncRwLock => ".write().await",
        NDRwLock => ".write().unwrap()",
        Single => ".borrow_mut()",
        StdRwLock => ".write().unwrap()",
        StdMutex => ".lock().unwrap()",
        ParkingLotRwLock => ".write()",
        ParkingLotMutex => ".lock()",
    };
    let read = match config.get_uber_store().unwrap() {
        Disabled => unreachable!(),
        AsyncRwLock => ".read().await",
        NDRwLock => ".read().unwrap()",
        Single => ".borrow()",
        StdRwLock => ".read().unwrap()",
        StdMutex => ".lock().unwrap()",
        ParkingLotRwLock => ".read()",
        ParkingLotMutex => ".lock()",
    };

    (read, write)
}

fn get_value_wrapper(
    is_uber: bool,
    config: &GraceConfig,
    obj: &Object,
    woog: &WoogStore,
    domain: &Domain,
) -> String {
    if is_uber {
        use UberStoreOptions::*;
        match config.get_uber_store().unwrap() {
            Disabled => unreachable!(),
            Single => format!(
                "Rc<RefCell<{}>>",
                obj.as_type(&Ownership::new_borrowed(), woog, domain)
            ),
            StdRwLock | ParkingLotRwLock | AsyncRwLock | NDRwLock => format!(
                "Arc<RwLock<{}>>",
                obj.as_type(&Ownership::new_borrowed(), woog, domain)
            ),
            StdMutex | ParkingLotMutex => format!(
                "Arc<Mutex<{}>>",
                obj.as_type(&Ownership::new_borrowed(), woog, domain)
            ),
        }
    } else {
        format!("{}", obj.as_type(&Ownership::new_borrowed(), woog, domain))
    }
}

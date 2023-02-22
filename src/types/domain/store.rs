//! Generate ObjectStore for use in sarzak Domain
//!
use std::{collections::HashMap, fmt::Write};

use sarzak::{
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::{
        macros::{
            sarzak_get_many_r_subs_across_r27, sarzak_get_one_obj_across_r15,
            sarzak_get_one_r_isa_across_r13, sarzak_maybe_get_many_r_sups_across_r14,
        },
        types::{Object, Subtype, Supertype},
    },
    v1::domain::Domain,
    woog::{
        store::ObjectStore as WoogStore,
        types::{Mutability, BORROWED, MUTABLE},
    },
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        generator::{CodeWriter, FileGenerator, GenerationAction},
        object_is_singleton, object_is_supertype,
        render::{RenderConst, RenderIdent, RenderType},
    },
    options::GraceConfig,
    types::ObjectStoreDefinition,
};

pub(crate) struct DomainStoreBuilder {
    definition: Option<Box<dyn ObjectStoreDefinition>>,
}

impl DomainStoreBuilder {
    pub(crate) fn new() -> Self {
        Self { definition: None }
    }

    pub(crate) fn definition(mut self, definition: Box<dyn ObjectStoreDefinition>) -> Self {
        self.definition = Some(definition);

        self
    }

    pub(crate) fn build(&mut self) -> Result<Box<DomainStoreGenerator>> {
        ensure!(
            self.definition.is_some(),
            CompilerSnafu {
                description: "DomainStoreBuilder::build called before definition".to_owned()
            }
        );

        Ok(Box::new(DomainStoreGenerator {
            definition: self.definition.take().unwrap(),
        }))
    }
}

pub(crate) struct DomainStoreGenerator {
    definition: Box<dyn ObjectStoreDefinition>,
}

impl FileGenerator for DomainStoreGenerator {
    fn generate(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        woog: &Option<&mut WoogStore>,
        imports: &Option<&HashMap<String, Domain>>,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<GenerationAction> {
        // Output the domain/module documentation/description
        emit!(buffer, "//! {} Object Store", module);
        emit!(buffer, "//!");
        emit!(
            buffer,
            "//! The ObjectStore contains instances of objects in the domain."
        );
        emit!(
            buffer,
            "//! The instances are stored in a hash map, keyed by the object's UUID."
        );
        emit!(
            buffer,
            "//! This is used during code generation, and probably not useful elsewhere."
        );
        let mut objects: Vec<(&Uuid, &Object)> = domain.sarzak().iter_object().collect();
        objects.sort_by(|a, b| a.1.name.cmp(&b.1.name));
        let objects = objects
            .iter()
            .filter(|(id, obj)| {
                // We have this odd construction because a supertype may actually be a singleton.
                object_is_supertype(obj, domain.sarzak())
                    || !object_is_singleton(obj, domain.sarzak())
                // Don't include imported objects
                && !config.is_imported(*id)
            })
            .collect::<Vec<_>>();

        // We don't want this to be edited -- there's no reason.
        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-object-store-file", module),
            |buffer| {
                emit!(buffer, "//!");
                emit!(buffer, "//! # Contents:");
                emit!(buffer, "//!");
                for (_, obj) in &objects {
                    emit!(
                        buffer,
                        "//! * [`{}`]",
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );
                }

                self.definition
                    .write_code(config, domain, woog, imports, module, obj_id, buffer)?;

                Ok(())
            },
        )?;

        Ok(GenerationAction::Write)
    }
}

pub(crate) struct DomainStore;

impl DomainStore {
    pub(crate) fn new() -> Box<dyn ObjectStoreDefinition> {
        Box::new(Self)
    }

    fn generate_store(
        &self,
        buffer: &mut Buffer,
        objects: &Vec<&(&Uuid, &Object)>,
        module: &str,
        domain: &Domain,
    ) -> Result<()> {
        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-object-store-methods", module),
            |buffer| {
                for (_, obj) in objects {
                    emit!(
                        buffer,
                        "/// Inter [`{}`] into the store.",
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );
                    emit!(buffer, "///");
                    emit!(
                        buffer,
                        "pub fn inter_{}(&mut self, {}: {}) {{",
                        obj.as_ident(),
                        obj.as_ident(),
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );

                    if object_is_supertype(obj, domain.sarzak()) {
                        emit!(
                            buffer,
                            "self.{}.insert({}.id(), {});",
                            obj.as_ident(),
                            obj.as_ident(),
                            obj.as_ident()
                        );
                    } else {
                        emit!(
                            buffer,
                            "self.{}.insert({}.id, {});",
                            obj.as_ident(),
                            obj.as_ident(),
                            obj.as_ident()
                        );
                    }
                    emit!(buffer, "}}");
                    emit!(buffer, "");
                    emit!(
                        buffer,
                        "/// Exhume [`{}`] from the store.",
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );
                    emit!(buffer, "///");
                    emit!(
                        buffer,
                        "pub fn exhume_{}(&self, id: &Uuid) -> Option<&{}> {{",
                        obj.as_ident(),
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );
                    emit!(buffer, "self.{}.get(id)", obj.as_ident());
                    emit!(buffer, "}}");
                    emit!(
                        buffer,
                        "/// Exhume [`{}`] from the store — mutably.",
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );
                    emit!(buffer, "///");
                    emit!(
                        buffer,
                        "pub fn exhume_{}_mut(&mut self, id: &Uuid) -> Option<&{}> {{",
                        obj.as_ident(),
                        obj.as_type(&Mutability::Mutable(MUTABLE), domain.sarzak())
                    );
                    emit!(buffer, "self.{}.get_mut(id)", obj.as_ident());
                    emit!(buffer, "}}");
                    emit!(
                        buffer,
                        "/// Get an iterator over the internal `HashMap<&Uuid, {}>`.",
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );
                    emit!(buffer, "///");
                    emit!(
                        buffer,
                        "pub fn iter_{}(&self) -> impl Iterator<Item = (&Uuid, &{})> {{",
                        obj.as_ident(),
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );
                    emit!(buffer, "self.{}.iter()", obj.as_ident());
                    emit!(buffer, "}}");
                }

                Ok(())
            },
        )?;

        Ok(())
    }

    fn generate_store_persistence(
        &self,
        buffer: &mut Buffer,
        objects: &Vec<&(&Uuid, &Object)>,
        module: &str,
        domain: &Domain,
    ) -> Result<()> {
        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-object-store-persistence", module),
            |buffer| {
                emit!(buffer, "/// Persist the store.");
                emit!(buffer, "///");
                emit!(buffer, "/// The store is persisted as a directory of JSON files. The intention");
                emit!(buffer, "/// is that this directory can be checked into version control.");
                emit!(buffer, "/// In fact, I intend to add automaagic git integration as an option.");
                emit!(
                    buffer,
                    "pub fn persist<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {{"
                );
                emit!(buffer, "let path = path.as_ref();");
                // This is such a great joke! 🤣
                emit!(buffer, "let path = path.join(\"{}.json\");", domain.name());
                emit!(buffer, "fs::create_dir_all(&path)?;");
                emit!(buffer, "");
                for (_, obj) in objects {
                    emit!(buffer, "// Persist {}.", obj.as_ident());
                    emit!(buffer, "{{");
                    emit!(buffer, "let path = path.join(\"{}.json\");", obj.as_ident());
                    emit!(buffer, "let file = fs::File::create(path)?;");
                    emit!(buffer, "let mut writer = io::BufWriter::new(file);");
                    emit!(
                        buffer,
                        "serde_json::to_writer_pretty(&mut writer, &self.{}.values().map(|x| x).collect::<Vec<_>>())?;",
                        obj.as_ident()
                    );
                    emit!(buffer, "}}");
                }
                emit!(buffer, "Ok(())");
                emit!(buffer, "}}");

                Ok(())
            },
        )?;

        Ok(())
    }
}

impl ObjectStoreDefinition for DomainStore {}

impl CodeWriter for DomainStore {
    fn write_code(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        _woog: &Option<&mut WoogStore>,
        _imports: &Option<&HashMap<String, Domain>>,
        module: &str,
        _obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        let mut objects: Vec<(&Uuid, &Object)> = domain.sarzak().iter_object().collect();
        objects.sort_by(|a, b| a.1.name.cmp(&b.1.name));
        let supertypes = objects
            .iter()
            .filter(|(_, obj)| object_is_supertype(obj, domain.sarzak()))
            .collect::<Vec<_>>();
        let objects = objects
            .iter()
            .filter(|(id, obj)| {
                // We have this odd construction because a supertype may actually be a singleton.
                // They are in fact singletons in the current implementation. What is this doing?
                // if it's a supertype, or it's not a  singleton, and it's not imported.
                object_is_supertype(obj, domain.sarzak())
                    || !object_is_singleton(obj, domain.sarzak())
                // Don't include imported objects
                && !config.is_imported(*id)
            })
            .collect::<Vec<_>>();

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-object-store-definition", module),
            |buffer| {
                let persist = if let Some(persist) = config.get_persist() {
                    persist
                } else {
                    false
                };
                let mut singleton_subs = false;

                if persist {
                    emit!(buffer, "use std::{{io, fs, path::Path}};");
                }
                emit!(buffer, "use std::collections::HashMap;");
                emit!(buffer, "");
                emit!(buffer, "use serde::{{Deserialize, Serialize}};");
                emit!(buffer, "use uuid::Uuid;");
                emit!(buffer, "");
                emit!(buffer, "use crate::{}::types::{{", module);

                for (_, obj) in &objects {
                    emit!(
                        buffer,
                        "{},",
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );
                }
                for (_, obj) in &supertypes {
                    // I'm convinced that R14 and R15 are broken.
                    let sup = sarzak_maybe_get_many_r_sups_across_r14!(obj, domain.sarzak());
                    let isa = sarzak_get_one_r_isa_across_r13!(sup[0], domain.sarzak());
                    let mut subtypes = sarzak_get_many_r_subs_across_r27!(isa, domain.sarzak());
                    subtypes.sort_by(|a, b| {
                        let a = sarzak_get_one_obj_across_r15!(a, domain.sarzak());
                        let b = sarzak_get_one_obj_across_r15!(b, domain.sarzak());
                        a.name.cmp(&b.name)
                    });

                    for subtype in subtypes {
                        let s_obj = sarzak_get_one_obj_across_r15!(subtype, domain.sarzak());
                        if object_is_singleton(&s_obj, domain.sarzak()) {
                            singleton_subs = true;
                            emit!(buffer, "{},", s_obj.as_const());
                        }
                    }
                }
                emit!(buffer, "}};");
                emit!(buffer, "");
                emit!(buffer, "#[derive(Clone, Debug, Deserialize, Serialize)]");
                emit!(buffer, "pub struct ObjectStore {{");
                for (_, obj) in &objects {
                    emit!(
                        buffer,
                        "{}: HashMap<Uuid,{}>,",
                        obj.as_ident(),
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );
                }
                emit!(buffer, "}}");
                emit!(buffer, "");
                emit!(buffer, "impl ObjectStore {{");
                emit!(buffer, "pub fn new() -> Self {{");
                if singleton_subs {
                    emit!(buffer, "let mut store = Self {{");
                } else {
                    emit!(buffer, "let store = Self {{");
                }
                for (_, obj) in &objects {
                    emit!(buffer, "{}: HashMap::new(),", obj.as_ident());
                }
                emit!(buffer, "}};");
                emit!(buffer, "");
                emit!(buffer, "// Initialize Singleton Subtypes");
                for (_, obj) in &supertypes {
                    // I'm convinced that R14 and R15 are broken.
                    let sup = sarzak_maybe_get_many_r_sups_across_r14!(obj, domain.sarzak());
                    let isa = sarzak_get_one_r_isa_across_r13!(sup[0], domain.sarzak());
                    let mut subtypes = sarzak_get_many_r_subs_across_r27!(isa, domain.sarzak());
                    subtypes.sort_by(|a, b| {
                        let a = sarzak_get_one_obj_across_r15!(a, domain.sarzak());
                        let b = sarzak_get_one_obj_across_r15!(b, domain.sarzak());
                        a.name.cmp(&b.name)
                    });

                    for subtype in subtypes {
                        let s_obj = sarzak_get_one_obj_across_r15!(subtype, domain.sarzak());
                        if object_is_singleton(&s_obj, domain.sarzak()) {
                            emit!(
                                buffer,
                                "store.inter_{}({}::{}({}));",
                                obj.as_ident(),
                                obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak()),
                                s_obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak()),
                                s_obj.as_const()
                            );
                        }
                    }
                }
                emit!(buffer, "");
                emit!(buffer, "store");
                emit!(buffer, "}}");
                emit!(buffer, "");

                self.generate_store(buffer, &objects, module, domain)?;

                if persist {
                    self.generate_store_persistence(buffer, &objects, module, domain)?;
                }

                emit!(buffer, "}}");
                Ok(())
            },
        )?;

        Ok(())
    }
}

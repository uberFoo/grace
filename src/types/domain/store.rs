//! Generate ObjectStore for use in sarzak Domain
//!
use std::{collections::HashMap, fmt::Write};

use sarzak::{
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::types::Object,
    v2::domain::Domain,
    woog::{
        store::ObjectStore as WoogStore,
        types::{Ownership, MUTABLE},
    },
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        generator::{CodeWriter, FileGenerator, GenerationAction},
        get_subtypes_sorted, inner_object_is_enum, inner_object_is_singleton,
        inner_object_is_supertype,
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
        package: &str,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<GenerationAction> {
        ensure!(
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by DomainStoreGenerator"
            }
        );

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
        let mut objects: Vec<&Object> = domain.sarzak().iter_object().collect();
        objects.sort_by(|a, b| a.name.cmp(&b.name));
        let objects = objects
            .iter()
            .filter(|obj| {
                // Don't include imported objects
                !config.is_imported(&obj.id) &&
                // We have this odd construction because a supertype may actually be a singleton.
                (inner_object_is_enum(obj, domain)
                    || !inner_object_is_singleton(obj, domain))
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
                for obj in &objects {
                    emit!(
                        buffer,
                        "//! * [`{}`]",
                        obj.as_type(&Ownership::new_borrowed(), woog.as_ref().unwrap(), domain)
                    );
                }

                self.definition.write_code(
                    config, domain, woog, imports, package, module, obj_id, buffer,
                )?;

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
        objects: &Vec<&&Object>,
        module: &str,
        woog: &WoogStore,
        domain: &Domain,
    ) -> Result<()> {
        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-object-store-methods", module),
            |buffer| {
                for obj in objects {
                    emit!(
                        buffer,
                        "/// Inter [`{}`] into the store.",
                        obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    );
                    emit!(buffer, "///");
                    emit!(
                        buffer,
                        "pub fn inter_{}(&mut self, {}: {}) {{",
                        obj.as_ident(),
                        obj.as_ident(),
                        obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    );

                    if inner_object_is_enum(obj, domain) {
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
                        obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    );
                    emit!(buffer, "///");
                    emit!(
                        buffer,
                        "pub fn exhume_{}(&self, id: &Uuid) -> Option<&{}> {{",
                        obj.as_ident(),
                        obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    );
                    emit!(buffer, "self.{}.get(id)", obj.as_ident());
                    emit!(buffer, "}}");
                    emit!(buffer, "");
                    emit!(
                        buffer,
                        "/// Exhume [`{}`] from the store — mutably.",
                        obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    );
                    emit!(buffer, "///");
                    emit!(
                        buffer,
                        "pub fn exhume_{}_mut(&mut self, id: &Uuid) -> Option<&{}> {{",
                        obj.as_ident(),
                        obj.as_type(&Ownership::Mutable(MUTABLE), woog, domain)
                    );
                    emit!(buffer, "self.{}.get_mut(id)", obj.as_ident());
                    emit!(buffer, "}}");
                    emit!(buffer, "");
                    emit!(
                        buffer,
                        "/// Get an iterator over the internal `HashMap<&Uuid, {}>`.",
                        obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    );
                    emit!(buffer, "///");
                    emit!(
                        buffer,
                        "pub fn iter_{}(&self) -> impl Iterator<Item = &{}> {{",
                        obj.as_ident(),
                        obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    );
                    emit!(buffer, "self.{}.values()", obj.as_ident());
                    emit!(buffer, "}}");
                    emit!(buffer, "");
                }

                Ok(())
            },
        )?;

        Ok(())
    }

    fn generate_store_persistence(
        &self,
        buffer: &mut Buffer,
        objects: &Vec<&&Object>,
        module: &str,
        woog: &WoogStore,
        domain: &Domain,
    ) -> Result<()> {
        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-object-store-persistence", module),
            |buffer| {
                emit!(buffer, "/// Persist the store.");
                emit!(buffer, "///");
                emit!(
                    buffer,
                    "/// The store is persisted as a directory of JSON files. The intention"
                );
                emit!(
                    buffer,
                    "/// is that this directory can be checked into version control."
                );
                emit!(
                    buffer,
                    "/// In fact, I intend to add automaagic git integration as an option."
                );
                emit!(
                    buffer,
                    "pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {{"
                );
                emit!(buffer, "let path = path.as_ref();");
                // This is such a great joke! 🤣
                emit!(buffer, "let path = path.join(\"{}.json\");", domain.name());
                emit!(buffer, "fs::create_dir_all(&path)?;");
                emit!(buffer, "");
                for obj in objects {
                    emit!(buffer, "// Persist {}.", obj.name);
                    emit!(buffer, "{{");
                    emit!(buffer, "let path = path.join(\"{}\");", obj.as_ident());
                    emit!(buffer, "fs::create_dir_all(&path)?;");
                    emit!(
                        buffer,
                        "for {} in self.{}.values() {{",
                        obj.as_ident(),
                        obj.as_ident()
                    );
                    if inner_object_is_enum(obj, domain) {
                        emit!(
                            buffer,
                            "let path = path.join(format!(\"{{}}.json\", {}.id()));",
                            obj.as_ident()
                        );
                    } else {
                        emit!(
                            buffer,
                            "let path = path.join(format!(\"{{}}.json\", {}.id));",
                            obj.as_ident()
                        );
                    }
                    emit!(buffer, "let file = fs::File::create(path)?;");
                    emit!(buffer, "let mut writer = io::BufWriter::new(file);");
                    emit!(
                        buffer,
                        "serde_json::to_writer_pretty(&mut writer, &{})?;",
                        obj.as_ident()
                    );
                    emit!(buffer, "}}");
                    emit!(buffer, "}}");
                    emit!(buffer, "");
                }
                emit!(buffer, "Ok(())");
                emit!(buffer, "}}");
                emit!(buffer, "");

                emit!(buffer, "/// Load the store.");
                emit!(buffer, "///");
                emit!(
                    buffer,
                    "/// The store is persisted as a directory of JSON files. The intention"
                );
                emit!(
                    buffer,
                    "/// is that this directory can be checked into version control."
                );
                emit!(
                    buffer,
                    "/// In fact, I intend to add automaagic git integration as an option."
                );
                emit!(
                    buffer,
                    "pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {{"
                );
                emit!(buffer, "let path = path.as_ref();");
                emit!(buffer, "let path = path.join(\"{}.json\");", domain.name());
                emit!(buffer, "");
                emit!(buffer, "let mut store = Self::new();");
                emit!(buffer, "");
                for obj in objects {
                    emit!(buffer, "// Load {}.", obj.name);
                    emit!(buffer, "{{");
                    emit!(buffer, "let path = path.join(\"{}\");", obj.as_ident());
                    emit!(buffer, "let mut entries = fs::read_dir(path)?;");
                    emit!(buffer, "while let Some(entry) = entries.next() {{");
                    emit!(buffer, "let entry = entry?;");
                    emit!(buffer, "let path = entry.path();");
                    emit!(buffer, "let file = fs::File::open(path)?;");
                    emit!(buffer, "let reader = io::BufReader::new(file);");
                    emit!(
                        buffer,
                        "let {}: {} = serde_json::from_reader(reader)?;",
                        obj.as_ident(),
                        obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    );
                    if inner_object_is_enum(obj, domain) {
                        emit!(
                            buffer,
                            "store.{}.insert({}.id(), {});",
                            obj.as_ident(),
                            obj.as_ident(),
                            obj.as_ident()
                        );
                    } else {
                        emit!(
                            buffer,
                            "store.{}.insert({}.id, {});",
                            obj.as_ident(),
                            obj.as_ident(),
                            obj.as_ident()
                        );
                    }
                    emit!(buffer, "}}");
                    emit!(buffer, "}}");
                    emit!(buffer, "");
                }
                emit!(buffer, "");
                emit!(buffer, "Ok(store)");
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
        woog: &Option<&mut WoogStore>,
        _imports: &Option<&HashMap<String, Domain>>,
        _package: &str,
        module: &str,
        _obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by DomainStore"
            }
        );
        let woog = woog.as_ref().unwrap();

        let mut objects: Vec<&Object> = domain.sarzak().iter_object().collect();
        objects.sort_by(|a, b| a.name.cmp(&b.name));
        let supertypes = objects
            .iter()
            .filter(|obj| !config.is_imported(&obj.id) && inner_object_is_enum(obj, domain))
            .collect::<Vec<_>>();
        let objects = objects
            .iter()
            .filter(|obj| {
                // We have this odd construction because a supertype may actually be a singleton.
                // They are in fact singletons in the current implementation. What is this doing?
                // if it's a supertype, or it's not a  singleton, and it's not imported.
                // Don't include imported objects
                !config.is_imported(&obj.id)
                    && (inner_object_is_enum(obj, domain)
                        || !inner_object_is_singleton(obj, domain))
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
                    emit!(buffer, "use std::{{io, fs, path::Path, time::SystemTime}};");
                }
                emit!(buffer, "use std::collections::HashMap;");
                emit!(buffer, "");
                emit!(buffer, "use serde::{{Deserialize, Serialize}};");
                emit!(buffer, "use uuid::Uuid;");
                emit!(buffer, "");
                emit!(buffer, "use crate::{}::types::{{", module);

                for obj in &objects {
                    emit!(
                        buffer,
                        "{},",
                        obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    );
                }
                for obj in &supertypes {
                    for subtype in get_subtypes_sorted!(obj, domain.sarzak()) {
                        let s_obj = subtype.r15_object(domain.sarzak())[0];
                        if !config.is_imported(&s_obj.id) {
                            if inner_object_is_singleton(s_obj, domain)
                                && !inner_object_is_supertype(s_obj, domain)
                            {
                                singleton_subs = true;
                                emit!(buffer, "{},", s_obj.as_const());
                            }
                        }
                    }
                }
                emit!(buffer, "}};");
                emit!(buffer, "");
                emit!(buffer, "#[derive(Clone, Debug, Deserialize, Serialize)]");
                emit!(buffer, "pub struct ObjectStore {{");
                for obj in &objects {
                    emit!(
                        buffer,
                        "{}: HashMap<Uuid,{}>,",
                        obj.as_ident(),
                        obj.as_type(&Ownership::new_borrowed(), woog, domain)
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
                for obj in &objects {
                    emit!(buffer, "{}: HashMap::new(),", obj.as_ident());
                }
                emit!(buffer, "}};");
                emit!(buffer, "");
                emit!(buffer, "// Initialize Singleton Subtypes");
                for obj in &supertypes {
                    for subtype in get_subtypes_sorted!(obj, domain.sarzak()) {
                        let s_obj = subtype.r15_object(domain.sarzak())[0];
                        if !config.is_imported(&s_obj.id) {
                            if inner_object_is_singleton(s_obj, domain)
                                && !inner_object_is_supertype(s_obj, domain)
                            {
                                emit!(
                                    buffer,
                                    "store.inter_{}({}::{}({}));",
                                    obj.as_ident(),
                                    obj.as_type(&Ownership::new_borrowed(), woog, domain),
                                    s_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                                    s_obj.as_const()
                                );
                            }
                        }
                    }
                }
                emit!(buffer, "");
                emit!(buffer, "store");
                emit!(buffer, "}}");
                emit!(buffer, "");

                self.generate_store(buffer, &objects, module, woog, domain)?;

                emit!(buffer, "");

                if persist {
                    self.generate_store_persistence(buffer, &objects, module, woog, domain)?;
                }

                emit!(buffer, "}}");
                Ok(())
            },
        )?;

        Ok(())
    }
}

//! Generate ObjectStore for use in sarzak Domain
//!
use std::{fmt::Write, sync::RwLock};

use fnv::{FnvHashMap as HashMap, FnvHashSet as HashSet};
use sarzak::{
    lu_dog::store::ObjectStore as LuDogStore,
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::types::{Object, Ty},
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
        get_subtypes_sorted_from_super_obj, local_object_is_enum, local_object_is_hybrid,
        local_object_is_singleton, local_object_is_subtype, local_object_is_supertype,
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
        _lu_dog: &Option<&RwLock<LuDogStore>>,
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
                (local_object_is_enum(obj, config, domain)
                    || !local_object_is_singleton(obj, config, domain))
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
                    config, domain, woog, _lu_dog, imports, package, module, obj_id, buffer,
                )?;

                Ok(())
            },
        )?;

        Ok(GenerationAction::FormatWrite)
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
        timestamp: bool,
        module: &str,
        config: &GraceConfig,
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
                    let id = if local_object_is_enum(obj, config, domain) {
                        "id()"
                    } else {
                        "id"
                    };

                    if timestamp {
                        if object_has_name(obj, domain) {
                            emit!(
                                buffer,
                                "let value = ({}, SystemTime::now());",
                                obj.as_ident()
                            );
                            emit!(
                                buffer,
                                "self.{}.insert(value.0.{id}, value.clone());",
                                obj.as_ident(),
                            );
                            emit!(
                                buffer,
                                "self.{}_by_name.insert(value.0.name.clone(), value);",
                                obj.as_ident()
                            );
                        } else {
                            emit!(
                                buffer,
                                "self.{}.insert({}.{id}, ({}, SystemTime::now()));",
                                obj.as_ident(),
                                obj.as_ident(),
                                obj.as_ident()
                            );
                        }
                    } else {
                        if object_has_name(obj, domain) {
                            emit!(
                                buffer,
                                "self.{}.insert({}.{id}, {}.clone());",
                                obj.as_ident(),
                                obj.as_ident(),
                                obj.as_ident(),
                            );
                            emit!(
                                buffer,
                                "self.{}_by_name.insert({}.name.clone(), {});",
                                obj.as_ident(),
                                obj.as_ident(),
                                obj.as_ident()
                            );
                        } else {
                            emit!(
                                buffer,
                                "self.{}.insert({}.{id}, {});",
                                obj.as_ident(),
                                obj.as_ident(),
                                obj.as_ident()
                            );
                        }
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
                    if timestamp {
                        emit!(
                            buffer,
                            "self.{}.get(id).map(|{}| &{}.0)",
                            obj.as_ident(),
                            obj.as_ident(),
                            obj.as_ident()
                        );
                    } else {
                        emit!(buffer, "self.{}.get(id)", obj.as_ident());
                    }
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
                    if timestamp {
                        emit!(
                            buffer,
                            "self.{}.get_mut(id).map(|{}| &mut {}.0)",
                            obj.as_ident(),
                            obj.as_ident(),
                            obj.as_ident()
                        );
                    } else {
                        emit!(buffer, "self.{}.get_mut(id)", obj.as_ident());
                    }
                    emit!(buffer, "}}");
                    emit!(buffer, "");
                    if object_has_name(obj, domain) {
                        emit!(
                            buffer,
                            "/// Exhume [`{}`] from the store by name.",
                            obj.as_type(&Ownership::new_borrowed(), woog, domain)
                        );
                        emit!(buffer, "///");
                        emit!(
                            buffer,
                            "pub fn exhume_{}_by_name(&self, name: &str) -> Option<&{}> {{",
                            obj.as_ident(),
                            obj.as_type(&Ownership::new_borrowed(), woog, domain)
                        );
                        if timestamp {
                            emit!(
                                buffer,
                                "self.{}_by_name.get(name).map(|{}| &{}.0)",
                                obj.as_ident(),
                                obj.as_ident(),
                                obj.as_ident()
                            );
                        } else {
                            emit!(buffer, "self.{}_by_name.get(name)", obj.as_ident());
                        }
                        emit!(buffer, "}}");
                        emit!(buffer, "");
                    }
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
                    if timestamp {
                        emit!(
                            buffer,
                            "self.{}.values().map(|{}| &{}.0)",
                            obj.as_ident(),
                            obj.as_ident(),
                            obj.as_ident()
                        );
                    } else {
                        emit!(buffer, "self.{}.values()", obj.as_ident());
                    }
                    emit!(buffer, "}}");
                    emit!(buffer, "");
                    if timestamp {
                        emit!(
                            buffer,
                            "/// Get the timestamp for {}.",
                            obj.as_type(&Ownership::new_borrowed(), woog, domain)
                        );
                        emit!(buffer, "///");
                        emit!(
                            buffer,
                            "pub fn {}_timestamp(&self, {}: &{}) -> SystemTime {{",
                            obj.as_ident(),
                            obj.as_ident(),
                            obj.as_type(&Ownership::new_borrowed(), woog, domain)
                        );
                        if local_object_is_enum(obj, config, domain) {
                            emit!(
                                buffer,
                                "self.{}.get(&{}.id()).map(|{}| {}.1).unwrap_or(SystemTime::now())",
                                obj.as_ident(),
                                obj.as_ident(),
                                obj.as_ident(),
                                obj.as_ident()
                            );
                        } else {
                            emit!(
                                buffer,
                                "self.{}.get(&{}.id).map(|{}| {}.1).unwrap_or(SystemTime::now())",
                                obj.as_ident(),
                                obj.as_ident(),
                                obj.as_ident(),
                                obj.as_ident()
                            );
                        }
                        emit!(buffer, "}}");
                        emit!(buffer, "");
                    }
                }

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
        _lu_dog: &Option<&RwLock<LuDogStore>>,
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

        fn emit_singleton_subtype_uses(
            supertypes: &[&&Object],
            config: &GraceConfig,
            domain: &Domain,
            woog: &WoogStore,
            buffer: &mut Buffer,
        ) -> Result<bool> {
            let mut includes = HashSet::default();

            for sup in supertypes {
                for subtype in get_subtypes_sorted_from_super_obj!(sup, domain.sarzak()) {
                    let s_obj = subtype.r15_object(domain.sarzak())[0];
                    if !config.is_imported(&s_obj.id) {
                        if local_object_is_supertype(s_obj, config, domain)
                            && !local_object_is_subtype(s_obj, config, domain)
                        {
                            // Ooooh. Look here — recursion.
                            includes.extend(emit_singleton_subtype_uses_inner(
                                s_obj, config, domain, woog,
                            )?);
                        } else if local_object_is_singleton(s_obj, config, domain) {
                            includes.insert(s_obj.as_const());
                        }
                    }
                }
            }

            if includes.is_empty() {
                return Ok(false);
            } else {
                for include in includes {
                    emit!(buffer, "{},", include);
                }

                return Ok(true);
            }
        }

        fn emit_singleton_subtype_uses_inner(
            sup: &Object,
            config: &GraceConfig,
            domain: &Domain,
            woog: &WoogStore,
        ) -> Result<HashSet<String>> {
            let mut includes = HashSet::default();

            for subtype in get_subtypes_sorted_from_super_obj!(sup, domain.sarzak()) {
                let s_obj = subtype.r15_object(domain.sarzak())[0];
                if !config.is_imported(&s_obj.id) {
                    if local_object_is_supertype(s_obj, config, domain)
                        && !local_object_is_subtype(s_obj, config, domain)
                    {
                        // Ooooh. Look here — recursion.
                        includes.extend(emit_singleton_subtype_uses_inner(
                            s_obj, config, domain, woog,
                        )?);
                    } else if local_object_is_singleton(s_obj, config, domain) {
                        includes.insert(s_obj.as_const());
                    }
                }
            }

            Ok(includes)
        }

        /// We are emitting a list of inter statements.
        /// It's assumed that the initial prefix is "store.inter_foo(", where foo is the supertype.
        /// Our job is to add the rest.
        /// It starts with "Foo::"", and for each subtype we'll either add Bar(BAR), or if bar is a supertype,
        /// we start over with "Bar::"", and continue as before, i.e., "Foo::Bar::Baz(BAZ)".
        fn emit_singleton_subtype_instances(
            sup: &Object,
            prefix: &str,
            suffix: &str,
            config: &GraceConfig,
            domain: &Domain,
            woog: &WoogStore,
            buffer: &mut Buffer,
        ) -> Result<()> {
            for subtype in get_subtypes_sorted_from_super_obj!(sup, domain.sarzak()) {
                let s_obj = subtype.r15_object(domain.sarzak())[0];

                if local_object_is_hybrid(sup, config, domain) {
                    continue;
                }

                let prefix = format!(
                    "{}{}::{}",
                    prefix,
                    sup.as_type(&Ownership::new_borrowed(), woog, domain),
                    s_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );

                if !config.is_imported(&s_obj.id) {
                    if local_object_is_supertype(s_obj, config, domain) {
                        let prefix = format!("{}(", prefix);
                        let suffix = format!(".id()){}", suffix);
                        emit_singleton_subtype_instances(
                            s_obj, &prefix, &suffix, config, domain, woog, buffer,
                        )?;
                    } else if local_object_is_singleton(s_obj, config, domain) {
                        writeln!(buffer, "{}({}){}", prefix, s_obj.as_const(), suffix)
                            .context(FormatSnafu)?;
                    }
                }
            }

            Ok(())
        }

        let mut objects: Vec<&Object> = domain.sarzak().iter_object().collect();
        objects.sort_by(|a, b| a.name.cmp(&b.name));
        let supertypes = objects
            .iter()
            .filter(|obj| !config.is_imported(&obj.id) && local_object_is_enum(obj, config, domain))
            .collect::<Vec<_>>();
        let objects = objects
            .iter()
            .filter(|obj| {
                // We have this odd construction because a supertype may actually be a singleton.
                // They are in fact singletons in the current implementation. What is this doing?
                // if it's a supertype, or it's not a  singleton, and it's not imported.
                // Don't include imported objects
                !config.is_imported(&obj.id)
                    && (local_object_is_enum(obj, config, domain)
                        || !local_object_is_singleton(obj, config, domain))
            })
            .collect::<Vec<_>>();

        let timestamp = config.get_persist_timestamps().unwrap_or(false);
        let is_meta = config.is_meta_model();

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-object-store-definition", module),
            |buffer| {
                let persist = if let Some(persist) = config.get_persist() {
                    persist
                } else {
                    false
                };

                if persist {
                    if timestamp {
                        emit!(buffer, "use std::{{io::{{self, prelude::*}}, fs, path::Path, time::SystemTime}};");
                        emit!(buffer, "");
                    } else {
                        emit!(buffer, "use std::{{io::{{self, prelude::*}}, fs, path::Path}};");
                        emit!(buffer, "");
                    }
                }
                emit!(buffer, "use fnv::FnvHashMap as HashMap;");
                emit!(buffer, "use serde::{{Deserialize, Serialize}};");
                emit!(buffer, "use uuid::Uuid;");
                if timestamp && is_meta && false {
                    emit!(buffer, "use snafu::prelude::*;");
                    emit!(buffer, "");
                    emit!(buffer, "use crate::mc::{{FileSnafu, Result}};");
                } else {
                    emit!(buffer, "");
                }
                emit!(buffer, "use crate::{}::types::{{", module);

                for obj in &objects {
                    emit!(
                        buffer,
                        "{},",
                        obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    );
                }
                let singleton_subs =
                    emit_singleton_subtype_uses(&supertypes, config, domain, woog, buffer)?;

                emit!(buffer, "}};");
                emit!(buffer, "");
                emit!(buffer, "#[derive(Clone, Debug, Deserialize, Serialize)]");
                emit!(buffer, "pub struct ObjectStore {{");
                for obj in &objects {
                    if timestamp {
                        emit!(
                            buffer,
                            "{}: HashMap<Uuid, ({}, SystemTime)>,",
                            obj.as_ident(),
                            obj.as_type(&Ownership::new_borrowed(), woog, domain)
                        );
                        if object_has_name(obj, domain) {
                            emit!(
                                buffer,
                                "{}_by_name: HashMap<String, ({}, SystemTime)>,",
                                obj.as_ident(),
                                obj.as_type(&Ownership::new_borrowed(), woog, domain)
                            );
                        }
                    } else {
                        emit!(
                            buffer,
                            "{}: HashMap<Uuid, {}>,",
                            obj.as_ident(),
                            obj.as_type(&Ownership::new_borrowed(), woog, domain)
                        );
                        if object_has_name(obj, domain) {
                            emit!(
                                buffer,
                                "{}_by_name: HashMap<String, {}>,",
                                obj.as_ident(),
                                obj.as_type(&Ownership::new_borrowed(), woog, domain)
                            );
                        }
                    }
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
                    emit!(buffer, "{}: HashMap::default(),", obj.as_ident());
                    if object_has_name(obj, domain) {
                        emit!(buffer, "{}_by_name: HashMap::default(),", obj.as_ident());
                    }

                }
                emit!(buffer, "}};");
                emit!(buffer, "");
                emit!(buffer, "// Initialize Singleton Subtypes");
                for obj in &supertypes {
                    emit_singleton_subtype_instances(
                        obj,
                        &format!("store.inter_{}(", obj.as_ident()),
                        &");",
                        config,
                        domain,
                        woog,
                        buffer,
                    )?;
                }
                emit!(buffer, "");
                emit!(buffer, "store");
                emit!(buffer, "}}");
                emit!(buffer, "");
                // End of new

                self.generate_store(buffer, &objects, timestamp, module, config, woog, domain)?;

                emit!(buffer, "");

                if persist {
                    generate_store_persistence(
                        buffer, &objects, timestamp, is_meta, module, config, woog, domain,
                    )?;
                }

                emit!(buffer, "}}");

                Ok(())
            },
        )?;

        Ok(())
    }
}

fn object_has_name(obj: &Object, domain: &Domain) -> bool {
    obj.r1_attribute(domain.sarzak())
        .iter()
        .find(|attr| {
            if attr.name == "name" {
                if let Ty::String(_) = attr.r2_ty(domain.sarzak())[0] {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        })
        .is_some()
}

fn generate_store_persistence(
    buffer: &mut Buffer,
    objects: &[&&Object],
    timestamp: bool,
    is_meta: bool,
    module: &str,
    config: &GraceConfig,
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
                "/// In fact, I intend to add automagic git integration as an option."
            );
            emit!(
                buffer,
                "pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {{"
            );
            emit!(buffer, "let path = path.as_ref();");
            emit!(buffer, "fs::create_dir_all(&path)?;");
            emit!(buffer, "");
            emit!(
                buffer,
                "let bin_path = path.clone().join(\"{}.bin\");",
                domain.name()
            );
            emit!(buffer, "let mut bin_file = fs::File::create(bin_path)?;");
            emit!(
                buffer,
                "let encoded: Vec<u8> = bincode::serialize(&self).unwrap();"
            );
            emit!(buffer, "bin_file.write_all(&encoded)?;");
            emit!(buffer, "");
            // This is such a great joke! 🤣
            emit!(buffer, "let path = path.join(\"{}.json\");", domain.name());
            emit!(buffer, "fs::create_dir_all(&path)?;");
            emit!(buffer, "");
            for obj in objects {
                emit!(buffer, "// Persist {}.", obj.name);
                emit!(buffer, "{{");
                emit!(buffer, "let path = path.join(\"{}\");", obj.as_ident());
                emit!(buffer, "fs::create_dir_all(&path)?;");
                if timestamp {
                    emit!(
                        buffer,
                        "for {}_tuple in self.{}.values() {{",
                        obj.as_ident(),
                        obj.as_ident()
                    );
                    if local_object_is_enum(obj, config, domain) {
                        emit!(
                            buffer,
                            "let path = path.join(format!(\"{{}}.json\", {}_tuple.0.id()));",
                            obj.as_ident()
                        );
                    } else {
                        emit!(
                            buffer,
                            "let path = path.join(format!(\"{{}}.json\", {}_tuple.0.id));",
                            obj.as_ident()
                        );
                    }
                    emit!(buffer, "if path.exists() {{");
                    emit!(buffer, "let file = fs::File::open(&path)?;");
                    emit!(buffer, "let reader = io::BufReader::new(file);");
                    emit!(
                        buffer,
                        "let on_disk: ({}, SystemTime) = serde_json::from_reader(reader)?;",
                        obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    );
                    emit!(buffer, "if on_disk.0 != {}_tuple.0 {{", obj.as_ident());
                    emit!(buffer, "let file = fs::File::create(path)?;");
                    emit!(buffer, "let mut writer = io::BufWriter::new(file);");
                    emit!(
                        buffer,
                        "serde_json::to_writer_pretty(&mut writer, &{}_tuple)?;",
                        obj.as_ident()
                    );
                    emit!(buffer, "}}");
                    emit!(buffer, "}} else {{");
                    emit!(buffer, "let file = fs::File::create(&path)?;");
                    emit!(buffer, "let mut writer = io::BufWriter::new(file);");
                    emit!(
                        buffer,
                        "serde_json::to_writer_pretty(&mut writer, &{}_tuple)?;",
                        obj.as_ident()
                    );
                    emit!(buffer, "}}");
                    emit!(buffer, "}}");
                    // Now we need to delete any files that correspond to something
                    // in the store that went away.
                    emit!(buffer, "for file in fs::read_dir(&path)? {{");
                    emit!(buffer, "let file = file?;");
                    emit!(buffer, "let path = file.path();");
                    emit!(
                        buffer,
                        "let file_name = path.file_name().unwrap().to_str().unwrap();"
                    );
                    emit!(buffer, "let id = file_name.split(\".\").next().unwrap();");
                    emit!(buffer, "if let Ok(id) = Uuid::parse_str(id) {{");
                    emit!(buffer, "if !self.{}.contains_key(&id) {{", obj.as_ident());
                    if is_meta && false {
                        emit!(buffer, "let result = fs::remove_file(path);");
                        emit!(buffer, "match result {{");
                        emit!(buffer, "Ok(_) => {{}}");
                        emit!(buffer, "Err(e) => match e.kind() {{");
                        emit!(buffer, "io::ErrorKind::NotFound => {{}}");
                        emit!(buffer, "_ => {{");
                        emit!(buffer, "return Err(e).context(FileSnafu {{");
                        emit!(buffer, "path,");
                        emit!(
                            buffer,
                            "description: \"Failed to remove file from store\".to_owned(),"
                        );
                        emit!(buffer, "}})");
                        emit!(buffer, "}}");
                        emit!(buffer, "}},");
                        emit!(buffer, "}}");
                    } else {
                        emit!(buffer, "fs::remove_file(path)?;");
                    }
                    emit!(buffer, "}}");
                    emit!(buffer, "}}");
                    emit!(buffer, "}}");
                    emit!(buffer, "}}");
                } else {
                    emit!(
                        buffer,
                        "for {} in self.{}.values() {{",
                        obj.as_ident(),
                        obj.as_ident()
                    );
                    if local_object_is_enum(obj, config, domain) {
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
                }
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
                let id = if local_object_is_enum(obj, config, domain) {
                    "id()"
                } else {
                    "id"
                };
                if timestamp {
                    emit!(
                        buffer,
                        "let {}: ({}, SystemTime) = serde_json::from_reader(reader)?;",
                        obj.as_ident(),
                        obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    );
                    if object_has_name(obj, domain) {
                        emit!(
                            buffer,
                            "store.{}_by_name.insert({}.0.name.clone(), {}.clone());",
                            obj.as_ident(),
                            obj.as_ident(),
                            obj.as_ident()
                        );
                    }
                    emit!(
                        buffer,
                        "store.{}.insert({}.0.{id}, {});",
                        obj.as_ident(),
                        obj.as_ident(),
                        obj.as_ident()
                    );
                } else {
                    emit!(
                        buffer,
                        "let {}: {} = serde_json::from_reader(reader)?;",
                        obj.as_ident(),
                        obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    );
                    if object_has_name(obj, domain) {
                        emit!(
                            buffer,
                            "store.{}_by_name.insert({}.name.clone(), {}.clone());",
                            obj.as_ident(),
                            obj.as_ident(),
                            obj.as_ident()
                        );
                    }
                    emit!(
                        buffer,
                        "store.{}.insert({}.{id}, {});",
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

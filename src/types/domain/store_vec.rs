//! Generate ObjectStore for use in sarzak Domain
//!
use std::fmt::Write;

use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use sarzak::{
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::types::Object,
    v2::domain::Domain,
    woog::{store::ObjectStore as WoogStore, types::Ownership},
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        generator::{CodeWriter, FileGenerator, GenerationAction},
        get_binary_referrers_sorted, get_subtypes_sorted_from_super_obj, local_object_is_enum,
        local_object_is_hybrid, local_object_is_singleton, local_object_is_subtype,
        local_object_is_supertype,
        render::{RenderConst, RenderIdent, RenderType},
    },
    options::{GraceConfig, UberStoreOptions},
    types::ObjectStoreDefinition,
};

pub(crate) struct DomainStoreVecGenerator {
    definition: Box<dyn ObjectStoreDefinition>,
}

impl FileGenerator for DomainStoreVecGenerator {
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
                    config, domain, woog, imports, package, module, obj_id, buffer,
                )?;

                Ok(())
            },
        )?;

        Ok(GenerationAction::FormatWrite)
    }
}

pub(crate) struct DomainStoreVec;

impl DomainStoreVec {
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
                let is_uber = config.is_uber_store();

                for obj in objects {
                    let obj_ident = obj.as_ident();
                    let obj_type = obj.as_type(&Ownership::new_borrowed(), woog, domain);
                    let thing = get_value_wrapper(is_uber, config, obj, woog, domain);

                    // 🚦
                    // Generate inter_ methods
                    emit!(
                        buffer,
                        "/// Inter (insert) [`{obj_type}`] into the store.",
                    );
                    emit!(buffer, "///");
                    emit!(buffer, "#[inline]");

                    if is_uber {
                        use UberStoreOptions::*;
                        match config.get_uber_store().unwrap() {
                            Disabled => unreachable!(),
                            AsyncRwLock => {
                                emit!(
                                    buffer,
                                    "pub async fn inter_{obj_ident}<F>(&mut self, {obj_ident}: F) -> {thing} where F: Fn(usize) -> {thing}, {{"
                                );
                            }
                            _ => {
                                emit!(
                                    buffer,
                                    "pub fn inter_{obj_ident}<F>(&mut self, {obj_ident}: F) -> {thing} where F: Fn(usize) -> {thing}, {{"
                                );
                            }
                        }
                    } else {
                        emit!(
                            buffer,
                            "pub fn inter_{obj_ident}(&mut self, {obj_ident}: {thing}) {{"
                        );
                    }

                    let id = if local_object_is_enum(obj, config, domain) {
                        "id()"
                    } else {
                        "id"
                    };

                    if is_uber {
                        let (read, write) = get_uber_read_write(config);
                        use UberStoreOptions::*;
                        match config.get_uber_store().unwrap() {
                            AsyncRwLock => {
                                emit!(buffer, "let _index = if let Some(_index) = self.{obj_ident}_free_list.lock().await.pop() {{");
                                emit!(buffer, "log::trace!(target: \"store\", \"recycling block {{_index}}.\");");
                                emit!(buffer, "_index");
                                emit!(buffer, "}} else {{");
                                emit!(buffer, "let _index = self.{obj_ident}{read}.len();");
                                emit!(buffer, "log::trace!(target: \"store\", \"allocating block {{_index}}.\");");
                                emit!(buffer, "self.{obj_ident}{write}.push(None);");
                                emit!(buffer, "_index");
                                emit!(buffer, "}};");
                                emit!(buffer, "");
                                emit!(buffer, "let {obj_ident} = {obj_ident}(_index);");
                                emit!(buffer, "");
                                emit!(buffer, "let iter = self.{obj_ident}{read};");
                                emit!(buffer, "let iter = iter.iter();");
                                emit!(buffer, "let iter = stream::iter(iter);");
                                emit!(buffer, "let found = iter.filter_map(|stored| {{");
                                emit!(buffer, "Box::pin({{");
                                emit!(buffer, "let stored = stored.clone();");
                                emit!(buffer, "async {{");
                                emit!(buffer, "if let Some(stored) = stored {{");
                                emit!(buffer, "if *stored{read} == *{obj_ident}{read} {{");
                                emit!(buffer, "Some(stored)");
                                emit!(buffer, "}} else {{");
                                emit!(buffer, "None");
                                emit!(buffer, "}}");
                                emit!(buffer, "}} else {{");
                                emit!(buffer, "None");
                                emit!(buffer, "}}");
                                emit!(buffer, "}}");
                                emit!(buffer, "}})");
                                emit!(buffer, "}})");
                                emit!(buffer, ".next()");
                                emit!(buffer, ".await;");
                                emit!(buffer, "");
                                if object_has_name(obj, domain) {
                                    emit!(buffer, "let {obj_ident} = ");
                                }
                                emit!(buffer, "if let Some({obj_ident}) = found {{");
                                emit!(buffer, "log::debug!(target: \"store\", \"found duplicate {{{obj_ident}:?}}.\");");
                                emit!(buffer, "self.{obj_ident}_free_list.lock().await.push(_index);");
                                emit!(buffer, "{obj_ident}.clone()");
                                emit!(buffer, "}} else {{");
                                emit!(buffer, "log::debug!(target: \"store\", \"interring {{{obj_ident}:?}}.\");");
                                emit!(buffer, "self.{obj_ident}{write}[_index] = Some({obj_ident}.clone());");
                                emit!(buffer, "{obj_ident}");
                                emit!(buffer, "}}");
                            },
                            StdRwLock | ParkingLotRwLock | NDRwLock => {
                                emit!(buffer, "let _index = if let Some(_index) = self.{obj_ident}_free_list.lock().unwrap().pop() {{");
                                emit!(buffer, "log::trace!(target: \"store\", \"recycling block {{_index}}.\");");
                                emit!(buffer, "_index");
                                emit!(buffer, "}} else {{");
                                emit!(buffer, "let _index = self.{obj_ident}{read}.len();");
                                emit!(buffer, "log::trace!(target: \"store\", \"allocating block {{_index}}.\");");
                                emit!(buffer, "self.{obj_ident}{write}.push(None);");
                                emit!(buffer, "_index");
                                emit!(buffer, "}};");
                                emit!(buffer, "");
                                emit!(buffer, "let {obj_ident} = {obj_ident}(_index);");
                                emit!(buffer, "");
                                emit!(buffer, "let found = if let Some({obj_ident}) = self.{obj_ident}{read}.iter().find(|stored| {{");
                                emit!(buffer, "if let Some(stored) = stored {{");
                                emit!(buffer, "*stored{read} == *{obj_ident}{read}");
                                emit!(buffer, "}} else {{");
                                emit!(buffer, "false");
                                emit!(buffer, "}}");
                                emit!(buffer, "}}) {{");
                                emit!(buffer, "{obj_ident}.clone()");
                                emit!(buffer, "}} else {{");
                                emit!(buffer, "None");
                                emit!(buffer, "}};");
                                emit!(buffer, "");
                                if object_has_name(obj, domain) {
                                    emit!(buffer, "let {obj_ident} = ");
                                }
                                emit!(buffer, "if let Some({obj_ident}) = found {{");
                                emit!(buffer, "log::debug!(target: \"store\", \"found duplicate {{{obj_ident}:?}}.\");");
                                emit!(buffer, "self.{obj_ident}_free_list.lock().unwrap().push(_index);");
                                emit!(buffer, "{obj_ident}.clone()");
                                emit!(buffer, "}} else {{");
                                emit!(buffer, "log::debug!(target: \"store\", \"interring {{{obj_ident}:?}}.\");");
                                emit!(buffer, "self.{obj_ident}{write}[_index] = Some({obj_ident}.clone());");
                                emit!(buffer, "{obj_ident}");
                                emit!(buffer, "}}");
                            },
                            Single => {
                                emit!(buffer, "let _index = if let Some(_index) = self.{obj_ident}_free_list.pop() {{");
                                emit!(buffer, "log::trace!(target: \"store\", \"recycling block {{_index}}.\");");
                                emit!(buffer, "_index");
                                emit!(buffer, "}} else {{");
                                emit!(buffer, "let _index = self.{obj_ident}.len();");
                                emit!(buffer, "log::trace!(target: \"store\", \"allocating block {{_index}}.\");");
                                emit!(buffer, "self.{obj_ident}.push(None);");
                                emit!(buffer, "_index");
                                emit!(buffer, "}};");
                                emit!(buffer, "");
                                emit!(buffer, "let {obj_ident} = {obj_ident}(_index);");
                                emit!(buffer, "");
                                if object_has_name(obj, domain) {
                                    emit!(buffer, "let {obj_ident} = ");
                                }
                                emit!(buffer, "if let Some(Some({obj_ident})) = self.{obj_ident}.iter().find(|stored| {{");
                                emit!(buffer, "if let Some(stored) = stored {{");
                                emit!(buffer, "*stored{read} == *{obj_ident}{read}");
                                emit!(buffer, "}} else {{");
                                emit!(buffer, "false");
                                emit!(buffer, "}}");
                                emit!(buffer, "}}) {{");
                                emit!(buffer, "log::debug!(target: \"store\", \"found duplicate {{{obj_ident}:?}}.\");");
                                emit!(buffer, "self.{obj_ident}_free_list.push(_index);");
                                emit!(buffer, "{obj_ident}.clone()");
                                emit!(buffer, "}} else {{");
                                emit!(buffer, "log::debug!(target: \"store\", \"interring {{{obj_ident}:?}}.\");");
                                emit!(buffer, "self.{obj_ident}[_index] = Some({obj_ident}.clone());");
                                emit!(buffer, "{obj_ident}");
                                emit!(buffer, "}}");
                            },
                            store => panic!("{store} is not currently supported"),
                        }
                        if object_has_name(obj, domain) {
                            emit!(buffer, ";");
                        }
                    }

                    if timestamp {
                        if object_has_name(obj, domain) {

                            if is_uber {
                                emit!(
                                    buffer,
                                    "let value = ({obj_ident}.clone(), SystemTime::now());"
                                );
                            } else {
                                emit!(
                                    buffer,
                                    "let value = ({obj_ident}, SystemTime::now());",
                                );
                            }

                            if is_uber {
                                let (_read, write) = get_uber_read_write(config);
                                emit!(
                                    buffer,
                                    "self.{obj_ident}_id_by_name{write}.insert(read.name.to_owned(), (read.{id}, value.1));",
                                );
                                emit!(
                                    buffer,
                                    "self.{obj_ident}{write}.insert(read.{id}, value);",
                                );
                            } else {
                                emit!(
                                    buffer,
                                    "self.{obj_ident}_id_by_name.insert(value.0.name.to_owned(), (value.0.{id}, value.1));",
                                );
                                emit!(
                                    buffer,
                                    "self.{obj_ident}.insert(value.0.{id}, value);",
                                );
                            }

                        } else if is_uber {
                            let (_read, write) = get_uber_read_write(config);
                            emit!(
                                buffer,
                                "self.{obj_ident}{write}.insert(read.{id}, ({obj_ident}.clone(), SystemTime::now()));",
                            );
                        } else {
                            emit!(
                                buffer,
                                "self.{obj_ident}.insert({obj_ident}.{id}, ({obj_ident}, SystemTime::now()));",
                            );
                        }
                    } else if object_has_name(obj, domain) {
                        if is_uber {
                            let (read, write) = get_uber_read_write(config);
                            use UberStoreOptions::*;
                            match config.get_uber_store().unwrap() {
                                AsyncRwLock | StdRwLock | ParkingLotRwLock | NDRwLock => {
                                    emit!(
                                        buffer,
                                        "self.{obj_ident}_id_by_name{write}.insert({obj_ident}{read}.name.to_owned(), {obj_ident}{read}.{id});",
                                    );
                                }
                                Single => {
                                    emit!(
                                        buffer,
                                        "self.{obj_ident}_id_by_name.insert({obj_ident}{read}.name.to_owned(), {obj_ident}{read}.{id});",
                                    );
                                }
                                store => panic!("{store} is not currently supported"),
                            }
                            emit!(buffer, "{obj_ident}");
                        } else {
                            emit!(
                                buffer,
                                "self.{obj_ident}_id_by_name.insert({obj_ident}.name.to_owned(), {obj_ident}.{id});",
                            );
                            emit!(
                                buffer,
                                "self.{obj_ident}.insert({obj_ident}.{id}, {obj_ident});",
                            );
                        }

                    } else if !is_uber {
                        emit!(
                            buffer,
                            "self.{obj_ident}.insert({obj_ident}.{id}, {obj_ident});",
                        );
                    }
                    emit!(buffer, "}}");
                    emit!(buffer, "");

                    // 🚦
                    // Generate exhume_ methods
                    let thing = get_value_wrapper(is_uber, config, obj, woog, domain);

                    emit!(
                        buffer,
                        "/// Exhume (get) [`{obj_type}`] from the store.",
                    );
                    emit!(buffer, "///");
                    emit!(buffer, "#[inline]");

                    if is_uber {
                        use UberStoreOptions::*;
                        match config.get_uber_store().unwrap() {
                            Disabled => unreachable!(),
                            AsyncRwLock => {
                                emit!(
                                    buffer,
                                    "pub async fn exhume_{obj_ident}(&self, id: &usize) -> Option<{thing}> {{",
                                );
                            }
                            _ => {
                                emit!(
                                    buffer,
                                    "pub fn exhume_{obj_ident}(&self, id: &usize) -> Option<{thing}> {{",
                                );
                            }
                        }
                    } else {
                        emit!(
                            buffer,
                            "pub fn exhume_{obj_ident}(&self, id: &Uuid) -> Option<&{thing}> {{",
                        );
                    }

                    if is_uber {
                        let _thing = get_value_wrapper(is_uber, config, obj, woog, domain);

                        let (read, _write) = get_uber_read_write(config);
                        if timestamp {
                            emit!(
                                buffer,
                                "self.{obj_ident}{read}.get(id).map(|{obj_ident}| {obj_ident}.0.clone())",

                            );
                        } else {
                            use UberStoreOptions::*;
                            match config.get_uber_store().unwrap() {
                                AsyncRwLock | StdRwLock | ParkingLotRwLock | NDRwLock =>  {
                                    emit!(
                                        buffer,
                                        "match self.{obj_ident}{read}.get(*id) {{",
                                    );
                                },
                                Single => {
                                    emit!(
                                        buffer,
                                        "match self.{obj_ident}.get(*id) {{",
                                    );
                                },
                                store => panic!("{store} is not currently supported"),
                            }
                            emit!(
                                buffer,
                                "Some({obj_ident}) => {obj_ident}.clone(),",
                            );
                            emit!(
                                buffer,
                                "None => None",
                            );
                            emit!(
                                buffer,
                                "}}",
                            );
                        }
                    } else if timestamp {
                        emit!(
                            buffer,
                            "self.{obj_ident}.get(id).map(|{obj_ident}| &{obj_ident}.0)",
                        );
                    } else {
                        emit!(buffer, "self.{obj_ident}.get(id)");
                    }
                    emit!(buffer, "}}");
                    emit!(buffer, "");

                    // 🚦
                    // Generate exorcise_ methods
                    let thing = get_value_wrapper(is_uber, config, obj, woog, domain);

                    emit!(
                        buffer,
                        "/// Exorcise (remove) [`{obj_type}`] from the store.",
                    );
                    emit!(buffer, "///");
                    emit!(buffer, "#[inline]");

                    if is_uber {
                        use UberStoreOptions::*;
                        match config.get_uber_store().unwrap() {
                            Disabled => unreachable!(),
                            AsyncRwLock => {
                                emit!(
                                    buffer,
                                    "pub async fn exorcise_{obj_ident}(&mut self, id: &usize) -> Option<{thing}> {{",
                                );
                            }
                            _ => {
                                emit!(
                                    buffer,
                                    "pub fn exorcise_{obj_ident}(&mut self, id: &usize) -> Option<{thing}> {{",
                                );
                            }
                        }
                    } else {
                        emit!(
                            buffer,
                            "pub fn exorcise_{obj_ident}(&mut self, id: &Uuid) -> Option<{thing}> {{",
                        );
                    }

                    emit!(buffer, "log::debug!(target: \"store\", \"exorcising {obj_ident} slot: {{id}}.\");");

                    if is_uber {
                        let (_read, write) = get_uber_read_write(config);
                        if timestamp {
                            emit!(
                                buffer,
                                "self.{0}{write}.remove(id).map(|{0}| {0}.0.clone())",
                                obj_ident
                            );
                        } else {
                            use UberStoreOptions::*;
                            match config.get_uber_store().unwrap() {
                                AsyncRwLock  =>  {
                                    emit!(buffer, "let result = self.{obj_ident}{write}[*id].take();");
                                    emit!(buffer, "self.{obj_ident}_free_list.lock().await.push(*id);");
                                },
                                StdRwLock | ParkingLotRwLock | NDRwLock =>  {
                                    emit!(buffer, "let result = self.{obj_ident}{write}[*id].take();");
                                    emit!(buffer, "self.{obj_ident}_free_list.lock().unwrap().push(*id);");
                                },
                                Single => {
                                    emit!(buffer, "let result = self.{obj_ident}[*id].take();");
                                    emit!(buffer, "self.{obj_ident}_free_list.push(*id);");
                                },
                                store => panic!("{store} is not currently supported"),
                            }
                            emit!(buffer, "result");
                        }
                    } else if timestamp {
                        emit!(
                            buffer,
                            "self.{0}.remove(id).map(|{0}| {0}.0)",
                            obj_ident
                        );
                    } else {
                        emit!(buffer, "self.{obj_ident}.remove(id)");
                    }
                    emit!(buffer, "}}");
                    emit!(buffer, "");

                    if object_has_name(obj, domain) {
                        emit!(
                            buffer,
                            "/// Exorcise [`{}`] id from the store by name.",
                            obj.as_type(&Ownership::new_borrowed(), woog, domain)
                        );
                        emit!(buffer, "///");
                        emit!(buffer, "#[inline]");

                        if is_uber {
                            let (read, _write) = get_uber_read_write(config);
                            use UberStoreOptions::*;
                            match config.get_uber_store().unwrap() {
                                Disabled => unreachable!(),
                                AsyncRwLock => {
                                    emit!(
                                        buffer,
                                        "pub async fn exhume_{obj_ident}_id_by_name(&self, name: &str) -> Option<usize> {{",
                                    );
                                }
                                _ => {
                                    emit!(
                                        buffer,
                                        "pub fn exhume_{obj_ident}_id_by_name(&self, name: &str) -> Option<usize> {{",
                                    );
                                }
                            }
                            if timestamp {
                                emit!(
                                    buffer,
                                    "self.{0}_id_by_name.get(name).map(|{0}| {0}.0)",
                                    obj_ident
                                );
                            } else {
                                match config.get_uber_store().unwrap() {
                                    AsyncRwLock | StdRwLock | ParkingLotRwLock | NDRwLock => {
                                        emit!(buffer, "self.{0}_id_by_name{read}.get(name).map(|{0}| *{0})", obj_ident);
                                    }
                                    Single => {
                                        emit!(buffer, "self.{0}_id_by_name.get(name).map(|{0}| *{0})", obj_ident);
                                    }
                                    store => panic!("{store} is not currently supported"),
                                }
                            }
                        } else if timestamp {
                            emit!(
                                buffer,
                                "pub fn exhume_{obj_ident}_id_by_name(&self, name: &str) -> Option<usize> {{",
                            );
                            emit!(
                                buffer,
                                "self.{obj_ident}_id_by_name.get(name).map(|{obj_ident}| {obj_ident}.0)",
                            );
                        } else {
                            // 🚧 Is this right? We are changing the signature of the method, which
                            // i think is bad as it will break existing code.
                            emit!(
                                buffer,
                                "pub fn exhume_{obj_ident}_id_by_name(&self, name: &str) -> Option<&usize> {{",
                            );
                            emit!(buffer, "self.{obj_ident}_id_by_name.get(name)");
                        }
                        emit!(buffer, "}}");
                        emit!(buffer, "");
                    }

                    // 🚦
                    // Generate iter_ methods
                    emit!(
                        buffer,
                        "/// Get an iterator over the internal `HashMap<&Uuid, {}>`.",
                        obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    );
                    emit!(buffer, "///");
                    emit!(buffer, "#[inline]");

                    if is_uber {
                        use UberStoreOptions::*;
                        let store_type = match config.get_uber_store().unwrap() {
                            Disabled => unreachable!(),
                            Single => format!(
                                "Rc<RefCell<{}>>",
                                obj.as_type(&Ownership::new_borrowed(), woog, domain)
                            ),
                            StdRwLock |
                            ParkingLotRwLock |
                            AsyncRwLock |
                            NDRwLock => format!(
                                "Arc<RwLock<{}>>",
                                obj.as_type(&Ownership::new_borrowed(), woog, domain)
                            ),
                            StdMutex | ParkingLotMutex => format!(
                                "Arc<Mutex<{}>>",
                                obj.as_type(&Ownership::new_borrowed(), woog, domain)
                            ),
                        };
                        match config.get_uber_store().unwrap() {
                            Disabled => unreachable!(),
                            AsyncRwLock => {
                                emit!(
                                    buffer,
                                    "pub async fn iter_{obj_ident}(&self) -> impl stream::Stream<Item = {store_type}> + '_ {{",
                                );
                            }
                            _ => {
                                emit!(
                                    buffer,
                                    "pub fn iter_{obj_ident}(&self) -> impl Iterator<Item = {store_type}> + '_ {{",
                                );
                            }
                        }

                    } else {
                        emit!(
                            buffer,
                            "pub fn iter_{obj_ident}(&self) -> impl Iterator<Item = &{obj_type}> {{",
                        );
                    }

                    if is_uber {
                        use UberStoreOptions::*;
                        let (read, _write) = get_uber_read_write(config);
                        let store_type = match config.get_uber_store().unwrap() {
                            Disabled => unreachable!(),
                            Single => format!(
                                "Vec<Option<Rc<RefCell<{}>>>>",
                                obj.as_type(&Ownership::new_borrowed(), woog, domain)
                            ),
                            StdRwLock | ParkingLotRwLock | AsyncRwLock | NDRwLock => format!(
                                "Vec<Option<Arc<RwLock<{}>>>>",
                                obj.as_type(&Ownership::new_borrowed(), woog, domain)
                            ),
                            StdMutex | ParkingLotMutex => format!(
                                "Vec<Arc<Mutex<{}>>>",
                                obj.as_type(&Ownership::new_borrowed(), woog, domain)
                            ),
                        };
                        if timestamp {
                            emit!(
                                buffer,
                                "let values: {store_type} = self.{obj_ident}{read}.values().map(|{obj_ident}| {obj_ident}.0.clone()).collect();",
                            );
                            emit!(
                            buffer,
                            "let len = values.len();"
                            );
                            emit!(
                                buffer,
                                "(0..len).filter(|i| values[*i].is_some()).map(move|i| values[i].clone())",
                            );
                        } else {
                            let (read, _write) = get_uber_read_write(config);
                            use UberStoreOptions::*;
                            match config.get_uber_store().unwrap() {
                                AsyncRwLock => {
                                    emit!(
                                        buffer,
                                        "let len = self.{obj_ident}{read}.len();"
                                    );
                                    emit!(
                                        buffer,
                                        "stream::iter((0..len)).filter_map(move |i| async move {{if self.{obj_ident}{read}[i].is_some(){{self.{obj_ident}{read}[i].clone()}} else {{ None }} }} )",
                                    );
                                },
                                StdRwLock | ParkingLotRwLock | NDRwLock => {
                                    emit!(
                                        buffer,
                                        "let len = self.{obj_ident}{read}.len();"
                                    );
                                    emit!(
                                        buffer,
                                        "(0..len).filter(|i| self.{obj_ident}{read}[*i].is_some()).map(move|i|{{self.{obj_ident}{read}[i].as_ref().map(|{obj_ident}| {obj_ident}.clone()).unwrap()}})",
                                    );
                                },
                                Single => {
                                    emit!(
                                        buffer,
                                        "let len = self.{obj_ident}.len();"
                                    );
                                    emit!(
                                        buffer,
                                        "(0..len).filter(|i| self.{obj_ident}[*i].is_some()).map(move|i|{{self.{obj_ident}[i].as_ref().map(|{obj_ident}| {obj_ident}.clone()).unwrap()}})",
                                    );
                                },
                                store => panic!("{store} is not currently supported"),
                            }
                        }
                    } else if timestamp {
                        emit!(
                            buffer,
                            "self.{obj_ident}.values().map(|{obj_ident}| &{obj_ident}.0)",
                        );
                    } else {
                        emit!(buffer, "self.{obj_ident}.values()");
                    }

                    emit!(buffer, "}}");
                    emit!(buffer, "");

                    // 🚦
                    // Generate code to get timestamp
                    if timestamp {
                        emit!(
                            buffer,
                            "/// Get the timestamp for {}.",
                            obj.as_type(&Ownership::new_borrowed(), woog, domain)
                        );
                        emit!(buffer, "///");
                        emit!(buffer, "#[inline]");

                        if is_uber {
                            use UberStoreOptions::*;
                            match config.get_uber_store().unwrap() {
                                Disabled => unreachable!(),
                                AsyncRwLock => {
                                    emit!(
                                        buffer,
                                        "pub async fn {obj_ident}_timestamp(&self, {obj_ident}: &{obj_type}) -> SystemTime {{",
                                    );
                                }
                                _ => {
                                    emit!(
                                        buffer,
                                        "pub fn {obj_ident}_timestamp(&self, {obj_ident}: &{obj_type}) -> SystemTime {{",
                                    );
                                }
                            }
                        } else {
                            emit!(
                                buffer,
                                "pub fn {obj_ident}_timestamp(&self, {obj_ident}: &{obj_type}) -> SystemTime {{",
                            );
                        }

                        let verb = if is_uber {
                            let (read, _write) = get_uber_read_write(config);
                            format!("{read}.get")
                        } else {
                            ".get".to_owned()
                        };

                        if local_object_is_enum(obj, config, domain) {
                            emit!(
                                buffer,
                                "self.{obj_ident}{verb}(&{obj_ident}.id()).map(|{obj_ident}| {obj_ident}.1).unwrap_or(SystemTime::now())",
                            );
                        } else {
                            emit!(
                                buffer,
                                "self.{obj_ident}{verb}(&{obj_ident}.id).map(|{obj_ident}| {obj_ident}.1).unwrap_or(SystemTime::now())",
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

impl ObjectStoreDefinition for DomainStoreVec {}

impl CodeWriter for DomainStoreVec {
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

        // This is used internally to generate some code to initialize the store
        // with const values. It's pretty gnarly.
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
                Ok(false)
            } else {
                for include in includes {
                    emit!(buffer, "{},", include);
                }

                Ok(true)
            }
        }

        // This is used internally to generate some code to initialize the store
        // with const values. It's pretty gnarly.
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

                let prefix = format!(
                    "{prefix} super::{}Enum::{}",
                    sup.as_type(&Ownership::new_borrowed(), woog, domain),
                    s_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );

                if !config.is_imported(&s_obj.id) {
                    // if local_object_is_supertype(s_obj, config, domain) {
                    //     let prefix = format!("{prefix}(");
                    //     let suffix = format!(".id()){suffix}");
                    //     emit_singleton_subtype_instances(
                    //         s_obj, &prefix, &suffix, config, domain, woog, buffer,
                    //     )?;
                    // } else if local_object_is_singleton(s_obj, config, domain) {
                    if local_object_is_singleton(s_obj, config, domain) {
                        writeln!(buffer, "{prefix}({}){suffix}", s_obj.as_const())
                            .context(FormatSnafu)?;
                    }
                }
            }

            Ok(())
        }

        // This is actually the beginning of the function.
        let mut objects: Vec<&Object> = domain.sarzak().iter_object().collect();
        objects.sort_by(|a, b| a.name.cmp(&b.name));
        let supertypes = objects
            .iter()
            .filter(|obj| {
                !config.is_imported(&obj.id) && local_object_is_supertype(obj, config, domain)
            })
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

        let timestamp = config.get_persist_timestamps();
        let is_meta = config.is_meta_model();
        let has_name = objects
            .iter()
            .map(|obj| object_has_name(obj, domain))
            .any(|x| x);
        let is_uber = config.is_uber_store();

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-object-store-definition", module),
            |buffer| {
                let persist = config.get_persist();

                if persist {
                    if timestamp {
                        emit!(buffer, "use std::{{io::{{self, prelude::*}}, fs, path::Path, time::SystemTime}};");
                    } else {
                        emit!(buffer, "use std::{{io::{{self, prelude::*}}, fs, path::Path}};");
                    }
                }
                if is_uber {
                    use UberStoreOptions::*;
                    match config.get_uber_store().unwrap() {
                        Disabled => unreachable!(),
                        AsyncRwLock => {
                            emit!(buffer, "use async_std::sync::Arc;");
                            emit!(buffer, "use async_std::sync::RwLock;");
                            emit!(buffer, "use std::fmt;");
                            emit!(buffer, "use serde::{{ser::SerializeStruct, Serializer, Deserializer, de::{{self, Visitor, MapAccess}}}};");
                            emit!(buffer, "use futures::stream::{{self, StreamExt}};");
                        }
                        NDRwLock => {
                            emit!(buffer, "use std::sync::Arc;");
                            emit!(buffer, "use no_deadlocks::RwLock;");
                        }
                        Single => {
                            emit!(buffer, "use std::cell::RefCell;");
                            emit!(buffer, "use std::rc::Rc;")
                        },
                        StdRwLock => {
                            emit!(buffer, "use std::sync::Arc;");
                            emit!(buffer, "use std::sync::RwLock;")
                        }
                        StdMutex => {
                            emit!(buffer, "use std::sync::Arc;");
                            emit!(buffer, "use std::sync::Mutex;")
                        }
                        ParkingLotRwLock => {
                            emit!(buffer, "use std::sync::Arc;");
                            emit!(buffer, "use parking_lot::RwLock;")
                        }
                        ParkingLotMutex => {
                            emit!(buffer, "use std::sync::Arc;");
                            emit!(buffer, "use parking_lot::Mutex;")
                        }
                    };
                }
                emit!(buffer, "");
                emit!(buffer, "use rustc_hash::FxHashMap as HashMap;");
                emit!(buffer, "use serde::{{Deserialize, Serialize}};");
                emit!(buffer, "use uuid::Uuid;");
                if has_name {
                    emit!(buffer, "use heck::ToUpperCamelCase;");
                }
                #[allow(clippy::overly_complex_bool_expr)]
                if timestamp && is_meta && false {
                    emit!(buffer, "use snafu::prelude::*;");
                    emit!(buffer, "");
                    emit!(buffer, "use crate::mc::{{FileSnafu, Result}};");
                } else {
                    emit!(buffer, "");
                }
                emit!(buffer, "");
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

                use UberStoreOptions::*;
                match config.get_uber_store().unwrap() {
                    AsyncRwLock => emit!(buffer, "#[derive(Debug)]"),
                    Single | ParkingLotRwLock | StdRwLock => emit!(buffer, "#[derive(Debug, Deserialize, Serialize)]"),
                    NDRwLock=> emit!(buffer, "#[derive(Debug)]"),
                    _ => emit!(buffer, "#[derive(Clone, Debug, Deserialize, Serialize)]")
                }

                emit!(buffer, "pub struct ObjectStore {{");
                for obj in &objects {
                    let obj_ident = obj.as_ident();

                    use UberStoreOptions::*;
                    match config.get_uber_store().unwrap() {
                        StdRwLock | ParkingLotRwLock
                        | NDRwLock => emit!(buffer, "{obj_ident}_free_list: std::sync::Mutex<Vec<usize>>,"),
                        AsyncRwLock => emit!(buffer, "{obj_ident}_free_list: async_std::sync::Mutex<Vec<usize>>,"),

                        Single => emit!(buffer, "{obj_ident}_free_list: Vec<usize>,"),
                        store => panic!("{store} is not currently supported"),
                    }

                    let value_type = get_value_wrapper(is_uber, config, obj, woog, domain);
                    if timestamp {
                        if is_uber {
                            use UberStoreOptions::*;
                            let mother_of_all_types = match config.get_uber_store().unwrap() {
                                Disabled => unreachable!(),
                                Single => format!(
                                    "Rc<RefCell<Vec<Option<({}, SystemTime)>>>>",
                                    value_type
                                ),
                                StdRwLock |
                                ParkingLotRwLock |
                                AsyncRwLock |
                                NDRwLock => format!(
                                    "Arc<RwLock<Vec<Option<({}, SystemTime)>>>>>",
                                    value_type
                                ),
                                StdMutex | ParkingLotMutex => format!(
                                    "Arc<Mutex<Vec<({}, SystemTime)>>>",
                                    value_type
                                ),
                            };
                            emit!( buffer, "{obj_ident}: {mother_of_all_types},");
                            if object_has_name(obj, domain) {
                                use UberStoreOptions::*;
                                let by_name_type = match config.get_uber_store().unwrap() {
                                    Disabled => unreachable!(),
                                    Single => "HashMap<String, (usize, SystemTime)>",
                                    StdRwLock |
                                    ParkingLotRwLock |
                                    AsyncRwLock |
                                    NDRwLock => "Arc<RwLock<HashMap<String, (usize, SystemTime)>>>",
                                    StdMutex | ParkingLotMutex => "Arc<Mutex<HashMap<String, (usize, SystemTime)>>>",
                                };
                                emit!(buffer, "{obj_ident}_id_by_name: {by_name_type},");
                            }
                        } else {
                            emit!(buffer, "{obj_ident}: Vec<({value_type}, SystemTime)>,");
                            if object_has_name(obj, domain) {
                                emit!(buffer, "{obj_ident}_id_by_name: HashMap<String, (usize, SystemTime)>,");
                            }
                        }
                    } else if is_uber {
                        use UberStoreOptions::*;
                        let mother_of_all_types = match config.get_uber_store().unwrap() {
                            Disabled => unreachable!(),
                            Single => format!("Vec<Option<{value_type}>>"),
                            StdRwLock |
                            ParkingLotRwLock |
                            AsyncRwLock |
                            NDRwLock => format!("Arc<RwLock<Vec<Option<{value_type}>>>>"),
                            StdMutex | ParkingLotMutex => format!("Arc<Mutex<Vec<{value_type}>>>"),
                        };
                        emit!(buffer, "{obj_ident}: {mother_of_all_types},");
                        if object_has_name(obj, domain) {
                            use UberStoreOptions::*;
                            let by_name_type = match config.get_uber_store().unwrap() {
                                Disabled => unreachable!(),
                                Single => "HashMap<String, usize>",
                                StdRwLock |
                                ParkingLotRwLock |
                                AsyncRwLock |
                                NDRwLock => "Arc<RwLock<HashMap<String, usize>>>",
                                StdMutex | ParkingLotMutex => "Arc<Mutex<HashMap<String, usize>>>",
                            };
                            emit!(buffer, "{obj_ident}_id_by_name: {by_name_type},");
                        }
                    } else {
                        emit!(buffer, "{obj_ident}: Vec<{value_type}>,");
                        if object_has_name(obj, domain) {
                            emit!(buffer, "{obj_ident}_id_by_name: HashMap<String, usize>,");
                        }
                    }
                }
                emit!(buffer, "}}");
                emit!(buffer, "");

                if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap() {
                    emit!(buffer, "impl Serialize for ObjectStore {{");
                    emit!(buffer, "fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>");
                    emit!(buffer, "where S: Serializer, {{");
                    emit!(buffer, "let mut map = serializer.serialize_struct(\"ObjectStore\", {})?;", objects.len());
                    for obj in &objects {
                        let obj_ident = obj.as_ident();
                        let obj_type = obj.as_type(&Ownership::new_borrowed(), woog, domain);
                        emit!(buffer, r#"
        let {obj_ident} = futures::executor::block_on(async {{ self.{obj_ident}.read().await }}).clone();
        let values: Vec<{obj_type}> = {obj_ident}
            .into_iter()
            .filter_map(|{obj_ident}| {{
                if let Some({obj_ident}) = {obj_ident} {{
                    Some(futures::executor::block_on(async {{ {obj_ident}.read().await }}).clone())
                }} else {{
                    None
                }}
            }})
            .collect();
        map.serialize_field("{obj_ident}", &values)?;
"#);
                        // emit!(buffer, "let {obj_ident} = (*futures::executor::block_on(async {{self.{obj_ident}.read().await}})).clone();");
                        // emit!(buffer, r#"map.serialize_entry("{obj_ident}".to_owned());"#);
                        // emit!(buffer, "for v in {obj_ident}.iter() {{");
                        // emit!(buffer, "// may as well compress it while we are here");
                        // emit!(buffer, "if let Some(v) = v {{");
                        // emit!(buffer, "seq.serialize_element(&(*futures::executor::block_on(async {{v.read().await}})).clone())?;");
                        // emit!(buffer, "}}");
                        // emit!(buffer, "}}");
                        // emit!(buffer, "let result = map.end();\n");
                    }
                    emit!(buffer, "map.end()");
                    emit!(buffer, "}}");
                    emit!(buffer, "}}\n");

                    emit!(buffer, "impl<'de> Deserialize<'de> for ObjectStore {{");
                    emit!(buffer, "fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>");
                    emit!(buffer, "where D: Deserializer<'de>, {{");
                    emit!(buffer, "enum SerdeField {{");
                    for obj in &objects {
                        let obj_type = obj.as_type(&Ownership::new_borrowed(), woog, domain);
                        emit!(
                            buffer,
                            "{obj_type},"
                        );
                    }
                    emit!(buffer, "}}");
                    emit!(buffer, "impl<'de> Deserialize<'de> for SerdeField {{");
                    emit!(buffer, "fn deserialize<D>(deserializer: D) -> Result<SerdeField, D::Error>");
                    emit!(buffer, "where D: Deserializer<'de>, {{");
                    emit!(buffer, "struct FieldVisitor;");
                    emit!(buffer, "impl<'de> Visitor<'de> for FieldVisitor {{");
                    emit!(buffer, "type Value = SerdeField;");
                    emit!(buffer, "fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {{");
                    emit!(buffer, "formatter.write_str(\"field identifier\")");
                    emit!(buffer, "}}");
                    emit!(buffer, "fn visit_str<E>(self, value: &str) -> Result<SerdeField, E>");
                    emit!(buffer, "where E: de::Error, {{");
                    emit!(buffer, "match value {{");
                    for obj in &objects {
                        let obj_ident = obj.as_ident();
                        let obj_type = obj.as_type(&Ownership::new_borrowed(), woog, domain);
                        emit!(
                            buffer,
                            "\"{obj_ident}\" => Ok(SerdeField::{obj_type}),"
                        );
                    }
                    emit!(buffer, "_ => Err(de::Error::unknown_field(value, FIELDS)),");
                    emit!(buffer, "}}");
                    emit!(buffer, "}}");
                    emit!(buffer, "}}");
                    emit!(buffer, "deserializer.deserialize_identifier(FieldVisitor)");
                    emit!(buffer, "}}");
                    emit!(buffer, "}}");
                    emit!(buffer, "struct ObjectStoreVisitor;");
                    emit!(buffer, "impl<'de> Visitor<'de> for ObjectStoreVisitor {{");
                    emit!(buffer, "type Value = ObjectStore;");

                    emit!(buffer, "fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {{");
                    emit!(buffer, "formatter.write_str(\"struct ObjectStore\")");
                    emit!(buffer, "}}");

                    emit!(buffer, "fn visit_map<A>(self, mut map: A) -> Result<ObjectStore, A::Error>");
                    emit!(buffer, "where A: MapAccess<'de>, {{");
                    emit!(buffer, "let result = ObjectStore::new();");
                    emit!(buffer, "let mut result = futures::executor::block_on(async {{ result.await }});");
                    emit!(buffer, "while let Some(key) = map.next_key()? {{");
                    emit!(buffer, "match key {{");
                    for obj in &objects {
                        let obj_ident = obj.as_ident();
                        let obj_type = obj.as_type(&Ownership::new_borrowed(), woog, domain);

                        emit!(
                            buffer,
                            r#"SerdeField::{obj_type} => {{
                                let mut guard = futures::executor::block_on(result.{obj_ident}.write());
                                let values: Vec<{obj_type}> = map.next_value()?;
                                for value in values {{
                                    guard.push(Some(Arc::new(RwLock::new(value))));
                                }}
                            }}"#
                        );
                    }
                    emit!(buffer, "}}");
                    emit!(buffer, "}}");
                    emit!(buffer, "Ok(result)");
                    emit!(buffer, "}}\n");

                    // emit!(buffer, "fn visit_seq<A>(self, mut seq: A) -> Result<ObjectStore, A::Error>");
                    // emit!(buffer, "where A: SeqAccess<'de>, {{");
                    // emit!(buffer, "let result = ObjectStore::new();");
                    // emit!(buffer, "let mut result = futures::executor::block_on(async {{ result.await }});");
                    // // for obj in &objects {
                    // //     let obj_ident = obj.as_ident();
                    // //     let obj_type = obj.as_type(&Ownership::new_borrowed(), woog, domain);

                    // //     emit!(
                    // //         buffer,
                    // //         r#"SerdeField::{obj_type} => futures::executor::block_on(async {{
                    // //         let guard = result.{obj_ident}.write().await;
                    // //         // This unwrap is unfortunate.
                    // //         for value in map.next_value::<Vec<Option<{obj_type}>>>().unwrap() {{
                    // //             let value = match value {{
                    // //                 Some(value) => Some(Arc::new(RwLock::new(value))),
                    // //                 None => None,
                    // //             }};
                    // //             guard.push(value);
                    // //         }}
                    // //     }}),"#,
                    // //     );
                    // // }
                    // for (n, obj) in objects.iter().enumerate() {
                    //     let obj_ident = obj.as_ident();

                    //     if n == 0 {
                    //         emit!(
                    //             buffer,
                    //             "result.{obj_ident} = Arc::new(RwLock::new(seq.next_element()?)).ok_or_else(|| de::Error::invalid_length({n}, &self))?;;",
                    //         );
                    //     }
                    // }
                    // emit!(buffer, "Ok(result)");
                    // emit!(buffer, "}}");
                    emit!(buffer, "}}\n");

                    for obj in &objects {
                        let _obj_ident = obj.as_ident();
                        let obj_type = obj.as_type(&Ownership::new_borrowed(), woog, domain);

                        emit!(buffer, "struct {obj_type}Visitor;");
                        emit!(buffer, "impl<'de> Visitor<'de> for {obj_type}Visitor {{");
                        emit!(buffer, "type Value = Arc<RwLock<HashMap<Uuid, Arc<RwLock<{obj_type}>>>>>;");
                        // emit!(buffer, "type Value = Arc<RwLock<HashMap<Uuid, (Arc<RwLock<{obj_type}>>, SystemTime)>>>;");
                        emit!(buffer, "fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {{");
                        emit!(buffer, "formatter.write_str(\"{obj_type} map\")");
                        emit!(buffer, "}}");
                        emit!(buffer, "fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>");
                        emit!(buffer, "where M: MapAccess<'de>, {{");
                        emit!(buffer, "let mut map = HashMap::default();");
                        emit!(buffer, "while let Some((key, value)) = access.next_entry::<Uuid, {obj_type}>()? {{");
                        // emit!(buffer, "while let Some((key, value)) = access.next_entry::<Uuid, ({obj_type}, SystemTime)>()? {{");
                        emit!(buffer, "map.insert(key, Arc::new(RwLock::new(value)));");
                        emit!(buffer, "}}");
                        emit!(buffer, "Ok(Arc::new(RwLock::new(map)))");
                        emit!(buffer, "}}}}\n");
                    }
                    emit!(buffer, "const FIELDS: &'static [&'static str] = &[");
                    for obj in &objects {
                        let obj_ident = obj.as_ident();
                        emit!(buffer, "\"{obj_ident}\",");
                    }
                    emit!(buffer, "];");
                    emit!(buffer, "deserializer.deserialize_struct(\"ObjectStore\", FIELDS, ObjectStoreVisitor)");
                    emit!(buffer, "}}}}\n");
                }


                // impl ObjectStore
                emit!(buffer, "impl ObjectStore {{");
                if is_uber {
                    use UberStoreOptions::*;
                    match config.get_uber_store().unwrap() {
                        Disabled => unreachable!(),
                        AsyncRwLock => {
                            emit!(buffer, "pub async fn new() -> Self {{");
                        }
                        _ => {
                            emit!(buffer, "pub fn new() -> Self {{");
                        }
                    }
                } else {
                    emit!(buffer, "pub fn new() -> Self {{");
                }
                if singleton_subs {
                    emit!(buffer, "let mut store = Self {{");
                } else {
                    emit!(buffer, "let store = Self {{");
                }
                for obj in &objects {
                    let obj_ident = obj.as_ident();
                    if is_uber {
                        use UberStoreOptions::*;
                        match config.get_uber_store().unwrap() {
                            StdRwLock | ParkingLotRwLock | NDRwLock => emit!(buffer, "{obj_ident}_free_list: std::sync::Mutex::new(Vec::new()),"),
                            AsyncRwLock => emit!(buffer, "{obj_ident}_free_list: async_std::sync::Mutex::new(Vec::new()),"),
                            Single => emit!(buffer, "{obj_ident}_free_list: Vec::new(),"),
                            store => panic!("{store} is not currently supported"),
                        }

                        let ctor = match config.get_uber_store().unwrap() {
                            Disabled => unreachable!(),
                            Single => "Vec::new()",
                            StdRwLock |
                            ParkingLotRwLock |
                            AsyncRwLock |
                            NDRwLock => "Arc::new(RwLock::new(Vec::new()))",
                            StdMutex |
                            ParkingLotMutex => "Arc::new(Mutex::new(HashMap::default()))",
                        };
                        emit!(buffer, "{}: {ctor},", obj.as_ident());

                        if object_has_name(obj, domain) {
                            match config.get_uber_store().unwrap() {
                                AsyncRwLock | StdRwLock | ParkingLotRwLock| NDRwLock => {
                                    emit!(buffer, "{obj_ident}_id_by_name: Arc::new(RwLock::new(HashMap::default())),");
                                }
                                Single => {
                                    emit!(buffer, "{obj_ident}_id_by_name: HashMap::default(),");
                                }
                                store => panic!("{store} is not currently supported"),
                            }
                        }
                    } else {
                        emit!(buffer, "{}: HashMap::default(),", obj.as_ident());
                        if object_has_name(obj, domain) {
                            emit!(buffer, "{obj_ident}_id_by_name: HashMap::default(),");
                        }
                    }

                }
                emit!(buffer, "}};");
                emit!(buffer, "");
                emit!(buffer, "// Initialize Singleton Subtypes");
                emit!(buffer, "// 💥 Look at how beautiful this generated code is for super/sub-type graphs!");
                emit!(buffer, "// I remember having a bit of a struggle making it work. It's recursive, with");
                emit!(buffer, "// a lot of special cases, and I think it calls other recursive functions...💥");
                for obj in &supertypes {
                    let obj_ident = obj.as_ident();

                    if is_uber {
                        use UberStoreOptions::*;
                        let (ctor, tail) = match config.get_uber_store().unwrap() {
                            Disabled => unreachable!(),
                            Single => (
                                format!(
                                    "Rc::new(RefCell::new({} {{ subtype: ",
                                    obj.as_type(&Ownership::new_borrowed(), woog, domain)),
                                ",id}))});"
                            ),
                            StdRwLock | NDRwLock => (
                                format!(
                                    "Arc::new(RwLock::new({} {{ subtype: ",
                                    obj.as_type(&Ownership::new_borrowed(), woog, domain)),
                                ",id}))});"
                            ),
                            ParkingLotRwLock => ("Arc::new(RwLock::new(".to_owned(), "))});"),
                            AsyncRwLock => (
                                format!(
                                    "Arc::new(RwLock::new({} {{ subtype: ",
                                    obj.as_type(&Ownership::new_borrowed(), woog, domain)),
                                ",id}))}).await;"
                            ),
                            StdMutex | ParkingLotMutex => ("Arc::new(Mutex::new(".to_owned(), ")));"),
                        };

                        let attrs = obj.r1_attribute(domain.sarzak());
                        let referrers = get_binary_referrers_sorted!(obj, domain.sarzak());
                        let mut attr_len = attrs.len() + referrers.len();
                       // The "hack" thing is added when we first process the
                       // domain. If it's a Vec store, we want to promote any
                       // enums to hybrids.
                        for attr in &attrs {
                            if attr.name == "hack" {
                                attr_len -= 1;
                                continue;
                            }
                        }
                        if attr_len < 2 {
                            emit_singleton_subtype_instances(
                                obj,
                                &format!("store.inter_{obj_ident}(|id| {{ {ctor}"),
                                tail,
                                config,
                                domain,
                                woog,
                                buffer,
                            )?;
                        }
                    } else {
                        emit_singleton_subtype_instances(
                            obj,
                            &format!("store.inter_{obj_ident}("),
                            ");",
                            config,
                            domain,
                            woog,
                            buffer,
                        )?;
                    }
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

/// Check to see if an object has a name attribute
///
/// I'm using this to generate "by name" lookup for objects that have a name.
/// This is, this is only useful for objects with unique names. And we only
/// seem to need it for `Object` so far.
///
/// So I'm short-circuiting this now.
///
/// What a hack. These really need to be colored, or marked, or whatever.
/// // 🚧 This needs to return the type of string manipulation to use on the
/// name. Or maybe we don't do one at all, and let the end user sort it out.
/// I sort of like that option better. I wonder how many errors will ensue...
fn object_has_name(obj: &Object, _domain: &Domain) -> bool {
    obj.name == "Object"
        || obj.name == "Struct"
        || obj.name == "Function"
        || obj.name == "Field"
        || obj.name == "Object Store"
        || obj.name == "Enumeration"
        || obj.name == "Plugin"
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
            let is_uber = config.is_uber_store();

            emit!(buffer, "/// Persist the store.");
            emit!(buffer, "///");
            emit!(
                buffer,
                "/// The store is persisted as a a bincode file."
            );
            emit!(
                buffer,
                "pub fn persist_bincode<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {{"
            );
            emit!(buffer, "let path = path.as_ref();");
            emit!(buffer, "let mut bin_file = fs::File::create(path)?;");
            emit!(
                buffer,
                "let encoded: Vec<u8> = bincode::serialize(&self).unwrap();"
            );
            emit!(buffer, "bin_file.write_all(&encoded)?;");
            emit!(buffer, "Ok(())");
            emit!(buffer, "}}\n");

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
            if is_uber {
                use UberStoreOptions::*;
                match config.get_uber_store().unwrap() {
                    Disabled => unreachable!(),
                    AsyncRwLock => {
                        emit!(
                            buffer,
                            "pub async fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {{"
                        );
                    }
                    _ => {
                        emit!(
                            buffer,
                            "pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {{"
                        );
                    }
                }
            } else {
                emit!(
                    buffer,
                    "pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {{"
                );
            }
            emit!(buffer, "let path = path.as_ref();");
            emit!(buffer, "fs::create_dir_all(path)?;");
            emit!(buffer, "");
            // This is such a great joke! 🤣
            emit!(buffer, "let path = path.join(\"{}.json\");", domain.name());
            emit!(buffer, "fs::create_dir_all(&path)?;");
            emit!(buffer, "");

            for obj in objects {
                let obj_ident = obj.as_ident();
                let obj_type = obj.as_type(&Ownership::new_borrowed(), woog, domain);
                let id = if local_object_is_enum(obj, config, domain) {
                    "id()"
                } else {
                    "id"
                };

                emit!(buffer, "// Persist {}.", obj.name);
                emit!(buffer, "{{");
                emit!(buffer, "let path = path.join(\"{}\");", obj.as_ident());
                emit!(buffer, "fs::create_dir_all(&path)?;");
                if timestamp {
                    if is_uber {
                        let (read, _write) = get_uber_read_write(config);
                        emit!(
                            buffer,
                            "for {obj_ident}_tuple in self.{obj_ident}{read}.values() {{"
                        );
                        emit!(
                            buffer,
                            "let path = path.join(format!(\"{{}}.json\", {obj_ident}_tuple.0{read}.{id}));"
                        );
                    } else {
                        emit!(
                            buffer,
                            "for {obj_ident}_tuple in self.{obj_ident}.values() {{"
                        );
                        emit!(
                            buffer,
                            "let path = path.join(format!(\"{{}}.json\", {obj_ident}_tuple.0.{id}));"
                        );
                    }

                    emit!(buffer, "if path.exists() {{");
                    emit!(buffer, "let file = fs::File::open(&path)?;");
                    emit!(buffer, "let reader = io::BufReader::new(file);");

                    if is_uber {
                        let store_type = get_value_wrapper(is_uber, config, obj, woog, domain);
                        use UberStoreOptions::*;
                        match config.get_uber_store().unwrap() {
                            Disabled => unreachable!(),
                            AsyncRwLock => {
                                emit!(
                                    buffer,
                                    "let on_disk: ({store_type}, SystemTime) = serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;"
                                );

                            }
                            _ => {
                                emit!(
                                    buffer,
                                    "let on_disk: ({store_type}, SystemTime) = serde_json::from_reader(reader)?;"
                                );
                            }
                        }
                    } else {
                        emit!(
                            buffer,
                            "let on_disk: ({obj_type}, SystemTime) = serde_json::from_reader(reader)?;"
                        );
                    }

                    if is_uber {
                        let (read, _write) = get_uber_read_write(config);
                        emit!(buffer,
                            "if on_disk.0{read}.to_owned() != {obj_ident}_tuple.0{read}.to_owned() {{"
                        );
                    } else {
                        emit!(buffer, "if on_disk.0 != {obj_ident}_tuple.0 {{");
                    }

                    emit!(buffer, "let file = fs::File::create(path)?;");
                    emit!(buffer, "let mut writer = io::BufWriter::new(file);");
                    if is_uber {
                        use UberStoreOptions::*;
                        let _store_type = get_value_wrapper(is_uber, config, obj, woog, domain);
                        match config.get_uber_store().unwrap() {
                            Disabled => unreachable!(),
                            AsyncRwLock => {
                                emit!(
                                    buffer,
                                    "serde_json::to_writer_pretty(&mut writer, &(&{obj_ident}_tuple.0.read().await.to_owned(), &{obj_ident}_tuple.1))?;"
                                );
                            }
                            _ => {
                                emit!(
                                    buffer,
                                    "serde_json::to_writer_pretty(&mut writer, &{obj_ident}_tuple)?;"
                                );
                            }
                        }
                    } else {
                        emit!(
                            buffer,
                            "serde_json::to_writer_pretty(&mut writer, &{obj_ident}_tuple)?;"
                        );
                    }
                    emit!(buffer, "}}");

                    emit!(buffer, "}} else {{");
                    emit!(buffer, "let file = fs::File::create(&path)?;");
                    emit!(buffer, "let mut writer = io::BufWriter::new(file);");
                    if is_uber {
                        use UberStoreOptions::*;
                        let _store_type = get_value_wrapper(is_uber, config, obj, woog, domain);
                        match config.get_uber_store().unwrap() {
                            Disabled => unreachable!(),
                            AsyncRwLock => {
                                emit!(
                                    buffer,
                                    "serde_json::to_writer_pretty(&mut writer, &(&{obj_ident}_tuple.0.read().await.to_owned(), &{obj_ident}_tuple.1))?;"
                                );
                            }
                            _ => {
                                emit!(
                                    buffer,
                                    "serde_json::to_writer_pretty(&mut writer, &{obj_ident}_tuple)?;"
                                );
                            }
                        }
                    } else {
                        emit!(
                            buffer,
                            "serde_json::to_writer_pretty(&mut writer, &{obj_ident}_tuple)?;"
                        );
                    }

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
                    emit!(buffer, "let id = file_name.split('.').next().unwrap();");
                    emit!(buffer, "if let Ok(id) = Uuid::parse_str(id) {{");
                    if is_uber {
                        let (read, _write) = get_uber_read_write(config);
                        emit!(buffer, "if !self.{obj_ident}{read}.contains_key(&id) {{");
                    } else {
                        emit!(buffer, "if !self.{obj_ident}.contains_key(&id) {{");
                    }
                    #[allow(clippy::overly_complex_bool_expr)]
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
                    if is_uber {
                        let (read, _write) = get_uber_read_write(config);
                        use UberStoreOptions::*;
                        match config.get_uber_store().unwrap() {
                            AsyncRwLock | StdRwLock | ParkingLotRwLock | NDRwLock => {
                                emit!(buffer, "for {obj_ident} in &*self.{obj_ident}{read} {{");
                            },
                            Single => {
                                emit!(buffer, "for {obj_ident} in &self.{obj_ident} {{");
                            },
                            store => panic!("{store} is not currently supported"),
                        }
                        emit!(
                            buffer,
                            "if let Some({obj_ident}) = {obj_ident} {{"
                        );
                        emit!(
                            buffer,
                            "let path = path.join(format!(\"{{}}.json\", {obj_ident}{read}.{id}));"
                        );
                    } else {
                        emit!(
                            buffer,
                            "for {obj_ident} in self.{obj_ident}.values() {{"
                        );
                        emit!(
                            buffer,
                            "let path = path.join(format!(\"{{}}.json\", {obj_ident}.{id}));"
                        );
                    }

                    emit!(buffer, "let file = fs::File::create(path)?;");
                    emit!(buffer, "let mut writer = io::BufWriter::new(file);");
                    if is_uber {
                        use UberStoreOptions::*;
                        match config.get_uber_store().unwrap() {
                            AsyncRwLock => {
                                emit!(
                                    buffer,
                                    "serde_json::to_writer_pretty(&mut writer, &({obj_ident}.read().await).clone())?;"
                                );
                                emit!(buffer, "}}");

                            }
                            _   => {
                                emit!(
                                    buffer,
                                    "serde_json::to_writer_pretty(&mut writer, &{obj_ident})?;"
                                );
                                emit!(buffer, "}}");
                            }
                        }
                    } else {
                        emit!(
                            buffer,
                            "serde_json::to_writer_pretty(&mut writer, &{obj_ident})?;"
                        );
                    }

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
            emit!(buffer, "pub fn from_bincode(code: &[u8]) -> io::Result<Self> {{");
            emit!(buffer, "Ok(bincode::deserialize(code).unwrap())");
            emit!(buffer, "}}\n");
            emit!(
                buffer,
                "/// The store is as a bincode file."
            );
            emit!(
                buffer,
                "pub fn load_bincode<P: AsRef<Path>>(path: P) -> io::Result<Self> {{"
            );
            emit!(buffer, "let path = path.as_ref();");
            emit!(buffer, "let bin_file = fs::File::open(path)?;");
            emit!(buffer, "Ok(bincode::deserialize_from(bin_file).unwrap())");
            emit!(buffer, "}}\n");

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
                "/// In fact, I intend to add automagic git integration as an option."
            );
            if is_uber {
                use UberStoreOptions::*;
                match config.get_uber_store().unwrap() {
                    Disabled => unreachable!(),
                    AsyncRwLock => {
                        emit!(
                            buffer,
                            "pub async fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {{"
                        );
                    }
                    _ => {
                        emit!(
                            buffer,
                            "pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {{"
                        );
                    }
                }
            } else {
                emit!(
                    buffer,
                    "pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {{"
                );
            }
            emit!(buffer, "let path = path.as_ref();");
            emit!(buffer, "let path = path.join(\"{}.json\");", domain.name());
            emit!(buffer, "");
            emit!(buffer, "let mut store = Self::new();");
            if is_uber {
                use UberStoreOptions::*;
                match config.get_uber_store().unwrap() {
                    AsyncRwLock => emit!(buffer, "let mut store = store.await;"),
                    _ => {}
                }
            }
            emit!(buffer, "");

            for obj in objects {
                let obj_ident = obj.as_ident();
                let _obj_type = obj.as_type(&Ownership::new_borrowed(), woog, domain);

                emit!(buffer, "// Load {}.", obj.name);
                emit!(buffer, "{{");
                emit!(buffer, "let path = path.join(\"{}\");", obj.as_ident());
                emit!(buffer, "let entries = fs::read_dir(path)?;");
                emit!(buffer, "for entry in entries {{");
                emit!(buffer, "let entry = entry?;");
                emit!(buffer, "let path = entry.path();");
                emit!(buffer, "let file = fs::File::open(path)?;");
                emit!(buffer, "let reader = io::BufReader::new(file);");
                let id = if local_object_is_enum(obj, config, domain) {
                    "id()"
                } else {
                    "id"
                };

                let store_type = get_value_wrapper(is_uber, config, obj, woog, domain);
                if timestamp {
                    if is_uber {
                        use UberStoreOptions::*;
                        match config.get_uber_store().unwrap() {
                            Disabled => unreachable!(),
                            AsyncRwLock => {
                                emit!(
                                    buffer,
                                    "let {obj_ident}: ({}, SystemTime) = serde_json::from_reader(reader).map(|(a, b)| (Arc::new(RwLock::new(a)), b))?;",
                                    store_type,
                                );
                            }
                            _ => {
                                emit!(
                                    buffer,
                                    "let {obj_ident}: ({}, SystemTime) = serde_json::from_reader(reader)?;",
                                    store_type,
                                );
                            }
                        }
                    } else {
                        emit!(
                            buffer,
                            "let {obj_ident}: ({}, SystemTime) = serde_json::from_reader(reader)?;",
                            store_type,
                        );
                    }

                    if object_has_name(obj, domain) {
                        if is_uber {
                            let (read, write) = get_uber_read_write(config);
                            use UberStoreOptions::*;
                            match config.get_uber_store().unwrap() {
                                AsyncRwLock | StdRwLock | ParkingLotRwLock | NDRwLock => {
                                    emit!(
                                        buffer,
                                        "store.{obj_ident}_id_by_name{write}.insert({obj_ident}.0{read}.name.to_owned(), ({obj_ident}.0{read}.{id}, {obj_ident}.1));"
                                    );
                                }
                                Single => {
                                    emit!(
                                        buffer,
                                        "store.{obj_ident}_id_by_name.insert({obj_ident}.0{read}.name.to_owned(), ({obj_ident}.0{read}.{id}, {obj_ident}.1));"
                                    );
                                }
                                store => panic!("{store} is not currently supported"),
                            }
                        } else {
                            emit!(
                                buffer,
                                "store.{obj_ident}_id_by_name.insert({obj_ident}.0.name.to_owned(), ({obj_ident}.0.{id}, {obj_ident}.1));"
                            );
                        }
                    }
                    if is_uber {
                        let (read, write) = get_uber_read_write(config);
                        emit!(
                            buffer,
                            "store.{obj_ident}{write}.insert({obj_ident}.0{read}.{id}, {obj_ident}.clone());"
                        );
                    } else {
                        emit!(
                            buffer,
                            "store.{obj_ident}.insert({obj_ident}.0.{id}, {obj_ident});"
                        );
                    }
                } else {
                    if is_uber {
                        use UberStoreOptions::*;
                        match config.get_uber_store().unwrap() {
                            Disabled => unreachable!(),
                            AsyncRwLock => {
                                emit!(
                                    buffer,
                                    "let {obj_ident}: {} = serde_json::from_reader(reader).map(|a| Arc::new(RwLock::new(a)))?;",
                                    store_type,
                                );
                            }
                            _ => {
                                emit!(
                                    buffer,
                                    "let {obj_ident}: {} = serde_json::from_reader(reader)?;",
                                    store_type,
                                );
                            }
                        }
                    } else {
                        emit!(
                            buffer,
                            "let {obj_ident}: {} = serde_json::from_reader(reader)?;",
                            store_type,
                        );
                    }

                    if object_has_name(obj, domain) {
                        if is_uber {
                            let (read, write) = get_uber_read_write(config);
                            use UberStoreOptions::*;
                            match config.get_uber_store().unwrap() {
                                AsyncRwLock| StdRwLock | ParkingLotRwLock | NDRwLock => {
                                    emit!(
                                        buffer,
                                        "store.{obj_ident}_id_by_name{write}.insert({obj_ident}{read}.name.to_owned(), {obj_ident}{read}.{id});"
                                    );
                                }
                                Single => {
                                    emit!(
                                        buffer,
                                        "store.{obj_ident}_id_by_name.insert({obj_ident}{read}.name.to_owned(), {obj_ident}{read}.{id});"
                                    );
                                }
                                store => panic!("{store} is not currently supported"),
                            }
                        } else {
                            emit!(
                                buffer,
                                "store.{obj_ident}_id_by_name.insert({obj_ident}.name.to_owned(), {obj_ident}.{id});"
                            );
                        }
                    }
                    if is_uber {
                        let (read, write) = get_uber_read_write(config);
                        use UberStoreOptions::*;
                        match config.get_uber_store().unwrap() {
                            AsyncRwLock | StdRwLock | ParkingLotRwLock |  NDRwLock => {
                                emit!(
                                    buffer,
                                    "store.{obj_ident}{write}.insert({obj_ident}{read}.{id}, Some({obj_ident}.clone()));"
                                );
                            }
                            Single => {
                                emit!(
                                    buffer,
                                    "store.{obj_ident}.insert({obj_ident}{read}.{id}, Some({obj_ident}.clone()));"
                                );
                            }
                            store => panic!("{store} is not currently supported"),
                        }
                    } else {
                        emit!(
                            buffer,
                            "store.{obj_ident}.insert({obj_ident}.{id}, {obj_ident});"
                        );
                    }
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
        obj.as_type(&Ownership::new_borrowed(), woog, domain)
    }
}

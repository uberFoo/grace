//! A type for generating an external type.
//!
use std::fmt::Write;

use rustc_hash::FxHashMap as HashMap;
use sarzak::{
    mc::{CompilerSnafu, FormatSnafu, Result},
    v2::domain::Domain,
    woog::{store::ObjectStore as WoogStore, types::Ownership},
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        emit_object_comments, find_store,
        generator::{FileGenerator, GenerationAction},
        render::{
            render_associative_attributes, render_attributes, render_binary_referential_attributes,
            RenderIdent, RenderType,
        },
    },
    options::GraceConfig,
    types::TypeImplementation,
};

pub(crate) struct ExternalBuilder {
    implementations: Vec<Box<dyn TypeImplementation>>,
}

impl ExternalBuilder {
    pub(crate) fn new() -> Self {
        ExternalBuilder {
            implementations: Vec::new(),
        }
    }

    pub(crate) fn implementation(mut self, implementation: Box<dyn TypeImplementation>) -> Self {
        self.implementations.push(implementation);

        self
    }

    pub(crate) fn build(self) -> Result<Box<ExternalGenerator>> {
        Ok(Box::new(ExternalGenerator {
            implementations: self.implementations,
        }))
    }
}

pub(crate) struct ExternalGenerator {
    implementations: Vec<Box<dyn TypeImplementation>>,
}

impl FileGenerator for ExternalGenerator {
    fn generate(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        woog_opt: &Option<&mut WoogStore>,
        imports: &Option<&HashMap<String, Domain>>,
        package: &str,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<GenerationAction> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by DomainStruct"
            }
        );
        let obj_id = obj_id.unwrap();
        ensure!(
            woog_opt.is_some(),
            CompilerSnafu {
                description: "woog is required by DomainStruct"
            }
        );
        let woog = woog_opt.as_ref().unwrap();
        ensure!(
            imports.is_some(),
            CompilerSnafu {
                description: "imports is required by DomainNewImpl"
            }
        );

        let object = domain.sarzak().exhume_object(obj_id).unwrap();
        let external = config.get_external(obj_id).unwrap();
        let store = find_store(module, woog, domain);

        emit!(buffer, "//! {} External Entity", object.name);
        emit!(buffer, "//!");

        // Write the use statements.
        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-ee-use-statements", object.as_ident()),
            |buffer| {
                emit!(buffer, "use {}::{};", external.path, external.name);
                emit!(buffer, "use uuid::Uuid;");
                emit!(buffer, "use crate::{}::UUID_NS;", module);

                // Add the use statements from the options.
                if let Some(use_paths) = config.get_use_paths(&object.id) {
                    for path in use_paths {
                        emit!(buffer, "use {};", path);
                    }
                }

                emit!(buffer, "use {} as {};", store.path, store.name);

                Ok(())
            },
        )?;

        // Documentation
        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-ee-documentation", object.as_ident()),
            |buffer| emit_object_comments(object.description.as_str(), "/// ", "", buffer),
        )?;

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-ee-definition", object.as_ident()),
            |buffer| {
                if let Some(derives) = config.get_derives(&object.id) {
                    write!(buffer, "#[derive(").context(FormatSnafu)?;
                    for d in derives {
                        write!(buffer, "{},", d).context(FormatSnafu)?;
                    }
                    emit!(buffer, ")]");
                }
                emit!(
                    buffer,
                    "pub struct {} {{",
                    object.as_type(&Ownership::new_borrowed(), woog, domain),
                );

                render_attributes(buffer, object, config, woog, domain)?;
                render_binary_referential_attributes(buffer, object, config, woog, domain)?;
                render_associative_attributes(buffer, object, config, woog, domain)?;
                emit!(buffer, "inner: {},", external.name);

                emit!(buffer, "}}");

                Ok(())
            },
        )?;

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-ee-impl", object.as_ident()),
            |buffer| {
                emit!(
                    buffer,
                    "impl {} {{",
                    object.as_type(&Ownership::new_borrowed(), woog, domain)
                );

                if let crate::options::OptimizationLevel::Vec = config.get_optimization_level() {
                    if config.is_uber_store() {
                        use crate::UberStoreOptions::*;
                        match config.get_uber_store().unwrap() {
                            StdRwLock => {
                                emit!(
                                    buffer,
                                    "pub fn new(store: &mut {}) -> std::sync::Arc<std::sync::RwLock<{}>> {{",
                                    store.name,
                                    object.as_type(&Ownership::new_borrowed(), woog, domain)
                                );
                                emit!(buffer, "store.inter_{}(|id| {{", object.as_ident());
                                emit!(
                                    buffer,
                                    "std::sync::Arc::new(std::sync::RwLock::new({} {{",
                                    object.as_type(&Ownership::new_borrowed(), woog, domain)
                                );
                            }
                            Single => {
                                emit!(
                                    buffer,
                                    "pub fn new(store: &mut {}) -> std::rc::Rc<std::cell::RefCell<{}>> {{",
                                    store.name,
                                    object.as_type(&Ownership::new_borrowed(), woog, domain)
                                );
                                emit!(buffer, "store.inter_{}(|id| {{", object.as_ident());
                                emit!(
                                    buffer,
                                    "std::rc::Rc::new(std::cell::RefCell::new({} {{",
                                    object.as_type(&Ownership::new_borrowed(), woog, domain)
                                );
                            }
                            store => panic!("{store} is not currently supported"),
                        }
                    } else {
                        emit!(
                            buffer,
                            "pub fn new(store: &mut {}) -> {} {{",
                            store.name,
                            object.as_type(&Ownership::new_borrowed(), woog, domain)
                        );

                        emit!(buffer, "store.inter_{}(|id| {{", object.as_ident());
                        emit!(
                            buffer,
                            "std::rc::Rc::new(std::cell::RefCell::new({} {{",
                            object.as_type(&Ownership::new_borrowed(), woog, domain)
                        );
                    }
                    emit!(buffer, "id,");
                    emit!(buffer, "inner: {}::{}(),", external.name, external.ctor);
                    emit!(buffer, "}}))");
                    emit!(buffer, "}})");
                } else {
                    if config.is_uber_store() {
                        use crate::UberStoreOptions::*;
                        match config.get_uber_store().unwrap() {
                            StdRwLock => {
                                emit!(
                                    buffer,
                                    "pub fn new(store: &mut {}) -> std::sync::Arc<std::sync::RwLock<{}>> {{",
                                    store.name,
                                    object.as_type(&Ownership::new_borrowed(), woog, domain)
                                );
                            }
                            Single => {
                                emit!(
                                    buffer,
                                    "pub fn new(store: &mut {}) -> std::rc::Rc<std::cell::RefCell<{}>> {{",
                                    store.name,
                                    object.as_type(&Ownership::new_borrowed(), woog, domain)
                                );
                            }
                            store => panic!("{store} is not currently supported"),
                        }
                    } else {
                        emit!(
                            buffer,
                            "pub fn new(store: &mut {}) -> {} {{",
                            store.name,
                            object.as_type(&Ownership::new_borrowed(), woog, domain)
                        );
                    }
                    emit!(
                        buffer,
                        "let inner = {}::{}();",
                        external.name,
                        external.ctor
                    );
                    emit!(
                        buffer,
                        "let id = Uuid::new_v5(&UUID_NS, format!(\"{{:?}}\", inner).as_bytes());"
                    );

                    if config.is_uber_store() {
                        use crate::UberStoreOptions::*;
                        match config.get_uber_store().unwrap() {
                            StdRwLock => {
                                emit!(
                                    buffer,
                                    "let new = std::sync::Arc::new(std::sync::RwLock::new({}{{",
                                    object.as_type(&Ownership::new_borrowed(), woog, domain)
                                );
                                emit!(buffer, "id: id,");
                                emit!(buffer, "inner: inner,");
                                emit!(buffer, "}}));");
                            }
                            Single => {
                                emit!(
                                    buffer,
                                    "let new = std::rc::Rc::new(std::cell::RefCell::new({}{{",
                                    object.as_type(&Ownership::new_borrowed(), woog, domain)
                                );
                                emit!(buffer, "id: id,");
                                emit!(buffer, "inner: inner,");
                                emit!(buffer, "}}));");
                            }
                            store => panic!("{store} is not currently supported"),
                        }
                    } else {
                        emit!(
                            buffer,
                            "let new = {} {{",
                            object.as_type(&Ownership::new_borrowed(), woog, domain)
                        );
                        emit!(buffer, "id: id,");
                        emit!(buffer, "inner: inner,");
                        emit!(buffer, "}};");
                    }

                    if config.is_uber_store() {
                        if let crate::options::UberStoreOptions::AsyncRwLock =
                            config.get_uber_store().unwrap()
                        {
                            emit!(
                                buffer,
                                "store.inter_{}(new.clone()).await;",
                                object.as_ident()
                            );
                        } else {
                            emit!(buffer, "store.inter_{}(new.clone());", object.as_ident());
                        }
                    } else {
                        emit!(buffer, "store.inter_{}(new.clone());", object.as_ident());
                    }
                    emit!(buffer, "new");
                }

                // Darn. So I need to insert a local here. And hybrid has similar needs.
                // render_method_new(buffer, object, config, imports, woog, domain)?;

                emit!(buffer, "}}");
                emit!(buffer, "}}");

                for implementation in &self.implementations {
                    implementation.write_code(
                        config,
                        domain,
                        woog_opt,
                        imports,
                        package,
                        module,
                        Some(obj_id),
                        buffer,
                    )?;
                }

                Ok(())
            },
        )?;

        Ok(GenerationAction::FormatWrite)
    }
}

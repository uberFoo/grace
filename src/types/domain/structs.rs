//! Domain Struct Generation
//!
//! Your one-stop-shop for everything to do with structs in Rust!
use std::fmt::Write;

use log;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
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
        generator::CodeWriter,
        get_assoc_referent_from_referrer_sorted,
        get_assoc_referrer_obj_from_obj_via_assoc_referent, get_binary_referents_sorted,
        get_binary_referrers_sorted, get_objs_for_assoc_referrers_sorted,
        get_objs_for_binary_referents_sorted, get_objs_for_binary_referrers_sorted,
        get_subtypes_sorted, local_object_is_hybrid, object_is_hybrid,
        render::{
            render_associative_attributes, render_attributes, render_binary_referential_attributes,
            RenderIdent, RenderType,
        },
        render_methods,
    },
    options::{GraceConfig, UberStoreOptions},
    types::{
        domain::rels::{
            generate_assoc_referent_rels, generate_assoc_referrer_rels,
            generate_binary_referent_rels, generate_binary_referrer_rels, generate_subtype_rels,
        },
        MethodImplementation, TypeDefinition, TypeImplementation, TypeImports,
    },
};

pub(crate) struct Imports;

impl Imports {
    pub(crate) fn new() -> Box<dyn TypeImports> {
        Box::new(Self)
    }
}

impl TypeImports for Imports {}

impl CodeWriter for Imports {
    fn write_code(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        woog: &Option<&mut WoogStore>,
        imports: &Option<&HashMap<String, Domain>>,
        _package: &str,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by DomainStruct"
            }
        );
        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();
        ensure!(
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by DomainStruct"
            }
        );
        let woog = woog.as_ref().unwrap();

        // These need to be sorted, as they are output as attributes and we require
        // stable output.
        let mut referrer_objs = get_objs_for_binary_referrers_sorted!(obj, domain.sarzak());
        referrer_objs.append(&mut get_assoc_referrer_obj_from_obj_via_assoc_referent!(
            obj,
            domain.sarzak()
        ));
        let referrer_objs: HashSet<_> = referrer_objs.into_iter().collect();
        // Remove ourselves, should that happen. Spoiler alert: it does.
        let referrer_objs: HashSet<_> = referrer_objs
            .into_iter()
            .filter(|r_obj| r_obj.id != obj.id)
            .collect();

        let mut referent_objs = get_objs_for_binary_referents_sorted!(obj, domain.sarzak());
        referent_objs.append(&mut get_objs_for_assoc_referrers_sorted!(
            obj,
            domain.sarzak()
        ));
        let referent_objs: HashSet<_> = referent_objs.into_iter().collect();
        // Remove ourselves, should that happen. Spoiler alert: it does.
        let referent_objs: HashSet<_> = referent_objs
            .into_iter()
            .filter(|r_obj| r_obj.id != obj.id)
            .collect();

        // Write the use statements.
        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-use-statements", obj.as_ident()),
            |buffer| {
                let mut imported_obj = HashSet::default();
                let mut uses = HashSet::default();

                if config.is_uber_store() {
                    use UberStoreOptions::*;
                    match config.get_uber_store().unwrap() {
                        Disabled => unreachable!(),
                        AsyncRwLock => {
                            emit!(buffer, "use async_std::sync::Arc;");
                            emit!(buffer, "use async_std::sync::RwLock;");
                            emit!(buffer, "use futures::stream::{{self, StreamExt}};");
                        }
                        NDRwLock => {
                            emit!(buffer, "use std::sync::Arc;");
                            emit!(buffer, "use no_deadlocks::RwLock;");
                        }
                        Single => {
                            emit!(buffer, "use std::cell::RefCell;");
                            emit!(buffer, "use std::rc::Rc;")
                        }
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

                    if config.get_tracy() {
                        emit!(buffer, "use tracy_client::span;");
                    }
                }

                // Everything has an `id`, everything needs this.
                // if config.get_optimization_level() == &crate::options::OptimizationLevel::None {
                emit!(buffer, "use uuid::Uuid;");
                // }
                emit!(buffer, "");

                // Add the use statements from the options.
                if let Some(use_paths) = config.get_use_paths(&obj.id) {
                    for path in use_paths {
                        uses.insert(format!("use {};", path));
                    }
                }

                // Add use statements for all the referrers.
                for r_obj in &referrer_objs {
                    if config.is_imported(&r_obj.id) {
                        let imported_object = config.get_imported(&r_obj.id).unwrap();
                        imported_obj.insert(imported_object.domain.as_str());
                        uses.insert(format!(
                            "use {}::types::{}::{};",
                            imported_object.domain,
                            r_obj.as_ident(),
                            r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                        ));
                    } else {
                        uses.insert(format!(
                            "use crate::{}::types::{}::{};",
                            module,
                            r_obj.as_ident(),
                            r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                        ));
                    }
                }

                // Add use statements for all the referents.
                for r_obj in &referent_objs {
                    if config.is_imported(&r_obj.id) {
                        let imported_object = config.get_imported(&r_obj.id).unwrap();
                        imported_obj.insert(imported_object.domain.as_str());
                        uses.insert(format!(
                            "use {}::types::{}::{};",
                            imported_object.domain,
                            r_obj.as_ident(),
                            r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                        ));
                    } else {
                        uses.insert(format!(
                            "use crate::{}::types::{}::{};",
                            module,
                            r_obj.as_ident(),
                            r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                        ));
                    }
                }

                // Add use statements for supertypes.
                for subtype in get_subtypes_sorted!(obj, domain.sarzak()) {
                    let isa = subtype.r27_isa(domain.sarzak())[0];
                    let supertype = isa.r13_supertype(domain.sarzak())[0];
                    let s_obj = supertype.r14_object(domain.sarzak())[0];

                    uses.insert(format!(
                        "use crate::{}::types::{}::{};",
                        module,
                        s_obj.as_ident(),
                        s_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    ));

                    if object_is_hybrid(s_obj, config, imports, domain)? {
                        uses.insert(format!(
                            "use crate::{}::types::{}::{}Enum;",
                            module,
                            s_obj.as_ident(),
                            s_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                        ));
                    }
                }

                // Add the use statements.
                for use_path in uses {
                    emit!(buffer, "{}", use_path);
                }

                // Add the ObjectStore, plus the store for any imported objects.
                imported_obj.insert(module);
                emit!(buffer, "");
                for import in imported_obj {
                    let store = find_store(import, woog, domain);
                    emit!(buffer, "use {} as {};", store.path, store.name);
                }

                Ok(())
            },
        )?;
        emit!(buffer, "");

        Ok(())
    }
}

/// Domain Struct Generator / CodeWriter
///
/// We need a builder for this so that we can add privacy modifiers, as
/// well as derives.
pub(crate) struct Struct;

impl Struct {
    pub(crate) fn new() -> Box<dyn TypeDefinition> {
        Box::new(Self)
    }
}

impl TypeDefinition for Struct {}

impl CodeWriter for Struct {
    fn write_code(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        woog: &Option<&mut WoogStore>,
        _imports: &Option<&HashMap<String, Domain>>,
        _package: &str,
        _module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by DomainStruct"
            }
        );
        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();
        ensure!(
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by DomainStruct"
            }
        );
        let woog = woog.as_ref().unwrap();

        log::debug!("writing Struct Definition for {}", obj.name);

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-struct-documentation", obj.as_ident()),
            |buffer| emit_object_comments(obj.description.as_str(), "/// ", "", buffer),
        )?;

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-struct-definition", obj.as_ident()),
            |buffer| {
                if let Some(derives) = config.get_derives(&obj.id) {
                    write!(buffer, "#[derive(").context(FormatSnafu)?;
                    for d in derives {
                        write!(buffer, "{},", d).context(FormatSnafu)?;
                    }
                    emit!(buffer, ")]");
                }

                emit!(
                    buffer,
                    "pub struct {} {{",
                    obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );

                render_attributes(buffer, obj, config, woog, domain)?;
                render_binary_referential_attributes(buffer, obj, config, woog, domain)?;
                render_associative_attributes(buffer, obj, config, woog, domain)?;

                emit!(buffer, "}}");
                Ok(())
            },
        )?;

        Ok(())
    }
}

pub(crate) struct DomainImplBuilder {
    for_trait: Option<String>,
    methods: Vec<Box<dyn MethodImplementation>>,
}

impl DomainImplBuilder {
    pub(crate) fn new() -> DomainImplBuilder {
        Self {
            for_trait: None,
            methods: Vec::new(),
        }
    }

    pub(crate) fn make_trait<S: AsRef<str>>(mut self, trait_name: S) -> Self {
        self.for_trait = Some(trait_name.as_ref().to_string());

        self
    }

    pub(crate) fn method(mut self, method: Box<dyn MethodImplementation>) -> Self {
        self.methods.push(method);

        self
    }

    pub(crate) fn build(self) -> Box<dyn TypeImplementation> {
        Box::new(DomainImplementation {
            for_trait: self.for_trait,
            methods: self.methods,
        })
    }
}

pub(crate) struct DomainImplementation {
    for_trait: Option<String>,
    methods: Vec<Box<dyn MethodImplementation>>,
}

impl TypeImplementation for DomainImplementation {}

impl CodeWriter for DomainImplementation {
    fn write_code(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        woog: &Option<&mut WoogStore>,
        imports: &Option<&HashMap<String, Domain>>,
        package: &str,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by DomainImplementation"
            }
        );
        let obj_id = obj_id.unwrap();
        let object = domain.sarzak().exhume_object(obj_id).unwrap();
        ensure!(
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by DomainImplementation"
            }
        );

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-implementation", object.as_ident()),
            |buffer| {
                if let Some(trait_name) = &self.for_trait {
                    emit!(
                        buffer,
                        "impl {} for {} {{",
                        trait_name,
                        object.as_type(&Ownership::new_borrowed(), woog.as_ref().unwrap(), domain)
                    );
                } else {
                    emit!(
                        buffer,
                        "impl {} {{",
                        object.as_type(&Ownership::new_borrowed(), woog.as_ref().unwrap(), domain)
                    );
                }

                for method in &self.methods {
                    method.write_code(
                        config,
                        domain,
                        woog,
                        imports,
                        package,
                        module,
                        Some(obj_id),
                        buffer,
                    )?;
                }

                emit!(buffer, "}}");

                Ok(())
            },
        )
    }
}

pub(crate) struct EqImpl;

impl EqImpl {
    pub(crate) fn new() -> Box<dyn MethodImplementation> {
        Box::new(Self)
    }
}

impl MethodImplementation for EqImpl {}

impl CodeWriter for EqImpl {
    fn write_code(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        _woog: &Option<&mut WoogStore>,
        _imports: &Option<&HashMap<String, Domain>>,
        _package: &str,
        _module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by DomainNewImpl"
            }
        );

        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();
        let skip_id = !config.is_external(obj_id);
        let is_hybrid = local_object_is_hybrid(obj, config, domain);

        let mut comps = Vec::new();

        if is_hybrid {
            comps.push("self.subtype == other.subtype".to_string());
        }

        let mut attrs = obj.r1_attribute(domain.sarzak());
        attrs.sort_by(|a, b| a.name.cmp(&b.name));
        for attr in attrs {
            if !(attr.name == "id" && skip_id) && attr.name != "hack" {
                comps.push(format!("self.{0} == other.{0}", attr.as_ident()));
            }
        }
        for referrer in get_binary_referrers_sorted!(obj, domain.sarzak()) {
            comps.push(format!(
                "self.{0} == other.{0}",
                referrer.referential_attribute.as_ident()
            ));
        }

        for assoc_referrer in obj.r26_associative_referrer(domain.sarzak()) {
            let assoc = assoc_referrer.r21_associative(domain.sarzak())[0];
            let referents =
                get_assoc_referent_from_referrer_sorted!(assoc_referrer, domain.sarzak());

            for referent in referents {
                let an_ass = referent.r22_an_associative_referent(domain.sarzak())[0];
                comps.push(format!(
                    "self.{0} == other.{0}",
                    an_ass.referential_attribute.as_ident()
                ));
            }
        }

        emit!(buffer, "fn eq(&self, other: &Self) -> bool {{");
        emit!(buffer, "{}", comps.join("&&"));
        emit!(buffer, "}}");

        Ok(())
    }
}

/// Domain Struct New Implementation
///
/// This generates a new implementation for the object. The new implementation
/// calculates the object's `id` based on the string representation of it's
/// attributes.
///
/// Sure wish I could figure out how to just take a reference to that HashMap...
///
/// __NB__ --- this implies that the lexicographical sum of it's attributes,
/// across all instances, must be unique.
pub(crate) struct StructNewImpl;

impl StructNewImpl {
    pub(crate) fn new() -> Box<dyn MethodImplementation> {
        Box::new(Self)
    }
}

impl MethodImplementation for StructNewImpl {}

impl CodeWriter for StructNewImpl {
    fn write_code(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        woog: &Option<&mut WoogStore>,
        imports: &Option<&HashMap<String, Domain>>,
        _package: &str,
        _module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by DomainNewImpl"
            }
        );
        ensure!(
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by DomainNewImpl"
            }
        );
        let woog = woog.as_ref().unwrap();
        ensure!(
            imports.is_some(),
            CompilerSnafu {
                description: "imports is required by DomainNewImpl"
            }
        );

        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();

        render_methods(buffer, obj, config, imports, woog, domain)
    }
}

/// Domain Relationship Navigation Implementation
///
/// This generates relationship navigation methods for a type. A method will be
/// generated for each relationship in which this object participates. This
/// applies to both formalizing and non-formalizing relationships.
pub(crate) struct StructRelNavImpl;

impl StructRelNavImpl {
    pub(crate) fn new() -> Box<dyn MethodImplementation> {
        Box::new(Self)
    }
}

impl MethodImplementation for StructRelNavImpl {}

impl CodeWriter for StructRelNavImpl {
    fn write_code(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        woog: &Option<&mut WoogStore>,
        _imports: &Option<&HashMap<String, Domain>>,
        _package: &str,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by StructRelNavImpl"
            }
        );
        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();
        ensure!(
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by StructRelNavImpl"
            }
        );
        let woog = woog.as_ref().unwrap();

        generate_binary_referrer_rels(buffer, config, module, obj, woog, domain)?;
        generate_binary_referent_rels(buffer, config, module, obj, "id", woog, domain)?;
        generate_assoc_referrer_rels(buffer, config, module, obj, woog, domain)?;
        generate_assoc_referent_rels(buffer, config, module, obj, "id", woog, domain)?;
        generate_subtype_rels(buffer, config, module, obj, woog, domain)?;

        Ok(())
    }
}

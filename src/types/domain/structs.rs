//! Domain Struct Generation
//!
//! Your one-stop-shop for everything to do with structs in Rust!
use std::fmt::Write;

use fnv::{FnvHashMap as HashMap, FnvHashSet as HashSet};
use log;
use sarzak::{
    mc::{CompilerSnafu, FormatSnafu, Result},
    v2::domain::Domain,
    woog::{store::ObjectStore as WoogStore, types::SHARED},
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        emit_object_comments, find_store,
        generator::CodeWriter,
        get_assoc_referrer_obj_from_obj_via_assoc_referent, get_binary_referents_sorted,
        get_binary_referrers_sorted, get_objs_for_assoc_referrers_sorted,
        get_objs_for_binary_referents_sorted, get_objs_for_binary_referrers_sorted,
        render::{
            render_associative_attributes, render_attributes, render_referential_attributes,
            RenderIdent, RenderType,
        },
        render_method,
    },
    options::GraceConfig,
    types::{
        domain::rels::{
            generate_assoc_referent_rels, generate_assoc_referrer_rels,
            generate_binary_referent_rels, generate_binary_referrer_rels, generate_subtype_rels,
        },
        MethodImplementation, TypeDefinition, TypeImplementation,
    },
};

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
                let mut imports = HashSet::default();
                let mut uses = HashSet::default();

                // Everything has an `id`, everything needs this.
                emit!(buffer, "use uuid::Uuid;");
                emit!(buffer, "");

                // We need this to create id's.
                emit!(buffer, "use crate::{}::UUID_NS;", module);

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
                        imports.insert(imported_object.domain.as_str());
                        uses.insert(format!(
                            "use crate::{}::types::{}::{};",
                            imported_object.domain,
                            r_obj.as_ident(),
                            r_obj.as_type(
                                &woog
                                    .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                                    .unwrap(),
                                woog,
                                domain
                            )
                        ));
                    } else {
                        uses.insert(format!(
                            "use crate::{}::types::{}::{};",
                            module,
                            r_obj.as_ident(),
                            r_obj.as_type(
                                &woog
                                    .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                                    .unwrap(),
                                woog,
                                domain
                            )
                        ));
                    }
                }

                // Add use statements for all the referents.
                for r_obj in &referent_objs {
                    uses.insert(format!(
                        "use crate::{}::types::{}::{};",
                        module,
                        r_obj.as_ident(),
                        r_obj.as_type(
                            &woog
                                .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                                .unwrap(),
                            woog,
                            domain
                        )
                    ));
                }

                // Ad use statements for supertypes.
                for subtype in obj.r15c_subtype(domain.sarzak()) {
                    let isa = subtype.r27_isa(domain.sarzak())[0];
                    let supertype = isa.r13_supertype(domain.sarzak())[0];
                    let s_obj = supertype.r14_object(domain.sarzak())[0];

                    uses.insert(format!(
                        "use crate::{}::types::{}::{};",
                        module,
                        s_obj.as_ident(),
                        s_obj.as_type(
                            &woog
                                .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                                .unwrap(),
                            woog,
                            domain
                        )
                    ));
                }

                // Add the use statements, plus the use for any imported objects.
                for use_path in uses {
                    emit!(buffer, "{}", use_path);
                }

                // Add the ObjectStore, plus the store for any imported objects.
                imports.insert(module);
                emit!(buffer, "");
                for import in imports {
                    let store = find_store(import, woog, domain);
                    emit!(buffer, "use {} as {};", store.path, store.name);
                }

                Ok(())
            },
        )?;
        emit!(buffer, "");

        log::debug!("writing Struct Definition for {}", obj.name);

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-struct-documentation", obj.as_ident()),
            |buffer| emit_object_comments(obj.description.as_str(), "///", buffer),
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
                    obj.as_type(
                        &woog
                            .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                            .unwrap(),
                        woog,
                        domain
                    )
                );

                render_attributes(buffer, obj, woog, domain)?;
                render_referential_attributes(buffer, obj, woog, domain)?;
                render_associative_attributes(buffer, obj, woog, domain)?;

                emit!(buffer, "}}");
                Ok(())
            },
        )?;

        Ok(())
    }
}

pub(crate) struct DomainImplBuilder {
    methods: Vec<Box<dyn MethodImplementation>>,
}

impl DomainImplBuilder {
    pub(crate) fn new() -> DomainImplBuilder {
        Self {
            methods: Vec::new(),
        }
    }

    pub(crate) fn method(mut self, method: Box<dyn MethodImplementation>) -> Self {
        self.methods.push(method);

        self
    }

    pub(crate) fn build(self) -> Box<dyn TypeImplementation> {
        Box::new(DomainImplementation {
            methods: self.methods,
        })
    }
}

pub(crate) struct DomainImplementation {
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
        let object = domain.sarzak().exhume_object(&obj_id).unwrap();
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
                let obj = domain.sarzak().exhume_object(&obj_id).unwrap();

                emit!(
                    buffer,
                    "impl {} {{",
                    obj.as_type(
                        &woog
                            .as_ref()
                            .unwrap()
                            .exhume_ownership(
                                &woog
                                    .as_ref()
                                    .unwrap()
                                    .exhume_borrowed(&SHARED)
                                    .unwrap()
                                    .id()
                            )
                            .unwrap(),
                        woog.as_ref().unwrap(),
                        domain
                    )
                );

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
        module: &str,
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

        // 🚧 Put this back in once I'm done moving to v2.
        // if options.get_doc_test() {
        //     buffer.block(
        //         DirectiveKind::IgnoreGenerated,
        //         format!("{}-struct-test-new", obj.as_ident()),
        //         |buffer| {
        //             let mut uses = HashSet::new();
        //             let stmts =
        //                 method.as_statement(package, module, woog, domain, &mut uses);
        //             emit!(buffer, "/// # Example");
        //             emit!(buffer, "///");
        //             emit!(buffer, "///```ignore");
        //             // for s in use_stmts.split_terminator('\n') {
        //             for s in uses.iter() {
        //                 emit!(buffer, "/// {}", s);
        //             }
        //             emit!(buffer, "///");
        //             // for s in stmts.split_terminator('\n') {
        //             for s in stmts.iter() {
        //                 emit!(buffer, "/// {} = {}", s.lvalue.name, s.rvalue.name);
        //             }
        //             emit!(buffer, "///```");

        //             Ok(())
        //         },
        //     )?;
        // }

        render_method(buffer, obj, config, imports, woog, domain)
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

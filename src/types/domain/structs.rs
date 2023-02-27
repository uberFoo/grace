//! Domain Struct Generation
//!
//! Your one-stop-shop for everything to do with structs in Rust!
use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
};

use log;
use sarzak::{
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::types::{
        Binary, Cardinality, Conditionality, External as SarzakExternal, Object, Referrer,
    },
    v2::domain::Domain,
    woog::{
        store::ObjectStore as WoogStore,
        types::{Ownership, BORROWED, MUTABLE, PUBLIC},
    },
};

use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        emit_object_comments, find_store,
        generator::CodeWriter,
        get_objs_for_assoc_referents_sorted, get_objs_for_assoc_referrers_sorted,
        get_objs_for_referents_sorted, get_objs_for_referrers_sorted, get_referents_sorted,
        get_referrers_sorted,
        render::{
            render_associative_attributes, render_attributes, render_referential_attributes,
            RenderIdent, RenderType,
        },
        render_make_uuid, render_method_definition, render_new_instance,
    },
    options::GraceConfig,
    todo::{GType, LValue, ObjectMethod, Parameter, RValue},
    types::{MethodImplementation, TypeDefinition, TypeImplementation},
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
        _woog: &Option<&mut WoogStore>,
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

        // These need to be sorted, as they are output as attributes and we require
        // stable output.
        let mut referrer_objs = get_objs_for_referrers_sorted!(obj, domain.sarzak());
        referrer_objs.append(&mut get_objs_for_assoc_referents_sorted!(
            obj,
            domain.sarzak()
        ));
        let referrer_objs: HashSet<_> = referrer_objs.into_iter().collect();
        // Remove ourselves, should that happen. Spoiler alert: it does.
        let referrer_objs: HashSet<_> = referrer_objs
            .into_iter()
            .filter(|r_obj| r_obj.id != obj.id)
            .collect();

        let mut referent_objs = get_objs_for_referents_sorted!(obj, domain.sarzak());
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

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-use-statements", obj.as_ident()),
            |buffer| {
                let mut imports = HashSet::new();

                // Everything has an `id`, everything needs this.
                emit!(buffer, "use uuid::Uuid;");
                emit!(buffer, "");

                // Add the use statements from the options.
                if let Some(use_paths) = config.get_use_paths(&obj.id) {
                    for path in use_paths {
                        emit!(buffer, "use {};", path);
                    }
                    emit!(buffer, "");
                }

                // We need this to create id's.
                emit!(buffer, "use crate::{}::UUID_NS;", module);

                // Add use statements for all the referrers.
                if referrer_objs.len() > 0 {
                    emit!(buffer, "");
                    emit!(buffer, "// Referrer imports");
                }
                for r_obj in &referrer_objs {
                    if config.is_imported(&r_obj.id) {
                        let imported_object = config.get_imported(&r_obj.id).unwrap();
                        imports.insert(imported_object.domain.as_str());
                        emit!(
                            buffer,
                            "use crate::{}::types::{}::{};",
                            imported_object.domain,
                            r_obj.as_ident(),
                            r_obj.as_type(&Ownership::Borrowed(BORROWED), domain)
                        );
                    } else {
                        emit!(
                            buffer,
                            "use crate::{}::types::{}::{};",
                            module,
                            r_obj.as_ident(),
                            r_obj.as_type(&Ownership::Borrowed(BORROWED), domain)
                        );
                    }
                }

                // Add use statements for all the referents.
                if referent_objs.len() > 0 {
                    emit!(buffer, "");
                    emit!(buffer, "// Referent imports");
                }
                for r_obj in &referent_objs {
                    emit!(
                        buffer,
                        "use crate::{}::types::{}::{};",
                        module,
                        r_obj.as_ident(),
                        r_obj.as_type(&Ownership::Borrowed(BORROWED), domain)
                    );
                }

                // Add the ObjectStore, plus the store for any imported objects.
                imports.insert(module);
                emit!(buffer, "");
                for import in imports {
                    let store = find_store(import, domain);
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
                    obj.as_type(&Ownership::Borrowed(BORROWED), domain)
                );

                render_attributes(buffer, obj, domain)?;

                render_referential_attributes(buffer, obj, domain)?;

                render_associative_attributes(buffer, obj, domain)?;

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

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-implementation", object.as_ident()),
            |buffer| {
                let obj = domain.sarzak().exhume_object(&obj_id).unwrap();

                emit!(
                    buffer,
                    "impl {} {{",
                    obj.as_type(&Ownership::Borrowed(BORROWED), domain)
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
        options: &GraceConfig,
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
        let woog = match woog {
            Some(ref woog) => woog,
            None => unreachable!(),
        };
        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();

        // These are more attributes on our object, and they should be sorted.
        let referrers = get_referrers_sorted!(obj, domain.sarzak());

        // Collect the attributes
        let mut params: Vec<Parameter> = Vec::new();
        // This is used in the new_instance call. These fields are meant to be
        // matched up with the input arguments, and type checked. Since I'm
        // generating both, I'm beginning to wonder what the point is.
        //
        // So just now the type system reminded me that I need to turn a reference
        // into a UUID. So maybe it's worth keeping.
        let mut fields: Vec<LValue> = Vec::new();
        // Collect the attributes
        let mut attrs = obj.r1_attribute(domain.sarzak());
        attrs.sort_by(|a, b| a.name.cmp(&b.name));
        for attr in attrs {
            // We are going to generate the id, so don't include it in the
            // list of parameters.
            if attr.name != "id" {
                let ty = attr.r2_ty(domain.sarzak())[0];
                fields.push(LValue::new(attr.name.as_ident(), ty.into()));
                params.push(Parameter::new(
                    BORROWED,
                    None,
                    ty.into(),
                    PUBLIC,
                    attr.as_ident(),
                ));
                // rvals.push(RValue::new(attr.as_ident(), &ty));
            }
        }

        // And the referential attributes
        for referrer in &referrers {
            let binary = referrer.r6_binary(domain.sarzak())[0];
            let referent = binary.r5_referent(domain.sarzak())[0];
            let r_obj = referent.r16_object(domain.sarzak())[0];
            let cond = referrer.r11_conditionality(domain.sarzak())[0];

            // If the relationship is conditional, then we need to make the
            // parameter an Option, and make the field match.
            match cond {
                Conditionality::Conditional(_) => {
                    fields.push(LValue::new(
                        referrer.referential_attribute.as_ident(),
                        GType::Option(Box::new(GType::Uuid)),
                    ));
                    params.push(Parameter::new(
                        BORROWED,
                        None,
                        GType::Option(Box::new(GType::Reference(r_obj.id))),
                        PUBLIC,
                        referrer.referential_attribute.as_ident(),
                    ));
                }
                Conditionality::Unconditional(_) => {
                    fields.push(LValue::new(
                        referrer.referential_attribute.as_ident(),
                        GType::Uuid,
                    ));
                    params.push(Parameter::new(
                        BORROWED,
                        None,
                        GType::Reference(r_obj.id),
                        PUBLIC,
                        referrer.referential_attribute.as_ident(),
                    ));
                }
            }

            //     rvals.push(RValue::new(
            //         referrer.referential_attribute.as_ident(),
            //         &Type::Reference(reference.id),
            //     ));
        }

        for assoc_referrer in obj.r26_associative_referrer(domain.sarzak()) {
            let assoc = assoc_referrer.r21_associative(domain.sarzak())[0];

            let one = assoc.r23_associative_referent(domain.sarzak())[0];
            let one_obj = one.r25_object(domain.sarzak())[0];

            let other = assoc.r22_associative_referent(domain.sarzak())[0];
            let other_obj = other.r25_object(domain.sarzak())[0];

            // This determines how a reference is stored in the struct. In this
            // case a UUID.
            fields.push(LValue::new(
                assoc_referrer.one_referential_attribute.as_ident(),
                GType::Uuid,
            ));
            params.push(Parameter::new(
                BORROWED,
                None,
                GType::Reference(one_obj.id),
                PUBLIC,
                assoc_referrer.one_referential_attribute.as_ident(),
            ));

            fields.push(LValue::new(
                assoc_referrer.other_referential_attribute.as_ident(),
                GType::Uuid,
            ));
            params.push(Parameter::new(
                BORROWED,
                None,
                GType::Reference(other_obj.id),
                PUBLIC,
                assoc_referrer.other_referential_attribute.as_ident(),
            ));
        }

        // Add the store to the end of the  input parameters
        let store = find_store(module, domain);
        params.push(Parameter::new(
            MUTABLE,
            None,
            GType::External(store),
            PUBLIC,
            "store".to_owned(),
        ));

        // Collect rvals for rendering the method.
        let rvals = params.clone();
        let mut rvals: Vec<RValue> = rvals.iter().map(|p| p.into()).collect();
        // Remove the store.
        rvals.pop();

        // Link the params. The result is the head of the list.
        let param = if params.len() > 0 {
            let mut iter = params.iter_mut().rev();
            let mut last = iter.next().unwrap();
            loop {
                match iter.next() {
                    Some(param) => {
                        param.next = Some(last);
                        last = param;
                    }
                    None => break,
                }
            }
            log::trace!("param: {:?}", last);
            Some(last.clone())
        } else {
            None
        };

        // Create an ObjectMethod
        // The uniqueness of this instance depends on the inputs to it's
        // new method. Param can be None, and two methods on the same
        // object will have the same obj. So it comes down to a unique
        // name for each object. So just "new" should suffice for name,
        // because it's scoped by obj already.
        let method = ObjectMethod::new(
            param.as_ref(),
            obj.id,
            GType::Object(obj.id),
            PUBLIC,
            "new".to_owned(),
            "Create a new instance".to_owned(),
        );

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-struct-impl-new", obj.as_ident()),
            |buffer| {
                // Output a docstring
                emit!(
                    buffer,
                    "/// Inter a new {} in the store, and return it's `id`.",
                    obj.as_type(&Ownership::Borrowed(BORROWED), domain)
                );

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

                // Output the top of the function definition
                render_method_definition(buffer, &method, woog, domain)?;

                // Output the code to create the `id`.
                let id = LValue::new("id", GType::Uuid);
                render_make_uuid(buffer, &id, &rvals, domain)?;

                // Output code to create the instance
                let new = LValue::new("new", GType::Reference(obj.id));
                render_new_instance(
                    buffer,
                    obj,
                    Some(&new),
                    &fields,
                    &rvals,
                    domain,
                    *imports,
                    &options,
                )?;

                emit!(buffer, "store.inter_{}(new.clone());", obj.as_ident());
                emit!(buffer, "new");
                emit!(buffer, "}}");

                Ok(())
            },
        )
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

    fn forward(
        buffer: &mut Buffer,
        obj: &Object,
        referrer: &Referrer,
        binary: &Binary,
        store: &SarzakExternal,
        r_obj: &Object,
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
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain),
                    binary.number,
                );
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain)
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
        store: &SarzakExternal,
        r_obj: &Object,
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
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain),
                    binary.number,
                );
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain)
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
        store: &SarzakExternal,
        referrer: &Referrer,
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
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain),
                    binary.number
                );
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain)
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
        store: &SarzakExternal,
        referrer: &Referrer,
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
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain),
                    binary.number
                );
                emit!(
                    buffer,
                    "pub fn r{}c_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain)
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
        store: &SarzakExternal,
        referrer: &Referrer,
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
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain),
                    binary.number
                );
                emit!(
                    buffer,
                    "pub fn r{}c_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain)
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
        store: &SarzakExternal,
        referrer: &Referrer,
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
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain),
                    binary.number
                );
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain)
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
        store: &SarzakExternal,
        referrer: &Referrer,
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
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain),
                    binary.number
                );
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    binary.number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain)
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
        store: &SarzakExternal,
        r_obj: &Object,
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
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain),
                    number,
                );
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain)
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
        store: &SarzakExternal,
        referential_attribute: &String,
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
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain),
                    number
                );
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain)
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
        store: &SarzakExternal,
        referential_attribute: &String,
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
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain),
                    number
                );
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain)
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
        store: &SarzakExternal,
        referential_attribute: &String,
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
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain),
                    number
                );
                emit!(
                    buffer,
                    "pub fn r{}_{}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    number,
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Ownership::Borrowed(BORROWED), domain)
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
}

impl MethodImplementation for StructRelNavImpl {}

impl CodeWriter for StructRelNavImpl {
    fn write_code(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        _woog: &Option<&mut WoogStore>,
        _imports: &Option<&HashMap<String, Domain>>,
        _package: &str,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by DomainRelNavImpl"
            }
        );
        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();

        // These are relationships that we formalize
        let referrers = get_referrers_sorted!(obj, domain.sarzak());
        // These are relationships of which we are the target
        let referents = get_referents_sorted!(obj, domain.sarzak());

        for referrer in &referrers {
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
            let store = find_store(module, domain);

            // Cardinality does not matter from the referrer, because it's always
            // one. This is because of the normalized, table-nature of the store,
            // and more importantly the method.
            match cond {
                Conditionality::Unconditional(_) => StructRelNavImpl::forward(
                    buffer,
                    obj,
                    referrer,
                    binary,
                    &store.into(),
                    r_obj,
                    &domain,
                )?,
                Conditionality::Conditional(_) => StructRelNavImpl::forward_conditional(
                    buffer,
                    obj,
                    referrer,
                    binary,
                    &store.into(),
                    r_obj,
                    &domain,
                )?,
            }
        }

        for referent in &referents {
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
            let store = find_store(module, domain);

            match card {
                Cardinality::One(_) => match my_cond {
                    Conditionality::Unconditional(_) => StructRelNavImpl::backward_one(
                        buffer,
                        obj,
                        r_obj,
                        binary,
                        &store.into(),
                        referrer,
                        &domain,
                    )?,
                    Conditionality::Conditional(_) => match other_cond {
                        Conditionality::Unconditional(_) => {
                            StructRelNavImpl::backward_one_conditional(
                                buffer,
                                obj,
                                r_obj,
                                binary,
                                &store.into(),
                                referrer,
                                &domain,
                            )?
                        }
                        Conditionality::Conditional(_) => {
                            StructRelNavImpl::backward_one_biconditional(
                                buffer,
                                obj,
                                r_obj,
                                binary,
                                &store.into(),
                                referrer,
                                &domain,
                            )?
                        }
                    },
                },
                // It's interesting that there are only really two possibilities, and
                // that neither of them depend on the conditionality of the this side.
                Cardinality::Many(_) => match other_cond {
                    Conditionality::Unconditional(_) => StructRelNavImpl::backward_1_m(
                        buffer,
                        obj,
                        r_obj,
                        binary,
                        &store.into(),
                        referrer,
                        &domain,
                    )?,
                    Conditionality::Conditional(_) => StructRelNavImpl::backward_1_mc(
                        buffer,
                        obj,
                        r_obj,
                        binary,
                        &store.into(),
                        referrer,
                        &domain,
                    )?,
                },
            }
        }

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
            let store = find_store(module, domain);

            StructRelNavImpl::forward_assoc(
                buffer,
                obj,
                &assoc_referrer.one_referential_attribute,
                assoc.number,
                &store.into(),
                one_obj,
                &domain,
            )?;

            let module = if config.is_imported(&other_obj.id) {
                config.get_imported(&one_obj.id).unwrap().domain.as_str()
            } else {
                module
            };

            // Grab a reference to the store so that we can use it to exhume
            // things.
            let store = find_store(module, domain);
            StructRelNavImpl::forward_assoc(
                buffer,
                obj,
                &assoc_referrer.other_referential_attribute,
                assoc.number,
                &store.into(),
                other_obj,
                &domain,
            )?;
        }

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
            let store = find_store(module, domain);

            match card {
                Cardinality::One(_) => match cond {
                    Conditionality::Conditional(_) => {
                        StructRelNavImpl::backward_assoc_one_conditional(
                            buffer,
                            obj,
                            r_obj,
                            assoc.number,
                            &store.into(),
                            referential_attribute,
                            &domain,
                        )?
                    }
                    Conditionality::Unconditional(_) => StructRelNavImpl::backward_assoc_one(
                        buffer,
                        obj,
                        r_obj,
                        assoc.number,
                        &store.into(),
                        referential_attribute,
                        &domain,
                    )?,
                },
                Cardinality::Many(_) => StructRelNavImpl::backward_assoc_many(
                    buffer,
                    obj,
                    r_obj,
                    assoc.number,
                    &store.into(),
                    referential_attribute,
                    &domain,
                )?,
            }
        }

        Ok(())
    }
}

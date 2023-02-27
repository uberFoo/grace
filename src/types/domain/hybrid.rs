//! Domain Enum with extras Generation
//!
//! Here we are.
use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
};

use sarzak::{
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::types::Conditionality,
    v2::domain::Domain,
    woog::{
        store::ObjectStore as WoogStore,
        types::{Mutability, BORROWED, MUTABLE, PUBLIC},
    },
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        emit_object_comments, find_store, get_objs_for_assoc_referents_sorted,
        get_objs_for_assoc_referrers_sorted, get_objs_for_referents_sorted,
        get_objs_for_referrers_sorted, get_referents_sorted, get_referrers_sorted,
        get_subtypes_sorted, object_is_singleton, object_is_supertype,
        render::{
            render_associative_attributes, render_attributes, render_referential_attributes,
            RenderConst, RenderIdent, RenderType,
        },
        render_make_uuid, render_method_definition, render_new_instance,
        run_func_on_imported_domain,
    },
    options::GraceConfig,
    todo::{GType, LValue, ObjectMethod, Parameter, RValue},
    types::{CodeWriter, MethodImplementation, TypeDefinition},
};

const SUBTYPE_ATTR: &str = "subtype";

/// Domain Hybrid Generator / CodeWriter
///
pub(crate) struct Hybrid;

impl Hybrid {
    pub(crate) fn new() -> Box<dyn TypeDefinition> {
        Box::new(Self)
    }
}

impl TypeDefinition for Hybrid {}

impl CodeWriter for Hybrid {
    fn write_code(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        _woog: &Option<&mut WoogStore>,
        imports: &Option<&HashMap<String, Domain>>,
        _package: &str,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by Hybrid"
            }
        );
        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();

        let subtypes = get_subtypes_sorted!(obj, domain.sarzak());

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
                let mut uses = HashSet::new();

                // Everything has an `id`, everything needs this.
                emit!(buffer, "use uuid::Uuid;");
                emit!(buffer, "");

                emit!(buffer, "use crate::{}::UUID_NS;", module);

                emit!(buffer, "");

                // Add the use statements from the options.
                if let Some(use_paths) = config.get_use_paths(&obj.id) {
                    for path in use_paths {
                        emit!(buffer, "use {};", path);
                    }
                }
                emit!(buffer, "");

                emit!(buffer, "// Subtype imports");
                for subtype in &subtypes {
                    let s_obj = subtype.r15_object(domain.sarzak())[0];
                    let is_singleton = object_is_singleton(s_obj, config, imports, domain)?;
                    let is_supertype = object_is_supertype(s_obj, config, imports, domain)?;

                    if is_singleton && !is_supertype {
                        emit!(
                            buffer,
                            "use crate::{}::types::{}::{};",
                            module,
                            s_obj.as_ident(),
                            s_obj.as_const()
                        );
                    } else {
                        emit!(
                            buffer,
                            "use crate::{}::types::{}::{};",
                            module,
                            s_obj.as_ident(),
                            s_obj.as_type(&Mutability::Borrowed(BORROWED), domain)
                        );
                    }
                }

                // Add use statements for all the referrers.
                if referrer_objs.len() > 0 {
                    emit!(buffer, "");
                    emit!(buffer, "// Referrer imports");
                }
                for r_obj in &referrer_objs {
                    if config.is_imported(&r_obj.id) {
                        let imported_object = config.get_imported(&r_obj.id).unwrap();
                        uses.insert(imported_object.domain.as_str());
                        emit!(
                            buffer,
                            "use crate::{}::types::{}::{};",
                            imported_object.domain,
                            r_obj.as_ident(),
                            r_obj.as_type(&Mutability::Borrowed(BORROWED), domain)
                        );
                    } else {
                        emit!(
                            buffer,
                            "use crate::{}::types::{}::{};",
                            module,
                            r_obj.as_ident(),
                            r_obj.as_type(&Mutability::Borrowed(BORROWED), domain)
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
                        r_obj.as_type(&Mutability::Borrowed(BORROWED), domain)
                    );
                }

                // Add the ObjectStore, plus the store for any imported objects.
                uses.insert(module);
                emit!(buffer, "");
                for import in uses {
                    let store = find_store(import, domain);
                    emit!(buffer, "use {} as {};", store.path, store.name);
                }

                Ok(())
            },
        )?;
        emit!(buffer, "");

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-hybrid-documentation", obj.as_ident()),
            |buffer| emit_object_comments(obj.description.as_str(), "///", buffer),
        )?;

        log::debug!("writing Enum Definition for {}", obj.name);

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-hybrid-enum-definition", obj.as_ident()),
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
                    "pub enum {}Enum {{",
                    obj.as_type(&Mutability::Borrowed(BORROWED), domain)
                );
                for subtype in &subtypes {
                    let s_obj = subtype.r15_object(domain.sarzak())[0];
                    emit!(
                        buffer,
                        "{}(Uuid),",
                        s_obj.as_type(&Mutability::Borrowed(BORROWED), domain),
                    );
                }
                emit!(buffer, "}}");
                Ok(())
            },
        )?;

        log::debug!("writing Struct Definition for {}", obj.name);

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-hybrid-struct-definition", obj.as_ident()),
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
                    obj.as_type(&Mutability::Borrowed(BORROWED), domain)
                );

                emit!(
                    buffer,
                    "pub {}: {}Enum,",
                    SUBTYPE_ATTR,
                    obj.as_type(&Mutability::Borrowed(BORROWED), domain)
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

/// Hybrid New Implementation
///
/// This generates new implementations for hybrid objects. Plural. One for each
/// subtype. This is sort of lame. Ideally, I think we would have a single
/// implementation that takes the enum that is our subtypes. However, because
/// this is a single object in the model, we have no way to distinguish between
/// the Hybrid enum, and struct. So we have multiple new methods and we never
/// surface the existence of the enum
///
/// __NB__ --- this implies that the lexicographical sum of it's attributes,
/// across all instances, must be unique.
pub(crate) struct HybridNewImpl;

impl HybridNewImpl {
    pub(crate) fn new() -> Box<dyn MethodImplementation> {
        Box::new(Self)
    }
}

impl MethodImplementation for HybridNewImpl {}

impl CodeWriter for HybridNewImpl {
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
                description: "obj_id is required by HybridNewImpl"
            }
        );
        ensure!(
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by HybridNewImpl"
            }
        );
        let woog = match woog {
            Some(ref woog) => woog,
            None => unreachable!(),
        };
        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();

        let subtypes = get_subtypes_sorted!(obj, domain.sarzak());

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

        for subtype in subtypes {
            let s_obj = subtype.r15_object(domain.sarzak())[0];
            let mut fields_ = fields.clone();
            let mut params_ = params.clone();

            // Insert the subtype here.
            //
            // There's a certain level of complexity that entertains such antics as
            // witnessed below. I'm pretty tired, so maybe there's a much better way,
            // but honestly, shit's getting complicated.
            // if object_is_singleton(&s_obj, domain) && !object_is_supertype(s_obj, domain) {
            //     fields_.push(LValue::new(
            //         SUBTYPE_ATTR.to_owned(),
            //         GType::Object(s_obj.id),
            //     ));
            // } else {
            fields_.push(LValue::new(
                SUBTYPE_ATTR.to_owned(),
                GType::Object(s_obj.id),
            ));
            // if object_is_singleton(&s_obj, domain) && !object_is_supertype(s_obj, domain) {
            // params_.push(Parameter::new(
            // BORROWED,
            // None,
            // GType::Uuid,
            // PUBLIC,
            // SUBTYPE_ATTR.to_owned(),
            // ));
            // } else {
            params_.push(Parameter::new(
                BORROWED,
                None,
                GType::Reference(s_obj.id),
                PUBLIC,
                SUBTYPE_ATTR.to_owned(),
            ));
            // }
            // }

            // Collect rvals for rendering the method.
            let mut rvals: Vec<RValue> = params_.iter().map(|p| p.into()).collect();

            let is_singleton = object_is_singleton(s_obj, config, imports, domain)?;
            let is_supertype = object_is_supertype(s_obj, config, imports, domain)?;

            // We don't want a parameter for a const, and we'll need to change the rval...
            if is_singleton && !is_supertype {
                params_.pop();
                rvals.pop();
                rvals.push(RValue::new(format!("{}", s_obj.as_const()), GType::Uuid));
            }

            // Add the store to the end of the  input parameters
            let store = find_store(module, domain);
            params_.push(Parameter::new(
                MUTABLE,
                None,
                GType::External(store),
                PUBLIC,
                "store".to_owned(),
            ));

            // Link the params. The result is the head of the list.
            let param = if params_.len() > 0 {
                let mut iter = params_.iter_mut().rev();
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
                format!("new_{}", s_obj.as_ident()),
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
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain)
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
                        &fields_,
                        &rvals,
                        domain,
                        *imports,
                        &config,
                    )?;

                    emit!(buffer, "store.inter_{}(new.clone());", obj.as_ident());
                    emit!(buffer, "new");
                    emit!(buffer, "}}");

                    Ok(())
                },
            )?;
        }

        Ok(())
    }
}

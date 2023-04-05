//! Things necessary for code generation
//!
pub(crate) mod buffer;
pub(crate) mod diff_engine;
pub(crate) mod generator;
pub(crate) mod render;
mod rustfmt;

use std::fmt::Write;

use fnv::FnvHashMap as HashMap;
use sarzak::{
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::types::{External, Object, Ty},
    v2::domain::Domain,
    woog::{
        store::ObjectStore as WoogStore,
        types::{
            Expression, FunctionEnum, GraceType, GraceTypeEnum, Literal, Local, ObjectMethod,
            Ownership, ReferenceType, Statement, StatementEnum, StructExpression, Variable,
            VariableEnum, OWNED, SHARED,
        },
    },
};
use snafu::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        render::{RenderIdent, RenderType},
    },
    options::GraceConfig,
    woog::typecheck_and_coerce,
};

macro_rules! get_subtypes_sorted {
    ($obj:expr, $store:expr) => {{
        let sup = $obj.r14_supertype($store)[0];
        let isa = sup.r13_isa($store)[0];
        let mut subtypes = isa.r27_subtype($store);
        subtypes.sort_by(|a, b| {
            let a = a.r15_object($store)[0];
            let b = b.r15_object($store)[0];
            a.name.cmp(&b.name)
        });

        subtypes
    }};
}
pub(crate) use get_subtypes_sorted;

macro_rules! get_assoc_referent_from_referrer_sorted {
    ($obj:expr, $store:expr) => {{
        let assoc = $obj.r21_associative($store)[0];
        let mut referrers = assoc
            .r22_an_associative_referent($store)
            .iter()
            .map(|r| r.r22_associative_referent($store)[0])
            .collect::<Vec<_>>();

        referrers.sort_by(|a, b| {
            let a = a.r25_object($store)[0];
            let b = b.r25_object($store)[0];
            a.name.cmp(&b.name)
        });

        referrers
    }};
}
pub(crate) use get_assoc_referent_from_referrer_sorted;

macro_rules! get_objs_for_assoc_referrers_sorted {
    ($obj:expr, $store:expr) => {{
        let mut objs = Vec::new();
        let referrers = $obj.r26_associative_referrer($store);
        for referrer in &referrers {
            // For some stupid reason the compiler can't see this macro.
            // let referents = get_assoc_referent_from_referrer_sorted!(referrer, $store);
            let assoc = referrer.r21_associative($store)[0];
            let referents = assoc
                .r22_an_associative_referent($store)
                .iter()
                .map(|r| {
                    let referent = r.r22_associative_referent($store)[0];
                    let obj = referent.r25_object($store)[0];
                    obj
                })
                .collect::<Vec<_>>();
            objs.extend(referents);
        }

        objs.sort_by(|a, b| a.name.cmp(&b.name));

        objs
    }};
}
pub(crate) use get_objs_for_assoc_referrers_sorted;

macro_rules! get_assoc_referrer_obj_from_obj_via_assoc_referent {
    ($obj:expr, $store:expr) => {{
        let mut objs = Vec::new();
        let referents = $obj.r25_associative_referent($store);
        for referent in &referents {
            let aar = referent.r22_an_associative_referent($store)[0];
            let assoc = aar.r22_associative($store)[0];
            let referrer = assoc.r21_associative_referrer($store)[0];
            objs.push(referrer.r26_object($store)[0]);
        }

        objs.sort_by(|a, b| a.name.cmp(&b.name));

        objs
    }};
}
pub(crate) use get_assoc_referrer_obj_from_obj_via_assoc_referent;

macro_rules! get_objs_for_binary_referrers_sorted {
    ($obj:expr, $store:expr) => {{
        let mut objs = Vec::new();
        let referrers = get_binary_referrers_sorted!($obj, $store);
        for referrer in &referrers {
            let binary = referrer.r6_binary($store)[0];
            let referent = binary.r5_referent($store)[0];
            let obj = referent.r16_object($store)[0];
            objs.push(obj);
        }

        objs
    }};
}
pub(crate) use get_objs_for_binary_referrers_sorted;

macro_rules! get_objs_for_binary_referents_sorted {
    ($obj:expr, $store:expr) => {{
        let mut objs = Vec::new();
        let referents = get_binary_referents_sorted!($obj, $store);
        for referent in &referents {
            let binary = referent.r5_binary($store)[0];
            let referrer = binary.r6_referrer($store)[0];
            let obj = referrer.r17_object($store)[0];
            objs.push(obj);
        }

        objs
    }};
}
pub(crate) use get_objs_for_binary_referents_sorted;

macro_rules! get_binary_referrers_sorted {
    ($obj:expr, $store:expr) => {{
        let mut referrers = $obj.r17_referrer($store);
        referrers.sort_by(|a, b| {
            let binary = a.r6_binary($store)[0];
            let referent = binary.r5_referent($store)[0];
            let obj_a = referent.r16_object($store)[0];

            let binary = b.r6_binary($store)[0];
            let referent = binary.r5_referent($store)[0];
            let obj_b = referent.r16_object($store)[0];

            obj_a.name.cmp(&obj_b.name)
        });

        referrers
    }};
}
pub(crate) use get_binary_referrers_sorted;

macro_rules! get_binary_referents_sorted {
    ($obj:expr, $store:expr) => {{
        let mut referents = $obj.r16_referent($store);
        referents.sort_by(|a, b| {
            let binary = a.r5_binary($store)[0];
            let referrer = binary.r6_referrer($store)[0];
            let obj_a = referrer.r17_object($store)[0];

            let binary = b.r5_binary($store)[0];
            let referrer = binary.r6_referrer($store)[0];
            let obj_b = referrer.r17_object($store)[0];

            obj_a.name.cmp(&obj_b.name)
        });

        referents
    }};
}
pub(crate) use get_binary_referents_sorted;

pub(crate) fn render_method_definition(
    buffer: &mut Buffer,
    method: &ObjectMethod,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    let object = domain.sarzak().exhume_object(&method.object).unwrap();

    log::debug!("Rendering new method definition for {}", object.as_ident());

    // Write the beginning of the definition
    write!(
        buffer,
        "pub fn {}(",
        method.r25_function(woog).pop().unwrap().as_ident()
    )
    .context(FormatSnafu)?;

    // By my calculations this should grab the first parameter in the list.
    // Not a very slick way of doing it.
    // 🚧 I suppose I could add a pointer to the first parameter as a relationship
    // on the method.
    let param = woog.iter_parameter().find(|p| {
        if let Some(func_id) = p.function {
            func_id == method.r25_function(woog).pop().unwrap().id
                && p.r1c_parameter(woog).len() == 0
        } else {
            false
        }
    });

    ensure!(
        param.is_some(),
        CompilerSnafu {
            description: format!(
                "No parameter found for {}::{}",
                object.as_type(&Ownership::Owned(OWNED), woog, domain),
                method.r25_function(woog).pop().unwrap().as_ident()
            )
        }
    );
    let mut param = param.unwrap();

    loop {
        let value = param
            .r8_variable(woog)
            .pop()
            .unwrap()
            .r7_value(woog)
            .pop()
            .unwrap();
        let ty = value.r3_grace_type(woog)[0];
        let access = value.r16_access(woog)[0];
        let mutability = access.r15_ownership(woog)[0];

        write!(
            buffer,
            "{}: {},",
            param.r8_variable(woog)[0].name.as_ident(),
            ty.as_type(&mutability, woog, domain)
        )
        .context(FormatSnafu)?;

        if let Some(next_param) = param.r1_parameter(woog).pop() {
            param = next_param;
        } else {
            break;
        }
    }

    // 🚧 This is incorrect, and I'm not yet sure what correct looks like.
    // I think it may be that we need to trace method -> call, and use the
    // type of call as the return type.
    // Finish the first line of the definition
    writeln!(
        buffer,
        ") -> {} {{",
        object.as_type(
            &woog
                .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                .unwrap(),
            woog,
            domain
        )
    )
    .context(FormatSnafu)?;

    Ok(())
}

pub(crate) fn render_make_uuid(
    buffer: &mut Buffer,
    var: &Local,
    method: &ObjectMethod,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    let ty = var
        .r8_variable(woog)
        .pop()
        .unwrap()
        .r7_value(woog)
        .pop()
        .unwrap()
        .r3_grace_type(woog)[0];

    ensure!(
        match ty.subtype {
            GraceTypeEnum::Ty(id) => {
                let sty = domain.sarzak().exhume_ty(&id).unwrap();
                match sty {
                    Ty::Uuid(_) => true,
                    _ => false,
                }
            }
            _ => false,
        },
        CompilerSnafu {
            description: format!("type mismatch: found `{:?}`, expected `Type::Uuid`", ty)
        }
    );

    let object = domain.sarzak().exhume_object(&method.object).unwrap();

    // We want to render a UUID made up of all of the parameters to the function.
    // So we do the cheap thing and just use the parameter list.
    let param = woog.iter_parameter().find(|p| {
        if let Some(func_id) = p.function {
            func_id == method.r25_function(woog).pop().unwrap().id
                && p.r1c_parameter(woog).len() == 0
        } else {
            false
        }
    });

    ensure!(
        param.is_some(),
        CompilerSnafu {
            description: format!(
                "No parameter found for {}::{}",
                object.as_type(&Ownership::Owned(OWNED), woog, domain),
                method.r25_function(woog).pop().unwrap().as_ident()
            )
        }
    );

    let mut param = param.unwrap();

    let mut format_string = String::new();
    let mut args = String::new();

    loop {
        let value = param
            .r8_variable(woog)
            .pop()
            .unwrap()
            .r7_value(woog)
            .pop()
            .unwrap();
        let ty = value.r3_grace_type(woog)[0];

        match ty.subtype {
            GraceTypeEnum::Reference(_) => {
                format_string.extend(["{:?}:"]);
                args.extend([param.r8_variable(woog)[0].name.as_ident(), ",".to_owned()]);
            }
            GraceTypeEnum::WoogOption(_) => {
                format_string.extend(["{:?}:"]);
                args.extend([param.r8_variable(woog)[0].name.as_ident(), ",".to_owned()]);
            }
            GraceTypeEnum::Ty(id) => {
                let ty = domain.sarzak().exhume_ty(&id).unwrap();
                match &ty {
                    // This is really about the store, and we don't want to include that.
                    // However, I don't think we'd want to try printing anything external,
                    // so this here is generally a Good Thing.
                    Ty::External(e) => {
                        let ext = domain.sarzak().exhume_external(e).unwrap();
                        // 🚧 This is lame. I need something better, and nothing comes
                        // immediately to mind.
                        if ext.name == "SystemTime" {
                            format_string.extend(["{:?}:"]);
                            args.extend([
                                param.r8_variable(woog)[0].name.as_ident(),
                                ",".to_owned(),
                            ]);
                        }
                    }
                    _ => {
                        format_string.extend(["{}:"]);
                        args.extend([param.r8_variable(woog)[0].name.as_ident(), ",".to_owned()]);
                    }
                }
            }
            _ => {
                format_string.extend(["{}:"]);
                args.extend([param.r8_variable(woog)[0].name.as_ident(), ",".to_owned()]);
            }
        }

        if let Some(next_param) = param.r1_parameter(woog).pop() {
            param = next_param;
        } else {
            break;
        }
    }

    // Remove the trailing ":"
    format_string.pop();
    // And the trailining ","
    args.pop();

    emit!(
        buffer,
        "let {} = Uuid::new_v5(&UUID_NS, format!(\"{}\", {}).as_bytes());",
        var.r8_variable(woog)[0].name,
        format_string,
        args
    );

    Ok(())
}

pub(crate) fn render_new_instance(
    buffer: &mut Buffer,
    object: &Object,
    var: &Variable,
    structure: &StructExpression,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    let ty = var.r7_value(woog).pop().unwrap().r3_grace_type(woog)[0];

    // Check that the type of the variable is a reference to the object that we
    // are instantiating.
    // This doesn't belong here. It should be part of a let statement renderer.
    // 🚧 These errors are terrible. You get a uuid that may not even be possible
    // to look up. It should print the generated type. That would be fucking slick.
    ensure!(
        match ty.subtype {
            GraceTypeEnum::Reference(id) => {
                let reference = woog.exhume_reference(&id).unwrap();
                let referent = reference.r13_reference_type(woog)[0];
                let ref_obj = match referent {
                    ReferenceType::Object(id) => domain.sarzak().exhume_object(&id).unwrap(),
                    ReferenceType::EnumerationField(id) => woog
                        .exhume_enumeration_field(&id)
                        .unwrap()
                        .r36_enumeration(woog)[0]
                        .r40_object(domain.sarzak())[0],
                    道 => todo!("Apparently you need to deal with {:?}", 道),
                };
                ensure!(ref_obj.id == object.id, {
                    CompilerSnafu {
                        description: format!(
                            "type mismatch: found `{}: &{}`, expected `{}: &{}`",
                            var.name.as_ident(),
                            ref_obj.as_type(
                                &woog
                                    .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                                    .unwrap(),
                                woog,
                                domain
                            ),
                            var.name.as_ident(),
                            object.as_type(
                                &woog
                                    .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                                    .unwrap(),
                                woog,
                                domain
                            ),
                        ),
                    }
                });
                true
            }
            _ => false,
        },
        CompilerSnafu {
            description: format!(
                "type mismatch: found `{:?}`, expected `SarzakType::Reference`",
                ty
            )
        }
    );

    // Get the fields for the struct, in the order in which god intended. It's a pain
    // in the ass. I do this elsewhere, and it's a pain in the ass there too. I would
    // think a macro possible...
    // The elsewhere is functions and parameters. From a modeling perspective this is
    // probably appropriate. I could add a relationship to the first field/param I
    // suppose...
    let mut first = structure
        .r27_struct_expression_field(woog)
        .iter()
        .find(|&&field| field.r30c_struct_expression_field(woog).len() == 0)
        .unwrap()
        .clone();

    let mut fields = vec![first];
    loop {
        if let Some(next) = first.r30_struct_expression_field(woog).pop() {
            fields.push(next);
            first = next;
        } else {
            break;
        }
    }

    // this should be done as part of a let statement rendering
    write!(buffer, "let {} = ", var.as_ident()).context(FormatSnafu)?;

    emit!(
        buffer,
        "{} {{",
        object.as_type(
            &woog
                .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                .unwrap(),
            woog,
            domain
        )
    );

    for field in fields {
        let expr = woog.exhume_expression(&field.expr).unwrap();
        let rhs = match expr {
            Expression::Literal(id) => {
                let literal = woog.exhume_literal(id).unwrap();
                match literal {
                    Literal::Hack(id) => {
                        let hack = woog.exhume_hack(id).unwrap();
                        &hack.value
                    }
                    道 => todo!("Apparently you need to deal with {:?}", 道),
                }
            }
            道 => todo!("Apparently you need to deal with {:?}", 道),
        };
        emit!(buffer, "{}: {},", field.name.as_ident(), rhs);
    }

    emit!(buffer, "}};");

    Ok(())
}

/// Perform type checking
///
/// I'm going to add to this piecemeal as I go.
pub(crate) fn xlet_type_check(
    var: &Variable,
    expr: &Expression,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    dbg!(&var, &expr);
    let lval_ty = var.r7_value(woog)[0].r3_grace_type(woog)[0];
    let rval_ty = expr.r7_value(woog)[0].r3_grace_type(woog)[0];

    match &lval_ty.subtype {
        GraceTypeEnum::Reference(id) => {
            // De-reference the variable and check that it matches the type of the
            // expression.
            let reference = woog.exhume_reference(&id).unwrap();
            let referent = reference.r13_reference_type(woog)[0];
            let ref_obj = match referent {
                ReferenceType::Object(id) => domain.sarzak().exhume_object(&id).unwrap(),
                道 => todo!("Apparently you need to deal with {:?}", 道),
            };

            // Note we are checking the rval here.
            match &rval_ty.subtype {
                GraceTypeEnum::Ty(id) => {
                    let ty = domain.sarzak().exhume_ty(&id).unwrap();
                    match ty {
                        // Object on object action...
                        Ty::Object(id) => {
                            ensure!(
                                ref_obj.id == *id,
                                CompilerSnafu {
                                    description: format!(
                                        "type mismatch: found `{}`, expected `{}`",
                                        print_type(&rval_ty, woog, domain),
                                        print_type(&lval_ty, woog, domain)
                                    )
                                }
                            );
                        }
                        道 => todo!("Apparently you need to deal with {:?}", 道),
                    }
                }
                道 => todo!(
                    "Apparently you need to deal with `{}`: {:?}",
                    print_type(&rval_ty, woog, domain),
                    道
                ),
            }
        }
        GraceTypeEnum::Ty(id) => {
            let ty = domain.sarzak().exhume_ty(&id).unwrap();
            match ty {
                Ty::Uuid(_) => match &rval_ty.subtype {
                    GraceTypeEnum::Ty(id) => {
                        let ty = domain.sarzak().exhume_ty(&id).unwrap();
                        match ty {
                            Ty::Uuid(_) => {}
                            _ => ensure!(
                                false,
                                CompilerSnafu {
                                    description: format!(
                                        "type mismatch: found `{}`, expected `{}`",
                                        print_type(&rval_ty, woog, domain),
                                        print_type(&lval_ty, woog, domain)
                                    )
                                }
                            ),
                        }
                    }
                    GraceTypeEnum::Function(id) => {
                        let fun = woog.exhume_function(&id).unwrap();
                        let ty = fun.r47_grace_type(woog)[0];
                        match &ty.subtype {
                            GraceTypeEnum::Ty(id) => {
                                let ty = domain.sarzak().exhume_ty(&id).unwrap();
                                match ty {
                                    Ty::Uuid(_) => {}
                                    _ => ensure!(
                                        false,
                                        CompilerSnafu {
                                            description: format!(
                                                "type mismatch: found `{}`, expected `{}`",
                                                print_type(&rval_ty, woog, domain),
                                                print_type(&lval_ty, woog, domain)
                                            )
                                        }
                                    ),
                                }
                            }
                            GraceTypeEnum::Reference(id) => {
                                let reference = woog.exhume_reference(&id).unwrap();
                                let referent = reference.r13_reference_type(woog)[0];
                                dbg!(print_type(&lval_ty, woog, domain), &referent);
                                match referent {
                                    ReferenceType::External(id) => {
                                        let ext = domain.sarzak().exhume_external(&id).unwrap();
                                        let obj = domain
                                            .sarzak()
                                            .exhume_object_by_name(&ext.name)
                                            .unwrap();
                                        ensure!(
                                            false,
                                            CompilerSnafu {
                                                description: "duh!".to_string()
                                            }
                                        );
                                    }
                                    道 => todo!("Apparently you need to deal with {:?}", 道),
                                }
                                ensure!(
                                    false,
                                    CompilerSnafu {
                                        description: "Unimplemented".to_string()
                                    }
                                );
                            }
                            道 => todo!(
                                "Apparently you need to deal with `{}`: {:?}",
                                print_type(&rval_ty, woog, domain),
                                道
                            ),
                        }
                    }
                    道 => todo!(
                        "Apparently you need to deal with `{}`: {:?}",
                        print_type(&rval_ty, woog, domain),
                        道
                    ),
                },
                Ty::External(id) => {
                    // You are here because the lval_ty is GraceTypeEnum::Ty::External.
                    let ext = domain.sarzak().exhume_external(id).unwrap();
                    dbg!(&lval_ty, &rval_ty);
                    dbg!(
                        print_type(&lval_ty, woog, domain),
                        print_type(&rval_ty, woog, domain)
                    );
                    match &rval_ty.subtype {
                        GraceTypeEnum::Ty(id) => {
                            let ty = domain.sarzak().exhume_ty(&id).unwrap();
                            match ty {
                                Ty::External(id) => {
                                    let ext2 = domain.sarzak().exhume_external(id).unwrap();
                                    ensure!(
                                        ext.name == ext2.name,
                                        CompilerSnafu {
                                            description: format!(
                                                "type mismatch: found `{}`, expected `{}`",
                                                print_type(&rval_ty, woog, domain),
                                                print_type(&lval_ty, woog, domain)
                                            )
                                        }
                                    );
                                }
                                _ => ensure!(
                                    false,
                                    CompilerSnafu {
                                        description: format!(
                                            "type mismatch: found `{}`, expected `{}`",
                                            print_type(&rval_ty, woog, domain),
                                            print_type(&lval_ty, woog, domain)
                                        )
                                    }
                                ),
                            }
                        }
                        道 => ensure!(
                            {
                                dbg!(print_type(&rval_ty, woog, domain));
                                false
                            },
                            CompilerSnafu {
                                description: format!(
                                    "type mismatch: found `{}`, expected `{}`",
                                    print_type(&rval_ty, woog, domain),
                                    print_type(&lval_ty, woog, domain)
                                )
                            }
                        ),
                    }
                }
                道 => todo!("Apparently you need to deal with {:?}", 道),
            }
        }
        道 => todo!(
            "Apparently you need to deal with `{}`: {:?}",
            print_type(&lval_ty, woog, domain),
            道
        ),
    }

    Ok(())
}

pub(crate) fn print_type(ty: &GraceType, woog: &WoogStore, domain: &Domain) -> String {
    dbg!("GraceType", &ty);
    match &ty.subtype {
        GraceTypeEnum::Reference(id) => {
            let reference = woog.exhume_reference(&id).unwrap();
            let referent = reference.r13_reference_type(woog)[0];
            match referent {
                ReferenceType::Object(id) => {
                    let ref_obj = domain.sarzak().exhume_object(&id).unwrap();
                    format!(
                        "&{}",
                        ref_obj.as_type(
                            &woog
                                .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                                .unwrap(),
                            woog,
                            domain
                        )
                    )
                }
                ReferenceType::EnumerationField(id) => {
                    let ref_obj = woog
                        .exhume_enumeration_field(&id)
                        .unwrap()
                        .r36_enumeration(woog)[0]
                        .r40_object(domain.sarzak())[0];
                    format!(
                        "&{}",
                        ref_obj.as_type(
                            &woog
                                .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                                .unwrap(),
                            woog,
                            domain
                        )
                    )
                }
                ReferenceType::External(id) => {
                    let ext = domain.sarzak().exhume_external(&id).unwrap();
                    format!("&{}", ext.name)
                }
                道 => todo!("Apparently you need to deal with {:?}", 道),
            }
        }
        GraceTypeEnum::Ty(id) => {
            let ty = domain.sarzak().exhume_ty(&id).unwrap();
            dbg!("GraceTypeEnum::Ty", &ty);
            match ty {
                Ty::Object(id) => domain.sarzak().exhume_object(&id).unwrap().as_type(
                    &woog
                        .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                        .unwrap(),
                    woog,
                    domain,
                ),
                Ty::Uuid(_) => "Uuid".to_owned(),
                Ty::External(id) => {
                    let ext = domain.sarzak().exhume_external(&id).unwrap();
                    ext.name.clone()
                }
                道 => todo!("Apparently you need to deal with {:?}", 道),
            }
        }
        GraceTypeEnum::Function(id) => {
            let function = woog.exhume_function(&id).unwrap();
            match function.subtype {
                FunctionEnum::ObjectMethod(id) => {
                    let obj = woog
                        .exhume_object_method(&id)
                        .unwrap()
                        .r4_object(domain.sarzak())[0];
                    format!(
                        "{}::{}(...)",
                        obj.as_type(
                            &woog
                                .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                                .unwrap(),
                            woog,
                            domain,
                        ),
                        function.name,
                    )
                }
                FunctionEnum::PlainOldFunction(_) => format!("{}(...)", function.name),
                FunctionEnum::Closure(_) => format!("{{{}}}(...)", function.name),
            }
        }
        道 => todo!(
            "Apparently you need to deal with `{}`: {:?}",
            print_type(&ty, woog, domain),
            道
        ),
    }
}

pub(crate) fn render_struct_expression(
    struct_expression: &StructExpression,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<String> {
    let mut result = String::new();

    let object = if let GraceTypeEnum::Ty(id) =
        struct_expression.r10_expression(woog)[0].r7_value(woog)[0].r3_grace_type(woog)[0].subtype
    {
        // This is cheating a little bit.Ty(id) actually maps to
        // Ty::Object(id), but the id is the same in both cases.
        // So, cheating.
        domain.sarzak().exhume_object(&id).unwrap()
    } else {
        ensure!(
            false,
            CompilerSnafu {
                description: "Expected Struct Expression to be an `Object`.".to_string()
            }
        );
        panic!()
    };

    // Get the fields for the struct, in the order in which god intended. It's a pain
    // in the ass. I do this elsewhere, and it's a pain in the ass there too. I would
    // think a macro possible...
    // The elsewhere is functions and parameters. From a modeling perspective this is
    // probably appropriate. I could add a relationship to the first field/param I
    // suppose...
    let mut first = struct_expression
        .r27_struct_expression_field(woog)
        .iter()
        .find(|&&field| field.r30c_struct_expression_field(woog).len() == 0)
        .unwrap()
        .clone();

    // Why do I collect these into a vec? I could do a loop...
    let mut fields = vec![first];
    loop {
        if let Some(next) = first.r30_struct_expression_field(woog).pop() {
            fields.push(next);
            first = next;
        } else {
            break;
        }
    }

    emit!(
        result,
        "{} {{",
        object.as_type(
            &woog
                .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                .unwrap(),
            woog,
            domain
        )
    );

    for field in fields {
        let expr = woog.exhume_expression(&field.expr).unwrap();
        let rhs = match expr {
            Expression::Literal(id) => {
                let literal = woog.exhume_literal(id).unwrap();
                match literal {
                    Literal::Hack(id) => {
                        let hack = woog.exhume_hack(id).unwrap();
                        hack.value.clone()
                    }
                    道 => todo!("Apparently you need to deal with {:?}", 道),
                }
            }
            Expression::Variable(id) => {
                let variable = woog.exhume_variable(&id).unwrap();
                variable.name.as_ident()
            }
            道 => todo!("Apparently you need to deal with {:?}", 道),
        };
        emit!(result, "{}: {},", field.name.as_ident(), rhs);
    }

    emit!(result, "}};");

    Ok(result)
}

/// 🚧 This renders the only method it can find, and the name of the local that
/// it's assigned to is hard-coded. This is a problem.
pub(crate) fn render_methods(
    buffer: &mut Buffer,
    obj: &Object,
    config: &GraceConfig,
    _imports: &Option<&HashMap<String, Domain>>,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    let mut methods: Vec<&ObjectMethod> = woog.iter_object_method().collect();
    methods.sort_by(|a, b| {
        a.r25_function(woog)[0]
            .name
            .cmp(&b.r25_function(woog)[0].name)
    });

    for method in methods {
        if method.object == obj.id {
            buffer.block(
                DirectiveKind::IgnoreOrig,
                format!("{}-struct-impl-new", obj.as_ident()),
                |buffer| {
                    dbg!(&method.r25_function(woog)[0].name);
                    // Output a docstring
                    emit!(
                        buffer,
                        "/// {}",
                        method.r25_function(woog).pop().unwrap().description
                    );

                    // This renders the method signature.
                    // It's probably ok as it is.
                    render_method_definition(buffer, &method, woog, domain)?;

                    // Find the properly scoped variable named `id`.
                    let block = method.r25_function(woog)[0].r23_block(woog)[0];
                    let table = block.r24_symbol_table(woog)[0];

                    // Iterate over the statements in the block, generating code
                    // for each.
                    dbg!(block.r12_statement(woog).len());
                    for stmt in block.r12_statement(woog) {
                        dbg!(&stmt);
                    }
                    if let Some(mut statement) = block
                        .r12_statement(woog)
                        .iter()
                        .cloned()
                        .find(|&s| s.r31c_statement(woog).len() == 0)
                    {
                        loop {
                            render_statement(buffer, statement, config, woog, domain)?;

                            let mut next = statement.r31_statement(woog);
                            statement = if let Some(next) = next.pop() {
                                next
                            } else {
                                break;
                            };
                        }
                    }

                    let var = &table
                        .r29_variable(woog)
                        .iter()
                        .find(|&&v| v.name == "id")
                        .unwrap()
                        .subtype;
                    let id = match var {
                        // This works because the id of the variable is the same as the id of the
                        // subtype enum.
                        VariableEnum::Local(id) => woog.exhume_local(&id).unwrap(),
                        _ => panic!("This should never happen"),
                    };

                    // // This renders a let statement, assigning a new uuid to the id variable.
                    // // This is where the work lies. I think that what I really want to do is
                    // // create (let) statements in the block whilst populating woog. Then
                    // // someplace else, maybe here, we iterate over the statements and generate
                    // // code. Maybe an as_statement trait, or something?
                    // render_make_uuid(buffer, &id, &method, woog, domain)?;

                    // // Now this is interesting. This is good. It's getting close to what I
                    // // was talking about above. In the woog population code, the function
                    // // for populating a new method I created a statement: a struct item.
                    // // It's the struct for Self. I pull that out here, and then use it when
                    // // I call the renderer.
                    // // 💥 put this back once things are sorted
                    // let let_stmt = match &method.r25_function(woog)[0]
                    //     .r23_block(woog)
                    //     .pop()
                    //     .unwrap()
                    //     .r12_statement(woog)
                    //     .pop()
                    //     .unwrap()
                    //     .subtype
                    // {
                    //     StatementEnum::XLet(id) => woog.exhume_x_let(id).unwrap(),
                    //     道 => todo!("Apparently you need to deal with `{}`: {:?}", print_type(&道, woog, domain), 道),
                    // };

                    // let var = let_stmt.r17_variable(woog)[0];
                    // let struct_expr = match &let_stmt.r18_expression(woog)[0] {
                    //     Expression::StructExpression(id) => {
                    //         woog.exhume_struct_expression(id).unwrap()
                    //     }
                    //     道 => todo!("Apparently you need to deal with `{}`: {:?}", print_type(&道, woog, domain), 道),
                    // };

                    // // I wrote this this morning, and already I'can't say how it works
                    // // exactly. It takes a structure, and not a statement, so it's
                    // // pretty low level. It's also assigning the let. Refactor time.
                    // render_new_instance(buffer, obj, &var, &struct_expr, woog, domain)?;

                    emit!(buffer, "store.inter_{}(new.clone());", obj.as_ident());
                    emit!(buffer, "new");
                    emit!(buffer, "}}");

                    Ok(())
                },
            )?;
        }
    }

    Ok(())
}

macro_rules! test_local_and_imports {
    ($name:ident, $func:ident) => {
        pub(crate) fn $name(
            object: &Object,
            config: &GraceConfig,
            imports: &Option<&HashMap<String, Domain>>,
            domain: &Domain,
        ) -> Result<bool> {
            if config.is_imported(&object.id) {
                let imported = config.get_imported(&object.id).unwrap();
                ensure!(
                    imports.is_some(),
                    CompilerSnafu {
                        description: format!(
                            "object `{}` is imported, but domain not found",
                            object.name
                        )
                    }
                );
                let imports = imports.unwrap();

                // We are shadowing domain here...
                let domain = imports.get(&imported.domain);
                ensure!(
                    domain.is_some(),
                    CompilerSnafu {
                        description: format!(
                            "object `{}` is imported, but domain not found",
                            object.name
                        )
                    }
                );
                let domain = domain.unwrap();

                ensure!(
                    domain.sarzak().exhume_object(&imported.id).is_some(),
                    CompilerSnafu {
                        description: format!(
                            "object `{}` ({}) is not found in imported domain {}",
                            object.name, object.id, imported.domain
                        )
                    }
                );

                let mut object = object.clone();
                object.id = imported.id;
                Ok($func(&object, config, domain))
            } else {
                Ok($func(object, config, domain))
            }
        }
    };
}

test_local_and_imports!(object_is_const, local_object_is_const);
pub(crate) fn local_object_is_const(
    object: &Object,
    config: &GraceConfig,
    domain: &Domain,
) -> bool {
    local_object_is_singleton(object, config, domain)
        && !local_object_is_supertype(object, config, domain)
}

pub(crate) fn local_object_is_struct(
    object: &Object,
    config: &GraceConfig,
    domain: &Domain,
) -> bool {
    !local_object_is_supertype(object, config, domain)
        && !local_object_is_singleton(object, config, domain)
}

test_local_and_imports!(object_is_hybrid, local_object_is_hybrid);
pub(crate) fn local_object_is_hybrid(
    object: &Object,
    config: &GraceConfig,
    domain: &Domain,
) -> bool {
    let attrs = object.r1_attribute(domain.sarzak());
    log::debug!("attrs: {:?}", attrs);

    local_object_is_supertype(object, config, domain)
        && (attrs.len() > 1 || local_object_is_referrer(object, config, domain))
}

test_local_and_imports!(object_is_enum, local_object_is_enum);
pub(crate) fn local_object_is_enum(object: &Object, config: &GraceConfig, domain: &Domain) -> bool {
    local_object_is_supertype(object, config, domain)
        && !local_object_is_hybrid(object, config, domain)
}

test_local_and_imports!(object_is_supertype, local_object_is_supertype);
pub(crate) fn local_object_is_supertype(
    object: &Object,
    _config: &GraceConfig,
    domain: &Domain,
) -> bool {
    let is_super = object.r14_supertype(domain.sarzak());
    log::debug!("is_super: {:?}", is_super);

    is_super.len() > 0
}

// test_local_and_imports!(object_is_subtype, local_object_is_subtype);
pub(crate) fn local_object_is_subtype(
    object: &Object,
    _config: &GraceConfig,
    domain: &Domain,
) -> bool {
    let is_sub = object.r15_subtype(domain.sarzak());
    log::debug!("is_sub: {:?}", is_sub);

    is_sub.len() > 0
}

test_local_and_imports!(object_is_singleton, local_object_is_singleton);
pub(crate) fn local_object_is_singleton(
    object: &Object,
    config: &GraceConfig,
    domain: &Domain,
) -> bool {
    if config.is_external(&object.id) {
        return false;
    }

    let attrs = object.r1_attribute(domain.sarzak());
    log::debug!("attrs: {:?}", attrs);

    attrs.len() < 2
        && !local_object_is_referrer(object, config, domain)
        && !local_object_is_supertype(object, config, domain)
}

// test_local_and_imports!(object_is_referrer, inner_object_is_referrer);
fn local_object_is_referrer(object: &Object, _config: &GraceConfig, domain: &Domain) -> bool {
    let referrers = object.r17_referrer(domain.sarzak());
    let assoc_referrers = object.r26_associative_referrer(domain.sarzak());
    log::debug!("referrers: {:?}", referrers);
    log::debug!("assoc_referrers: {:?}", assoc_referrers);

    referrers.len() > 0 || assoc_referrers.len() > 0
}

/// Generate struct/enum Documentation
///
/// The text from the tool is really long lines separated by `\n`. We split
/// the lines up on unicode word boundaries and then reconstitute keeping the
/// generated line length less than `MAX_LEN` characters.
///
/// It would be extra sweet to extract the doc links and construct pointers to
/// known types. For example, "points at an [`Object`]", would turn into
/// "points at an [`Object`][o]", and we'd generate an "[o]: nut::sarzak::Object"
/// at the bottom of the comments.
///
/// This is still pretty cool compared to before. The long strings really got
/// to me.
pub(crate) fn emit_object_comments(input: &str, comment: &str, context: &mut Buffer) -> Result<()> {
    const MAX_LEN: usize = 90;

    if input.len() > 0 {
        for line in input.split('\n') {
            write!(context, "{} ", comment).context(FormatSnafu)?;
            let mut length = 4;

            // Split the string by words, and append a word until we run out
            // of room in the line. Then start another.
            for word in line.split_word_bounds() {
                match length {
                    n if n < MAX_LEN + word.len() => {
                        write!(context, "{}", word).context(FormatSnafu)?;
                        length += word.len();
                    }
                    _ => {
                        // Trim the trailing space, which I think is guaranteed to
                        // be there, but I'll be cautious anyway. Oh, but I can't
                        // because I don't own the buffer. Shit.

                        // Add a newline
                        emit!(context, "");
                        length = 0;

                        write!(context, "{}{}", comment, word).context(FormatSnafu)?;
                        length += word.len() + 3;
                    }
                }
            }

            // Add a trailing newline
            emit!(context, "");
        }

        emit!(context, "{}", comment);
    }

    Ok(())
}

pub(crate) fn find_store<'a>(name: &str, woog: &WoogStore, domain: &'a Domain) -> &'a External {
    let name = if name.contains("::") {
        name.split("::")
            .last()
            .expect(format!("Can't parse store from {}", name).as_str())
    } else {
        name.split("/")
            .last()
            .expect(format!("Can't parse store from {}", name).as_str())
    };
    let name = format!(
        "{}Store",
        name.as_type(
            &woog
                .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                .unwrap(),
            woog,
            domain
        )
    );

    domain.sarzak().exhume_external_by_name(&name).unwrap()
}

const BUILD_TIME: &str = include!(concat!(env!("OUT_DIR"), "/timestamp.txt"));

pub(crate) fn is_object_stale(object: &Object, woog: &WoogStore, domain: &Domain) -> bool {
    let last_time = if let Some(gu) = woog
        .iter_generation_unit()
        .find(|gu| gu.object == object.id)
    {
        log::debug!("Found generation unit for object {}", object.name);
        woog.generation_unit_timestamp(gu)
    } else {
        log::debug!("No generation unit for object {}", object.name);
        return true;
    };

    // Always rebuild with a newer compiler.
    let built_time = chrono::DateTime::parse_from_rfc3339(&BUILD_TIME).unwrap();
    if last_time < built_time.into() {
        return true;
    }

    if domain.sarzak().object_timestamp(object) > last_time {
        return true;
    }

    for attr in object.r1_attribute(domain.sarzak()) {
        if domain.sarzak().attribute_timestamp(&attr) > last_time {
            return true;
        }
    }

    for supertype in object.r14_supertype(domain.sarzak()) {
        if domain.sarzak().supertype_timestamp(supertype) > last_time {
            return true;
        }
    }

    for subtype in object.r15_subtype(domain.sarzak()) {
        if domain.sarzak().subtype_timestamp(subtype) > last_time {
            return true;
        }
    }

    for referent in object.r16_referent(domain.sarzak()) {
        if domain.sarzak().referent_timestamp(referent) > last_time {
            return true;
        }
    }

    for referrer in object.r17_referrer(domain.sarzak()) {
        if domain.sarzak().referrer_timestamp(referrer) > last_time {
            return true;
        }
    }

    for assoc_referent in object.r25_associative_referent(domain.sarzak()) {
        if domain
            .sarzak()
            .associative_referent_timestamp(assoc_referent)
            > last_time
        {
            return true;
        }
    }

    for assoc_referrer in object.r26_associative_referrer(domain.sarzak()) {
        if domain
            .sarzak()
            .associative_referrer_timestamp(assoc_referrer)
            > last_time
        {
            return true;
        }
    }

    for state in object.r18_state(domain.sarzak()) {
        if domain.sarzak().state_timestamp(state) > last_time {
            return true;
        }
    }

    for event in object.r19_event(domain.sarzak()) {
        if domain.sarzak().event_timestamp(event) > last_time {
            return true;
        }
    }

    return false;
}

fn render_statement(
    buffer: &mut Buffer,
    statement: &Statement,
    config: &GraceConfig,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    match &statement.subtype {
        StatementEnum::XLet(id) => {
            let xlet = woog.exhume_x_let(&id).unwrap();
            let var = xlet.r17_variable(woog)[0];
            let expr = xlet.r18_expression(woog)[0];

            log::trace!("render_statement: let {} = {:?}", var.name, expr);

            // Type check the let statement
            xlet_type_check(&var, &expr, woog, domain).map_err(|e| {
                let ty = expr.r7_value(woog)[0].r3_grace_type(woog)[0];
                eprintln!(
                    "let {} = {};",
                    var.name.as_ident(),
                    print_type(ty, woog, domain)
                );
                e
            })?;

            let expr = match &expr {
                Expression::Literal(id) => {
                    let literal = woog.exhume_literal(&id).unwrap();
                    match literal {
                        Literal::Hack(id) => {
                            let hack = woog.exhume_hack(&id).unwrap();
                            log::trace!("literal_hack: {}", hack.value);
                            hack.value.clone()
                        }
                        道 => todo!("Apparently you need to deal with {:?}", 道),
                    }
                }
                Expression::StructExpression(id) => {
                    log::trace!("literal_struct_expression: {}", id);
                    let struct_expression = woog.exhume_struct_expression(&id).unwrap();
                    render_struct_expression(struct_expression, woog, domain)?
                }
                Expression::Variable(id) => {
                    let variable = woog.exhume_variable(&id).unwrap();
                    log::trace!("literal_variable: {}", variable.name);
                    let value = variable.r7_value(woog)[0];
                    let ty = var.r7_value(woog)[0].r3_grace_type(woog)[0];
                    // What's the difference between this and
                    // type_check above? Besides operating on
                    // different parameters? Here we are using the string output
                    // and above we are just testing.
                    // Above is a test is specialized to `let`: it checks a var
                    // against an expression. Below is more general, it tests
                    // takes an expression and a type, and squirts out some
                    // text to make it work.
                    typecheck_and_coerce(&ty, &value, config, woog, domain)?
                }
                Expression::Call(id) => {
                    let call = woog.exhume_call(&id).unwrap();
                    log::trace!("literal_call: {}", {
                        let function = call.r19_function(woog)[0];
                        &function.name
                    });
                    let value = call.r10_expression(woog)[0].r7_value(woog)[0];
                    let ty = var.r7_value(woog)[0].r3_grace_type(woog)[0];

                    typecheck_and_coerce(&ty, &value, config, woog, domain)?
                }
                道 => {
                    todo!("Apparently you need to deal with {:?}", 道)
                }
            };

            emit!(buffer, "let {} = {};", var.name.as_ident(), expr);
        }
        道 => todo!("Apparently you need to deal with {:?}", 道),
    }

    Ok(())
}

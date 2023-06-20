//! Things necessary for code generation
//!
pub(crate) mod buffer;
pub(crate) mod diff_engine;
pub(crate) mod generator;
pub(crate) mod render;
mod rustfmt;

use std::{fmt::Write, iter::zip, sync::Arc};

use fnv::FnvHashMap as HashMap;
use sarzak::{
    lu_dog::{
        types::{ValueType, WoogOption},
        Reference,
    },
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::types::{Conditionality, External, Object, Ty},
    v2::domain::Domain,
    woog::{
        store::ObjectStore as WoogStore,
        types::{
            GraceType, Item, Local, ObjectMethod as WoogObjectMethod, Ownership, StatementEnum,
            Structure, SymbolTable, Variable, VariableEnum,
        },
    },
};
use snafu::prelude::*;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        render::{ForStore, RenderIdent, RenderType},
    },
    options::{GraceConfig, UberStoreOptions},
    target::dwarf::LU_DOG,
    todo::{GType, LValue, ObjectMethod, RValue},
    Lock,
};

macro_rules! get_subtypes_sorted {
    ($obj:expr, $store:expr) => {{
        let mut subtypes = $obj.r15_subtype($store);
        subtypes.sort_by(|a, b| {
            // We can't go back across r15 to object, because it's the subtype, and
            // it'll be the same for all. Instead we go to the supertype object and
            // sort on it.
            let a = a.r27_isa($store)[0].r13_supertype($store)[0].r14_object($store)[0];
            let b = b.r27_isa($store)[0].r13_supertype($store)[0].r14_object($store)[0];
            a.name.cmp(&b.name)
        });

        subtypes
    }};
}
pub(crate) use get_subtypes_sorted;

macro_rules! get_subtypes_sorted_from_super_obj {
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
pub(crate) use get_subtypes_sorted_from_super_obj;

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
    config: &GraceConfig,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    let is_uber = config.is_uber_store();

    // Write the beginning of the definition
    if is_uber {
        if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap() {
            write!(buffer, "pub async fn {}(", method.name).context(FormatSnafu)?;
        } else {
            write!(buffer, "pub fn {}(", method.name).context(FormatSnafu)?;
        }
    } else {
        write!(buffer, "pub fn {}(", method.name).context(FormatSnafu)?;
    }

    // Write the parameter list.
    // TODO: This is so clumsy! I should clean it up.
    if let Some(mut param) = method.param {
        let mutability = woog.exhume_ownership(&param.mutability).unwrap();

        if is_uber {
            write!(
                buffer,
                "{}: {},",
                param.as_ident(),
                param.ty.for_store(&mutability, config, woog, domain),
            )
            .context(FormatSnafu)?;
        } else {
            write!(
                buffer,
                "{}: {},",
                param.as_ident(),
                param.ty.as_type(&mutability, woog, domain),
            )
            .context(FormatSnafu)?;
        }

        while let Some(next_param) = param.next {
            let mutability = woog.exhume_ownership(&next_param.mutability).unwrap();

            if is_uber {
                write!(
                    buffer,
                    "{}: {},",
                    // Why do I need to drill down to name?
                    next_param.name.as_ident(),
                    next_param.ty.for_store(&mutability, config, woog, domain),
                )
                .context(FormatSnafu)?;
            } else {
                write!(
                    buffer,
                    "{}: {},",
                    // Why do I need to drill down to name?
                    next_param.name.as_ident(),
                    next_param.ty.as_type(&mutability, woog, domain),
                )
                .context(FormatSnafu)?;
            }

            param = &next_param;
        }
    }

    // Finish the first line of the definition
    if is_uber {
        use UberStoreOptions::*;
        let store_type = match config.get_uber_store().unwrap() {
            Disabled => unreachable!(),
            Single => format!(
                "Rc<RefCell<{}>>",
                method.ty.as_type(&Ownership::new_borrowed(), woog, domain)
            ),
            StdRwLock | ParkingLotRwLock | AsyncRwLock | NDRwLock => format!(
                "Arc<RwLock<{}>>",
                method.ty.as_type(&Ownership::new_borrowed(), woog, domain)
            ),
            StdMutex | ParkingLotMutex => format!(
                "Arc<Mutex<{}>>",
                method.ty.as_type(&Ownership::new_borrowed(), woog, domain)
            ),
        };

        writeln!(buffer, ") -> {store_type} {{",).context(FormatSnafu)?;
    } else {
        writeln!(
            buffer,
            ") -> {} {{",
            method.ty.as_type(&Ownership::new_borrowed(), woog, domain)
        )
        .context(FormatSnafu)?;
    }

    Ok(())
}

pub(crate) fn render_method_definition_new(
    buffer: &mut Buffer,
    method: &WoogObjectMethod,
    config: &GraceConfig,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    let object = domain.sarzak().exhume_object(&method.object).unwrap();
    let is_uber = config.is_uber_store();

    log::debug!("Rendering new method definition for {}", object.as_ident());

    // Write the beginning of the definition
    if is_uber {
        if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap() {
            write!(
                buffer,
                "pub async fn {}(",
                method.r25_function(woog).pop().unwrap().name
            )
            .context(FormatSnafu)?;
        } else {
            write!(
                buffer,
                "pub fn {}(",
                method.r25_function(woog).pop().unwrap().name
            )
            .context(FormatSnafu)?;
        }
    } else {
        write!(
            buffer,
            "pub fn {}(",
            method.r25_function(woog).pop().unwrap().name
        )
        .context(FormatSnafu)?;
    }

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

    if param.is_some() {
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

            let param_name = param.r8_variable(woog)[0].name.as_ident();

            if is_uber && param_name != "store" {
                write!(
                    buffer,
                    "{}: {},",
                    param_name,
                    ty.for_store(&mutability, config, woog, domain)
                )
                .context(FormatSnafu)?;
            } else {
                write!(
                    buffer,
                    "{}: {},",
                    param_name,
                    ty.as_type(&mutability, woog, domain)
                )
                .context(FormatSnafu)?;
            }

            if let Some(next_param) = param.r1_parameter(woog).pop() {
                param = next_param;
            } else {
                break;
            }
        }
    }

    // 🚧 This is incorrect, and I'm not yet sure what correct looks like.
    // I think it may be that we need to trace method -> call, and use the
    // type of call as the return type.
    // Finish the first line of the definition
    if is_uber {
        use UberStoreOptions::*;
        let store_type = match config.get_uber_store().unwrap() {
            Disabled => unreachable!(),
            Single => format!(
                "Rc<RefCell<{}>>",
                object.as_type(&Ownership::new_borrowed(), woog, domain)
            ),
            StdRwLock | ParkingLotRwLock | AsyncRwLock | NDRwLock => format!(
                "Arc<RwLock<{}>>",
                object.as_type(&Ownership::new_borrowed(), woog, domain)
            ),
            StdMutex | ParkingLotMutex => format!(
                "Arc<Mutex<{}>>",
                object.as_type(&Ownership::new_borrowed(), woog, domain)
            ),
        };

        writeln!(buffer, ") -> {store_type} {{",).context(FormatSnafu)?;
    } else {
        writeln!(
            buffer,
            ") -> {} {{",
            object.as_type(&Ownership::new_borrowed(), woog, domain)
        )
        .context(FormatSnafu)?;
    }

    Ok(())
}

/// Generate code to create a new UUID
///
/// Hmmm. This is a function call. I happen to be modeling one of these. Let's
/// see if we can't get it to work with our brand new method definition and
/// friends.
///
/// We've got E_CALL, and expression. It's related to METH via R19. So I need
/// a METH for this. I'm already using METH to generate the new method declaration.
/// So, do I need a METH for this? Well, I don't generate a definition for the
/// uuid methods. A METH would give me information about how to call it though.
///
/// Put a pin in that for a second, and discuss parameter. What's here is gonig
/// away. I have both lvals (variables) and rvals (expressions).
///
/// This function is generating a statement. The lval is just used as a name for
/// the LHS. The rvals are use as the arguments to the function call. So, we'll
/// want to use/replace everything with entities from woog.
///
/// Now, the first thing that annoys me is the arguments to the function. They
/// are locals, and should be in scope. And, now I notice that I don't have
/// that in this model. So, I will merge that other branch as I was considering.

pub(crate) fn render_make_uuid(
    buffer: &mut Buffer,
    lval: &LValue,
    rvals: &Vec<RValue>,
    _domain: &Domain,
) -> Result<()> {
    ensure!(
        lval.ty == GType::Uuid,
        CompilerSnafu {
            description: format!(
                "type mismatch, found `{:?}`, expected `GType::Uuid`",
                lval.ty
            )
        }
    );

    let mut format_string = String::new();
    let mut args = String::new();
    for val in rvals {
        match &val.ty {
            GType::Reference(_) => {
                format_string.extend(["{:?}:"]);
            }
            GType::Option(_) => {
                format_string.extend(["{:?}:"]);
            }
            _ => {
                format_string.extend(["{}:"]);
            }
        }

        args.extend([val.name.to_owned(), ",".to_owned()]);
    }
    // Remove the trailing ":"
    format_string.pop();
    // And the trailining ","
    args.pop();

    // emit!(
    //     buffer,
    //     "let {} = Uuid::new_v5(&UUID_NS, format!(\"{}\", {}).as_bytes());",
    //     lval.name,
    //     format_string,
    //     args
    // );

    emit!(buffer, "let {} = Uuid::new_v4();", lval.name);

    Ok(())
}

pub(crate) fn render_make_uuid_new(
    buffer: &mut Buffer,
    var: &Local,
    method: &WoogObjectMethod,
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
        match ty {
            GraceType::Ty(id) => {
                let sty = domain.sarzak().exhume_ty(id).unwrap();
                match sty {
                    Ty::SUuid(_) => true,
                    _ => false,
                }
            }
            _ => false,
        },
        CompilerSnafu {
            description: format!("type mismatch: found `{:?}`, expected `Type::Uuid`", ty)
        }
    );

    let _object = domain.sarzak().exhume_object(&method.object).unwrap();

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

    if param.is_some() {
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

            match &ty {
                GraceType::Reference(_) => {
                    format_string.extend(["{:?}:"]);
                    args.extend([param.r8_variable(woog)[0].name.as_ident(), ",".to_owned()]);
                }
                GraceType::WoogOption(_) => {
                    format_string.extend(["{:?}:"]);
                    args.extend([param.r8_variable(woog)[0].name.as_ident(), ",".to_owned()]);
                }
                GraceType::Ty(id) => {
                    let ty = domain.sarzak().exhume_ty(id).unwrap();
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
                            args.extend([
                                param.r8_variable(woog)[0].name.as_ident(),
                                ",".to_owned(),
                            ]);
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
    }

    // emit!(
    //     buffer,
    //     "let {} = Uuid::new_v5(&UUID_NS, format!(\"{}\", {}).as_bytes());",
    //     var.r8_variable(woog)[0].name,
    //     format_string,
    //     args
    // );

    emit!(
        buffer,
        "let {} = Uuid::new_v4();",
        var.r8_variable(woog)[0].name
    );

    Ok(())
}

pub(crate) fn render_new_instance(
    buffer: &mut Buffer,
    object: &Object,
    lval: Option<&LValue>,
    fields: &Vec<LValue>,
    rvals: &Vec<RValue>,
    config: &GraceConfig,
    imports: &Option<&HashMap<String, Domain>>,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    if let Some(lval) = lval {
        assert!(lval.ty == GType::Reference(object.id));
        write!(buffer, "let {} = ", lval.name).context(FormatSnafu)?;
    }

    if config.is_uber_store() {
        use UberStoreOptions::*;
        let store_ctor = match config.get_uber_store().unwrap() {
            Disabled => unreachable!(),
            Single => format!(
                "Rc::new(RefCell::new({} {{",
                object.as_type(&Ownership::new_borrowed(), woog, domain)
            ),
            StdRwLock | ParkingLotRwLock | AsyncRwLock | NDRwLock => format!(
                "Arc::new(RwLock::new({} {{",
                object.as_type(&Ownership::new_borrowed(), woog, domain)
            ),
            StdMutex | ParkingLotMutex => format!(
                "Arc::new(Mutex::new({} {{",
                object.as_type(&Ownership::new_borrowed(), woog, domain)
            ),
        };
        emit!(buffer, "{store_ctor}");
    } else {
        emit!(
            buffer,
            "{} {{",
            object.as_type(&Ownership::new_borrowed(), woog, domain)
        );
    }

    let is_uber = config.is_uber_store();

    let tuples = zip(fields, rvals);

    // Gee. I have a list of fields, and a list of parameters. How do I match
    // them up? I could infer by type, and the UUID will be tricky, because
    // how do I know that I cet get a UUID from an Object by calling id()?
    // I think that maybe the best we can do is typecheck the incoming values
    // against expected. Do that id() thing here, because we know. I think that
    // maybe I'm forgetting that I'm the one calling this. Maybe I'm being too
    // weird, and I just need a template engine. But then again, I'll be generating
    // unit tests, and the more I have, the better I think I'll be.
    for (field, rval) in tuples {
        // 🚧: This type conversion should likely be a function.
        match &field.ty {
            GType::Object(obj) => {
                let obj = domain.sarzak().exhume_object(&obj).unwrap();
                // If this is a subtype, grab the supertype object and if it's a hybrid, we need to
                // handle the inner enum specially.
                //
                // 🚧: There are now multiple subtypes per object, and we don't know which one
                // to grab. So if there are multiple hybrid supertypes, then we don't output
                // the correct `{}Enum`. See grace#58.
                //
                // Grace #58 is closed, and yet I don't know how this is selecting the correct
                // subtype. So, I'm just going to leave the commentary until I figure it out.
                if let Some(sub) = obj.r15_subtype(domain.sarzak()).pop() {
                    let super_obj = sub.r27_isa(domain.sarzak())[0].r13_supertype(domain.sarzak())
                        [0]
                    .r14_object(domain.sarzak())[0];

                    // This is just an ugly-ass mess. I'm not even sure why this works.
                    let (is_hybrid, foo_super_obj) = if let Some(GType::Object(id)) = field.hack {
                        let obj = domain.sarzak().exhume_object(&id).unwrap();
                        let is_hybrid = local_object_is_hybrid(obj, config, domain);
                        (is_hybrid, Some(obj))
                    } else {
                        (local_object_is_hybrid(super_obj, config, domain), None)
                    };

                    if is_hybrid {
                        match rval.ty {
                            GType::Uuid => {
                                emit!(
                                    buffer,
                                    "{}: {}Enum::{}({}),",
                                    field.name,
                                    super_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                                    obj.as_type(&Ownership::new_borrowed(), woog, domain),
                                    rval.name
                                )
                            }
                            GType::Reference(r_obj) => {
                                let r_obj = domain.sarzak().exhume_object(&r_obj).unwrap();
                                let id = if object_is_enum(r_obj, config, imports, domain)? {
                                    "id()"
                                } else {
                                    "id"
                                };

                                if is_uber {
                                    use UberStoreOptions::*;
                                    let (read, _write) = get_uber_read_write(config);
                                    emit!(
                                        buffer,
                                        "{}: {}Enum::{}({}{read}.{id}),",
                                        field.name,
                                        foo_super_obj.unwrap().as_type(
                                            &Ownership::new_borrowed(),
                                            woog,
                                            domain
                                        ),
                                        r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                                        rval.name
                                    )
                                } else {
                                    emit!(
                                        buffer,
                                        "{}: {}Enum::{}({}.{id}),",
                                        field.name,
                                        foo_super_obj.unwrap().as_type(
                                            &Ownership::new_borrowed(),
                                            woog,
                                            domain
                                        ),
                                        r_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                                        rval.name
                                    )
                                }
                            }
                            _ => {
                                if is_uber {
                                    use UberStoreOptions::*;
                                    let (read, _write) = get_uber_read_write(config);
                                    emit!(
                                        buffer,
                                        "{}: {}Enum::{}({}{read}.id),",
                                        field.name,
                                        super_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                                        obj.as_type(&Ownership::new_borrowed(), woog, domain),
                                        rval.name
                                    )
                                } else {
                                    emit!(
                                        buffer,
                                        "{}: {}Enum::{}({}.id),",
                                        field.name,
                                        super_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                                        obj.as_type(&Ownership::new_borrowed(), woog, domain),
                                        rval.name
                                    )
                                }
                            }
                        }
                    } else {
                        emit!(buffer, "{}: {},", field.name, rval.name)
                    }
                } else {
                    emit!(buffer, "{}: {},", field.name, rval.name)
                }
            }
            // The LHS is a Uuid, so we need to surface the correct means of
            // getting to the Uuid, given the RHS.
            GType::Uuid => match &rval.ty {
                GType::Uuid => emit!(buffer, "{}: {},", field.name, rval.name),
                GType::Reference(obj_id) => {
                    let obj = domain.sarzak().exhume_object(&obj_id).unwrap();

                    let id = if local_object_is_enum(obj, config, domain) {
                        "id()"
                    } else {
                        "id"
                    };

                    if is_uber {
                        let (read, _write) = get_uber_read_write(config);
                        emit!(buffer, "{}: {}{read}.{id},", field.name, rval.name)
                    } else {
                        emit!(buffer, "{}: {}.{id},", field.name, rval.name)
                    }
                }
                _ => ensure!(
                    field.ty == rval.ty,
                    CompilerSnafu {
                        description: format!(
                            "type mismatch, found `{}: {:?}`, expected `{}: {:?}`",
                            rval.name, rval.ty, field.name, field.ty
                        )
                    }
                ),
            },
            // The LHS is an Option<Uuid>, so we need to surface the correct means of
            // getting to the Uuid, given the RHS.
            GType::Option(_left) => match &rval.ty {
                GType::Option(right) => match **right {
                    GType::Reference(obj_id) => {
                        let obj = domain.sarzak().exhume_object(&obj_id).unwrap();
                        let obj_ident = obj.as_ident();

                        let id = if local_object_is_enum(obj, config, domain) {
                            "id()"
                        } else {
                            "id"
                        };

                        if is_uber {
                            let (read, _write) = get_uber_read_write(config);
                            if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap()
                            {
                                emit!(
                                    buffer,
                                    "{}: futures::future::OptionFuture::from({}.map(|{obj_ident}| async {{{obj_ident}{read}.{id}}})).await,",
                                    field.name,
                                    rval.name,
                                )
                            } else {
                                emit!(
                                    buffer,
                                    "{}: {}.map(|{obj_ident}| {obj_ident}{read}.{id}),",
                                    field.name,
                                    rval.name,
                                )
                            }
                        } else {
                            emit!(
                                buffer,
                                "{}: {}.map(|{obj_ident}| {obj_ident}.{id}),",
                                field.name,
                                rval.name,
                            )
                        }
                    }
                    _ => {
                        ensure!(
                            field.ty == rval.ty,
                            CompilerSnafu {
                                description: format!(
                                    "type mismatch, found `{}: {:?}`, expected `{}: {:?}`",
                                    rval.name, rval.ty, field.name, field.ty
                                )
                            }
                        );
                    }
                },
                _ => ensure!(
                    field.ty == rval.ty,
                    CompilerSnafu {
                        description: format!(
                            "type mismatch, found `{}: {:?}`, expected `{}: {:?}`",
                            rval.name, rval.ty, field.name, field.ty
                        )
                    }
                ),
            },
            _ => {
                ensure!(
                    field.ty == rval.ty,
                    CompilerSnafu {
                        description: format!(
                            "type mismatch, found `{}: {:?}`, expected `{}: {:?}`",
                            rval.name, rval.ty, field.name, field.ty
                        )
                    }
                );
                emit!(buffer, "{}: {},", field.name, rval.name)
            }
        }
    }

    emit!(buffer, "id");

    if is_uber {
        emit!(buffer, "}}))");
    } else {
        write!(buffer, "}}").context(FormatSnafu)?;
    }

    if lval.is_some() {
        emit!(buffer, ";");
    }

    Ok(())
}

pub(crate) fn render_new_instance_new(
    buffer: &mut Buffer,
    object: &Object,
    var: &Local,
    structure: &Structure,
    table: &SymbolTable,
    config: &GraceConfig,
    imports: &Option<&HashMap<String, Domain>>,
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

    // Check that the type of the variable is a reference to the object that we
    // are instantiating.
    // This doesn't belong here. It should be part of a let statement renderer.
    // 🚧 These errors are terrible. You get a uuid that may not even be possible
    // to look up. It should print the generated type. That would be fucking slick.
    ensure!(
        match ty {
            GraceType::Reference(id) => {
                let reference = woog.exhume_reference(&id).unwrap();
                ensure!(
                    reference.object == object.id,
                    CompilerSnafu {
                        description: format!(
                            "type mismatch: found `{:?}`, expected `{:?}`",
                            reference, object
                        )
                    }
                );
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

    let is_uber = config.is_uber_store();

    let mut first = structure
        .r27_structure_field(woog)
        .iter()
        .find(|&&field| field.r30c_structure_field(woog).len() == 0)
        .unwrap()
        .clone();

    let mut fields = vec![first];
    loop {
        if let Some(next) = first.r30_structure_field(woog).pop() {
            fields.push(next);
            first = next;
        } else {
            break;
        }
    }

    write!(buffer, "let {} = ", var.r8_variable(woog)[0].name).context(FormatSnafu)?;

    if is_uber {
        use UberStoreOptions::*;
        let store_ctor = match config.get_uber_store().unwrap() {
            Disabled => unreachable!(),
            Single => format!(
                "Rc::new(RefCell::new({} {{",
                object.as_type(&Ownership::new_borrowed(), woog, domain)
            ),
            StdRwLock | ParkingLotRwLock | AsyncRwLock | NDRwLock => format!(
                "Arc::new(RwLock::new({} {{",
                object.as_type(&Ownership::new_borrowed(), woog, domain)
            ),
            StdMutex | ParkingLotMutex => format!(
                "Arc::new(Mutex::new({} {{",
                object.as_type(&Ownership::new_borrowed(), woog, domain)
            ),
        };
        emit!(buffer, "{store_ctor}");
    } else {
        emit!(
            buffer,
            "{} {{",
            object.as_type(&Ownership::new_borrowed(), woog, domain)
        );
    }

    // Now we need to extract the values for the fields from the symbol table.
    // Except that it's not so simple since it's not a map. So really, it needs
    // to become a map asap. Maybe after I get EE's working? I haven't thought
    // about it too much. For now I guess I'll do a linear search.
    let rvals = fields
        .iter()
        .map(|field| {
            table
                .r20_variable(woog)
                .iter()
                .find(|&var| var.name == field.r27_field(woog)[0].name)
                .unwrap()
                .clone()
        })
        .collect::<Vec<_>>();

    let tuples = zip(fields, rvals);

    for (field, rval) in tuples {
        let f = field.r27_field(woog)[0];
        let ty = f.r29_grace_type(woog)[0];
        let rval_string = typecheck_and_coerce(ty, rval, config, imports, woog, domain)?;
        // Stupid clippy...
        if f.as_ident() == rval_string {
            emit!(buffer, "{rval_string},");
        } else {
            emit!(buffer, "{}: {rval_string},", f.as_ident());
        }
    }

    if is_uber {
        emit!(buffer, "}}));");
    } else {
        emit!(buffer, "}};");
    }

    Ok(())
}

/// This function takes a type, presumably from the left-hand side of an assignment,
/// and a variable, presumably from the right-hand side of an assignment, and checks
/// that the types are compatible. The result, assuming compatibility, is a string
/// representation of what the right-hand side of the assignment should be in able
/// to match the types.
fn typecheck_and_coerce(
    lhs_ty: &GraceType,
    rhs: &Variable,
    config: &GraceConfig,
    imports: &Option<&HashMap<String, Domain>>,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<String> {
    let rhs_ty = rhs.r7_value(woog)[0].r3_grace_type(woog)[0];
    let is_uber = config.is_uber_store();
    let rhs_ident = rhs.as_ident();

    Ok(match &lhs_ty {
        GraceType::WoogOption(_) => {
            // ✨ Until this comment changes, i.e., until this is used by more than
            // rendering a new Self item, the type of the lhs option is uuid.
            match &rhs_ty {
                GraceType::WoogOption(id) => {
                    let opt = woog.exhume_woog_option(&id).unwrap();
                    let opt_ty = opt.r20_grace_type(woog)[0];
                    match &opt_ty {
                        GraceType::Reference(id) => {
                            let reference = woog.exhume_reference(&id).unwrap();
                            let object = reference.r13_object(domain.sarzak())[0];
                            let obj_ident = object.as_ident();

                            let imported = config.is_imported(&object.id);

                            let id = if object_is_enum(object, config, imports, domain)? {
                                "id()"
                            } else {
                                "id"
                            };

                            if is_uber {
                                let (read, _write) = get_uber_read_write(config);
                                if let UberStoreOptions::AsyncRwLock =
                                    config.get_uber_store().unwrap()
                                {
                                    format!(
                                    "futures::future::OptionFuture::from({rhs_ident}.map(|{obj_ident}| async {{{obj_ident}{read}.{id}}})).await"
                                )
                                } else {
                                    format!("{rhs_ident}.map(|{obj_ident}| {obj_ident}{read}.{id})")
                                }
                            } else {
                                format!("{rhs_ident}.map(|{obj_ident}| {obj_ident}.{id})")
                            }
                        }
                        _ => {
                            ensure!(
                                &lhs_ty == &rhs_ty,
                                CompilerSnafu {
                                    description: format!(
                                        "type mismatch: found `{:?}`, expected `{:?}`",
                                        rhs_ty, lhs_ty
                                    )
                                }
                            );
                            rhs.as_ident()
                        }
                    }
                }
                _ => {
                    ensure!(
                        &lhs_ty == &rhs_ty,
                        CompilerSnafu {
                            description: format!(
                                "type mismatch: found `{:?}`, expected `{:?}`",
                                rhs_ty, lhs_ty
                            )
                        }
                    );
                    rhs.as_ident()
                }
            }
        }
        // GraceType::TimeStamp(id) => {}
        GraceType::Ty(id) => {
            let ty = domain.sarzak().exhume_ty(&id).unwrap();
            match ty {
                Ty::SUuid(_) => {
                    // If the lhs is a uuid, and the rhs is a reference, we need to
                    // pull it's id.
                    match &rhs_ty {
                        GraceType::Reference(id) => {
                            let obj = woog
                                .exhume_reference(&id)
                                .unwrap()
                                .r13_object(domain.sarzak())[0];

                            let is_imported = config.is_imported(&obj.id);

                            let id = if object_is_enum(obj, config, imports, domain)? {
                                "id()"
                            } else {
                                "id"
                            };

                            if is_uber {
                                let (read, _write) = get_uber_read_write(config);
                                format!("{}{read}.{id}", rhs.as_ident())
                            } else {
                                format!("{}.{id}", rhs.as_ident())
                            }
                        }
                        _ => {
                            ensure!(
                                &lhs_ty == &rhs_ty,
                                CompilerSnafu {
                                    description: format!(
                                        "type mismatch: found `{:?}`, expected `{:?}`",
                                        rhs_ty, lhs_ty
                                    )
                                }
                            );
                            rhs.as_ident()
                        }
                    }
                }
                _ => {
                    ensure!(
                        &lhs_ty == &rhs_ty,
                        CompilerSnafu {
                            description: format!(
                                "type mismatch: found `{:?}`, expected `{:?}`",
                                rhs_ty, lhs_ty
                            )
                        }
                    );
                    rhs.as_ident()
                }
            }
        }
        _ => {
            ensure!(
                &lhs_ty == &rhs_ty,
                CompilerSnafu {
                    description: format!(
                        "type mismatch: found `{:?}`, expected `{:?}`",
                        rhs_ty, lhs_ty
                    )
                }
            );

            if is_uber {
                let (read, _write) = get_uber_read_write(config);
                format!("{}{read}.to_owned()", rhs.as_ident())
            } else {
                rhs.as_ident()
            }
        }
    })
}

pub(crate) fn render_methods(
    buffer: &mut Buffer,
    obj: &Object,
    config: &GraceConfig,
    imports: &Option<&HashMap<String, Domain>>,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    let mut methods: Vec<&WoogObjectMethod> = woog
        .iter_object_method()
        .filter(|m| m.object == obj.id)
        .collect();

    methods.sort_by(|a, b| {
        let a = a.r25_function(woog)[0];
        let b = b.r25_function(woog)[0];
        a.name.cmp(&b.name)
    });

    for method in methods {
        let func = method.r25_function(woog).pop().unwrap();

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-struct-impl-{}", obj.as_ident(), func.name),
            |buffer| {
                // Output a docstring
                emit!(buffer, "/// {}", func.description);

                // This renders the method signature.
                // It's probably ok as it is.
                render_method_definition_new(buffer, &method, config, woog, domain)?;

                // Find the properly scoped variable named `id`.
                let table = method.r23_block(woog)[0].r24_symbol_table(woog)[0];
                let var = &table
                    .r20_variable(woog)
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

                // This renders a let statement, assigning a new uuid to the id variable.
                // This is where the work lies. I think that what I really want to do is
                // create (let) statements in the block whilst populating woog. Then
                // someplace else, maybe here, we iterate over the statements and generate
                // code. Maybe an as_statement trait, or something?
                render_make_uuid_new(buffer, &id, &method, woog, domain)?;

                // Look up the properly scoped variable named `new`.
                let var = &table
                    .r20_variable(woog)
                    .iter()
                    .find(|&&v| v.name == "new")
                    .unwrap()
                    .subtype;
                let new = match var {
                    VariableEnum::Local(id) => woog.exhume_local(&id).unwrap(),
                    _ => panic!("This should never happen"),
                };

                // Now this is interesting. This is good. It's getting close to what I
                // was talking about above. In the woog population code, the function
                // for populating a new method I created a statement: a struct item.
                // It's the struct for Self. I pull that out here, and then use when
                // I call the renderer.
                let stmt = match &method
                    .r23_block(woog)
                    .pop()
                    .unwrap()
                    .r12_statement(woog)
                    .pop()
                    .unwrap()
                    .subtype
                {
                    StatementEnum::Item(id) => {
                        let item = woog.exhume_item(id).unwrap();
                        match item {
                            Item::Structure(id) => woog.exhume_structure(id).unwrap(),
                            _ => unimplemented!(),
                        }
                    }
                    _ => unimplemented!(),
                };

                // I wrote this this morning, and already I'can't say how it works
                // exactly. It takes a structure, and not a statement, so it's
                // pretty low level. It's also assigning the let. Refactor time.
                render_new_instance_new(
                    buffer,
                    obj,
                    &new,
                    &stmt,
                    &method
                        .r23_block(woog)
                        .pop()
                        .unwrap()
                        .r24_symbol_table(woog)
                        .pop()
                        .unwrap(),
                    config,
                    imports,
                    woog,
                    domain,
                )?;

                if func.name == "new" {
                    if config.is_uber_store() {
                        if let UberStoreOptions::AsyncRwLock = config.get_uber_store().unwrap() {
                            emit!(buffer, "store.inter_{}(new.clone()).await;", obj.as_ident());
                        } else {
                            emit!(buffer, "store.inter_{}(new.clone());", obj.as_ident());
                        }
                    } else {
                        emit!(buffer, "store.inter_{}(new.clone());", obj.as_ident());
                    }
                }
                emit!(buffer, "new");
                emit!(buffer, "}}");

                Ok(())
            },
        )?;
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
                    // This is not the domain you were passed.
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
    log::debug!("{} is_hybrid attrs: {:?}", object.name, attrs);

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
    log::debug!("{} is_super: {:?}", object.name, is_super);

    is_super.len() > 0
}

// test_local_and_imports!(object_is_subtype, local_object_is_subtype);
pub(crate) fn local_object_is_subtype(
    object: &Object,
    _config: &GraceConfig,
    domain: &Domain,
) -> bool {
    let is_sub = object.r15_subtype(domain.sarzak());
    log::debug!("{} is_sub: {:?}", object.name, is_sub);

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
    log::debug!("{} is_singleton attrs: {:?}", object.name, attrs);

    attrs.len() < 2
        && !local_object_is_referrer(object, config, domain)
        && !local_object_is_supertype(object, config, domain)
}

// test_local_and_imports!(object_is_referrer, inner_object_is_referrer);
fn local_object_is_referrer(object: &Object, _config: &GraceConfig, domain: &Domain) -> bool {
    let referrers = object.r17_referrer(domain.sarzak());
    let assoc_referrers = object.r26_associative_referrer(domain.sarzak());
    log::debug!("{} is_referrer referrers: {:?}", object.name, referrers);
    log::debug!(
        "{} is_referrer assoc_referrers: {:?}",
        object.name,
        assoc_referrers
    );

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
pub(crate) fn emit_object_comments(
    input: &str,
    prefix: &str,
    suffix: &str,
    context: &mut Buffer,
) -> Result<()> {
    const MAX_LEN: usize = 90;

    if input.len() > 0 {
        for line in input.split('\n') {
            write!(context, "{}", prefix).context(FormatSnafu)?;
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

                        // No clue what I was going on about up there.
                        write!(context, "{}", suffix).context(FormatSnafu)?;

                        // Add a newline
                        emit!(context, "");
                        length = 0;

                        write!(context, "{}{}", prefix, word).context(FormatSnafu)?;
                        length += word.len() + 3;
                    }
                }
            }

            write!(context, "{}", suffix).context(FormatSnafu)?;

            // Add a trailing newline
            emit!(context, "");
        }

        emit!(context, "{}{}", prefix, suffix);
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
        name.as_type(&Ownership::new_borrowed(), woog, domain)
    );

    let mut iter = domain.sarzak().iter_ty();
    loop {
        let ty = iter.next();
        match ty {
            Some(ty) => match ty {
                Ty::External(e) => {
                    let ext = domain.sarzak().exhume_external(&e).unwrap();
                    if ext.name == name {
                        break ext;
                    }
                }
                _ => continue,
            },
            None => panic!("Could not find store type for {}", name),
        }
    }
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

pub(crate) trait AttributeBuilder<A> {
    fn new(name: String, ty: Arc<Lock<ValueType>>) -> A;
}

/// Walk the object hierarchy to collect attributes for an object
///
/// The attributes are generated in a stable order.
///
/// This is only applicable to generating dwarf code, and I think it should be
/// moved.
pub(crate) fn collect_attributes<A>(obj: &Object, domain: &Domain) -> Vec<A>
where
    A: AttributeBuilder<A>,
{
    let lu_dog = &LU_DOG;
    let mut result: Vec<A> = Vec::new();

    // Collect the local attributes
    let mut attrs = obj.r1_attribute(domain.sarzak());
    attrs.sort_by(|a, b| a.name.cmp(&b.name));
    for attr in attrs {
        let ty = attr.r2_ty(domain.sarzak())[0];
        let mut lu_dog = lu_dog.write().unwrap();
        // let ty = ValueType::new_ty(ty, &mut lu_dog);
        let ty = ValueType::new_ty(&Arc::new(Lock::new(ty.to_owned())), &mut lu_dog);

        let attr = A::new(attr.as_ident(), ty.clone());
        result.push(attr);
    }

    // These are more attributes on our object, and they should be sorted.
    let referrers = get_binary_referrers_sorted!(obj, domain.sarzak());
    // And the referential attributes
    for referrer in &referrers {
        let binary = referrer.r6_binary(domain.sarzak())[0];
        let referent = binary.r5_referent(domain.sarzak())[0];
        let r_obj = referent.r16_object(domain.sarzak())[0];
        let cond = referrer.r11_conditionality(domain.sarzak())[0];

        let attr_name = referrer.referential_attribute.as_ident();

        let ty = domain.sarzak().exhume_ty(&r_obj.id).unwrap();
        let mut lu_dog = lu_dog.write().unwrap();
        // let ty = ValueType::new_ty(ty, &mut lu_dog);
        let ty = ValueType::new_ty(&Arc::new(Lock::new(ty.to_owned())), &mut lu_dog);
        let ty = Reference::new(Uuid::new_v4(), false, &ty, &mut lu_dog);
        let ty = ValueType::new_reference(&ty, &mut lu_dog);

        // This determines how a reference is stored in the struct. In this
        // case a UUID.
        match cond {
            // If it's conditional build a parameter that's an optional reference
            // to the referent.
            Conditionality::Conditional(_) => {
                let option = WoogOption::new_z_none(&ty, &mut lu_dog);
                let ty = ValueType::new_woog_option(&option, &mut lu_dog);

                let attr = A::new(attr_name, ty.clone());
                result.push(attr);
            }
            // An unconditional reference translates into a reference to the referent.
            Conditionality::Unconditional(_) => {
                let attr = A::new(attr_name, ty.clone());
                result.push(attr);
            }
        }
    }

    // And the associative attributes
    for assoc_referrer in obj.r26_associative_referrer(domain.sarzak()) {
        let referents = get_assoc_referent_from_referrer_sorted!(assoc_referrer, domain.sarzak());

        for referent in referents {
            let an_ass = referent.r22_an_associative_referent(domain.sarzak())[0];
            let obj = referent.r25_object(domain.sarzak())[0];

            let ty = domain.sarzak().exhume_ty(&obj.id).unwrap();
            let mut lu_dog = lu_dog.write().unwrap();
            // let ty = ValueType::new_ty(ty, &mut lu_dog);
            let ty = ValueType::new_ty(&Arc::new(Lock::new(ty.to_owned())), &mut lu_dog);
            let ty = Reference::new(Uuid::new_v4(), false, &ty, &mut lu_dog);
            let ty = ValueType::new_reference(&ty, &mut lu_dog);

            let attr_name = an_ass.referential_attribute.as_ident();

            let attr = A::new(attr_name, ty.clone());
            result.push(attr);
        }
    }

    result
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

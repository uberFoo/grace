//! Things necessary for code generation
//!
pub(crate) mod buffer;
pub(crate) mod diff_engine;
pub(crate) mod generator;
pub(crate) mod render;
mod rustfmt;

use std::{collections::HashMap, fmt::Write, iter::zip, time::SystemTime};

use sarzak::{
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::types::{External, Object, Ty},
    v2::domain::Domain,
    woog::{
        store::ObjectStore as WoogStore,
        types::{ObjectMethod as WoogObjectMethod, Ownership, OWNED},
    },
};
use snafu::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        render::{RenderIdent, RenderType},
    },
    options::GraceConfig,
    todo::{GType, LValue, ObjectMethod, RValue},
};

macro_rules! get_subtypes_sorted {
    ($obj:expr, $store:expr) => {{
        // I'm convinced that R14 and R15 are broken.
        // They are probably aliased. I wonder why I was convinced of that...
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

macro_rules! get_objs_for_assoc_referrers_sorted {
    ($obj:expr, $store:expr) => {{
        let mut objs = Vec::new();
        let referrers = $obj.r26_associative_referrer($store);
        for referrer in &referrers {
            let assoc = referrer.r21_associative($store)[0];
            let one = assoc.r23_associative_referent($store)[0];
            let other = assoc.r22_associative_referent($store)[0];
            objs.push(one.r25_object($store)[0]);
            objs.push(other.r25_object($store)[0]);
        }

        objs.sort_by(|a, b| a.name.cmp(&b.name));

        objs
    }};
}
pub(crate) use get_objs_for_assoc_referrers_sorted;

macro_rules! get_objs_for_assoc_referents_sorted {
    ($obj:expr, $store:expr) => {{
        let mut objs = Vec::new();
        let referents = $obj.r25_associative_referent($store);
        for referent in &referents {
            let r23 = referent.r23c_associative($store);

            if r23.is_empty() {
                let assoc = referent.r22c_associative($store)[0];
                let referrer = assoc.r21_associative_referrer($store)[0];
                objs.push(referrer.r26_object($store)[0]);
            } else {
                let assoc = r23[0];
                let referrer = assoc.r21_associative_referrer($store)[0];
                objs.push(referrer.r26_object($store)[0]);
            }

            // if let Some(assoc) = sarzak_get_one_r_assoc_across_r23!(referent, $store) {
            //     let referrer = sarzak_get_one_ass_from_across_r21!(assoc, $store);
            //     objs.push(sarzak_get_one_obj_across_r26!(referrer, $store));
            // } else {
            //     let assoc = sarzak_get_one_r_assoc_across_r22!(referent, $store);
            //     let referrer = sarzak_get_one_ass_from_across_r21!(assoc, $store);
            //     objs.push(sarzak_get_one_obj_across_r26!(referrer, $store));
            // }
        }

        objs.sort_by(|a, b| a.name.cmp(&b.name));

        objs
    }};
}
pub(crate) use get_objs_for_assoc_referents_sorted;

macro_rules! get_objs_for_referrers_sorted {
    ($obj:expr, $store:expr) => {{
        let mut objs = Vec::new();
        let referrers = get_referrers_sorted!($obj, $store);
        for referrer in &referrers {
            let binary = referrer.r6_binary($store)[0];
            let referent = binary.r5_referent($store)[0];
            let obj = referent.r16_object($store)[0];
            objs.push(obj);
        }

        objs
    }};
}
pub(crate) use get_objs_for_referrers_sorted;

macro_rules! get_objs_for_referents_sorted {
    ($obj:expr, $store:expr) => {{
        let mut objs = Vec::new();
        let referents = get_referents_sorted!($obj, $store);
        for referent in &referents {
            let binary = referent.r5_binary($store)[0];
            let referrer = binary.r6_referrer($store)[0];
            let obj = referrer.r17_object($store)[0];
            objs.push(obj);
        }

        objs
    }};
}
pub(crate) use get_objs_for_referents_sorted;

macro_rules! get_referrers_sorted {
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
pub(crate) use get_referrers_sorted;

macro_rules! get_referents_sorted {
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
pub(crate) use get_referents_sorted;

pub(crate) fn render_method_definition(
    buffer: &mut Buffer,
    method: &ObjectMethod,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    // Write the beginning of the definition
    write!(buffer, "pub fn {}(", method.as_ident()).context(FormatSnafu)?;

    // Write the parameter list.
    // TODO: This is so clumsy! I should clean it up.
    if let Some(mut param) = method.param {
        let mutability = woog.exhume_ownership(&param.mutability).unwrap();
        write!(
            buffer,
            "{}: {},",
            param.as_ident(),
            param.ty.as_type(&mutability, woog, domain),
        )
        .context(FormatSnafu)?;

        while let Some(next_param) = param.next {
            let mutability = woog.exhume_ownership(&next_param.mutability).unwrap();
            write!(
                buffer,
                "{}: {},",
                // Why do I need to drill down to name?
                next_param.name.as_ident(),
                next_param.ty.as_type(&mutability, woog, domain),
            )
            .context(FormatSnafu)?;

            param = &next_param;
        }
    }

    // Finish the first line of the definition
    writeln!(
        buffer,
        ") -> {} {{",
        method.ty.as_type(&Ownership::new_borrowed(), woog, domain)
    )
    .context(FormatSnafu)?;

    Ok(())
}

pub(crate) fn render_method_definition_new(
    buffer: &mut Buffer,
    method: &WoogObjectMethod,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    let object = domain.sarzak().exhume_object(&method.object).unwrap();

    // Write the beginning of the definition
    write!(buffer, "pub fn {}(", method.as_ident()).context(FormatSnafu)?;

    // By my calculations this should grab the first parameter in the list.
    // Not a very slick way of doing it.
    // 🚧 I suppose I could add a pointer to the first parameter as a relationship
    // on the method.
    let param = woog
        .iter_parameter()
        .find(|p| p.method == method.id && p.r1c_parameter(woog).len() == 0);

    ensure!(
        param.is_some(),
        CompilerSnafu {
            description: format!(
                "No parameter found for {}::{}",
                object.as_type(&Ownership::Owned(OWNED), woog, domain),
                method.as_ident()
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
            param.as_ident(),
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
        object.as_type(&Ownership::new_borrowed(), woog, domain)
    )
    .context(FormatSnafu)?;

    Ok(())
}

/// Generate code to create a new UUID
///
/// TODO: We should be taking a list of rvals to use, and not [`Parameter`]s.
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
    let mut params = String::new();
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

        params.extend([val.name.to_owned(), ",".to_owned()]);
    }
    // Remove the trailing ":"
    format_string.pop();
    // And the trailining ","
    params.pop();

    emit!(
        buffer,
        "let {} = Uuid::new_v5(&UUID_NS, format!(\"{}\", {}).as_bytes());",
        lval.name,
        format_string,
        params
    );

    Ok(())
}

pub(crate) fn render_new_instance(
    buffer: &mut Buffer,
    object: &Object,
    lval: Option<&LValue>,
    fields: &Vec<LValue>,
    rvals: &Vec<RValue>,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    if let Some(lval) = lval {
        assert!(lval.ty == GType::Reference(object.id));
        write!(buffer, "let {} = ", lval.name).context(FormatSnafu)?;
    }
    emit!(
        buffer,
        "{} {{",
        object.as_type(&Ownership::new_borrowed(), woog, domain)
    );

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
                if let Some(sub) = obj.r15c_subtype(domain.sarzak()).pop() {
                    let s_obj = sub.r27_isa(domain.sarzak())[0].r13_supertype(domain.sarzak())[0]
                        .r14_object(domain.sarzak())[0];
                    if inner_object_is_hybrid(s_obj, domain) {
                        match rval.ty {
                            GType::Uuid => {
                                emit!(
                                    buffer,
                                    "{}: {}Enum::{}({}),",
                                    field.name,
                                    s_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                                    obj.as_type(&Ownership::new_borrowed(), woog, domain),
                                    rval.name
                                )
                            }
                            GType::Reference(r_obj) => {
                                let r_obj = domain.sarzak().exhume_object(&r_obj).unwrap();
                                if inner_object_is_enum(r_obj, domain) {
                                    emit!(
                                        buffer,
                                        "{}: {}Enum::{}({}.id()),",
                                        field.name,
                                        s_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                                        obj.as_type(&Ownership::new_borrowed(), woog, domain),
                                        rval.name
                                    )
                                } else {
                                    emit!(
                                        buffer,
                                        "{}: {}Enum::{}({}.id),",
                                        field.name,
                                        s_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                                        obj.as_type(&Ownership::new_borrowed(), woog, domain),
                                        rval.name
                                    )
                                }
                            }
                            _ => {
                                emit!(
                                    buffer,
                                    "{}: {}Enum::{}({}.id),",
                                    field.name,
                                    s_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                                    obj.as_type(&Ownership::new_borrowed(), woog, domain),
                                    rval.name
                                )
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

                    if inner_object_is_enum(obj, domain) {
                        emit!(buffer, "{}: {}.id(),", field.name, rval.name)
                    } else {
                        emit!(buffer, "{}: {}.id,", field.name, rval.name)
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

                        if inner_object_is_enum(obj, domain) {
                            emit!(
                                buffer,
                                "{}: {}.map(|{}| {}.id()),",
                                field.name,
                                rval.name,
                                obj.as_ident(),
                                obj.as_ident()
                            )
                        } else {
                            emit!(
                                buffer,
                                "{}: {}.map(|{}| {}.id),",
                                field.name,
                                rval.name,
                                obj.as_ident(),
                                obj.as_ident()
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

    if lval.is_some() {
        emit!(buffer, "}};");
    } else {
        emit!(buffer, "}}")
    };

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
                    domain.sarzak().exhume_object(&object.id).is_some(),
                    CompilerSnafu {
                        description: format!(
                            "object `{}` is not found in imported domain {}",
                            object.name, imported.domain
                        )
                    }
                );

                Ok($func(object, domain))
            } else {
                Ok($func(object, domain))
            }
        }
    };
}

// test_local_and_imports!(object_is_hybrid, inner_object_is_hybrid);
pub(crate) fn inner_object_is_hybrid(object: &Object, domain: &Domain) -> bool {
    inner_object_is_supertype(object, domain) && !inner_object_is_singleton(object, domain)
}

test_local_and_imports!(object_is_enum, inner_object_is_enum);
pub(crate) fn inner_object_is_enum(object: &Object, domain: &Domain) -> bool {
    inner_object_is_supertype(object, domain) && inner_object_is_singleton(object, domain)
}

test_local_and_imports!(object_is_supertype, inner_object_is_supertype);
pub(crate) fn inner_object_is_supertype(object: &Object, domain: &Domain) -> bool {
    let is_super = object.r14_supertype(domain.sarzak());
    log::debug!("is_super: {:?}", is_super);

    is_super.len() > 0
}

test_local_and_imports!(object_is_singleton, inner_object_is_singleton);
pub(crate) fn inner_object_is_singleton(object: &Object, domain: &Domain) -> bool {
    let attrs = object.r1_attribute(domain.sarzak());
    log::debug!("attrs: {:?}", attrs);

    attrs.len() < 2 && !inner_object_is_referrer(object, domain)
}

// test_local_and_imports!(object_is_referrer, inner_object_is_referrer);
fn inner_object_is_referrer(object: &Object, domain: &Domain) -> bool {
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

pub(crate) fn is_object_stale(object: &Object, woog: &WoogStore, domain: &Domain) -> bool {
    let time = if let Some(gu) = woog
        .iter_generation_unit()
        .find(|gu| gu.object == object.id)
    {
        woog.generation_unit_timestamp(gu)
    } else {
        SystemTime::now()
    };

    if domain.sarzak().object_timestamp(object) > time {
        return true;
    }

    for attr in object.r1_attribute(domain.sarzak()) {
        if domain.sarzak().attribute_timestamp(&attr) > time {
            return true;
        }
    }

    for supertype in object.r14_supertype(domain.sarzak()) {
        if domain.sarzak().supertype_timestamp(supertype) > time {
            return true;
        }
    }

    for subtype in object.r15c_subtype(domain.sarzak()) {
        if domain.sarzak().subtype_timestamp(subtype) > time {
            return true;
        }
    }

    for referent in object.r16_referent(domain.sarzak()) {
        if domain.sarzak().referent_timestamp(referent) > time {
            return true;
        }
    }

    for referrer in object.r17_referrer(domain.sarzak()) {
        if domain.sarzak().referrer_timestamp(referrer) > time {
            return true;
        }
    }

    for assoc_referent in object.r25_associative_referent(domain.sarzak()) {
        if domain
            .sarzak()
            .associative_referent_timestamp(assoc_referent)
            > time
        {
            return true;
        }
    }

    for assoc_referrer in object.r26_associative_referrer(domain.sarzak()) {
        if domain
            .sarzak()
            .associative_referrer_timestamp(assoc_referrer)
            > time
        {
            return true;
        }
    }

    for state in object.r18_state(domain.sarzak()) {
        if domain.sarzak().state_timestamp(state) > time {
            return true;
        }
    }

    for event in object.r19_event(domain.sarzak()) {
        if domain.sarzak().event_timestamp(event) > time {
            return true;
        }
    }

    return false;
}

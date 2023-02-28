//! Things necessary for code generation
//!

pub(crate) mod buffer;
pub(crate) mod diff_engine;
pub(crate) mod generator;
pub(crate) mod render;
mod rustfmt;

use std::{collections::HashMap, fmt::Write, iter::zip};

use sarzak::{
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::types::{External, Object, Ty},
    v2::domain::Domain,
    woog::{
        store::ObjectStore as WoogStore,
        types::{ObjectMethod as WoogObjectMethod, Ownership, BORROWED},
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
            param.ty.as_type(&mutability, domain),
        )
        .context(FormatSnafu)?;

        while let Some(next_param) = param.next {
            let mutability = woog.exhume_ownership(&next_param.mutability).unwrap();
            write!(
                buffer,
                "{}: {},",
                // Why do I need to drill down to name?
                next_param.name.as_ident(),
                next_param.ty.as_type(&mutability, domain),
            )
            .context(FormatSnafu)?;

            param = &next_param;
        }
    }

    // Finish the first line of the definition
    writeln!(
        buffer,
        ") -> {} {{",
        method.ty.as_type(&Ownership::Borrowed(BORROWED), domain)
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
    // Write the beginning of the definition
    write!(buffer, "pub fn {}(", method.as_ident()).context(FormatSnafu)?;

    // By my calculations this should grab the first parameter in the list.
    // Not a very slick way of doing it.
    // 🚧 I suppose I could add a pointer to the first parameter as a relationship
    // on the method.
    let mut param = woog
        .iter_parameter()
        .find(|p| p.method == method.id && p.r1c_parameter(woog).len() == 0)
        .unwrap();

    loop {
        let value = woog.exhume_value(&param.id).unwrap();
        let access = value.r16_access(woog)[0];
        let mutability = r15_ownership(woog);
    }

    // Write the parameter list.
    // TODO: This is so clumsy! I should clean it up.
    if let Some(mut param) = method.param {
        let mutability = woog.exhume_ownership(&param.mutability).unwrap();
        write!(
            buffer,
            "{}: {},",
            param.as_ident(),
            param.ty.as_type(&mutability, domain),
        )
        .context(FormatSnafu)?;

        while let Some(next_param) = param.next {
            let mutability = woog.exhume_ownership(&next_param.mutability).unwrap();
            write!(
                buffer,
                "{}: {},",
                // Why do I need to drill down to name?
                next_param.name.as_ident(),
                next_param.ty.as_type(&mutability, domain),
            )
            .context(FormatSnafu)?;

            param = &next_param;
        }
    }

    // Finish the first line of the definition
    writeln!(
        buffer,
        ") -> {} {{",
        method.ty.as_type(&Ownership::Borrowed(BORROWED), domain)
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
    assert!(lval.ty == GType::Uuid);

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
    domain: &Domain,
    _imports: Option<&HashMap<String, Domain>>,
    _config: &GraceConfig,
) -> Result<()> {
    if let Some(lval) = lval {
        assert!(lval.ty == GType::Reference(object.id));
        write!(buffer, "let {} = ", lval.name).context(FormatSnafu)?;
    }
    emit!(
        buffer,
        "{} {{",
        object.as_type(&Ownership::Borrowed(BORROWED), domain)
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
                                    s_obj.as_type(&Ownership::Borrowed(BORROWED), domain),
                                    obj.as_type(&Ownership::Borrowed(BORROWED), domain),
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
                                        s_obj.as_type(&Ownership::Borrowed(BORROWED), domain),
                                        obj.as_type(&Ownership::Borrowed(BORROWED), domain),
                                        rval.name
                                    )
                                } else {
                                    emit!(
                                        buffer,
                                        "{}: {}Enum::{}({}.id),",
                                        field.name,
                                        s_obj.as_type(&Ownership::Borrowed(BORROWED), domain),
                                        obj.as_type(&Ownership::Borrowed(BORROWED), domain),
                                        rval.name
                                    )
                                }
                            }
                            _ => {
                                emit!(
                                    buffer,
                                    "{}: {}Enum::{}({}.id),",
                                    field.name,
                                    s_obj.as_type(&Ownership::Borrowed(BORROWED), domain),
                                    obj.as_type(&Ownership::Borrowed(BORROWED), domain),
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
                    // let is_supertype = if let Some(imports) = imports {
                    // run_func_on_imported_domain(obj, config, domain, imports, object_is_enum)
                    // } else {
                    // object_is_enum(obj, domain)
                    // };

                    // if is_supertype {
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
                        // let is_supertype = if let Some(imports) = imports {
                        //     run_func_on_imported_domain(
                        //         obj,
                        //         config,
                        //         domain,
                        //         imports,
                        //         object_is_enum,
                        //     )
                        // } else {
                        //     object_is_enum(obj, domain)
                        // };

                        // if is_supertype {
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

pub(crate) fn find_store<'a>(name: &str, domain: &'a Domain) -> &'a External {
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
        name.as_type(&Ownership::Borrowed(BORROWED), domain)
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

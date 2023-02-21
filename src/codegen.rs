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
    sarzak::{
        macros::{
            sarzak_get_many_as_across_r1, sarzak_maybe_get_many_ass_froms_across_r26,
            sarzak_maybe_get_many_r_froms_across_r17, sarzak_maybe_get_many_r_sups_across_r14,
        },
        store::ObjectStore as SarzakStore,
        types::{AssociativeReferrer, Attribute, Object, Referrer, Supertype, Type},
    },
    v1::domain::Domain,
    woog::{
        store::ObjectStore as WoogStore,
        types::{Mutability, BORROWED},
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
    todo::{External, GType, LValue, ObjectMethod, RValue},
};

macro_rules! get_subtypes_sorted {
    ($obj:expr, $store:expr) => {{
        // I'm convinced that R14 and R15 are broken.
        let sup = sarzak_maybe_get_many_r_sups_across_r14!($obj, $store);
        let isa = sarzak_get_one_r_isa_across_r13!(sup[0], $store);
        let mut subtypes = sarzak_get_many_r_subs_across_r27!(isa, $store);
        subtypes.sort_by(|a, b| {
            let a = sarzak_get_one_obj_across_r15!(a, $store);
            let b = sarzak_get_one_obj_across_r15!(b, $store);
            a.name.cmp(&b.name)
        });

        subtypes
    }};
}
pub(crate) use get_subtypes_sorted;

macro_rules! get_objs_for_assoc_referrers_sorted {
    ($obj:expr, $store:expr) => {{
        let mut objs = Vec::new();
        let referrers = sarzak_maybe_get_many_ass_froms_across_r26!($obj, $store);
        for referrer in &referrers {
            let assoc = sarzak_get_one_r_assoc_across_r21!(referrer, $store);
            let one = sarzak_get_one_ass_to_across_r23!(assoc, $store);
            let other = sarzak_get_one_ass_to_across_r22!(assoc, $store);
            objs.push(sarzak_get_one_obj_across_r25!(one, $store));
            objs.push(sarzak_get_one_obj_across_r25!(other, $store));
        }

        objs.sort_by(|a, b| a.name.cmp(&b.name));

        objs
    }};
}
pub(crate) use get_objs_for_assoc_referrers_sorted;

macro_rules! get_objs_for_assoc_referents_sorted {
    ($obj:expr, $store:expr) => {{
        let mut objs = Vec::new();
        let referents = sarzak_maybe_get_many_ass_tos_across_r25!($obj, $store);
        for referent in &referents {
            if let Some(assoc) = sarzak_get_one_r_assoc_across_r23!(referent, $store) {
                let referrer = sarzak_get_one_ass_from_across_r21!(assoc, $store);
                objs.push(sarzak_get_one_obj_across_r26!(referrer, $store));
            } else {
                let assoc = sarzak_get_one_r_assoc_across_r22!(referent, $store);
                let referrer = sarzak_get_one_ass_from_across_r21!(assoc, $store);
                objs.push(sarzak_get_one_obj_across_r26!(referrer, $store));
            }
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
            let binary = sarzak_get_one_r_bin_across_r6!(referrer, $store);
            let referent = sarzak_get_one_r_to_across_r5!(binary, $store);
            let obj = sarzak_get_one_obj_across_r16!(referent, $store);
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
            let binary = sarzak_get_one_r_bin_across_r5!(referent, $store);
            let referrer = sarzak_get_one_r_from_across_r6!(binary, $store);
            let obj = sarzak_get_one_obj_across_r17!(referrer, $store);
            objs.push(obj);
        }

        objs
    }};
}
pub(crate) use get_objs_for_referents_sorted;

macro_rules! get_referrers_sorted {
    ($obj:expr, $store:expr) => {{
        let mut referrers = sarzak_maybe_get_many_r_froms_across_r17!($obj, $store);
        referrers.sort_by(|a, b| {
            let binary = sarzak_get_one_r_bin_across_r6!(&a, $store);
            let referent = sarzak_get_one_r_to_across_r5!(binary, $store);
            let obj_a = sarzak_get_one_obj_across_r16!(referent, $store);

            let binary = sarzak_get_one_r_bin_across_r6!(&b, $store);
            let referent = sarzak_get_one_r_to_across_r5!(binary, $store);
            let obj_b = sarzak_get_one_obj_across_r16!(referent, $store);

            obj_a.name.cmp(&obj_b.name)
        });

        referrers
    }};
}
pub(crate) use get_referrers_sorted;

macro_rules! get_referents_sorted {
    ($obj:expr, $store:expr) => {{
        let mut referents = sarzak_maybe_get_many_r_tos_across_r16!($obj, $store);
        referents.sort_by(|a, b| {
            let binary = sarzak_get_one_r_bin_across_r5!(&a, $store);
            let referrer = sarzak_get_one_r_from_across_r6!(binary, $store);
            let obj_a = sarzak_get_one_obj_across_r17!(referrer, $store);

            let binary = sarzak_get_one_r_bin_across_r5!(&b, $store);
            let referrer = sarzak_get_one_r_from_across_r6!(binary, $store);
            let obj_b = sarzak_get_one_obj_across_r17!(referrer, $store);

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
    sarzak: &SarzakStore,
) -> Result<()> {
    // Write the beginning of the definition
    write!(buffer, "pub fn {}(", method.as_ident()).context(FormatSnafu)?;

    // Write the parameter list.
    // TODO: This is so clumsy! I should clean it up.
    if let Some(mut param) = method.param {
        let mutability = woog.exhume_mutability(&param.mutability).unwrap();
        write!(
            buffer,
            "{}: {},",
            param.name.as_ident(),
            param.ty.as_type(&mutability, &sarzak),
        )
        .context(FormatSnafu)?;

        while let Some(next_param) = param.next {
            let mutability = woog.exhume_mutability(&next_param.mutability).unwrap();
            write!(
                buffer,
                "{}: {},",
                // Why do I need to drill down to name?
                next_param.name.as_ident(),
                next_param.ty.as_type(&mutability, &sarzak),
            )
            .context(FormatSnafu)?;

            param = &next_param;
        }
    }

    // Finish the first line of the definition
    writeln!(
        buffer,
        ") -> {} {{",
        method.ty.as_type(&Mutability::Borrowed(BORROWED), sarzak)
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
    _store: &SarzakStore,
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

        params.extend([val.name.as_ident(), ",".to_owned()]);
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
    store: &SarzakStore,
    imports: Option<&HashMap<String, Domain>>,
    config: &GraceConfig,
) -> Result<()> {
    if let Some(lval) = lval {
        assert!(lval.ty == GType::Reference(object.id));
        write!(buffer, "let {} = ", lval.name).context(FormatSnafu)?;
    }
    emit!(
        buffer,
        "{} {{",
        object.as_type(&Mutability::Borrowed(BORROWED), &store)
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
            GType::Uuid => match &rval.ty {
                GType::Uuid => emit!(buffer, "{}: {},", field.name, rval.name),
                GType::Reference(obj_id) => {
                    let obj = store.exhume_object(&obj_id).unwrap();
                    let is_supertype = if let Some(imports) = imports {
                        flubber_imports(obj, config, store, imports, object_is_supertype)
                    } else {
                        object_is_supertype(obj, store)
                    };

                    if is_supertype {
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
            GType::Option(_left) => match &rval.ty {
                GType::Option(right) => match **right {
                    GType::Reference(obj_id) => {
                        let obj = store.exhume_object(&obj_id).unwrap();
                        let is_supertype = if let Some(imports) = imports {
                            flubber_imports(obj, config, store, imports, object_is_supertype)
                        } else {
                            object_is_supertype(obj, store)
                        };

                        if is_supertype {
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
    emit!(buffer, "}};");

    Ok(())
}

// pub(crate) fn introspect_object<G>(&object: &Object) -> G {
// G::new()
// }

/// I was at a loss for words.
///
/// I'm not even sure what it does right now.
///
/// It appears to apply a function and or it's results with the results of
/// applying that function to objects the imported objects in this domain.
///
/// Flubber.
pub(crate) fn flubber_imports(
    object: &Object,
    config: &GraceConfig,
    domain_store: &SarzakStore,
    imports: &HashMap<String, Domain>,
    function: fn(&Object, &SarzakStore) -> bool,
) -> bool {
    function(object, domain_store)
        || imports
            .iter()
            .find(|(name, domain)| {
                let object = if config.is_imported(&object.id) {
                    let imported = config.get_imported(&object.id).unwrap();
                    if let Some(object) = domain.sarzak().exhume_object(&imported.id) {
                        object
                    } else {
                        object
                    }
                } else {
                    object
                };

                log::debug!(
                    "checking {}({}) against imported domain: {} ({})",
                    object.name,
                    object.id,
                    domain.name(),
                    name,
                );
                function(object, domain.sarzak())
            })
            .is_some()
}

pub(crate) fn object_is_supertype(object: &Object, store: &SarzakStore) -> bool {
    let is_super = sarzak_maybe_get_many_r_sups_across_r14!(object, store);

    is_super.len() > 0
}

pub(crate) fn object_is_singleton(object: &Object, store: &SarzakStore) -> bool {
    let attrs = sarzak_get_many_as_across_r1!(object, store);
    let referrers = sarzak_maybe_get_many_r_froms_across_r17!(object, store);
    let assoc_referrers = sarzak_maybe_get_many_ass_froms_across_r26!(object, store);

    attrs.len() < 2 && referrers.len() < 1 && assoc_referrers.len() < 1
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

pub(crate) fn find_store(name: &str, domain: &Domain) -> External {
    let name = name
        .split("::")
        .last()
        .expect(format!("Can't parse store from {}", name).as_str());
    let name = format!(
        "{}Store",
        name.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
    );

    let mut iter = domain.sarzak().iter_ty();
    loop {
        let ty = iter.next();
        match ty {
            Some((_, ty)) => match ty {
                Type::External(e) => {
                    let ext = domain.sarzak().exhume_external(&e).unwrap();
                    if ext.name == name {
                        break External::new(ext.name.clone(), ext.path.clone(), None);
                    }
                }
                _ => continue,
            },
            None => panic!("Could not find store type for {}", name),
        }
    }
}

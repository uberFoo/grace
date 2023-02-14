//! Things necessary for code generation
//!

pub(crate) mod buffer;
pub(crate) mod diff_engine;
pub(crate) mod generator;
pub(crate) mod render;
mod rustfmt;

use std::{fmt::Write, iter::zip};

use sarzak::{
    domain::Domain,
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::{
        macros::{
            sarzak_get_many_as_across_r1, sarzak_maybe_get_many_ass_froms_across_r26,
            sarzak_maybe_get_many_r_froms_across_r17, sarzak_maybe_get_many_r_sups_across_r14,
        },
        store::ObjectStore as SarzakStore,
        types::{AssociativeReferrer, Attribute, Object, Referrer, Supertype},
    },
    woog::{
        store::ObjectStore as WoogStore,
        types::{Mutability, BORROWED},
    },
};
use snafu::prelude::*;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        render::{RenderIdent, RenderType},
    },
    todo::{GType, LValue, ObjectMethod, RValue},
};

macro_rules! get_referrers {
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
pub(crate) use get_referrers;

macro_rules! get_referents {
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
pub(crate) use get_referents;

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
    store: &SarzakStore,
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
        // TODO: This type conversion should likely be a function.
        match &field.ty {
            GType::Uuid => match &rval.ty {
                GType::Uuid => emit!(buffer, "{}: {},", field.name, rval.name),
                GType::Reference(_) => emit!(buffer, "{}: {}.id,", field.name, rval.name),
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
                        emit!(
                            buffer,
                            "{}: {}.map(|{}| {}.id),",
                            field.name,
                            rval.name,
                            obj.as_ident(),
                            obj.as_ident()
                        )
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

pub(crate) fn object_is_supertype(object: &Object, domain: &Domain) -> bool {
    let is_super = sarzak_maybe_get_many_r_sups_across_r14!(object, domain.sarzak());

    is_super.len() > 0
}

pub(crate) fn object_is_singleton(object: &Object, domain: &Domain) -> bool {
    let attrs = sarzak_get_many_as_across_r1!(object, domain.sarzak());
    let referrers = sarzak_maybe_get_many_r_froms_across_r17!(object, domain.sarzak());
    let assoc_referrers = sarzak_maybe_get_many_ass_froms_across_r26!(object, domain.sarzak());

    attrs.len() < 2 && referrers.len() < 1 && assoc_referrers.len() < 1
}

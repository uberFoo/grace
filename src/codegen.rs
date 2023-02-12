//! Things necessary for code generation
//!

pub(crate) mod buffer;
pub(crate) mod diff_engine;
pub(crate) mod generator;
pub(crate) mod render;
mod rustfmt;

use std::{fmt::Write, iter::zip};

use sarzak::{
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::{
        store::ObjectStore as SarzakStore,
        types::{Object, Type, UUID},
    },
    woog::{
        macros::{woog_maybe_get_one_param_across_r1, woog_maybe_get_one_param_across_r5},
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
            GType::Option(left) => match &rval.ty {
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

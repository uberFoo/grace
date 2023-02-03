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
        types::{ObjectMethod, Parameter},
    },
};
use snafu::prelude::*;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        render::{RenderIdent, RenderType},
    },
    todo::{LValue, RValue},
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
    if let Some(mut param) = woog_maybe_get_one_param_across_r5!(method, woog) {
        let ty = sarzak.exhume_ty(&param.ty).unwrap();
        write!(
            buffer,
            "{}: {},",
            param.name.as_ident(),
            ty.as_type(&sarzak),
        )
        .context(FormatSnafu)?;

        while let Some(next_param) = woog_maybe_get_one_param_across_r1!(param, woog) {
            let ty = sarzak.exhume_ty(&next_param.ty).unwrap();
            write!(
                buffer,
                "{}: {},",
                next_param.as_ident(),
                ty.as_type(&sarzak),
            )
            .context(FormatSnafu)?;

            param = next_param;
        }
    }

    // Finish the first line of the definition
    let ty = sarzak.exhume_ty(&method.ty).unwrap();
    writeln!(buffer, ") -> {} {{", ty.as_type(sarzak)).context(FormatSnafu)?;

    Ok(())
}

/// Generate code to create a new UUID
///
/// TODO: We should be taking a list of rvals to use, and not [`Parameter`]s.
pub(crate) fn render_make_uuid(
    buffer: &mut Buffer,
    lval: &LValue,
    rvals: &Vec<Parameter>,
    store: &SarzakStore,
) -> Result<()> {
    assert!(lval.ty == UUID);

    let mut format_string = String::new();
    let mut params = String::new();
    for val in rvals {
        let ty = store.exhume_ty(&val.ty).unwrap();

        if let Type::Reference(_) = ty {
            format_string.extend(["{:?}:"]);
        } else {
            format_string.extend(["{}:"]);
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
        assert!(lval.ty == object.id);
        write!(buffer, "let {} = ", lval.name).context(FormatSnafu)?;
    }
    emit!(buffer, "{} {{", object.as_type(&store));

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
        let field_type = store.exhume_ty(&field.ty).unwrap();
        log::trace!("field type {} {:?}", field.name, field_type);
        let rval_type = store.exhume_ty(&rval.ty).unwrap();
        log::trace!("rval type {} {:?}", rval.name, rval_type);

        // TODO: This type conversion should likely be a function.
        match field_type {
            Type::Uuid(_) => match rval_type {
                Type::Uuid(_) => emit!(buffer, "{}: {},", field.name, rval.name),
                Type::Reference(_) => emit!(buffer, "{}: {}.id,", field.name, rval.name),
                _ => ensure!(
                    field_type == rval_type,
                    CompilerSnafu {
                        description: "type mismatch"
                    }
                ),
            },
            _ => {
                ensure!(
                    field_type == rval_type,
                    CompilerSnafu {
                        description: "type mismatch"
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

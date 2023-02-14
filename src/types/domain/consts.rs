//! Domain Const Generation
//!
//! There we were.
use std::fmt::Write;

use sarzak::{
    domain::Domain,
    mc::{CompilerSnafu, FormatSnafu, Result},
    woog::store::ObjectStore as WoogStore,
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        render::{RenderConst, RenderIdent},
    },
    options::GraceCompilerOptions,
    types::{CodeWriter, TypeDefinition},
};

/// Domain Enum Generator / CodeWriter
///
pub(crate) struct DomainConst;

impl DomainConst {
    pub(crate) fn new() -> Box<dyn TypeDefinition> {
        Box::new(Self)
    }
}

impl TypeDefinition for DomainConst {}

impl CodeWriter for DomainConst {
    fn write_code(
        &self,
        options: &GraceCompilerOptions,
        domain: &Domain,
        _woog: &mut WoogStore,
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

        log::debug!("writing Const Definition for {}", obj.name);

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-use-statements", obj.as_ident()),
            |buffer| {
                // Everything has an `id`, everything needs this.
                emit!(buffer, "use uuid::{{Uuid, uuid}};");

                Ok(())
            },
        )?;
        emit!(buffer, "");

        buffer.block(
            DirectiveKind::CommentOrig,
            format!("{}-const-documentation", obj.as_ident()),
            |buffer| {
                for line in obj.description.split_terminator('\n') {
                    emit!(buffer, "/// {}", line);
                }
                Ok(())
            },
        )?;

        let domain_id = Uuid::from_slice(domain.id().as_bytes()).unwrap();
        let id = Uuid::new_v5(&domain_id, obj.name.as_bytes());
        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-const-definition", obj.as_ident()),
            |buffer| {
                emit!(
                    buffer,
                    "pub const {}: Uuid = uuid![\"{}\"];",
                    obj.as_const(),
                    id
                );
                Ok(())
            },
        )?;

        Ok(())
    }
}

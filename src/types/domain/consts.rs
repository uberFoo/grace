//! Domain Const Generation
//!
//! There we were.
use std::fmt::Write;

use sarzak::{
    domain::Domain,
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::{
        macros::{
            sarzak_get_many_r_subs_across_r27, sarzak_get_one_obj_across_r15,
            sarzak_get_one_obj_across_r17, sarzak_get_one_r_bin_across_r5,
            sarzak_get_one_r_from_across_r6, sarzak_get_one_r_isa_across_r13,
            sarzak_maybe_get_many_r_sups_across_r14, sarzak_maybe_get_many_r_tos_across_r16,
        },
        types::{Referent, Subtype, Supertype, Type},
    },
    woog::{store::ObjectStore as WoogStore, Mutability, BORROWED},
};
use snafu::prelude::*;
use uuid::{uuid, Uuid};

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        get_referents,
        render::{RenderConst, RenderIdent, RenderType},
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

//! Domain Enum Generation
//!
//! Here we are.
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
        types::{Referent, Subtype, Supertype},
    },
    woog::{store::ObjectStore as WoogStore, Mutability, BORROWED},
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        emit_object_comments, get_referents,
        render::{RenderIdent, RenderType},
    },
    options::GraceConfig,
    types::{CodeWriter, MethodImplementation, TypeDefinition},
};

/// Domain Enum Generator / CodeWriter
///
pub(crate) struct DomainEnum;

impl DomainEnum {
    pub(crate) fn new() -> Box<dyn TypeDefinition> {
        Box::new(Self)
    }
}

impl TypeDefinition for DomainEnum {}

impl CodeWriter for DomainEnum {
    fn write_code(
        &self,
        config: &GraceConfig,
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

        let referents = get_referents!(obj, domain.sarzak());

        // I'm convinced that R14 and R15 are broken.
        let sup = sarzak_maybe_get_many_r_sups_across_r14!(obj, domain.sarzak());
        let isa = sarzak_get_one_r_isa_across_r13!(sup[0], domain.sarzak());
        let mut subtypes = sarzak_get_many_r_subs_across_r27!(isa, domain.sarzak());
        subtypes.sort_by(|a, b| {
            let a = sarzak_get_one_obj_across_r15!(a, domain.sarzak());
            let b = sarzak_get_one_obj_across_r15!(b, domain.sarzak());
            a.name.cmp(&b.name)
        });

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-use-statements", obj.as_ident()),
            |buffer| {
                // Everything has an `id`, everything needs this.
                emit!(buffer, "use uuid::Uuid;");
                emit!(buffer, "");

                // Add the use statements from the options.
                if let Some(use_paths) = config.get_use_paths(&obj.id) {
                    for path in use_paths {
                        emit!(buffer, "use {};", path);
                    }
                    emit!(buffer, "");
                }

                Ok(())
            },
        )?;
        emit!(buffer, "");

        log::debug!("writing Enum Definition for {}", obj.name);

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-enum-documentation", obj.as_ident()),
            |buffer| emit_object_comments(obj.description.as_str(), "///", buffer),
        )?;

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-enum-definition", obj.as_ident()),
            |buffer| {
                if let Some(derives) = config.get_derives(&obj.id) {
                    write!(buffer, "#[derive(").context(FormatSnafu)?;
                    for d in derives {
                        write!(buffer, "{},", d).context(FormatSnafu)?;
                    }
                    emit!(buffer, ")]");
                }

                emit!(
                    buffer,
                    "pub enum {} {{",
                    obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                );
                for subtype in &subtypes {
                    let obj = sarzak_get_one_obj_across_r15!(subtype, domain.sarzak());
                    emit!(
                        buffer,
                        "{}(Uuid),",
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak()),
                    );
                }
                emit!(buffer, "}}");
                Ok(())
            },
        )?;

        Ok(())
    }
}

pub(crate) struct DomainEnumGetIdImpl;

impl DomainEnumGetIdImpl {
    pub(crate) fn new() -> Box<dyn MethodImplementation> {
        Box::new(Self)
    }
}

impl MethodImplementation for DomainEnumGetIdImpl {}

impl CodeWriter for DomainEnumGetIdImpl {
    fn write_code(
        &self,
        _config: &GraceConfig,
        domain: &Domain,
        _woog: &mut WoogStore,
        _module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by DomainNewImpl"
            }
        );
        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();

        // I'm convinced that R14 and R15 are broken.
        let sup = sarzak_maybe_get_many_r_sups_across_r14!(obj, domain.sarzak());
        let isa = sarzak_get_one_r_isa_across_r13!(sup[0], domain.sarzak());
        let mut subtypes = sarzak_get_many_r_subs_across_r27!(isa, domain.sarzak());
        subtypes.sort_by(|a, b| {
            let a = sarzak_get_one_obj_across_r15!(a, domain.sarzak());
            let b = sarzak_get_one_obj_across_r15!(b, domain.sarzak());
            a.name.cmp(&b.name)
        });

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-get-id-impl", obj.as_ident()),
            |buffer| {
                emit!(buffer, "pub fn id(&self) -> Uuid {{");
                emit!(buffer, "match self {{");
                for subtype in subtypes {
                    let s_obj = sarzak_get_one_obj_across_r15!(subtype, domain.sarzak());
                    emit!(
                        buffer,
                        "{}::{}(id) => *id,",
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak()),
                        s_obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak()),
                    );
                }
                emit!(buffer, "}}");
                emit!(buffer, "}}");
                Ok(())
            },
        )?;

        Ok(())
    }
}

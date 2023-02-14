//! Domain Enum Generation
//!
//! Here we are.
use std::fmt::Write;

use sarzak::{
    domain::Domain,
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::{
        macros::{
            sarzak_get_many_as_across_r1, sarzak_get_many_r_subs_across_r27,
            sarzak_get_one_obj_across_r15, sarzak_get_one_obj_across_r17,
            sarzak_get_one_r_bin_across_r5, sarzak_get_one_r_from_across_r6,
            sarzak_get_one_r_isa_across_r13, sarzak_maybe_get_many_r_sups_across_r14,
            sarzak_maybe_get_many_r_tos_across_r16,
        },
        types::{Attribute, Referent, Subtype, Supertype, Type},
    },
    woog::{store::ObjectStore as WoogStore, Mutability, BORROWED},
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        get_referents,
        render::{RenderConst, RenderIdent, RenderType},
    },
    options::GraceCompilerOptions,
    types::{CodeWriter, MethodImplementation, TypeDefinition},
};

/// Domain Enum Generator / CodeWriter
///
pub(crate) struct DomainEnum;

impl DomainEnum {
    pub(crate) fn new() -> Box<dyn TypeDefinition> {
        Box::new(Self)
    }

    fn render_subtype(subtype: &Subtype, domain: &Domain) -> String {
        let obj = sarzak_get_one_obj_across_r15!(subtype, domain.sarzak());
        let attrs = sarzak_get_many_as_across_r1!(obj, domain.sarzak());

        if attrs.len() == 1 {
            obj.as_const()
        } else {
            obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
        }
    }
}

impl TypeDefinition for DomainEnum {}

impl CodeWriter for DomainEnum {
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
                if let Some(use_paths) = &options.use_paths {
                    for path in use_paths {
                        emit!(buffer, "use {};", path);
                    }
                    emit!(buffer, "");
                }

                // We need this to create id's.
                emit!(buffer, "use crate::{}::UUID_NS;", module);

                // Add the ObjectStore
                let mut iter = domain.sarzak().iter_ty();
                let name = format!(
                    "{}Store",
                    module.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                );
                let store = loop {
                    let ty = iter.next();
                    match ty {
                        Some((_, ty)) => match ty {
                            Type::External(e) => {
                                let ext = domain.sarzak().exhume_external(&e).unwrap();
                                if ext.name == name {
                                    break ext;
                                }
                            }
                            _ => continue,
                        },
                        None => panic!("Could not find store type for {}", module),
                    }
                };
                emit!(buffer, "use {} as {};", store.path, store.name);

                // Add imports for our subtypes.
                emit!(buffer, "");
                emit!(buffer, "// Subtype imports");
                for subtype in &subtypes {
                    let obj = sarzak_get_one_obj_across_r15!(subtype, domain.sarzak());
                    emit!(
                        buffer,
                        "use crate::{}::types::{}::{};",
                        module,
                        obj.as_ident(),
                        DomainEnum::render_subtype(subtype, domain)
                    );
                }

                // Add use statements for all the referents.
                if referents.len() > 0 {
                    emit!(buffer, "");
                    emit!(buffer, "// Referent imports");
                }
                for referent in &referents {
                    let binary = sarzak_get_one_r_bin_across_r5!(referent, domain.sarzak());
                    let referrer = sarzak_get_one_r_from_across_r6!(binary, domain.sarzak());
                    let r_obj = sarzak_get_one_obj_across_r17!(referrer, domain.sarzak());

                    emit!(
                        buffer,
                        "use crate::{}::types::{}::{};",
                        module,
                        r_obj.as_ident(),
                        r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                    );
                }

                Ok(())
            },
        )?;
        emit!(buffer, "");

        log::debug!("writing Enum Definition for {}", obj.name);

        buffer.block(
            DirectiveKind::CommentOrig,
            format!("{}-enum-documentation", obj.as_ident()),
            |buffer| {
                for line in obj.description.split_terminator('\n') {
                    emit!(buffer, "/// {}", line);
                }
                Ok(())
            },
        )?;

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-enum-definition", obj.as_ident()),
            |buffer| {
                if let Some(derive) = &options.derive {
                    write!(buffer, "#[derive(").context(FormatSnafu)?;
                    for d in derive {
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
        _options: &GraceCompilerOptions,
        domain: &Domain,
        woog: &mut WoogStore,
        module: &str,
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

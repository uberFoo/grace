//! Domain Const Generation
//!
//! There we were.
use std::fmt::Write;

use fnv::FnvHashMap as HashMap;
use sarzak::{
    mc::{CompilerSnafu, FormatSnafu, Result},
    v2::domain::Domain,
    woog::{store::ObjectStore as WoogStore, types::Ownership},
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        emit_object_comments,
        render::{RenderConst, RenderIdent, RenderType},
    },
    options::GraceConfig,
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
        config: &GraceConfig,
        domain: &Domain,
        woog: &Option<&mut WoogStore>,
        _imports: &Option<&HashMap<String, Domain>>,
        _package: &str,
        _module: &str,
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
        ensure!(
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by DomainStruct"
            }
        );
        let woog = woog.as_ref().unwrap();

        log::debug!("writing Const Definition for {}", obj.name);

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-use-statements", obj.as_ident()),
            |buffer| {
                // Everything has an `id`, everything needs this.
                emit!(buffer, "use uuid::{{Uuid, uuid}};");

                // Add the use statements from the options.
                if let Some(use_paths) = config.get_use_paths(&obj.id) {
                    for path in use_paths {
                        emit!(buffer, "use {};", path);
                    }
                }

                Ok(())
            },
        )?;
        emit!(buffer, "");

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-const-documentation", obj.as_ident()),
            |buffer| emit_object_comments(obj.description.as_str(), "/// ", "", buffer),
        )?;

        let domain_id = Uuid::from_slice(domain.id().as_bytes()).unwrap();
        // 🚧 There is a domain_ns attribute on paper, that we should probably use here.
        // It's not currently being captured by nut. Maybe we just stick with this.
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

                emit!(buffer, "");
                if let Some(derives) = config.get_derives(&obj.id) {
                    write!(buffer, "#[derive(").context(FormatSnafu)?;
                    for d in derives {
                        write!(buffer, "{},", d).context(FormatSnafu)?;
                    }
                    emit!(buffer, ")]");
                }
                emit!(
                    buffer,
                    "pub struct {};",
                    obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
                emit!(buffer, "");
                emit!(
                    buffer,
                    "impl {} {{",
                    obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
                emit!(buffer, "    pub fn new() -> Self {{");
                emit!(buffer, "        Self {{}}");
                emit!(buffer, "    }}");
                emit!(buffer, "");
                emit!(buffer, "    pub fn id(&self) -> Uuid {{");
                emit!(buffer, "        {}", obj.as_const());
                emit!(buffer, "    }}");
                emit!(buffer, "}}\n");

                emit!(
                    buffer,
                    "impl Default for {} {{",
                    obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );
                emit!(buffer, "    fn default() -> Self {{");
                emit!(buffer, "        Self::new()");
                emit!(buffer, "    }}");
                emit!(buffer, "}}");

                Ok(())
            },
        )?;

        Ok(())
    }
}

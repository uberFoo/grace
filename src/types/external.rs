//! A type for generating an external type.
//!
use std::{collections::HashMap, fmt::Write};

use sarzak::{
    mc::{CompilerSnafu, FormatSnafu, Result},
    v2::domain::Domain,
    woog::{store::ObjectStore as WoogStore, types::Ownership},
};
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        emit_object_comments,
        generator::{FileGenerator, GenerationAction},
        render::{RenderIdent, RenderType},
        render_method_definition_new,
    },
    options::GraceConfig,
};
use snafu::prelude::*;

pub(crate) struct ExternalGenerator;

impl ExternalGenerator {
    pub(crate) fn new() -> Box<dyn FileGenerator> {
        Box::new(Self)
    }
}

impl FileGenerator for ExternalGenerator {
    fn generate(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        woog: &Option<&mut WoogStore>,
        _imports: &Option<&HashMap<String, Domain>>,
        _package: &str,
        _module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<GenerationAction> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by DomainStruct"
            }
        );
        let obj_id = obj_id.unwrap();
        ensure!(
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by DomainStruct"
            }
        );
        let woog = woog.as_ref().unwrap();

        let object = domain.sarzak().exhume_object(&obj_id).unwrap();
        let external = config.get_external(&obj_id).unwrap();

        emit!(buffer, "//! {} External Entity", object.name);
        emit!(buffer, "//!");

        // Write the use statements.
        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-ee-use-statements", object.as_ident()),
            |buffer| {
                emit!(buffer, "use {}::{};", external.path, external.name);
                emit!(buffer, "use uuid::Uuid;");

                Ok(())
            },
        )?;

        // Documentation
        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-ee-documentation", object.as_ident()),
            |buffer| emit_object_comments(object.description.as_str(), "///", buffer),
        )?;

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-ee-definition", object.as_ident()),
            |buffer| {
                emit!(
                    buffer,
                    "pub struct {} {{pub id: Uuid, pub value: {}}}",
                    object.as_type(&Ownership::new_borrowed(), woog, domain),
                    external.name
                );

                Ok(())
            },
        )?;

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-ee-impl", object.as_ident()),
            |buffer| {
                let method = woog
                    .iter_object_method()
                    .find(|m| m.object == object.id)
                    .unwrap();
                emit!(
                    buffer,
                    "impl {} {{",
                    object.as_type(&Ownership::new_borrowed(), woog, domain)
                );

                render_method_definition_new(buffer, &method, woog, domain)?;
                // emit!(buffer, "    pub fn new() -> Self {{");
                // emit!(
                //     buffer,
                //     "let value = {}::{}();",
                //     external.name,
                //     external.ctor
                // );
                // emit!(buffer, "        Self {{");
                // emit!(
                //     buffer,
                //     "            id: Uuid::new_v5(format!(\"{{}}\", value)),"
                // );
                // emit!(buffer, "            value,");
                // emit!(buffer, "        }}");
                // emit!(buffer, "    }}");
                emit!(buffer, "}}");

                Ok(())
            },
        )?;

        Ok(GenerationAction::Write)
    }
}

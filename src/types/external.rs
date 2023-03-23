//! A type for generating an external type.
//!
use std::fmt::Write;

use fnv::FnvHashMap as HashMap;
use sarzak::{
    mc::{CompilerSnafu, FormatSnafu, Result},
    v2::domain::Domain,
    woog::{store::ObjectStore as WoogStore, types::Ownership, SHARED},
};
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        emit_object_comments, find_store,
        generator::{FileGenerator, GenerationAction},
        render::{
            render_associative_attributes, render_attributes, render_referential_attributes,
            RenderIdent, RenderType,
        },
        render_method_new,
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
        imports: &Option<&HashMap<String, Domain>>,
        _package: &str,
        module: &str,
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
        ensure!(
            imports.is_some(),
            CompilerSnafu {
                description: "imports is required by DomainNewImpl"
            }
        );

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
                emit!(buffer, "use crate::{}::UUID_NS;", module);

                // Add the use statements from the options.
                if let Some(use_paths) = config.get_use_paths(&object.id) {
                    for path in use_paths {
                        emit!(buffer, "use {};", path);
                    }
                }

                let store = find_store(module, woog, domain);
                emit!(buffer, "use {} as {};", store.path, store.name);

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
                if let Some(derives) = config.get_derives(&object.id) {
                    write!(buffer, "#[derive(").context(FormatSnafu)?;
                    for d in derives {
                        write!(buffer, "{},", d).context(FormatSnafu)?;
                    }
                    emit!(buffer, ")]");
                }
                emit!(
                    buffer,
                    "pub struct {} {{",
                    object.as_type(
                        &woog
                            .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                            .unwrap(),
                        woog,
                        domain
                    ),
                );

                render_attributes(buffer, object, woog, domain)?;
                render_referential_attributes(buffer, object, woog, domain)?;
                render_associative_attributes(buffer, object, woog, domain)?;
                emit!(buffer, "ext_value: {},", external.name);

                emit!(buffer, "}}");

                Ok(())
            },
        )?;

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-ee-impl", object.as_ident()),
            |buffer| {
                emit!(
                    buffer,
                    "impl {} {{",
                    object.as_type(
                        &woog
                            .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                            .unwrap(),
                        woog,
                        domain
                    )
                );

                // Darn. So I need to insert a local here. And hybrid has similar needs.
                render_method_new(buffer, object, config, imports, woog, domain)?;

                emit!(buffer, "}}");

                Ok(())
            },
        )?;

        Ok(GenerationAction::Write)
    }
}

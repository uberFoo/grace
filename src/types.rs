//! Generate Types
//!
//! This is the entry point for all type generation.
use std::fmt::Write;

use log;
use sarzak::{
    domain::Domain,
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::{
        macros::{
            sarzak_get_many_as_across_r1, sarzak_get_one_obj_across_r16,
            sarzak_get_one_r_bin_across_r6, sarzak_get_one_r_to_across_r5,
            sarzak_get_one_t_across_r2, sarzak_maybe_get_many_r_froms_across_r17,
        },
        types::{Attribute, Referrer},
    },
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::codegen::{
    buffer::{Buffer, Directive},
    generator::{CodeWriter, FileGenerator},
    render::{RenderIdent, RenderType},
};

pub(crate) struct DefaultStructBuilder<'a> {
    definition: Option<Box<dyn StructDefinition + 'a>>,
}

impl<'a> DefaultStructBuilder<'a> {
    pub(crate) fn new() -> Self {
        DefaultStructBuilder { definition: None }
    }

    pub(crate) fn definition(mut self, definition: Box<dyn StructDefinition + 'a>) -> Self {
        self.definition = Some(definition);

        self
    }

    pub(crate) fn build(self) -> Result<Box<DefaultStructGenerator<'a>>> {
        ensure!(
            self.definition.is_some(),
            CompilerSnafu {
                description: "missing StructDefinition"
            }
        );

        Ok(Box::new(DefaultStructGenerator {
            definition: self.definition.unwrap(),
        }))
    }
}

/// Generator -- Code Generator Engine
///
/// This is supposed to be general, but it's very much geared towards generating
/// a file that contains a struct definition and implementations. I need to
/// do some refactoring.
///
/// As just hinted at, the idea is that you plug in different code writers that
/// know how to write different parts of some rust code. This one is for
/// structs.
pub(crate) struct DefaultStructGenerator<'a> {
    definition: Box<dyn StructDefinition + 'a>,
}

impl<'a> FileGenerator for DefaultStructGenerator<'a> {
    fn generate(&self, domain: &Domain, buffer: &mut Buffer) -> Result<()> {
        buffer.block(
            Directive::Provenance,
            "something better than this",
            |buffer| {
                // It's important that we maintain ordering for code injection and
                // redaction. We begin with the struct definition.
                self.definition.write_code(domain, buffer)?;

                Ok(())
            },
        )?;

        Ok(())
    }
}

pub(crate) trait StructDefinition: CodeWriter {}

/// Default Struct Generator / CodeWriter
///
/// We need a builder for this so that we can add privacy modifiers, as
/// well as derives.
pub(crate) struct DefaultStruct<'a> {
    obj_id: &'a Uuid,
}

impl<'a> DefaultStruct<'a> {
    pub(crate) fn new(obj_id: &'a Uuid) -> Box<Self> {
        Box::new(Self { obj_id })
    }
}

impl<'a> StructDefinition for DefaultStruct<'a> {}

impl<'a> CodeWriter for DefaultStruct<'a> {
    fn write_code(&self, store: &Domain, buffer: &mut Buffer) -> Result<()> {
        let obj = store.sarzak().exhume_object(self.obj_id).unwrap();
        let referrers = sarzak_maybe_get_many_r_froms_across_r17!(obj, store.sarzak());
        let has_referential_attrs = referrers.len() > 0;

        // Everything has an `id`, everything needs this.
        writeln!(buffer, "use uuid::Uuid;").context(FormatSnafu)?;
        writeln!(buffer).context(FormatSnafu)?;

        let mut paste = Buffer::new();
        buffer.block(
            Directive::PreferNewCommentOld,
            format!("{}-referrer-use-statements", obj.as_ident()),
            |buffer| {
                // This is sort of long, and sticks out. Maybe it goes into a function?
                for referrer in &referrers {
                    let binary = sarzak_get_one_r_bin_across_r6!(referrer, store.sarzak());
                    let referent = sarzak_get_one_r_to_across_r5!(binary, store.sarzak());
                    let r_obj = sarzak_get_one_obj_across_r16!(referent, store.sarzak());

                    writeln!(
                        buffer,
                        "use crate::everything::types::{}::{};",
                        r_obj.as_ident(),
                        r_obj.as_type()
                    )
                    .context(FormatSnafu)?;

                    writeln!(paste, "/// R{}: {}", binary.number, referrer.description)
                        .context(FormatSnafu)?;
                    writeln!(
                        paste,
                        "pub {}: &'a {}",
                        referrer.referential_attribute,
                        r_obj.as_type()
                    )
                    .context(FormatSnafu)?;
                }

                Ok(())
            },
        )?;

        log::debug!("writing Struct Definition for {}", obj.name);

        buffer.block(
            Directive::PreferNewCommentOld,
            format!("{}-struct-definition", obj.as_ident()),
            |buffer| {
                if has_referential_attrs {
                    // Lifetime parameters. Really, we should assign one for each attribute. TBD.
                    writeln!(buffer, "pub struct {}<'a> {{", obj.as_type()).context(FormatSnafu)?;
                } else {
                    writeln!(buffer, "pub struct {} {{", obj.as_type()).context(FormatSnafu)?;
                }

                let mut attrs = sarzak_get_many_as_across_r1!(obj, store.sarzak());
                attrs.sort_by(|a, b| a.name.cmp(&b.name));
                for attr in attrs {
                    let ty = sarzak_get_one_t_across_r2!(attr, store.sarzak());
                    writeln!(buffer, "pub {}: {},", attr.as_ident(), ty.as_type())
                        .context(FormatSnafu)?;
                }

                // Paste in the referential attributes, computed above.
                *buffer += paste;

                writeln!(buffer, "}}").context(FormatSnafu)?;
                Ok(())
            },
        )?;

        Ok(())
    }
}

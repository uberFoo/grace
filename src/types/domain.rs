//! Domain Struct Handling
//!
//! This is for generating structs that are used as part of a Domain.
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

use crate::{
    codegen::{
        buffer::{Buffer, Directive},
        generator::{CodeWriter, FileGenerator},
        render::{RenderIdent, RenderType},
    },
    options::GraceCompilerOptions,
    types::StructDefinition,
};

/// Domain Struct Generator / CodeWriter
///
/// We need a builder for this so that we can add privacy modifiers, as
/// well as derives.
pub(crate) struct DomainStruct<'a> {
    obj_id: &'a Uuid,
}

impl<'a> DomainStruct<'a> {
    pub(crate) fn new(obj_id: &'a Uuid) -> Box<Self> {
        Box::new(Self { obj_id })
    }
}

impl<'a> StructDefinition for DomainStruct<'a> {}

impl<'a> CodeWriter for DomainStruct<'a> {
    fn write_code(
        &self,
        options: &GraceCompilerOptions,
        store: &Domain,
        module: &str,
        buffer: &mut Buffer,
    ) -> Result<()> {
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
                        "use crate::{}::types::{}::{};",
                        module,
                        r_obj.as_ident(),
                        r_obj.as_type()
                    )
                    .context(FormatSnafu)?;

                    writeln!(paste, "/// R{}: {}", binary.number, referrer.description)
                        .context(FormatSnafu)?;
                    writeln!(paste, "pub {}: Uuid", referrer.referential_attribute,)
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
                if let Some(derive) = &options.derive {
                    write!(buffer, "#[derive(").context(FormatSnafu)?;
                    for d in derive {
                        write!(buffer, "{},", d).context(FormatSnafu)?;
                    }
                    writeln!(buffer, ")]").context(FormatSnafu)?;
                }

                writeln!(buffer, "pub struct {} {{", obj.as_type()).context(FormatSnafu)?;

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

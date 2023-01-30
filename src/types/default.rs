//! Default Struct Handling
//!
//! This is the place to find all the default implementations for generating structs.
//! These are meant to be used in an application domain.
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
        types::{Attribute, Object, Referrer},
    },
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::Buffer,
        generator::{CodeWriter, FileGenerator},
        render::{RenderIdent, RenderType},
        DirectiveKind,
    },
    options::GraceCompilerOptions,
    types::{ModuleDefinition, StructDefinition},
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
    fn generate(
        &self,
        options: &GraceCompilerOptions,
        domain: &Domain,
        module: &str,
        buffer: &mut Buffer,
    ) -> Result<()> {
        buffer.block(
            DirectiveKind::AllowEditing,
            format!("{}-struct-definition-file", "no-obj-here"),
            |buffer| {
                // It's important that we maintain ordering for code injection and
                // redaction. We begin with the struct definition.
                self.definition
                    .write_code(options, domain, module, buffer)?;

                Ok(())
            },
        )?;

        Ok(())
    }
}

/// Default Struct Generator / CodeWriter
///
/// We need a builder for this so that we can add privacy modifiers, as
/// well as derives.
pub(crate) struct DefaultStruct<'a> {
    obj_id: &'a Uuid,
}

impl<'a> DefaultStruct<'a> {
    pub(crate) fn new(obj_id: &'a Uuid) -> Box<dyn StructDefinition + 'a> {
        Box::new(Self { obj_id })
    }
}

impl<'a> StructDefinition for DefaultStruct<'a> {}

impl<'a> CodeWriter for DefaultStruct<'a> {
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
            DirectiveKind::IgnoreOrig,
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
                    writeln!(
                        paste,
                        "pub {}: &'a {},",
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
            DirectiveKind::CommentOrig,
            format!("{}-struct-documentation", obj.as_ident()),
            |buffer| {
                for line in obj.description.split_terminator('\n') {
                    writeln!(buffer, "/// {}", line).context(FormatSnafu)?;
                }
                Ok(())
            },
        )?;

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-struct-definition", obj.as_ident()),
            |buffer| {
                if let Some(derive) = &options.derive {
                    write!(buffer, "#[derive(").context(FormatSnafu)?;
                    for d in derive {
                        write!(buffer, "{},", d).context(FormatSnafu)?;
                    }
                    writeln!(buffer, ")]").context(FormatSnafu)?;
                }

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

pub(crate) struct DefaultModuleBuilder<'a> {
    definition: Option<Box<dyn ModuleDefinition + 'a>>,
}

impl<'a> DefaultModuleBuilder<'a> {
    pub(crate) fn new() -> Self {
        DefaultModuleBuilder { definition: None }
    }

    pub(crate) fn definition(mut self, definition: Box<dyn ModuleDefinition + 'a>) -> Self {
        self.definition = Some(definition);

        self
    }

    pub(crate) fn build(self) -> Result<Box<DefaultModuleGenerator<'a>>> {
        ensure!(
            self.definition.is_some(),
            CompilerSnafu {
                description: "missing ModuleDefinition"
            }
        );

        Ok(Box::new(DefaultModuleGenerator {
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
pub(crate) struct DefaultModuleGenerator<'a> {
    definition: Box<dyn ModuleDefinition + 'a>,
}

impl<'a> FileGenerator for DefaultModuleGenerator<'a> {
    fn generate(
        &self,
        options: &GraceCompilerOptions,
        domain: &Domain,
        module: &str,
        buffer: &mut Buffer,
    ) -> Result<()> {
        // Output the domain/module documentation/description
        writeln!(buffer, "//! {}", domain.description()).context(FormatSnafu)?;

        buffer.block(
            DirectiveKind::AllowEditing,
            format!("{}-module-definition-file", module),
            |buffer| {
                // It's important that we maintain ordering for code injection and
                // redaction. We begin with the struct definition.
                self.definition
                    .write_code(options, domain, module, buffer)?;

                Ok(())
            },
        )?;

        Ok(())
    }
}

/// Default Types Module Generator / CodeWriter
///
/// This generates a rust file that imports the generated type implementations.
pub(crate) struct DefaultModule {}

impl<'a> DefaultModule {
    pub(crate) fn new() -> Box<dyn ModuleDefinition + 'a> {
        Box::new(Self {})
    }
}

impl ModuleDefinition for DefaultModule {}

impl<'a> CodeWriter for DefaultModule {
    fn write_code(
        &self,
        _options: &GraceCompilerOptions,
        store: &Domain,
        module: &str,
        buffer: &mut Buffer,
    ) -> Result<()> {
        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-module-definition", module),
            |buffer| {
                let mut objects: Vec<(&Uuid, &Object)> = store.sarzak().iter_object().collect();
                objects.sort_by(|a, b| a.1.name.cmp(&b.1.name));
                for (_, obj) in objects {
                    writeln!(buffer, "pub mod {};", obj.as_ident()).context(FormatSnafu)?;
                }
                Ok(())
            },
        )?;

        Ok(())
    }
}

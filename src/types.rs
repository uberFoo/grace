//! Generate Types
//!
//! This is the entry point for all type generation.
use std::{fmt::Write, io};

use log;
use sarzak::{
    domain::Domain,
    mc::{CompilerSnafu, FormatSnafu, IOSnafu, Result},
    sarzak::{
        macros::{sarzak_get_many_as_across_r1, sarzak_get_one_t_across_r2},
        types::Attribute,
    },
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::codegen::{
    buffer::Buffer,
    generator::{CodeWriter, FileGenerator},
    render::{RenderIdent, RenderType},
};

pub(crate) struct DefaultStructBuilder<'a> {
    store: Option<&'a Domain>,
    definition: Option<Box<dyn StructDefinition + 'a>>,
}

impl<'a> DefaultStructBuilder<'a> {
    pub(crate) fn new() -> Self {
        DefaultStructBuilder {
            store: None,
            definition: None,
        }
    }

    pub(crate) fn definition(mut self, definition: Box<dyn StructDefinition + 'a>) -> Self {
        self.definition = Some(definition);

        self
    }

    pub(crate) fn store(mut self, store: &'a Domain) -> Self {
        self.store = Some(store);

        self
    }

    pub(crate) fn build(self) -> Result<Box<DefaultStructGenerator<'a>>> {
        ensure!(
            self.store.is_some(),
            CompilerSnafu {
                description: "missing domain store"
            }
        );
        ensure!(
            self.definition.is_some(),
            CompilerSnafu {
                description: "missing StructDefinition"
            }
        );

        Ok(Box::new(DefaultStructGenerator {
            store: self.store.unwrap(),
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
    store: &'a Domain,
    definition: Box<dyn StructDefinition + 'a>,
}

impl<'a> FileGenerator for DefaultStructGenerator<'a> {
    fn generate(&self, mut writer: Box<dyn io::Write>) -> Result<()> {
        let mut buffer = Buffer::new();

        // It's important that we maintain ordering for code injection and
        // redaction. We begin with the struct definition.
        self.definition.write_code(self.store, &mut buffer)?;

        // Write it.
        writer
            .write_all(buffer.dump().as_bytes())
            .context(IOSnafu)?;

        Ok(())
    }
}

pub(crate) trait StructDefinition: CodeWriter {}

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
        writeln!(buffer, "use uuid::Uuid;");

        log::debug!("writing Struct Definition for {}", obj.name);
        // We need a builder for this so that we can add privacy modifiers, as
        // well as derives.
        writeln!(buffer, "pub struct {} {{", obj.as_type()).context(FormatSnafu)?;

        for attr in sarzak_get_many_as_across_r1!(obj, store.sarzak()) {
            // Already bumping into problems. Now better than later. The issue
            // is that we need to import Uuid, and we don't know about it until
            // now. We in fact need to know about it long before now. There may
            // very well be things in the impl that need to be used, and they
            // will have the same type of problem as here.
            //
            // Seems like DefaultStructGenerator should have some means of
            // collection use statements and spitting them out at the beginning
            // of the buffer.
            let ty = sarzak_get_one_t_across_r2!(attr, store.sarzak());
            writeln!(buffer, "pub {}: {},", attr.as_ident(), ty.as_type()).context(FormatSnafu)?;
        }

        writeln!(buffer, "}}").context(FormatSnafu)?;

        Ok(())
    }
}

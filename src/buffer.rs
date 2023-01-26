//! A buffer for building files
//!
use std::{
    fmt,
    fs::File,
    io,
    path::{Path, PathBuf},
};

use sarzak::mc::{CompilerSnafu, FileSnafu, IOSnafu, Result};
use snafu::prelude::*;

use crate::types::Type;

pub(crate) struct GeneratorBuilder<'a> {
    writer: Option<Box<dyn io::Write>>,
    type_writer: Option<Box<dyn TypeWriter + 'a>>,
}

impl<'a> GeneratorBuilder<'a> {
    pub fn new() -> Self {
        GeneratorBuilder {
            writer: None,
            type_writer: None,
        }
    }

    pub fn path<P: AsRef<Path>>(mut self, path: P) -> Result<Self> {
        self.writer = Some(Box::new(File::create(path.as_ref()).context(
            FileSnafu {
                path: path.as_ref().clone(),
            },
        )?));

        Ok(self)
    }

    pub fn add_type(mut self, writer: Box<dyn TypeWriter + 'a>) -> Self {
        self.type_writer = Some(writer);

        self
    }

    pub fn build(self) -> Result<Generator<'a>> {
        ensure!(
            self.writer.is_some(),
            CompilerSnafu {
                description: "missing writer"
            }
        );

        Ok(Generator {
            writer: self.writer.unwrap(),
            type_writer: self.type_writer,
        })
    }
}

pub(crate) struct Generator<'a> {
    writer: Box<dyn io::Write>,
    type_writer: Option<Box<dyn TypeWriter + 'a>>,
}

impl<'a> Generator<'a> {
    pub fn generate(&mut self) -> Result<()> {
        if let Some(generator) = &self.type_writer {
            let mut buffer = Buffer::new();
            generator.write_code(&mut buffer);
            self.writer
                .write_all(buffer.dump().as_bytes())
                .context(IOSnafu)?
        }

        Ok(())
    }
}

pub(crate) trait TypeWriter: CodeWriter {}

pub(crate) trait CodeWriter {
    fn write_code(&self, buffer: &mut Buffer);
}

pub(crate) struct Buffer {
    buffer: String,
}

impl Buffer {
    pub(crate) fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub(crate) fn dump(&self) -> &String {
        &self.buffer
    }
}

impl fmt::Write for Buffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.buffer.write_str(s)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.buffer.write_char(c)
    }
}

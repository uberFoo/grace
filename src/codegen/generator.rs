//! Generator Root
//!
//!
use std::{fs::File, io, path::Path};

use sarzak::{
    domain::Domain,
    mc::{CompilerSnafu, FileSnafu, Result},
};
use snafu::prelude::*;

use super::buffer::Buffer;

pub(crate) struct GeneratorBuilder<'a> {
    writer: Option<Box<dyn io::Write>>,
    generator: Option<Box<dyn FileGenerator + 'a>>,
}

impl<'a> GeneratorBuilder<'a> {
    pub fn new() -> Self {
        GeneratorBuilder {
            writer: None,
            generator: None,
        }
    }

    // I tried 'a, and it didn't work...
    pub fn writer<W: io::Write + 'static>(mut self, writer: W) -> Self {
        self.writer = Some(Box::new(writer));

        self
    }

    pub fn path<P: AsRef<Path>>(mut self, path: P) -> Result<Self> {
        self.writer = Some(Box::new(File::create(path.as_ref()).context(
            FileSnafu {
                path: path.as_ref().clone(),
            },
        )?));

        Ok(self)
    }

    pub fn generator(mut self, generator: Box<dyn FileGenerator + 'a>) -> Self {
        self.generator = Some(generator);

        self
    }

    pub fn generate(self) -> Result<()> {
        ensure!(
            self.writer.is_some(),
            CompilerSnafu {
                description: "missing writer"
            }
        );

        ensure!(
            self.generator.is_some(),
            CompilerSnafu {
                description: "missing FileGenerator"
            }
        );

        self.generator.unwrap().generate(self.writer.unwrap())
    }
}

pub(crate) trait FileGenerator {
    fn generate(&self, writer: Box<dyn io::Write>) -> Result<()>;
}

/// CodeWriter
///
/// This trait is implemented for types that write code. The key element to note
/// is the `Buffer` parameter. That's a dead giveaway that the rubber is hitting
/// the road.
pub(crate) trait CodeWriter {
    fn write_code(&self, store: &Domain, buffer: &mut Buffer) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_builder_error() {
        let gb = GeneratorBuilder::new().generate();
        assert!(gb.is_err());
    }
}

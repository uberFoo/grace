//! Generator Root
//!
//!
use std::{fs::File, io::prelude::*, path::Path};

use sarzak::{
    domain::Domain,
    mc::{CompilerSnafu, FileSnafu, IOSnafu, Result},
};
use snafu::prelude::*;

use crate::{
    codegen::{buffer::Buffer, rustfmt::format},
    options::GraceCompilerOptions,
};

pub(crate) struct GeneratorBuilder<'a> {
    original: Option<String>,
    writer: Option<Box<dyn Write>>,
    generator: Option<Box<dyn FileGenerator + 'a>>,
    domain: Option<&'a Domain>,
    options: Option<&'a GraceCompilerOptions>,
    module: Option<&'a str>,
}

impl<'a> GeneratorBuilder<'a> {
    pub fn new() -> Self {
        GeneratorBuilder {
            original: None,
            writer: None,
            generator: None,
            domain: None,
            options: None,
            module: None,
        }
    }

    pub fn options(mut self, options: &'a GraceCompilerOptions) -> Self {
        self.options = Some(options);

        self
    }

    pub fn path<P: AsRef<Path>>(mut self, path: P) -> Result<Self> {
        let path = path.as_ref();
        if path.exists() {
            let mut file = File::open(&path).context(FileSnafu { path: path })?;
            let mut buffer = String::new();
            file.read_to_string(&mut buffer);

            self.original = Some(format(&buffer)?);
        }

        self.writer = Some(Box::new(
            File::create(path).context(FileSnafu { path: path })?,
        ));

        Ok(self)
    }

    pub fn generator(mut self, generator: Box<dyn FileGenerator + 'a>) -> Self {
        self.generator = Some(generator);

        self
    }

    pub(crate) fn module(mut self, module: &'a str) -> Self {
        self.module = Some(module);

        self
    }

    pub(crate) fn domain(mut self, domain: &'a Domain) -> Self {
        self.domain = Some(domain);

        self
    }

    pub fn generate(self) -> Result<()> {
        ensure!(
            self.options.is_some(),
            CompilerSnafu {
                description: "missing compiler options"
            }
        );

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

        ensure!(
            self.domain.is_some(),
            CompilerSnafu {
                description: "missing domain"
            }
        );

        ensure!(
            self.module.is_some(),
            CompilerSnafu {
                description: "missing module"
            }
        );

        let mut writer = self.writer.unwrap();

        let mut buffer = Buffer::new();
        match self.generator.unwrap().generate(
            &self.options.unwrap(),
            &self.domain.unwrap(),
            self.module.unwrap(),
            &mut buffer,
        ) {
            Ok(_) => {
                if let Some(_original) = self.original {
                    // Diff the buffers and write the output
                    let generated = format(&buffer.dump())?;
                    writer.write_all(generated.as_bytes()).context(IOSnafu)?
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

pub(crate) trait FileGenerator {
    fn generate(
        &self,
        options: &GraceCompilerOptions,
        domain: &Domain,
        module: &str,
        buffer: &mut Buffer,
    ) -> Result<()>;
}

/// CodeWriter
///
/// This trait is implemented for types that write code. The key element to note
/// is the `Buffer` parameter. That's a dead giveaway that the rubber is hitting
/// the road.
pub(crate) trait CodeWriter {
    fn write_code(
        &self,
        options: &GraceCompilerOptions,
        domain: &Domain,
        module: &str,
        buffer: &mut Buffer,
    ) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_builder_error() {
        let gb = GeneratorBuilder::new().generate();
        assert!(gb.is_err());

        let gb = GeneratorBuilder::new().path("/tmp/foo").unwrap().generate();
        assert!(gb.is_err());

        let _domain = sarzak::domain::DomainBuilder::new()
            .cuckoo_model("tests/mdd/models/everything.json")
            .unwrap()
            .build()
            .unwrap();
    }
}

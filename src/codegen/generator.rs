//! Generator Root
//!
//!
use std::{
    fs::{self, File},
    io::prelude::*,
    path::{Path, PathBuf},
};

use sarzak::{
    domain::Domain,
    mc::{CompilerSnafu, FileSnafu, IOSnafu, Result},
};
use snafu::prelude::*;

use crate::{
    codegen::{buffer::Buffer, process_diff, rustfmt::format, DirectiveKind},
    options::GraceCompilerOptions,
};

pub(crate) struct GeneratorBuilder<'a> {
    path: Option<PathBuf>,
    generator: Option<Box<dyn FileGenerator + 'a>>,
    domain: Option<&'a Domain>,
    options: Option<&'a GraceCompilerOptions>,
    module: Option<&'a str>,
}

impl<'a> GeneratorBuilder<'a> {
    pub fn new() -> Self {
        GeneratorBuilder {
            path: None,
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

        self.path = Some(path.to_path_buf());

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
            self.path.is_some(),
            CompilerSnafu {
                description: "missing output path"
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

        let mut buffer = Buffer::new();
        match self.generator.unwrap().generate(
            &self.options.unwrap(),
            &self.domain.unwrap(),
            self.module.unwrap(),
            &mut buffer,
        ) {
            Ok(_) => {
                // Generation was successful, write the output.
                //
                // Because of the way `rustfmt` works (it acts like it's running
                // the compiler) the file needs to be in place (I've only
                // found this to be true for files that are declaring modules).
                // I'd prefer to format a temporary file, and avoid all this
                // shuffling. It's what I did in nut...

                // First, we need to see if the file exists, if not it's the easy
                // path. We write the file and format it in place.
                //
                // Otherwise, we format the existing file, so that we are on as
                // level a field as possible. Once it's formatted we read it
                // into a String. Then...
                //
                // We need to format the generated code. To keep `rustfmt` happy,
                // we _overwrite_ the existing file with the generated code.
                // Format it, and then read it into a String.
                //
                // Finally we can diff the two Strings and write the output.
                // I don't think that it should have to be formatted.
                //
                // Whew!
                //
                let path = self.path.unwrap();

                if path.exists() {
                    // Format the original. We get some validation from ^rustfmt`,
                    // so if it fails, we'll just stop.
                    match format(&path) {
                        Ok(_) => {
                            // Grab the formatted output.
                            let orig = fs::read_to_string(&path).context(IOSnafu)?;

                            // Format the generated buffer
                            let mut file =
                                File::create(&path).context(FileSnafu { path: &path })?;
                            file.write_all(&buffer.dump().as_bytes()).context(IOSnafu)?;
                            match format(&path) {
                                Ok(_) => {
                                    // Grab the generated output
                                    let incoming = fs::read_to_string(&path).context(IOSnafu)?;

                                    // This is where we diff and write the output.
                                    let diffed = process_diff(
                                        orig.as_str(),
                                        incoming.as_str(),
                                        DirectiveKind::IgnoreGenerated,
                                    );

                                    // Write the file
                                    let mut file =
                                        File::create(&path).context(FileSnafu { path: &path })?;
                                    file.write_all(&diffed.as_bytes()).context(IOSnafu)?;
                                }
                                Err(e) => {
                                    eprintln!("{}", e)
                                }
                            };
                        }
                        Err(e) => eprintln!("{}", e),
                    }
                } else {
                    let mut file = File::create(&path).context(FileSnafu { path: &path })?;
                    file.write_all(&buffer.dump().as_bytes()).context(IOSnafu)?;

                    match format(&path) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("{}", e)
                        }
                    };
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

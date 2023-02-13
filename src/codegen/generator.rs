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
    woog::store::ObjectStore as WoogStore,
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::Buffer,
        diff_engine::{process_diff, DirectiveKind},
        rustfmt::format,
    },
    options::GraceCompilerOptions,
};

pub(crate) struct GeneratorBuilder<'a> {
    path: Option<PathBuf>,
    generator: Option<Box<dyn FileGenerator + 'a>>,
    domain: Option<&'a Domain>,
    woog: Option<&'a mut WoogStore>,
    options: Option<&'a GraceCompilerOptions>,
    module: Option<&'a str>,
    obj_id: Option<&'a Uuid>,
}

impl<'a> GeneratorBuilder<'a> {
    pub fn new() -> Self {
        GeneratorBuilder {
            path: None,
            generator: None,
            domain: None,
            woog: None,
            options: None,
            module: None,
            obj_id: None,
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

    pub(crate) fn compiler_domain(mut self, domain: &'a mut WoogStore) -> Self {
        self.woog = Some(domain);

        self
    }

    pub(crate) fn obj_id(mut self, obj_id: &'a Uuid) -> Self {
        self.obj_id = Some(obj_id);

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
            self.woog.is_some(),
            CompilerSnafu {
                description: "missing compiler domain"
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
            &mut self.woog.unwrap(),
            self.module.unwrap(),
            self.obj_id,
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
                    let result = format(&path, false);
                    ensure!(
                        result.is_ok(),
                        CompilerSnafu {
                            description: format!(
                                "rustfmt failed on existing file: {}",
                                path.display()
                            )
                        }
                    );

                    // Grab the formatted output.
                    let orig = fs::read_to_string(&path).context(IOSnafu)?;

                    // Format the generated buffer
                    let mut file = File::create(&path).context(FileSnafu { path: &path })?;
                    file.write_all(&buffer.dump().as_bytes()).context(IOSnafu)?;
                    match format(&path, true) {
                        Ok(_) => {
                            // Grab the generated output
                            let incoming = fs::read_to_string(&path).context(IOSnafu)?;

                            let mut file =
                                File::create(&path).context(FileSnafu { path: &path })?;
                            // This is where we diff and write the output.
                            if orig.len() > 0 {
                                let diffed = process_diff(
                                    orig.as_str(),
                                    incoming.as_str(),
                                    DirectiveKind::AllowEditing,
                                );

                                // Write the file
                                file.write_all(&diffed.as_bytes()).context(IOSnafu)?;
                            } else {
                                // Write the file
                                file.write_all(&incoming.as_bytes()).context(IOSnafu)?;
                            }
                        }
                        Err(e) => {
                            // Put the original back.
                            let mut file =
                                File::create(&path).context(FileSnafu { path: &path })?;
                            file.write_all(&orig.as_bytes()).context(IOSnafu)?;

                            eprintln!("{}", e);

                            // This is as weird way to go about things.
                            ensure!(
                                result.is_ok(),
                                CompilerSnafu {
                                    description: "rustfmt failed on generated file"
                                }
                            );
                        }
                    };
                } else {
                    let mut file = File::create(&path).context(FileSnafu { path: &path })?;
                    file.write_all(&buffer.dump().as_bytes()).context(IOSnafu)?;

                    match format(&path, false) {
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
        woog: &mut WoogStore,
        module: &str,
        obj_id: Option<&Uuid>,
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
        woog: &mut WoogStore,
        module: &str,
        obj_id: Option<&Uuid>,
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

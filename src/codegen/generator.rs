//! Generator Root
//!
//!
use std::{
    fs::{self, File},
    io::prelude::*,
    path::{Path, PathBuf},
    sync::RwLock,
};

use fnv::FnvHashMap as HashMap;
use sarzak::{
    lu_dog::store::ObjectStore as LuDogStore,
    mc::{CompilerSnafu, FileSnafu, IOSnafu, Result},
    v2::domain::Domain,
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
    options::GraceConfig,
};

/// Generator Builder
///
/// This is the builder for a [`Generator`], and what the end user will be
/// interacting with to generate files. That's what this does, it generates
/// a file, given a bunch of possible inputs.
///
/// To be honest, most of the inputs are required, and it will continue to
/// evolve to meet the needs of code generation.
pub(crate) struct GeneratorBuilder<'a> {
    /// Output Path
    ///
    /// This is the path where the generated file will be written. It is inclusive
    /// of the file name to which to write.
    path: Option<PathBuf>,
    /// File Generator
    ///
    /// This is the specific code generate that will be invoked to generate
    /// code for the output file.
    generator: Option<Box<dyn FileGenerator + 'a>>,
    /// Domain
    ///
    /// This is the domain of instances from the sarzak/cuckoo model. It
    /// contains [`ObjectStore`]s for both sarzak and the drawing domains.
    domain: Option<&'a Domain>,
    /// Woog Store
    ///
    /// The woog ObjectStore. The woog domain contains types specific to code
    /// generation, and perhaps this code generator specifically. I am intending
    /// to make it generic, but Rust things are creeping in. Like mutability,
    /// and crate public.
    woog: Option<&'a mut WoogStore>,
    /// Lu Dog Store
    ///
    /// The lu_dog ObjectStore. This domain is a model of the DSL I'm calling dwarf.
    lu_dog: Option<&'a RwLock<LuDogStore>>,
    /// Grace Compiler Configuration
    ///
    /// These are the [`ConfigValue`]s to the model compiler -- the compiler's
    /// configuration options.
    config: Option<&'a GraceConfig>,
    /// Package Name
    ///
    /// The Rust package for which we are generating code. Some may call this
    /// the crate, although that's technically incorrect.
    package: Option<&'a str>,
    /// Module Name
    ///
    /// The Rust module to which we are generating code. It's not quite synonymous
    /// with a domain name. There is a 1:M relationship between domains and modules,
    /// so we need to have a module name.
    module: Option<String>,
    /// Object ID
    ///
    /// Here's an optional one, that is indeed optional. It's used by the
    /// [`DefaultStructGenerator`] to pass on the specific object for which
    /// structs will be built. I expect that there will be a [`DefaultEnumGenerator`]
    /// eventually as well.
    obj_id: Option<&'a Uuid>,
    imports: Option<&'a HashMap<String, Domain>>,
}

impl<'a> GeneratorBuilder<'a> {
    pub fn new() -> Self {
        GeneratorBuilder {
            path: None,
            generator: None,
            domain: None,
            woog: None,
            lu_dog: None,
            config: None,
            package: None,
            module: None,
            obj_id: None,
            imports: None,
        }
    }

    pub(crate) fn config(mut self, config: &'a GraceConfig) -> Self {
        self.config = Some(config);

        self
    }

    pub(crate) fn path<P: AsRef<Path>>(mut self, path: P) -> Result<Self> {
        let path = path.as_ref();

        self.path = Some(path.to_path_buf());

        Ok(self)
    }

    pub(crate) fn generator(mut self, generator: Box<dyn FileGenerator + 'a>) -> Self {
        self.generator = Some(generator);

        self
    }

    pub(crate) fn module(mut self, module: &'a str) -> Self {
        self.module = Some(module.replace("/", "::"));

        self
    }

    pub(crate) fn package(mut self, package: &'a str) -> Self {
        self.package = Some(package);

        self
    }

    pub(crate) fn domain(mut self, domain: &'a Domain) -> Self {
        self.domain = Some(domain);

        self
    }

    pub(crate) fn woog(mut self, domain: &'a mut WoogStore) -> Self {
        self.woog = Some(domain);

        self
    }

    pub(crate) fn lu_dog(mut self, domain: &'a RwLock<LuDogStore>) -> Self {
        self.lu_dog = Some(domain);

        self
    }

    pub(crate) fn obj_id(mut self, obj_id: &'a Uuid) -> Self {
        self.obj_id = Some(obj_id);

        self
    }

    pub(crate) fn imports(mut self, imports: &'a HashMap<String, Domain>) -> Self {
        self.imports = Some(imports);

        self
    }

    pub fn generate(self) -> Result<()> {
        ensure!(
            self.config.is_some(),
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

        ensure!(
            self.package.is_some(),
            CompilerSnafu {
                description: "missing package"
            }
        );

        let mut buffer = Buffer::new();
        match self.generator.unwrap().generate(
            &self.config.unwrap(),
            &self.domain.unwrap(),
            &self.woog,
            &self.lu_dog,
            &self.imports,
            self.package.unwrap(),
            self.module.unwrap().as_str(),
            self.obj_id,
            &mut buffer,
        ) {
            Ok(action) => {
                match action {
                    GenerationAction::Write => {
                        let path = self.path.unwrap();
                        let mut file = File::create(&path).context(FileSnafu {
                            description: "writing generated file".to_owned(),
                            path: &path,
                        })?;
                        file.write_all(&buffer.dump().as_bytes()).context(IOSnafu {
                            description: "writing generated file".to_owned(),
                        })?;
                    }
                    GenerationAction::FormatWrite => {
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

                            // Grab the original, formatted output.
                            let orig = fs::read_to_string(&path).context(IOSnafu {
                                description: "reading existing file".to_owned(),
                            })?;

                            // Format the generated buffer
                            let mut file = File::create(&path).context(FileSnafu {
                                description: "writing generated file for formatting".to_owned(),
                                path: &path,
                            })?;
                            file.write_all(&buffer.dump().as_bytes()).context(IOSnafu {
                                description: "writing generated file for formatting".to_owned(),
                            })?;
                            match format(&path, true) {
                                Ok(_) => {
                                    // Grab the formatted, generated output
                                    let incoming = fs::read_to_string(&path).context(IOSnafu {
                                        description: "reading generated file".to_owned(),
                                    })?;

                                    let mut file = File::create(&path).context(FileSnafu {
                                        description: "writing generated file to diff".to_owned(),
                                        path: &path,
                                    })?;
                                    // This is where we diff and write the output.
                                    if orig.len() > 0 {
                                        let diffed = process_diff(
                                            &path,
                                            orig.trim(),
                                            incoming.trim(),
                                            // Default to overwriting so that doc comments are overwritten
                                            // and not left to grow without bound.
                                            DirectiveKind::IgnoreOrig,
                                        );

                                        // Write the file
                                        file.write_all(&diffed.as_bytes()).context(IOSnafu {
                                            description: "writing generated file".to_owned(),
                                        })?;
                                    } else {
                                        // Write the file
                                        file.write_all(&incoming.as_bytes()).context(IOSnafu {
                                            description: "writing generated file".to_owned(),
                                        })?;
                                    }
                                }
                                Err(e) => {
                                    // Put the original back.
                                    let mut file = File::create(&path).context(FileSnafu {
                                        description: "writing original source file".to_owned(),
                                        path: &path,
                                    })?;
                                    file.write_all(&orig.as_bytes()).context(IOSnafu {
                                        description: "writing original file".to_owned(),
                                    })?;

                                    eprintln!("{}", e);

                                    // This is as weird way to go about things. WTF is going on here?
                                    ensure!(
                                        result.is_ok(),
                                        CompilerSnafu {
                                            description: "rustfmt failed on generated file"
                                        }
                                    );
                                }
                            };
                        } else {
                            // Path does not exist.
                            let mut file = File::create(&path).context(FileSnafu {
                                description: "writing source file".to_owned(),
                                path: &path,
                            })?;
                            file.write_all(&buffer.dump().as_bytes()).context(IOSnafu {
                                description: "writing new file".to_owned(),
                            })?;

                            match format(&path, false) {
                                Ok(_) => {}
                                Err(e) => {
                                    // Don't write garbage.
                                    File::create(&path).context(FileSnafu {
                                        description: "removing irregular source file".to_owned(),
                                        path: &path,
                                    })?;
                                    eprintln!("{}", e);
                                }
                            };
                        }
                    }
                    GenerationAction::Skip => {}
                }

                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

pub(crate) enum GenerationAction {
    FormatWrite,
    Write,
    Skip,
}

pub(crate) trait FileGenerator {
    fn generate(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        woog: &Option<&mut WoogStore>,
        lu_dog: &Option<&RwLock<LuDogStore>>,
        imports: &Option<&HashMap<String, Domain>>,
        package: &str,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<GenerationAction>;
}

/// CodeWriter
///
/// This trait is implemented for types that write code. The key element to note
/// is the `Buffer` parameter. That's a dead giveaway that the rubber is hitting
/// the road.
pub(crate) trait CodeWriter {
    fn write_code(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        woog: &Option<&mut WoogStore>,
        lu_dog: &Option<&RwLock<LuDogStore>>,
        imports: &Option<&HashMap<String, Domain>>,
        package: &str,
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
            .build_v2()
            .unwrap();
    }
}

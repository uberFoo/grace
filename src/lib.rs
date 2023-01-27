use std::{
    fs,
    path::{Path, PathBuf},
};

use sarzak::mc::ModelCompilerOptions;
use snafu::prelude::*;
use uuid::Uuid;

mod codegen;
pub mod options;
mod types;

pub use options::GraceCompilerOptions;
pub use sarzak::mc::{FileSnafu, ModelCompilerError, SarzakModelCompiler};

use codegen::{generator::GeneratorBuilder, render::RenderIdent};
use sarzak::sarzak::types::Object;
use types::{DefaultStruct, DefaultStructBuilder};

const RS_EXT: &str = "rs";
const TYPES: &str = "types";

#[derive(Default)]
pub struct ModelCompiler {}

impl SarzakModelCompiler for ModelCompiler {
    fn compile<P: AsRef<Path>>(
        &self,
        model: &sarzak::domain::Domain,
        module: &str,
        src_path: P,
        _options: Box<&dyn ModelCompilerOptions>,
        _test: bool,
    ) -> Result<(), ModelCompilerError> {
        // Generate types.rs

        // First deal with the path
        let mut types = PathBuf::from(src_path.as_ref());
        types.push(module);
        types.push(TYPES);
        fs::create_dir_all(&types).context(FileSnafu { path: &types })?;
        types.push("discard");

        // Sort the objects -- I need to figure out how to do this automagically.
        let mut objects: Vec<(&Uuid, &Object)> = model.sarzak().iter_object().collect();
        objects.sort_by(|a, b| a.1.name.cmp(&b.1.name));

        // Iterate over the objects, generating an implementation for file each.
        for (id, obj) in objects {
            types.set_file_name(obj.as_ident());
            types.set_extension(RS_EXT);

            // Here's the generation.
            GeneratorBuilder::new()
                // Where to write
                .path(&types)?
                // What to write
                .generator(
                    // Struct
                    DefaultStructBuilder::new()
                        // Definition type
                        .definition(DefaultStruct::new(&id))
                        // Store? For each type? Lame.
                        .store(&model)
                        .build()?,
                )
                .generate()?;
        }

        // // Generate macros.rs
        // let mut types = PathBuf::from(src_path.as_ref());
        // types.push(module);
        // types.push("macros.rs");

        // GeneratorBuilder::new()
        //     .path(&types)?
        //     .generate()?
        //     .generate()?;

        // // Generate store.rs
        // let mut types = PathBuf::from(src_path.as_ref());
        // types.push(module);
        // types.push("store.rs");

        // GeneratorBuilder::new()
        //     .path(&types)?
        //     .generate()?
        //     .generate()?;

        Ok(())
    }
}

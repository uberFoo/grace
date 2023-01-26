use std::path::{Path, PathBuf};

use sarzak::mc::ModelCompilerOptions;

mod buffer;
mod codegen;
pub mod options;
mod types;

pub use options::GraceCompilerOptions;
pub use sarzak::mc::{ModelCompilerError, SarzakModelCompiler};

use buffer::GeneratorBuilder;
use codegen::RenderIdent;
use types::{DefaultStructBuilder, TypeBuilder};

const RS_EXT: &str = "rs";

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
        let mut types = PathBuf::from(src_path.as_ref());
        types.push(module);
        types.push("discard");

        for (_id, obj) in model.sarzak().iter_object() {
            types.set_file_name(obj.as_ident());
            types.set_extension(RS_EXT);

            GeneratorBuilder::new()
                .path(&types)?
                .add_type(
                    TypeBuilder::new(&obj)
                        .using_struct_defn(DefaultStructBuilder::new())?
                        .build()?,
                )
                .build()?
                .generate()?;
        }

        // Generate macros.rs
        let mut types = PathBuf::from(src_path.as_ref());
        types.push(module);
        types.push("macros.rs");

        GeneratorBuilder::new().path(&types)?.build()?.generate()?;

        // Generate store.rs
        let mut types = PathBuf::from(src_path.as_ref());
        types.push(module);
        types.push("store.rs");

        GeneratorBuilder::new().path(&types)?.build()?.generate()?;

        Ok(())
    }
}

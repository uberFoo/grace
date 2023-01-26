use std::path::{Path, PathBuf};

use sarzak::mc::ModelCompilerOptions;

pub mod buffer;
pub mod options;

pub use options::GraceCompilerOptions;
pub use sarzak::mc::{ModelCompilerError, SarzakModelCompiler};

use buffer::BufferBuilder;

#[derive(Default)]
pub struct ModelCompiler {}

impl SarzakModelCompiler for ModelCompiler {
    fn compile<P: AsRef<Path>>(
        &self,
        _model: &sarzak::domain::Domain,
        module: &str,
        src_path: P,
        _options: Box<&dyn ModelCompilerOptions>,
        _test: bool,
    ) -> Result<(), ModelCompilerError> {
        // Generate types.rs
        let mut types = PathBuf::from(src_path.as_ref());
        types.push(module);
        types.push("types.rs");

        BufferBuilder::new().path(&types).build()?.write_file()?;

        // Generate macros.rs
        let mut types = PathBuf::from(src_path.as_ref());
        types.push(module);
        types.push("macros.rs");

        BufferBuilder::new().path(&types).build()?.write_file()?;

        // Generate store.rs
        let mut types = PathBuf::from(src_path.as_ref());
        types.push(module);
        types.push("store.rs");

        BufferBuilder::new().path(&types).build()?.write_file()?;

        Ok(())
    }
}

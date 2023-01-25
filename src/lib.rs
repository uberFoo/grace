use nut::sarzak::mc::{ModelCompilerError, ModelCompilerOptions, SarzakModelCompiler};

pub mod options;

pub use options::GraceCompilerOptions;

#[derive(Default)]
pub struct ModelCompiler {}

impl SarzakModelCompiler for ModelCompiler {
    fn compile(
        &self,
        _model: &nut::sarzak::SarzakModel,
        _package: &str,
        _output: &std::path::PathBuf,
        _options: Box<&dyn ModelCompilerOptions>,
        _test: bool,
    ) -> Result<(), ModelCompilerError> {
        Ok(())
    }
}

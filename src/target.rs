use sarzak::mc::{ModelCompilerError, Result};

pub(crate) mod application;
pub(crate) mod domain;
pub(crate) mod dwarf;
pub(crate) mod svm;

pub(crate) trait Target {
    /// Compile the target
    ///
    fn compile(&mut self) -> Result<usize, ModelCompilerError>;

    /// The above one is pretty obvious, but what's this for?
    fn domain(&self) -> &str;
}

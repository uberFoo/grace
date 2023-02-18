use sarzak::mc::{ModelCompilerError, Result};

pub(crate) mod application;
pub(crate) mod domain;

pub(crate) trait Target {
    /// Compile the target
    ///
    fn compile(&mut self) -> Result<(), ModelCompilerError>;

    /// The above one is pretty obvious, but what's this for?
    fn domain(&self) -> &str;
}

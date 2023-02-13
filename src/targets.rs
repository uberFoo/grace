use sarzak::mc::{ModelCompilerError, Result};

pub(crate) mod application;
pub(crate) mod domain;

pub(crate) trait Target {
    fn compile(&mut self) -> Result<(), ModelCompilerError>;
    fn domain(&self) -> &str;
}

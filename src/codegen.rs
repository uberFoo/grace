//! Things necessary for code generation
//!

pub(crate) mod buffer;
pub(crate) mod diff_engine;
pub(crate) mod generator;
pub(crate) mod render;
mod rustfmt;

use sarzak::woog::types::ObjectMethod;

// pub (crate) fn output_parameters(param: &Parameter)

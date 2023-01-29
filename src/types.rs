//! Generate Types
//!
//! This is the entry point for all type generation.

use crate::codegen::generator::CodeWriter;

pub(crate) mod default;
pub(crate) mod domain;

pub(crate) trait StructDefinition: CodeWriter {}

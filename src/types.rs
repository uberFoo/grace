//! Generate Types
//!
//! This is the entry point for all type generation.

use crate::codegen::generator::CodeWriter;

pub(crate) mod default;
pub(crate) mod domain;

/// Struct Definition Trait
///
/// This trait is implemented by types that are capable of generating a struct
/// definition.
pub(crate) trait StructDefinition: CodeWriter {}

/// Struct Implementation Trait
///
/// This trait is implemented by types that are capable of generating a struct
/// implementation.
pub(crate) trait StructImplementation: CodeWriter {}

/// Module Definition Trait
///
/// This trait is implemented by types that are capable of generationg a module
/// definition.
pub(crate) trait ModuleDefinition: CodeWriter {}

pub(crate) trait ObjectStoreDefinition: CodeWriter {}

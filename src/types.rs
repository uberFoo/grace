//! Generate Types
//!
//! This is the entry point for all type generation.

use crate::codegen::generator::CodeWriter;

pub(crate) mod default;
pub(crate) mod domain;
pub(crate) mod dwarf;
pub(crate) mod external;
pub(crate) mod null;
pub(crate) mod svm;

/// Type Imports Trait
///
/// This trait is implemented by types that can render the imports for a type.
pub(crate) trait TypeImports: CodeWriter {}

/// Type Definition Trait
///
/// This trait is implemented by types that are capable of generating a
/// definition for an enum or a struct.
pub(crate) trait TypeDefinition: CodeWriter {}

/// Type Implementation Trait
///
/// This trait is implemented by types that are capable of generating an enum or struct
/// implementation. It's basically just a container for [`MethodImplementation`]
/// implementors.
pub(crate) trait TypeImplementation: CodeWriter {}

/// Method Trait
///
/// This trait is implemented by types that are capable of generating a struct
/// method inside a struct implementation.
pub(crate) trait MethodImplementation: CodeWriter {}

/// Module Definition Trait
///
/// This trait is implemented by types that are capable of generating a module
/// definition.
pub(crate) trait ModuleDefinition: CodeWriter {}

pub(crate) trait DwarfDefinition: CodeWriter {}

pub(crate) trait ChaChaDefinition: CodeWriter {}

pub(crate) trait ObjectStoreDefinition: CodeWriter {}

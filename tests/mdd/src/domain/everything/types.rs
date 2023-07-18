//! Domain to test an Object with attributes of all types.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::everything-module-definition-file"}}}
pub mod everything;
pub mod rando_object;

pub use crate::domain::everything::everything::Everything;
pub use crate::domain::everything::rando_object::RandoObject;
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

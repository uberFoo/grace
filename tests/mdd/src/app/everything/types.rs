//! Domain to test an Object with attributes of all types.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"app::everything-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"app::everything-module-definition"}}}
pub mod everything;
pub mod rando_object;

pub use crate::app::everything::everything::Everything;
pub use crate::app::everything::rando_object::RandoObject;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

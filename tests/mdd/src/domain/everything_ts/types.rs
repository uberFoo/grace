//! Domain to test an Object with attributes of all types.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::everything_ts-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::everything_ts-module-definition"}}}
pub mod everything;
pub mod rando_object;

pub use crate::domain::everything_ts::everything::Everything;
pub use crate::domain::everything_ts::rando_object::RandoObject;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

//! Domain to test importing an Object.
//!
//! We are importing an object from the sarzak domain. We do some sick stuff importing objects...
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::imported_object_ts-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::imported_object_ts-module-definition"}}}
pub mod another_object;

pub use crate::domain::imported_object_ts::another_object::AnotherObject;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

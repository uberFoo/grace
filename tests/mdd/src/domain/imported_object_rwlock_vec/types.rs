//! Domain to test importing an Object.
//!
//! We are importing an object from the sarzak domain. We do some sick stuff importing objects...
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::imported_object_rwlock_vec-module-definition-file"}}}
pub mod another_object;

pub use crate::domain::imported_object_rwlock_vec::another_object::AnotherObject;
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

//! Domain to test importing an Object.
//!
//! We are importing an object from the sarzak domain. We do some sick stuff importing objects...
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"imported_object_domain-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"imported_object_domain-module-definition"}}}
pub mod another_object;

pub use crate::imported_object_domain::another_object::AnotherObject;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

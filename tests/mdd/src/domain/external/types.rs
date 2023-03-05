//! This model is meant to test extenral entities in grace.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::external-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::external-module-definition"}}}
pub mod nunchuck;
pub mod timestamp;

pub use crate::domain::external::nunchuck::Nunchuck;
pub use crate::domain::external::timestamp::Timestamp;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

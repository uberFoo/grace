//! This model is meant to test extenral entities in grace.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::external-module-definition-file"}}}
pub mod nunchuck;
pub mod timestamp;

pub use crate::domain::external::nunchuck::Nunchuck;
pub use crate::domain::external::timestamp::Timestamp;
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

//! This model is meant to test extenral entities in grace.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::external_ts-module-definition-file"}}}
pub mod nunchuck;
pub mod timestamp;

pub use crate::domain::external_ts::nunchuck::Nunchuck;
pub use crate::domain::external_ts::timestamp::Timestamp;
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

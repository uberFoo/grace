//! This model is meant to test extenral entities in grace.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::external_ts-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::external_ts-module-definition"}}}
pub mod nunchuck;
pub mod timestamp;

pub use crate::domain::external_ts::nunchuck::Nunchuck;
pub use crate::domain::external_ts::timestamp::Timestamp;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

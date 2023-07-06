//! This model is meant to test extenral entities in grace.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::external_rwlock_vec-module-definition-file"}}}
pub mod nunchuck;
pub mod timestamp;

pub use crate::domain::external_rwlock_vec::nunchuck::Nunchuck;
pub use crate::domain::external_rwlock_vec::timestamp::Timestamp;
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

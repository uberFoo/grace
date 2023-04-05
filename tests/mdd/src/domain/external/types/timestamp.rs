//! Timestamp External Entity
//!
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"timestamp-ee-use-statements"}}}
use crate::domain::external::store::ObjectStore as ExternalStore;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"timestamp-ee-documentation"}}}
/// 🐶 {"external_entity": {"ctor":"now", "name":"SystemTime", "path": "std::time"}}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"timestamp-ee-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Timestamp {
    pub id: Uuid,
    inner: SystemTime,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"timestamp-ee-impl"}}}
impl Timestamp {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"timestamp-struct-impl-new"}}}
    /// Create a new instance of the external entity, 'SystemTime', wrapped in an Timestamp.
    pub fn now(inner: SystemTime, store: &mut ExternalStore) -> Timestamp {
        let id = Uuid::new_v4();
        let new = Timestamp {
            id: id,
            inner: inner,
        };
        store.inter_timestamp(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

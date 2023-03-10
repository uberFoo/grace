//! Timestamp External Entity
//!
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"timestamp-ee-use-statements"}}}
use crate::domain::external::store::ObjectStore as ExternalStore;
use crate::domain::external::UUID_NS;
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
    ext_value: SystemTime,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"timestamp-ee-impl"}}}
impl Timestamp {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"timestamp-struct-impl-new"}}}
    /// Create a new instance of the external entity,  'SystemTime', wrapped in an Timestamp.
    pub fn now(ext_value: SystemTime, store: &mut ExternalStore) -> Timestamp {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}", ext_value).as_bytes());
        let new = Timestamp {
            ext_value: ext_value,
            id: id,
        };
        store.inter_timestamp(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

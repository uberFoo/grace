//! Timestamp External Entity
//!
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"timestamp-ee-use-statements"}}}
use crate::domain::external_ts::store::ObjectStore as ExternalTsStore;
use crate::domain::external_ts::UUID_NS;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"timestamp-ee-documentation"}}}
/// 🐶 {"external_entity": {"ctor":"now", "name":"SystemTime", "path": "std::time"}}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"timestamp-ee-definition"}}}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Timestamp {
    pub id: Uuid,
    inner: SystemTime,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"timestamp-ee-impl"}}}
impl Timestamp {
    pub fn new(store: &mut ExternalTsStore) -> Timestamp {
        let inner = SystemTime::now();
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}", inner).as_bytes());
        let new = Timestamp {
            id: id,
            inner: inner,
        };
        store.inter_timestamp(new.clone());
        new
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

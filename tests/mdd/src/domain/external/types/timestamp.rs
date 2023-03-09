//! Timestamp External Entity
//!
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"timestamp-ee-use-statements"}}}
use std::time::SystemTime;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"timestamp-ee-documentation"}}}
/// 🐶 {"external_entity": {"ctor":"now", "name":"SystemTime", "path": "std::time"}}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"timestamp-ee-definition"}}}
pub struct Timestamp {
    pub id: Uuid,
    pub value: SystemTime,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"timestamp-ee-impl"}}}
impl Timestamp {
    pub fn new(store: &mut ExternalStore) -> Timestamp {
        let id = Uuid::new_v5(&UUID_NS, format!("",).as_bytes());
        let new = Timestamp { id: id };
        store.inter_timestamp(new.clone());
        new
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

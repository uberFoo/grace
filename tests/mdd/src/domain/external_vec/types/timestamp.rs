//! Timestamp External Entity
//!
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"timestamp-ee-use-statements"}}}
use crate::domain::external_vec::store::ObjectStore as ExternalVecStore;
use crate::domain::external_vec::UUID_NS;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"timestamp-ee-documentation"}}}
/// 🐶 {"external_entity": {"ctor":"now", "name":"SystemTime", "path": "std::time"}}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"timestamp-ee-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Timestamp {
    pub id: usize,
    inner: SystemTime,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"timestamp-ee-impl"}}}
impl Timestamp {
    pub fn new(store: &mut ExternalVecStore) -> std::rc::Rc<std::cell::RefCell<Timestamp>> {
        store.inter_timestamp(|id| {
            std::rc::Rc::new(std::cell::RefCell::new(Timestamp {
                id,
                inner: SystemTime::now(),
            }))
        })
    }
}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"timestamp-implementation"}}}
impl PartialEq for Timestamp {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

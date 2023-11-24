// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"nunchuck-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::external_rwlock_vec::types::timestamp::Timestamp;
use serde::{Deserialize, Serialize};

use crate::domain::external_rwlock_vec::store::ObjectStore as ExternalRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-struct-documentation"}}}
/// Bruce Lee
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Nunchuck {
    pub id: usize,
    /// R1: [`Nunchuck`] 'needs a' [`Timestamp`]
    pub time: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-implementation"}}}
impl Nunchuck {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-struct-impl-new"}}}
    /// Inter a new 'Nunchuck' in the store, and return it's `id`.
    pub fn new(
        time: &Arc<RwLock<Timestamp>>,
        store: &mut ExternalRwlockVecStore,
    ) -> Arc<RwLock<Nunchuck>> {
        store.inter_nunchuck(|id| {
            Arc::new(RwLock::new(Nunchuck {
                id,
                time: time.read().unwrap().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-struct-impl-nav-forward-to-time"}}}
    /// Navigate to [`Timestamp`] across R1(1-*)
    pub fn r1_timestamp<'a>(
        &'a self,
        store: &'a ExternalRwlockVecStore,
    ) -> Vec<Arc<RwLock<Timestamp>>> {
        vec![store.exhume_timestamp(&self.time).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-implementation"}}}
impl PartialEq for Nunchuck {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

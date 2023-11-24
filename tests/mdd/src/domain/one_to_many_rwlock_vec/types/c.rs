// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"c-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::one_to_many_rwlock_vec::types::referent::Referent;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_many_rwlock_vec::store::ObjectStore as OneToManyRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-documentation"}}}
/// This is the [`Referrent`] side of a 1-Mc
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct C {
    pub id: usize,
    pub jackpot: f64,
    /// R3: [`C`] 'points at' [`Referent`]
    pub ptr: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-implementation"}}}
impl C {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-impl-new"}}}
    /// Inter a new 'C' in the store, and return it's `id`.
    pub fn new(
        jackpot: f64,
        ptr: &Arc<RwLock<Referent>>,
        store: &mut OneToManyRwlockVecStore,
    ) -> Arc<RwLock<C>> {
        store.inter_c(|id| {
            Arc::new(RwLock::new(C {
                id,
                jackpot,
                ptr: ptr.read().unwrap().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-impl-nav-forward-to-ptr"}}}
    /// Navigate to [`Referent`] across R3(1-*)
    pub fn r3_referent<'a>(
        &'a self,
        store: &'a OneToManyRwlockVecStore,
    ) -> Vec<Arc<RwLock<Referent>>> {
        vec![store.exhume_referent(&self.ptr).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-implementation"}}}
impl PartialEq for C {
    fn eq(&self, other: &Self) -> bool {
        self.jackpot == other.jackpot && self.ptr == other.ptr
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

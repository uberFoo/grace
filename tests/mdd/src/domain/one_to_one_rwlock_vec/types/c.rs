// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"c-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::one_to_one_rwlock_vec::types::referent::Referent;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_one_rwlock_vec::store::ObjectStore as OneToOneRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-documentation"}}}
/// C: Referrer to [`Referent`] Bi-Conditional
///
/// This will be an interesting one to translate. Hopefully not too gnarly.🤘
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct C {
    pub id: usize,
    pub like_water: f64,
    /// R3: [`C`] 'points at' [`Referent`]
    pub ptr: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-implementation"}}}
impl C {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-impl-new"}}}
    /// Inter a new 'C' in the store, and return it's `id`.
    pub fn new(
        like_water: f64,
        ptr: Option<&Arc<RwLock<Referent>>>,
        store: &mut OneToOneRwlockVecStore,
    ) -> Arc<RwLock<C>> {
        store.inter_c(|id| {
            Arc::new(RwLock::new(C {
                id,
                like_water,
                ptr: ptr.map(|referent| referent.read().unwrap().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-impl-nav-forward-cond-to-ptr"}}}
    /// Navigate to [`Referent`] across R3(1-*c)
    pub fn r3_referent<'a>(
        &'a self,
        store: &'a OneToOneRwlockVecStore,
    ) -> Vec<Arc<RwLock<Referent>>> {
        match self.ptr {
            Some(ref ptr) => vec![store.exhume_referent(&ptr).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-implementation"}}}
impl PartialEq for C {
    fn eq(&self, other: &Self) -> bool {
        self.like_water == other.like_water && self.ptr == other.ptr
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

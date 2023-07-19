// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"d-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::one_to_many_rwlock_vec::types::referent::Referent;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_many_rwlock_vec::store::ObjectStore as OneToManyRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-struct-documentation"}}}
/// Just an unassuming D
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct D {
    pub appellation: String,
    pub id: usize,
    /// R4: [`D`] 'points at' [`Referent`]
    pub ptr: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-implementation"}}}
impl D {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-struct-impl-new"}}}
    /// Inter a new 'D' in the store, and return it's `id`.
    pub fn new(
        appellation: String,
        ptr: Option<&Arc<RwLock<Referent>>>,
        store: &mut OneToManyRwlockVecStore,
    ) -> Arc<RwLock<D>> {
        store.inter_d(|id| {
            Arc::new(RwLock::new(D {
                appellation: appellation.to_owned(),
                id,
                ptr: ptr.map(|referent| referent.read().unwrap().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-struct-impl-nav-forward-cond-to-ptr"}}}
    /// Navigate to [`Referent`] across R4(1-*c)
    pub fn r4_referent<'a>(
        &'a self,
        store: &'a OneToManyRwlockVecStore,
    ) -> Vec<Arc<RwLock<Referent>>> {
        span!("r4_referent");
        match self.ptr {
            Some(ref ptr) => vec![store.exhume_referent(&ptr).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-implementation"}}}
impl PartialEq for D {
    fn eq(&self, other: &Self) -> bool {
        self.appellation == other.appellation && self.ptr == other.ptr
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

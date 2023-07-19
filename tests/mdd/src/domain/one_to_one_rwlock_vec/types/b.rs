// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"b-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::one_to_one_rwlock_vec::types::referent::Referent;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_one_rwlock_vec::store::ObjectStore as OneToOneRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-documentation"}}}
/// B: Referrer Unconditional to Referent
///
/// This is a plain Jayne 😉 1-1 relationship, where this guy is formalizing.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct B {
    pub bit: bool,
    pub id: usize,
    /// R2: [`B`] 'points at' [`Referent`]
    pub ptr: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-implementation"}}}
impl B {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-impl-new"}}}
    /// Inter a new 'B' in the store, and return it's `id`.
    pub fn new(
        bit: bool,
        ptr: &Arc<RwLock<Referent>>,
        store: &mut OneToOneRwlockVecStore,
    ) -> Arc<RwLock<B>> {
        store.inter_b(|id| {
            Arc::new(RwLock::new(B {
                bit,
                id,
                ptr: ptr.read().unwrap().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-impl-nav-forward-to-ptr"}}}
    /// Navigate to [`Referent`] across R2(1-*)
    pub fn r2_referent<'a>(
        &'a self,
        store: &'a OneToOneRwlockVecStore,
    ) -> Vec<Arc<RwLock<Referent>>> {
        span!("r2_referent");
        vec![store.exhume_referent(&self.ptr).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-implementation"}}}
impl PartialEq for B {
    fn eq(&self, other: &Self) -> bool {
        self.bit == other.bit && self.ptr == other.ptr
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"a-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::one_to_many_rwlock_vec::types::referent::Referent;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_many_rwlock_vec::store::ObjectStore as OneToManyRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-struct-documentation"}}}
/// This is the [`Referrer`] side of a 1-M relationship
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct A {
    pub id: usize,
    pub name: String,
    /// R1: [`A`] 'points at' [`Referent`]
    pub ptr: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-implementation"}}}
impl A {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-struct-impl-new"}}}
    /// Inter a new 'A' in the store, and return it's `id`.
    pub fn new(
        name: String,
        ptr: &Arc<RwLock<Referent>>,
        store: &mut OneToManyRwlockVecStore,
    ) -> Arc<RwLock<A>> {
        store.inter_a(|id| {
            Arc::new(RwLock::new(A {
                id,
                name: name.to_owned(),
                ptr: ptr.read().unwrap().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-struct-impl-nav-forward-to-ptr"}}}
    /// Navigate to [`Referent`] across R1(1-*)
    pub fn r1_referent<'a>(
        &'a self,
        store: &'a OneToManyRwlockVecStore,
    ) -> Vec<Arc<RwLock<Referent>>> {
        vec![store.exhume_referent(&self.ptr).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-implementation"}}}
impl PartialEq for A {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.ptr == other.ptr
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

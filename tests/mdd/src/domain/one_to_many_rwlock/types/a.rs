// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"a-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::one_to_many_rwlock::types::referent::Referent;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_many_rwlock::store::ObjectStore as OneToManyRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-struct-documentation"}}}
/// This is the [`Referrer`] side of a 1-M relationship
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-struct-definition"}}}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct A {
    pub id: Uuid,
    pub name: String,
    /// R1: [`A`] 'points at' [`Referent`]
    pub ptr: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-implementation"}}}
impl A {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-struct-impl-new"}}}
    /// Inter a new 'A' in the store, and return it's `id`.
    pub fn new(
        name: String,
        ptr: &Arc<RwLock<Referent>>,
        store: &mut OneToManyRwlockStore,
    ) -> Arc<RwLock<A>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(A {
            id,
            name,
            ptr: ptr.read().unwrap().id,
        }));
        store.inter_a(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-struct-impl-nav-forward-to-ptr"}}}
    /// Navigate to [`Referent`] across R1(1-*)
    pub fn r1_referent<'a>(
        &'a self,
        store: &'a OneToManyRwlockStore,
    ) -> Vec<Arc<RwLock<Referent>>> {
        vec![store.exhume_referent(&self.ptr).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

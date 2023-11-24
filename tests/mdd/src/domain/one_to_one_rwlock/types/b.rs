// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"b-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::one_to_one_rwlock::types::referent::Referent;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_one_rwlock::store::ObjectStore as OneToOneRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-documentation"}}}
/// B: Referrer Unconditional to Referent
///
/// This is a plain Jayne 😉 1-1 relationship, where this guy is formalizing.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-definition"}}}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct B {
    pub bit: bool,
    pub id: Uuid,
    /// R2: [`B`] 'points at' [`Referent`]
    pub ptr: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-implementation"}}}
impl B {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-impl-new"}}}
    /// Inter a new 'B' in the store, and return it's `id`.
    pub fn new(
        bit: bool,
        ptr: &Arc<RwLock<Referent>>,
        store: &mut OneToOneRwlockStore,
    ) -> Arc<RwLock<B>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(B {
            bit,
            id,
            ptr: ptr.read().unwrap().id,
        }));
        store.inter_b(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-impl-nav-forward-to-ptr"}}}
    /// Navigate to [`Referent`] across R2(1-*)
    pub fn r2_referent<'a>(&'a self, store: &'a OneToOneRwlockStore) -> Vec<Arc<RwLock<Referent>>> {
        vec![store.exhume_referent(&self.ptr).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

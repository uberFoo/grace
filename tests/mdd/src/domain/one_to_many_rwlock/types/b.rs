// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"b-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::one_to_many_rwlock::types::referent::Referent;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_many_rwlock::store::ObjectStore as OneToManyRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-documentation"}}}
/// Connected to TGT via _R2_.
///
/// This is for testing a 1c-M relationship.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-definition"}}}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct B {
    pub baz: String,
    pub id: Uuid,
    /// R2: [`B`] 'points at' [`Referent`]
    pub ptr: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-implementation"}}}
impl B {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-impl-new"}}}
    /// Inter a new 'B' in the store, and return it's `id`.
    pub fn new(
        baz: String,
        ptr: Option<&Arc<RwLock<Referent>>>,
        store: &mut OneToManyRwlockStore,
    ) -> Arc<RwLock<B>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(B {
            baz,
            id,
            ptr: ptr.map(|referent| referent.read().unwrap().id),
        }));
        store.inter_b(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-impl-nav-forward-cond-to-ptr"}}}
    /// Navigate to [`Referent`] across R2(1-*c)
    pub fn r2_referent<'a>(
        &'a self,
        store: &'a OneToManyRwlockStore,
    ) -> Vec<Arc<RwLock<Referent>>> {
        match self.ptr {
            Some(ref ptr) => vec![store.exhume_referent(&ptr).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

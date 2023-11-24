// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"d-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::one_to_many_rwlock::types::referent::Referent;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_many_rwlock::store::ObjectStore as OneToManyRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-struct-documentation"}}}
/// Just an unassuming D
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-struct-definition"}}}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct D {
    pub appellation: String,
    pub id: Uuid,
    /// R4: [`D`] 'points at' [`Referent`]
    pub ptr: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-implementation"}}}
impl D {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-struct-impl-new"}}}
    /// Inter a new 'D' in the store, and return it's `id`.
    pub fn new(
        appellation: String,
        ptr: Option<&Arc<RwLock<Referent>>>,
        store: &mut OneToManyRwlockStore,
    ) -> Arc<RwLock<D>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(D {
            appellation,
            id,
            ptr: ptr.map(|referent| referent.read().unwrap().id),
        }));
        store.inter_d(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-struct-impl-nav-forward-cond-to-ptr"}}}
    /// Navigate to [`Referent`] across R4(1-*c)
    pub fn r4_referent<'a>(
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

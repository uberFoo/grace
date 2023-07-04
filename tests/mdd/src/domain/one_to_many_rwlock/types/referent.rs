// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"referent-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::one_to_many_rwlock::types::a::A;
use crate::domain::one_to_many_rwlock::types::b::B;
use crate::domain::one_to_many_rwlock::types::c::C;
use crate::domain::one_to_many_rwlock::types::d::D;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_many_rwlock::store::ObjectStore as OneToManyRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-documentation"}}}
/// The object of so many relationships
///
/// I’m related to stuff.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Referent {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-implementation"}}}
impl Referent {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-new"}}}
    /// Inter a new 'Referent' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut OneToManyRwlockStore) -> Arc<RwLock<Referent>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Referent { id, name }));
        store.inter_referent(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-1_M-to-a"}}}
    /// Navigate to [`A`] across R1(1-M)
    pub fn r1_a<'a>(&'a self, store: &'a OneToManyRwlockStore) -> Vec<Arc<RwLock<A>>> {
        span!("r1_a");
        store
            .iter_a()
            .filter(|a| a.read().unwrap().ptr == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-1_Mc-to-b"}}}
    /// Navigate to [`B`] across R2(1-Mc)
    pub fn r2_b<'a>(&'a self, store: &'a OneToManyRwlockStore) -> Vec<Arc<RwLock<B>>> {
        span!("r2_b");
        store
            .iter_b()
            .filter(|b| b.read().unwrap().ptr == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-1_M-to-c"}}}
    /// Navigate to [`C`] across R3(1-M)
    pub fn r3_c<'a>(&'a self, store: &'a OneToManyRwlockStore) -> Vec<Arc<RwLock<C>>> {
        span!("r3_c");
        store
            .iter_c()
            .filter(|c| c.read().unwrap().ptr == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-1_Mc-to-d"}}}
    /// Navigate to [`D`] across R4(1-Mc)
    pub fn r4_d<'a>(&'a self, store: &'a OneToManyRwlockStore) -> Vec<Arc<RwLock<D>>> {
        span!("r4_d");
        store
            .iter_d()
            .filter(|d| d.read().unwrap().ptr == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

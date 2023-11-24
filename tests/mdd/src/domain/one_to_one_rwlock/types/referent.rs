// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"referent-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::one_to_one_rwlock::types::a::A;
use crate::domain::one_to_one_rwlock::types::b::B;
use crate::domain::one_to_one_rwlock::types::c::C;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_one_rwlock::store::ObjectStore as OneToOneRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-documentation"}}}
/// The target of our relationship tests.
///
/// It is conditionally related to [`OneToOneConditional`] across _R2_, and it is unconditionally
///  related to [`OneToOneUnconditional`] across _R1_.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-definition"}}}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Referent {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-implementation"}}}
impl Referent {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-new"}}}
    /// Inter a new 'Referent' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut OneToOneRwlockStore) -> Arc<RwLock<Referent>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Referent { id, name }));
        store.inter_referent(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-cond-to-a"}}}
    /// Navigate to [`A`] across R1(1-1c)
    pub fn r1c_a<'a>(&'a self, store: &'a OneToOneRwlockStore) -> Vec<Arc<RwLock<A>>> {
        let a = store.iter_a().find(|a| a.read().unwrap().ptr == self.id);
        match a {
            Some(ref a) => vec![a.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-one-to-b"}}}
    /// Navigate to [`B`] across R2(1-1)
    pub fn r2_b<'a>(&'a self, store: &'a OneToOneRwlockStore) -> Vec<Arc<RwLock<B>>> {
        vec![store
            .iter_b()
            .find(|b| b.read().unwrap().ptr == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-one-bi-cond-to-c"}}}
    /// Navigate to [`C`] across R3(1c-1c)
    pub fn r3c_c<'a>(&'a self, store: &'a OneToOneRwlockStore) -> Vec<Arc<RwLock<C>>> {
        let c = store
            .iter_c()
            .find(|c| c.read().unwrap().ptr == Some(self.id));
        match c {
            Some(ref c) => vec![c.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

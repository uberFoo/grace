// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"c-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::one_to_many_rwlock::types::referent::Referent;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_many_rwlock::store::ObjectStore as OneToManyRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-documentation"}}}
/// This is the [`Referrent`] side of a 1-Mc
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-definition"}}}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct C {
    pub id: Uuid,
    pub jackpot: f64,
    /// R3: [`C`] 'points at' [`Referent`]
    pub ptr: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-implementation"}}}
impl C {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-impl-new"}}}
    /// Inter a new 'C' in the store, and return it's `id`.
    pub fn new(
        jackpot: f64,
        ptr: &Arc<RwLock<Referent>>,
        store: &mut OneToManyRwlockStore,
    ) -> Arc<RwLock<C>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(C {
            id,
            jackpot,
            ptr: ptr.read().unwrap().id,
        }));
        store.inter_c(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-impl-nav-forward-to-ptr"}}}
    /// Navigate to [`Referent`] across R3(1-*)
    pub fn r3_referent<'a>(
        &'a self,
        store: &'a OneToManyRwlockStore,
    ) -> Vec<Arc<RwLock<Referent>>> {
        span!("r3_referent");
        vec![store.exhume_referent(&self.ptr).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"c-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::one_to_many_domain::UUID_NS;

// Referrer imports
use crate::one_to_many_domain::types::referent::Referent;

use crate::one_to_many_domain::store::ObjectStore as OneToManyDomainStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-documentation"}}}
/// This is the [`Referrent`] side of a 1-Mc
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct C {
    pub id: Uuid,
    pub jackpot: f64,
    /// R3: [`C`] 'points at' [`Referent`]
    pub ptr: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-implementation"}}}
impl C {
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"c-struct-impl-new"}}}
    /// Inter a new C in the store, and return it's `id`.
    pub fn new(jackpot: f64, ptr: &Referent, store: &mut OneToManyDomainStore) -> C {
        let id = Uuid::new_v5(&UUID_NS, format!("{}:{:?}", jackpot, ptr).as_bytes());
        let new = C {
            jackpot: jackpot,
            ptr: ptr.id,
            id,
        };
        store.inter_c(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"c-struct-impl-nav-forward-to-ptr"}}}
    /// Navigate to [`Referent`] across R3(1-?)
    pub fn referent<'a>(&'a self, store: &'a OneToManyDomainStore) -> Vec<&Referent> {
        vec![store.exhume_referent(&self.ptr).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

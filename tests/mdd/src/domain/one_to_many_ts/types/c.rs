// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"c-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-use-statements"}}}
use uuid::Uuid;

use crate::domain::one_to_many_ts::types::referent::Referent;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_many_ts::store::ObjectStore as OneToManyTsStore;
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
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-impl-new_"}}}
    /// Inter a new 'C' in the store, and return it's `id`.
    pub fn new(jackpot: f64, ptr: &Referent, store: &mut OneToManyTsStore) -> C {
        let id = Uuid::new_v4();
        let new = C {
            id,
            jackpot,
            ptr: ptr.id,
        };
        store.inter_c(new.clone());
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-impl-new"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-impl-new_"}}}
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-impl-nav-forward-to-ptr"}}}
    /// Navigate to [`Referent`] across R3(1-*)
    pub fn r3_referent<'a>(&'a self, store: &'a OneToManyTsStore) -> Vec<&Referent> {
        vec![store.exhume_referent(&self.ptr).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

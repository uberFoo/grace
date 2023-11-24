// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"c-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::domain::one_to_many_vec::types::referent::Referent;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_many_vec::store::ObjectStore as OneToManyVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-documentation"}}}
/// This is the [`Referrent`] side of a 1-Mc
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct C {
    pub id: usize,
    pub jackpot: f64,
    /// R3: [`C`] 'points at' [`Referent`]
    pub ptr: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-implementation"}}}
impl C {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-impl-new"}}}
    /// Inter a new 'C' in the store, and return it's `id`.
    pub fn new(
        jackpot: f64,
        ptr: &Rc<RefCell<Referent>>,
        store: &mut OneToManyVecStore,
    ) -> Rc<RefCell<C>> {
        store.inter_c(|id| {
            Rc::new(RefCell::new(C {
                id,
                jackpot,
                ptr: ptr.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-impl-nav-forward-to-ptr"}}}
    /// Navigate to [`Referent`] across R3(1-*)
    pub fn r3_referent<'a>(&'a self, store: &'a OneToManyVecStore) -> Vec<Rc<RefCell<Referent>>> {
        vec![store.exhume_referent(&self.ptr).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-implementation"}}}
impl PartialEq for C {
    fn eq(&self, other: &Self) -> bool {
        self.jackpot == other.jackpot && self.ptr == other.ptr
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"referent-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::one_to_many_vec::types::a::A;
use crate::domain::one_to_many_vec::types::b::B;
use crate::domain::one_to_many_vec::types::c::C;
use crate::domain::one_to_many_vec::types::d::D;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_many_vec::store::ObjectStore as OneToManyVecStore;
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
    pub id: usize,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-implementation"}}}
impl Referent {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-new"}}}
    /// Inter a new 'Referent' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut OneToManyVecStore) -> Rc<RefCell<Referent>> {
        store.inter_referent(|id| {
            Rc::new(RefCell::new(Referent {
                id,
                name: name.to_owned(),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-1_M-to-a"}}}
    /// Navigate to [`A`] across R1(1-M)
    pub fn r1_a<'a>(&'a self, store: &'a OneToManyVecStore) -> Vec<Rc<RefCell<A>>> {
        span!("r1_a");
        store
            .iter_a()
            .filter(|a| a.borrow().ptr == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-1_Mc-to-b"}}}
    /// Navigate to [`B`] across R2(1-Mc)
    pub fn r2_b<'a>(&'a self, store: &'a OneToManyVecStore) -> Vec<Rc<RefCell<B>>> {
        span!("r2_b");
        store
            .iter_b()
            .filter(|b| b.borrow().ptr == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-1_M-to-c"}}}
    /// Navigate to [`C`] across R3(1-M)
    pub fn r3_c<'a>(&'a self, store: &'a OneToManyVecStore) -> Vec<Rc<RefCell<C>>> {
        span!("r3_c");
        store
            .iter_c()
            .filter(|c| c.borrow().ptr == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-1_Mc-to-d"}}}
    /// Navigate to [`D`] across R4(1-Mc)
    pub fn r4_d<'a>(&'a self, store: &'a OneToManyVecStore) -> Vec<Rc<RefCell<D>>> {
        span!("r4_d");
        store
            .iter_d()
            .filter(|d| d.borrow().ptr == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

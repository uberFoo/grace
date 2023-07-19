// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"referent-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::one_to_one_vec::types::a::A;
use crate::domain::one_to_one_vec::types::b::B;
use crate::domain::one_to_one_vec::types::c::C;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_one_vec::store::ObjectStore as OneToOneVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-documentation"}}}
/// The target of our relationship tests.
///
/// It is conditionally related to [`OneToOneConditional`] across _R2_, and it is unconditionally
///  related to [`OneToOneUnconditional`] across _R1_.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Referent {
    pub id: usize,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-implementation"}}}
impl Referent {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-new"}}}
    /// Inter a new 'Referent' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut OneToOneVecStore) -> Rc<RefCell<Referent>> {
        store.inter_referent(|id| {
            Rc::new(RefCell::new(Referent {
                id,
                name: name.to_owned(),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-cond-to-a"}}}
    /// Navigate to [`A`] across R1(1-1c)
    pub fn r1c_a<'a>(&'a self, store: &'a OneToOneVecStore) -> Vec<Rc<RefCell<A>>> {
        span!("r1_a");
        let a = store.iter_a().find(|a| a.borrow().ptr == self.id);
        match a {
            Some(ref a) => vec![a.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-one-to-b"}}}
    /// Navigate to [`B`] across R2(1-1)
    pub fn r2_b<'a>(&'a self, store: &'a OneToOneVecStore) -> Vec<Rc<RefCell<B>>> {
        span!("r2_b");
        vec![store.iter_b().find(|b| b.borrow().ptr == self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-one-bi-cond-to-c"}}}
    /// Navigate to [`C`] across R3(1c-1c)
    pub fn r3c_c<'a>(&'a self, store: &'a OneToOneVecStore) -> Vec<Rc<RefCell<C>>> {
        span!("r3_c");
        let c = store.iter_c().find(|c| c.borrow().ptr == Some(self.id));
        match c {
            Some(ref c) => vec![c.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-implementation"}}}
impl PartialEq for Referent {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

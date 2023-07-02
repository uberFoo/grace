// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"a-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::one_to_one_vec::types::referent::Referent;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_one_vec::store::ObjectStore as OneToOneVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-struct-documentation"}}}
/// A: Referrer with Conditional [`Referent`]
///
/// This type is related to the [`Referent`] across a conditional relationship. This is 1-1c
/// , and given that I am the referrer, I have the referential attribute/I am formalizing the
///  relationship. I think I prefer the latter language, but the former is very descriptive.
/// ..
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct A {
    pub id: usize,
    pub number: i64,
    /// R1: [`A`] 'points at' [`Referent`]
    pub ptr: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-implementation"}}}
impl A {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-struct-impl-new"}}}
    /// Inter a new 'A' in the store, and return it's `id`.
    pub fn new(
        number: i64,
        ptr: &Rc<RefCell<Referent>>,
        store: &mut OneToOneVecStore,
    ) -> Rc<RefCell<A>> {
        store.inter_a(|id| {
            Rc::new(RefCell::new(A {
                id,
                number,
                ptr: ptr.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-struct-impl-nav-forward-to-ptr"}}}
    /// Navigate to [`Referent`] across R1(1-*)
    pub fn r1_referent<'a>(&'a self, store: &'a OneToOneVecStore) -> Vec<Rc<RefCell<Referent>>> {
        span!("r1_referent");
        vec![store.exhume_referent(&self.ptr).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

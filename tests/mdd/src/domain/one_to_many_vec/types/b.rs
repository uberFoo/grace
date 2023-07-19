// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"b-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::one_to_many_vec::types::referent::Referent;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_many_vec::store::ObjectStore as OneToManyVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-documentation"}}}
/// Connected to TGT via _R2_.
///
/// This is for testing a 1c-M relationship.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct B {
    pub baz: String,
    pub id: usize,
    /// R2: [`B`] 'points at' [`Referent`]
    pub ptr: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-implementation"}}}
impl B {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-impl-new"}}}
    /// Inter a new 'B' in the store, and return it's `id`.
    pub fn new(
        baz: String,
        ptr: Option<&Rc<RefCell<Referent>>>,
        store: &mut OneToManyVecStore,
    ) -> Rc<RefCell<B>> {
        store.inter_b(|id| {
            Rc::new(RefCell::new(B {
                baz: baz.to_owned(),
                id,
                ptr: ptr.map(|referent| referent.borrow().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-impl-nav-forward-cond-to-ptr"}}}
    /// Navigate to [`Referent`] across R2(1-*c)
    pub fn r2_referent<'a>(&'a self, store: &'a OneToManyVecStore) -> Vec<Rc<RefCell<Referent>>> {
        span!("r2_referent");
        match self.ptr {
            Some(ref ptr) => vec![store.exhume_referent(&ptr).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-implementation"}}}
impl PartialEq for B {
    fn eq(&self, other: &Self) -> bool {
        self.baz == other.baz && self.ptr == other.ptr
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

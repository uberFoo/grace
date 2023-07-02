// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"d-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::one_to_many_vec::types::referent::Referent;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_many_vec::store::ObjectStore as OneToManyVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-struct-documentation"}}}
/// Just an unassuming D
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct D {
    pub appellation: String,
    pub id: usize,
    /// R4: [`D`] 'points at' [`Referent`]
    pub ptr: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-implementation"}}}
impl D {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-struct-impl-new"}}}
    /// Inter a new 'D' in the store, and return it's `id`.
    pub fn new(
        appellation: String,
        ptr: Option<&Rc<RefCell<Referent>>>,
        store: &mut OneToManyVecStore,
    ) -> Rc<RefCell<D>> {
        store.inter_d(|id| {
            Rc::new(RefCell::new(D {
                appellation: appellation.to_owned(),
                id,
                ptr: ptr.map(|referent| referent.borrow().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-struct-impl-nav-forward-cond-to-ptr"}}}
    /// Navigate to [`Referent`] across R4(1-*c)
    pub fn r4_referent<'a>(&'a self, store: &'a OneToManyVecStore) -> Vec<Rc<RefCell<Referent>>> {
        span!("r4_referent");
        match self.ptr {
            Some(ref ptr) => vec![store.exhume_referent(&ptr).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

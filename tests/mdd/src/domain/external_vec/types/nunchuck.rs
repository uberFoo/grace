// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"nunchuck-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::external_vec::types::timestamp::Timestamp;
use serde::{Deserialize, Serialize};

use crate::domain::external_vec::store::ObjectStore as ExternalVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-struct-documentation"}}}
/// Bruce Lee
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Nunchuck {
    pub id: usize,
    /// R1: [`Nunchuck`] 'needs a' [`Timestamp`]
    pub time: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-implementation"}}}
impl Nunchuck {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-struct-impl-new"}}}
    /// Inter a new 'Nunchuck' in the store, and return it's `id`.
    pub fn new(
        time: &Rc<RefCell<Timestamp>>,
        store: &mut ExternalVecStore,
    ) -> Rc<RefCell<Nunchuck>> {
        store.inter_nunchuck(|id| {
            Rc::new(RefCell::new(Nunchuck {
                id,
                time: time.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-struct-impl-nav-forward-to-time"}}}
    /// Navigate to [`Timestamp`] across R1(1-*)
    pub fn r1_timestamp<'a>(&'a self, store: &'a ExternalVecStore) -> Vec<Rc<RefCell<Timestamp>>> {
        span!("r1_timestamp");
        vec![store.exhume_timestamp(&self.time).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-implementation"}}}
impl PartialEq for Nunchuck {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

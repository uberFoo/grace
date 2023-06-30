// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"henry-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"henry-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::isa_vec::types::simple_subtype_a::SimpleSubtypeA;
use serde::{Deserialize, Serialize};

use crate::domain::isa_vec::store::ObjectStore as IsaVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"henry-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Henry {
    pub id: usize,
    pub last_name: String,
    /// R3: [`Henry`] 'foo' [`SimpleSubtypeA`]
    pub bar: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"henry-implementation"}}}
impl Henry {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"henry-struct-impl-new"}}}
    /// Inter a new 'Henry' in the store, and return it's `id`.
    pub fn new(
        last_name: String,
        bar: &Rc<RefCell<SimpleSubtypeA>>,
        store: &mut IsaVecStore,
    ) -> Rc<RefCell<Henry>> {
        store.inter_henry(|id| {
            Rc::new(RefCell::new(Henry {
                id,
                last_name: last_name.to_owned(),
                bar: bar.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"henry-struct-impl-nav-forward-to-bar"}}}
    /// Navigate to [`SimpleSubtypeA`] across R3(1-*)
    pub fn r3_simple_subtype_a<'a>(
        &'a self,
        store: &'a IsaVecStore,
    ) -> Vec<Rc<RefCell<SimpleSubtypeA>>> {
        span!("r3_simple_subtype_a");
        vec![store.exhume_simple_subtype_a(self.bar).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

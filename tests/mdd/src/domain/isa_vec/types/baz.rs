// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"baz-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"baz-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::isa_vec::types::simple_supertype::SimpleSupertype;
use serde::{Deserialize, Serialize};

use crate::domain::isa_vec::store::ObjectStore as IsaVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"baz-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Baz {
    pub id: usize,
    pub insanity: f64,
    /// R4: [`Baz`] 'chord' [`SimpleSupertype`]
    pub fugue: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"baz-implementation"}}}
impl Baz {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"baz-struct-impl-new"}}}
    /// Inter a new 'Baz' in the store, and return it's `id`.
    pub fn new(
        insanity: f64,
        fugue: &Rc<RefCell<SimpleSupertype>>,
        store: &mut IsaVecStore,
    ) -> Rc<RefCell<Baz>> {
        store.inter_baz(|id| {
            Rc::new(RefCell::new(Baz {
                id,
                insanity,
                fugue: fugue.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"baz-struct-impl-nav-forward-to-fugue"}}}
    /// Navigate to [`SimpleSupertype`] across R4(1-*)
    pub fn r4_simple_supertype<'a>(
        &'a self,
        store: &'a IsaVecStore,
    ) -> Vec<Rc<RefCell<SimpleSupertype>>> {
        span!("r4_simple_supertype");
        vec![store.exhume_simple_supertype(self.fugue).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

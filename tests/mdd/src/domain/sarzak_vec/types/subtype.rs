// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"subtype-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::sarzak_vec::types::isa::Isa;
use crate::domain::sarzak_vec::types::object::Object;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak_vec::store::ObjectStore as SarzakVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype-struct-documentation"}}}
/// The *subtype* in a *supertype-subtype* relationship.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Subtype {
    pub id: usize,
    /// R27: [`Subtype`] 'formalize an' [`Isa`]
    pub isa: usize,
    /// R15: [`Subtype`] 'is an instance of an' [`Object`]
    pub obj_id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype-implementation"}}}
impl Subtype {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype-struct-impl-new"}}}
    /// Inter a new 'Subtype' in the store, and return it's `id`.
    pub fn new(
        isa: &Rc<RefCell<Isa>>,
        obj_id: &Rc<RefCell<Object>>,
        store: &mut SarzakVecStore,
    ) -> Rc<RefCell<Subtype>> {
        store.inter_subtype(|id| {
            Rc::new(RefCell::new(Subtype {
                id,
                isa: isa.borrow().id,
                obj_id: obj_id.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype-struct-impl-nav-forward-to-isa"}}}
    /// Navigate to [`Isa`] across R27(1-*)
    pub fn r27_isa<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Isa>>> {
        span!("r27_isa");
        vec![store.exhume_isa(&self.isa).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype-struct-impl-nav-forward-to-obj_id"}}}
    /// Navigate to [`Object`] across R15(1-*)
    pub fn r15_object<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Object>>> {
        span!("r15_object");
        vec![store.exhume_object(&self.obj_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

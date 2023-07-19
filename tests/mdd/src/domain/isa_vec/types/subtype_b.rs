// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"subtype_b-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::isa_vec::types::super_t::SuperT;
use crate::domain::isa_vec::types::super_t::SuperTEnum;
use serde::{Deserialize, Serialize};

use crate::domain::isa_vec::store::ObjectStore as IsaVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-struct-documentation"}}}
/// This [`Subtype`][s] has a number
///
/// [s]: nut::sarzak::Subtype
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubtypeB {
    pub id: usize,
    pub number: i64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-implementation"}}}
impl SubtypeB {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-struct-impl-new"}}}
    /// Inter a new 'Subtype B' in the store, and return it's `id`.
    pub fn new(number: i64, store: &mut IsaVecStore) -> Rc<RefCell<SubtypeB>> {
        store.inter_subtype_b(|id| Rc::new(RefCell::new(SubtypeB { id, number })))
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-impl-nav-subtype-to-supertype-super_t"}}}
    // Navigate to [`SuperT`] across R2(isa)
    pub fn r2_super_t<'a>(&'a self, store: &'a IsaVecStore) -> Vec<Rc<RefCell<SuperT>>> {
        span!("r2_super_t");
        vec![store
            .iter_super_t()
            .find(|super_t| {
                if let SuperTEnum::SubtypeB(id) = super_t.borrow().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-implementation"}}}
impl PartialEq for SubtypeB {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

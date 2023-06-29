// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"subtype_a-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_a-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::isa_vec::types::super_t::SuperT;
use crate::domain::isa_vec::types::super_t::SuperTEnum;
use serde::{Deserialize, Serialize};

use crate::domain::isa_vec::store::ObjectStore as IsaVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_a-struct-documentation"}}}
/// This [`Subtype`][s] has [`Attribute`][a]s
///
/// [a]: nut::sarzak::Attribute
/// [s]: nut::sarzak::Subtype
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_a-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct SubtypeA {
    pub id: usize,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_a-implementation"}}}
impl SubtypeA {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_a-struct-impl-new"}}}
    /// Inter a new 'Subtype A' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut IsaVecStore) -> Rc<RefCell<SubtypeA>> {
        store.inter_subtype_a(|id| {
            Rc::new(RefCell::new(SubtypeA {
                id,
                name: name.to_owned(),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_a-impl-nav-subtype-to-supertype-super_t"}}}
    // Navigate to [`SuperT`] across R2(isa)
    pub fn r2_super_t<'a>(&'a self, store: &'a IsaVecStore) -> Vec<Rc<RefCell<SuperT>>> {
        span!("r2_super_t");
        vec![store
            .iter_super_t()
            .find(|super_t| {
                if let SuperTEnum::SubtypeA(id) = super_t.borrow().subtype {
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
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

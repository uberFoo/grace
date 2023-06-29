// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"super_t-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::isa_vec::types::not_important::NotImportant;
use crate::domain::isa_vec::types::reference::Reference;
use crate::domain::isa_vec::types::subtype_a::SubtypeA;
use crate::domain::isa_vec::types::subtype_b::SubtypeB;
use serde::{Deserialize, Serialize};

use crate::domain::isa_vec::store::ObjectStore as IsaVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-hybrid-documentation"}}}
/// A [`Supertype`] with normal [`Subtype`]s
///
/// This was called "Super". Rust didn't like it when it became "super". There needs to be
/// a way of fixing keywords.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-hybrid-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct SuperT {
    pub subtype: SuperTEnum,
    pub id: usize,
    /// R88: [`SuperT`] 'refers to' [`Reference`]
    pub pointer: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-hybrid-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum SuperTEnum {
    SubtypeA(usize),
    SubtypeB(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-implementation"}}}
impl SuperT {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-struct-impl-new_subtype_a"}}}
    /// Inter a new SuperT in the store, and return it's `id`.
    pub fn new_subtype_a(
        pointer: &Rc<RefCell<Reference>>,
        subtype: &Rc<RefCell<SubtypeA>>,
        store: &mut IsaVecStore,
    ) -> Rc<RefCell<SuperT>> {
        store.inter_super_t(|id| {
            Rc::new(RefCell::new(SuperT {
                pointer: pointer.borrow().id,
                subtype: SuperTEnum::SubtypeA(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-struct-impl-new_subtype_b"}}}
    /// Inter a new SuperT in the store, and return it's `id`.
    pub fn new_subtype_b(
        pointer: &Rc<RefCell<Reference>>,
        subtype: &Rc<RefCell<SubtypeB>>,
        store: &mut IsaVecStore,
    ) -> Rc<RefCell<SuperT>> {
        store.inter_super_t(|id| {
            Rc::new(RefCell::new(SuperT {
                pointer: pointer.borrow().id,
                subtype: SuperTEnum::SubtypeB(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-struct-impl-nav-forward-to-pointer"}}}
    /// Navigate to [`Reference`] across R88(1-*)
    pub fn r88_reference<'a>(&'a self, store: &'a IsaVecStore) -> Vec<Rc<RefCell<Reference>>> {
        span!("r88_reference");
        vec![store.exhume_reference(self.pointer).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-struct-impl-nav-backward-cond-to-not_important"}}}
    /// Navigate to [`NotImportant`] across R888(1-1c)
    pub fn r888c_not_important<'a>(
        &'a self,
        store: &'a IsaVecStore,
    ) -> Vec<Rc<RefCell<NotImportant>>> {
        span!("r888_not_important");
        let not_important = store
            .iter_not_important()
            .find(|not_important| not_important.borrow().x_ref == self.id);
        match not_important {
            Some(ref not_important) => vec![not_important.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

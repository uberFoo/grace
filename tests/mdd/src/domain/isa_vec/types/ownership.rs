// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"ownership-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::domain::isa_vec::types::borrowed::Borrowed;
use crate::domain::isa_vec::types::owned::OWNED;
use serde::{Deserialize, Serialize};

use crate::domain::isa_vec::store::ObjectStore as IsaVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-enum-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-hybrid-documentation"}}}
/// Type Ownership
///
/// This is tied closely with Rust. There are tthree possible options: owned, mutable and borrowed
/// .
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-enum-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-hybrid-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ownership {
    pub subtype: OwnershipEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-hybrid-enum-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum OwnershipEnum {
    Borrowed(usize),
    Owned(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-implementation"}}}
impl Ownership {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-new-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-struct-impl-new_borrowed"}}}
    /// Inter a new Ownership in the store, and return it's `id`.
    pub fn new_borrowed(
        subtype: &Rc<RefCell<Borrowed>>,
        store: &mut IsaVecStore,
    ) -> Rc<RefCell<Ownership>> {
        store.inter_ownership(|id| {
            Rc::new(RefCell::new(Ownership {
                subtype: OwnershipEnum::Borrowed(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-get-id-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-struct-impl-new_owned"}}}
    /// Inter a new Ownership in the store, and return it's `id`.
    pub fn new_owned(store: &mut IsaVecStore) -> Rc<RefCell<Ownership>> {
        store.inter_ownership(|id| {
            Rc::new(RefCell::new(Ownership {
                subtype: OwnershipEnum::Owned(OWNED),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-implementation"}}}
impl PartialEq for Ownership {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

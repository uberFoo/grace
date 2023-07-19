// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"borrowed-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::isa_vec::types::mutable::MUTABLE;
use crate::domain::isa_vec::types::ownership::Ownership;
use crate::domain::isa_vec::types::ownership::OwnershipEnum;
use crate::domain::isa_vec::types::shared::SHARED;
use serde::{Deserialize, Serialize};

use crate::domain::isa_vec::store::ObjectStore as IsaVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-enum-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-hybrid-documentation"}}}
/// Borrowed
///
/// The type is declared as borrowed.
///
/// ❗️{"singleton_object": true}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-enum-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-hybrid-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Borrowed {
    pub subtype: BorrowedEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-hybrid-enum-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum BorrowedEnum {
    Mutable(Uuid),
    Shared(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-implementation"}}}
impl Borrowed {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-new-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-struct-impl-new_mutable"}}}
    /// Inter a new Borrowed in the store, and return it's `id`.
    pub fn new_mutable(store: &mut IsaVecStore) -> Rc<RefCell<Borrowed>> {
        store.inter_borrowed(|id| {
            Rc::new(RefCell::new(Borrowed {
                subtype: BorrowedEnum::Mutable(MUTABLE),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-get-id-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-struct-impl-new_shared"}}}
    /// Inter a new Borrowed in the store, and return it's `id`.
    pub fn new_shared(store: &mut IsaVecStore) -> Rc<RefCell<Borrowed>> {
        store.inter_borrowed(|id| {
            Rc::new(RefCell::new(Borrowed {
                subtype: BorrowedEnum::Shared(SHARED),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-impl-nav-subtype-to-supertype-ownership"}}}
    // Navigate to [`Ownership`] across R9(isa)
    pub fn r9_ownership<'a>(&'a self, store: &'a IsaVecStore) -> Vec<Rc<RefCell<Ownership>>> {
        span!("r9_ownership");
        vec![store
            .iter_ownership()
            .find(|ownership| {
                if let OwnershipEnum::Borrowed(id) = ownership.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-implementation"}}}
impl PartialEq for Borrowed {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

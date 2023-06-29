// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"borrowed-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-use-statements"}}}
use crate::domain::isa_vec::store::ObjectStore as IsaVecStore;
use crate::domain::isa_vec::types::mutable::MUTABLE;
use crate::domain::isa_vec::types::ownership::Ownership;
use crate::domain::isa_vec::types::shared::SHARED;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-enum-documentation"}}}
/// Borrowed
///
/// The type is declared as borrowed.
///
/// ❗️{"singleton_object": true}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-enum-definition"}}}
#[derive(Copy, Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum Borrowed {
    Mutable = 0,
    Shared,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-implementation"}}}
impl Borrowed {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-new-impl"}}}
    /// Create a new instance of Borrowed::Mutable
    pub fn new_mutable(store: &IsaVecStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_borrowed(Self::Mutable as usize).unwrap()
    }

    /// Create a new instance of Borrowed::Shared
    pub fn new_shared(store: &IsaVecStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_borrowed(Self::Shared as usize).unwrap()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-get-id-impl"}}}
    pub fn id(&self) -> usize {
        match self {
            Borrowed::Mutable => Borrowed::Mutable as usize,
            Borrowed::Shared => Borrowed::Shared as usize,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-impl-nav-subtype-to-supertype-ownership"}}}
    // Navigate to [`Ownership`] across R9(isa)
    pub fn r9_ownership<'a>(&'a self, store: &'a IsaVecStore) -> Vec<Rc<RefCell<Ownership>>> {
        span!("r9_ownership");
        vec![store.exhume_ownership(self.id()).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

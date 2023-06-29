// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"ownership-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-use-statements"}}}
use crate::domain::isa_vec::store::ObjectStore as IsaVecStore;
use crate::domain::isa_vec::types::borrowed::Borrowed;
use crate::domain::isa_vec::types::owned::OWNED;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-enum-documentation"}}}
/// Type Ownership
///
/// This is tied closely with Rust. There are tthree possible options: owned, mutable and borrowed
/// .
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-enum-definition"}}}
#[derive(Copy, Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum Ownership {
    // What I want:
    BorrowedMutable = 0,
    BorrowedShared = 1,
    Owned = 2,
    // Borrowed(Borrowed),
    // Owned,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-implementation"}}}
impl Ownership {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-new-impl"}}}
    /// Create a new instance of Ownership::Borrowed
    pub fn new_borrowed(
        borrowed: &Rc<RefCell<Borrowed>>,
        store: &mut IsaVecStore,
    ) -> Rc<RefCell<Self>> {
        let id = borrowed.borrow().id();
        store.exhume_ownership(id).unwrap()
    }

    /// Create a new instance of Ownership::Owned
    pub fn new_owned(store: &IsaVecStore) -> Rc<RefCell<Self>> {
        // This is already in the store.
        store.exhume_ownership(Self::Owned as usize).unwrap()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-get-id-impl"}}}
    pub fn id(&self) -> usize {
        match self {
            Ownership::BorrowedMutable => Self::BorrowedMutable as usize,
            Ownership::BorrowedShared => Self::BorrowedShared as usize,
            Ownership::Owned => Ownership::Owned as usize,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

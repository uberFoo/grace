// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"ownership-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::isa_rwlock_vec::types::borrowed::Borrowed;
use crate::domain::isa_rwlock_vec::types::owned::OWNED;
use serde::{Deserialize, Serialize};

use crate::domain::isa_rwlock_vec::store::ObjectStore as IsaRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-hybrid-documentation"}}}
/// Type Ownership
///
/// This is tied closely with Rust. There are tthree possible options: owned, mutable and borrowed
/// .
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
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
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-struct-impl-new_borrowed"}}}
    /// Inter a new Ownership in the store, and return it's `id`.
    pub fn new_borrowed(
        subtype: &Arc<RwLock<Borrowed>>,
        store: &mut IsaRwlockVecStore,
    ) -> Arc<RwLock<Ownership>> {
        store.inter_ownership(|id| {
            Arc::new(RwLock::new(Ownership {
                subtype: OwnershipEnum::Borrowed(subtype.read().unwrap().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-struct-impl-new_owned"}}}
    /// Inter a new Ownership in the store, and return it's `id`.
    pub fn new_owned(store: &mut IsaRwlockVecStore) -> Arc<RwLock<Ownership>> {
        store.inter_ownership(|id| {
            Arc::new(RwLock::new(Ownership {
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

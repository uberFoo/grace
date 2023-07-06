// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"borrowed-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::isa_rwlock_vec::types::mutable::MUTABLE;
use crate::domain::isa_rwlock_vec::types::ownership::Ownership;
use crate::domain::isa_rwlock_vec::types::ownership::OwnershipEnum;
use crate::domain::isa_rwlock_vec::types::shared::SHARED;
use serde::{Deserialize, Serialize};

use crate::domain::isa_rwlock_vec::store::ObjectStore as IsaRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-hybrid-documentation"}}}
/// Borrowed
///
/// The type is declared as borrowed.
///
/// ❗️{"singleton_object": true}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-hybrid-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Borrowed {
    pub subtype: BorrowedEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-hybrid-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum BorrowedEnum {
    Mutable(Uuid),
    Shared(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-implementation"}}}
impl Borrowed {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-struct-impl-new_mutable"}}}
    /// Inter a new Borrowed in the store, and return it's `id`.
    pub fn new_mutable(store: &mut IsaRwlockVecStore) -> Arc<RwLock<Borrowed>> {
        store.inter_borrowed(|id| {
            Arc::new(RwLock::new(Borrowed {
                subtype: BorrowedEnum::Mutable(MUTABLE),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-struct-impl-new_shared"}}}
    /// Inter a new Borrowed in the store, and return it's `id`.
    pub fn new_shared(store: &mut IsaRwlockVecStore) -> Arc<RwLock<Borrowed>> {
        store.inter_borrowed(|id| {
            Arc::new(RwLock::new(Borrowed {
                subtype: BorrowedEnum::Shared(SHARED),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-impl-nav-subtype-to-supertype-ownership"}}}
    // Navigate to [`Ownership`] across R9(isa)
    pub fn r9_ownership<'a>(&'a self, store: &'a IsaRwlockVecStore) -> Vec<Arc<RwLock<Ownership>>> {
        span!("r9_ownership");
        vec![store
            .iter_ownership()
            .find(|ownership| {
                if let OwnershipEnum::Borrowed(id) = ownership.read().unwrap().subtype {
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

// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"borrowed-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-use-statements"}}}
use crate::domain::isa_rwlock::store::ObjectStore as IsaRwlockStore;
use crate::domain::isa_rwlock::types::mutable::MUTABLE;
use crate::domain::isa_rwlock::types::ownership::Ownership;
use crate::domain::isa_rwlock::types::shared::SHARED;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-hybrid-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-enum-documentation"}}}
/// Borrowed
///
/// The type is declared as borrowed.
///
/// ❗️{"singleton_object": true}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-hybrid-struct-definition"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-hybrid-enum-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-enum-definition"}}}
#[derive(Copy, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Borrowed {
    Mutable(Uuid),
    Shared(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-implementation"}}}
impl Borrowed {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-struct-impl-new_mutable"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-new-impl"}}}
    /// Create a new instance of Borrowed::Mutable
    pub fn new_mutable(store: &IsaRwlockStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_borrowed(&MUTABLE).unwrap()
    }

    /// Create a new instance of Borrowed::Shared
    pub fn new_shared(store: &IsaRwlockStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_borrowed(&SHARED).unwrap()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-struct-impl-new_shared"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::Mutable(id) => *id,
            Self::Shared(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-impl-nav-subtype-to-supertype-ownership"}}}
    // Navigate to [`Ownership`] across R9(isa)
    pub fn r9_ownership<'a>(&'a self, store: &'a IsaRwlockStore) -> Vec<Arc<RwLock<Ownership>>> {
        span!("r9_ownership");
        vec![store.exhume_ownership(&self.id()).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

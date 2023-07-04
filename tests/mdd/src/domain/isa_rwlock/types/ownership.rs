// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"ownership-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-use-statements"}}}
use crate::domain::isa_rwlock::store::ObjectStore as IsaRwlockStore;
use crate::domain::isa_rwlock::types::borrowed::Borrowed;
use crate::domain::isa_rwlock::types::owned::OWNED;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-hybrid-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-enum-documentation"}}}
/// Type Ownership
///
/// This is tied closely with Rust. There are tthree possible options: owned, mutable and borrowed
/// .
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-hybrid-struct-definition"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-hybrid-enum-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-enum-definition"}}}
#[derive(Copy, Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum Ownership {
    Borrowed(Uuid),
    Owned(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-implementation"}}}
impl Ownership {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-struct-impl-new_borrowed"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-new-impl"}}}
    /// Create a new instance of Ownership::Borrowed
    pub fn new_borrowed(
        borrowed: &Arc<RwLock<Borrowed>>,
        store: &mut IsaRwlockStore,
    ) -> Arc<RwLock<Self>> {
        let id = borrowed.read().unwrap().id();
        if let Some(borrowed) = store.exhume_ownership(&id) {
            borrowed
        } else {
            let new = Arc::new(RwLock::new(Self::Borrowed(id)));
            store.inter_ownership(new.clone());
            new
        }
    }

    /// Create a new instance of Ownership::Owned
    pub fn new_owned(store: &IsaRwlockStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_ownership(&OWNED).unwrap()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-struct-impl-new_owned"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ownership-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::Borrowed(id) => *id,
            Self::Owned(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

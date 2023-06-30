// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"borrowed-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-use-statements"}}}
use crate::domain::isa_ts::store::ObjectStore as IsaTsStore;
use crate::domain::isa_ts::types::mutable::MUTABLE;
use crate::domain::isa_ts::types::ownership::Ownership;
use crate::domain::isa_ts::types::shared::SHARED;
use serde::{Deserialize, Serialize};
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
    Mutable(Uuid),
    Shared(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-implementation"}}}
impl Borrowed {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"borrowed-new-impl"}}}
    /// Create a new instance of Borrowed::Mutable
    pub fn new_mutable() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Mutable(MUTABLE)
    }

    /// Create a new instance of Borrowed::Shared
    pub fn new_shared() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Shared(SHARED)
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
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
    pub fn r9_ownership<'a>(&'a self, store: &'a IsaTsStore) -> Vec<&Ownership> {
        vec![store.exhume_ownership(&self.id()).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

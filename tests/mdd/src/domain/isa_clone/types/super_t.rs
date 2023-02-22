// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"super_t-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
use crate::domain::isa_clone::store::ObjectStore as IsaCloneStore;
use crate::domain::isa_clone::types::subtype_a::SubtypeA;
use crate::domain::isa_clone::types::subtype_b::SubtypeB;

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-enum-documentation"}}}
/// A [`Supertype`] with normal [`Subtype`]s
///
/// This was called "Super". Rust didn't like it when it became "super". There needs to be
///a way of fixing keywords.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum SuperT {
    SubtypeA(Uuid),
    SubtypeB(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-implementation"}}}
impl SuperT {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-new-impl"}}}
    /// Create a new instance of SuperT::SubtypeA
    pub fn new_subtype_a(subtype_a: &SubtypeA, store: &mut IsaCloneStore) -> Self {
        let new = Self::SubtypeA(subtype_a.id);
        store.inter_super_t(new.clone());
        new
    }

    /// Create a new instance of SuperT::SubtypeB
    pub fn new_subtype_b(subtype_b: &SubtypeB, store: &mut IsaCloneStore) -> Self {
        let new = Self::SubtypeB(subtype_b.id);
        store.inter_super_t(new.clone());
        new
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            SuperT::SubtypeA(id) => *id,
            SuperT::SubtypeB(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

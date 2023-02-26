// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"simple_subtype_a-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

// Subtype imports
use crate::domain::isa_clone::types::oh_boy::OhBoy;

use crate::domain::isa_clone::store::ObjectStore as IsaCloneStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-enum-documentation"}}}
/// Simple [`Subtype`] A
///
/// This is represented as a singleton.
///
/// ❗️{ "singleton_object": true }
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum SimpleSubtypeA {
    OhBoy(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-implementation"}}}
impl SimpleSubtypeA {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-new-impl"}}}
    /// Create a new instance of SimpleSubtypeA::OhBoy
    pub fn new_oh_boy(oh_boy: &OhBoy, store: &mut IsaCloneStore) -> Self {
        let new = Self::OhBoy(oh_boy.id);
        store.inter_simple_subtype_a(new.clone());
        new
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            SimpleSubtypeA::OhBoy(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

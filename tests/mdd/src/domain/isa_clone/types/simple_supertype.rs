// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"simple_supertype-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::domain::isa_clone::types::simple_subtype_a::SIMPLE_SUBTYPE_A;
use crate::domain::isa_clone::types::simple_subtype_b::SIMPLE_SUBTYPE_B;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-enum-documentation"}}}
/// This [`Supertype`] is Simple
///
/// By that I mean that it's [`Subtypes`] consist only of singletons.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum SimpleSupertype {
    SimpleSubtypeA(Uuid),
    SimpleSubtypeB(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-implementation"}}}
impl SimpleSupertype {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-new-impl"}}}
    /// Create a new instance of SimpleSupertype::SimpleSubtypeA
    pub fn new_simple_subtype_a() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::SimpleSubtypeA(SIMPLE_SUBTYPE_A)
    }

    /// Create a new instance of SimpleSupertype::SimpleSubtypeB
    pub fn new_simple_subtype_b() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::SimpleSubtypeB(SIMPLE_SUBTYPE_B)
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            SimpleSupertype::SimpleSubtypeA(id) => *id,
            SimpleSupertype::SimpleSubtypeB(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"conditionality-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

// Subtype imports
use crate::domain::sarzak::types::conditional::CONDITIONAL;
use crate::domain::sarzak::types::unconditional::UNCONDITIONAL;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum Conditionality {
    Conditional(Uuid),
    Unconditional(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-implementation"}}}
impl Conditionality {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-new-impl"}}}
    /// Create a new instance of Conditionality::Conditional
    pub fn new_conditional() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Conditional(CONDITIONAL)
    }

    /// Create a new instance of Conditionality::Unconditional
    pub fn new_unconditional() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Unconditional(UNCONDITIONAL)
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Conditionality::Conditional(id) => *id,
            Conditionality::Unconditional(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"simple_subtype_b-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_b-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_b-const-documentation"}}}
/// Simple [`Subtype`] B
///
/// This is represented as a singleton.
///
/// ❗️{ "singleton_object": true }
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_b-const-definition"}}}
pub const SIMPLE_SUBTYPE_B: usize = 0;

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct SimpleSubtypeB;

impl SimpleSubtypeB {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> usize {
        SIMPLE_SUBTYPE_B
    }
}

impl Default for SimpleSubtypeB {
    fn default() -> Self {
        Self::new()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"mutable-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"mutable-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"mutable-const-documentation"}}}
/// Mutable
///
/// The type is declared as `mut`.
///
/// ❗️{"singleton_object": true}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"mutable-const-definition"}}}
pub const MUTABLE: usize = 0;

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Mutable;

impl Mutable {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> usize {
        MUTABLE
    }
}

impl Default for Mutable {
    fn default() -> Self {
        Self::new()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

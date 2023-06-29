// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"shared-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"shared-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"shared-const-documentation"}}}
/// A shared borrow.
///
/// According to rust rules, you may have any number of shared references outstanding at one
///  time. Just as long as there are zero mutable references.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"shared-const-definition"}}}
pub const SHARED: usize = 1;

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Shared;

impl Shared {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> usize {
        SHARED
    }
}

impl Default for Shared {
    fn default() -> Self {
        Self::new()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

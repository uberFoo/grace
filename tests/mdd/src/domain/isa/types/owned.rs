// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"owned-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"owned-use-statements"}}}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"owned-const-documentation"}}}
/// An owned value
///
/// Someone has to be responsible for it, right?
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"owned-const-definition"}}}
pub const OWNED: Uuid = uuid!["a4d78b0c-2ee9-5de8-a7a2-b920ad719e7e"];

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Owned;

impl Owned {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        OWNED
    }
}

impl Default for Owned {
    fn default() -> Self {
        Self::new()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

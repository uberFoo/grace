// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"mutable-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"mutable-use-statements"}}}
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
pub const MUTABLE: Uuid = uuid!["6978adbf-87ab-5b55-ac7c-1ddbd32ffae8"];

pub struct Mutable;

impl Mutable {
    pub fn new() -> Self {
        Self {}
    }

    pub fn id(&self) -> Uuid {
        MUTABLE
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

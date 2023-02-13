// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"simple_subtype_b-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_b-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::isa_domain::UUID_NS;

use crate::isa_domain::store::ObjectStore as IsaDomainStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"simple_subtype_b-struct-documentation"}}}
/// Simple [`Subtype`] B
///
/// This is represented as a singleton.
///
/// ❗️{ "singleton_object": true }
// {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_b-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct SimpleSubtypeB {
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_b-struct-implementation"}}}
impl SimpleSubtypeB {
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"simple_subtype_b-struct-impl-new"}}}
    /// Inter a new SimpleSubtypeB in the store, and return it's `id`.
    pub fn new(store: &mut IsaDomainStore) -> SimpleSubtypeB {
        let id = Uuid::new_v5(&UUID_NS, format!("",).as_bytes());
        let new = SimpleSubtypeB { id };
        store.inter_simple_subtype_b(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

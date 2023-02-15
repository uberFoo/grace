// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"simple_supertype-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"simple_supertype-struct-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"simple_supertype-enum-documentation"}}}
/// This [`Supertype`] is Simple
///
/// By that I mean that it's [`Subtypes`] consist only of singletons.
// {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-struct-definition"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-struct-implementation"}}}
// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"simple_supertype-struct-impl-new"}}}
/// Inter a new SimpleSupertype in the store, and return it's `id`.
//     pub fn new(store: &mut IsaDomainStore) -> SimpleSupertype {
//         let id = Uuid::new_v5(&UUID_NS, format!("",).as_bytes());
//         let new = SimpleSupertype { id };
//         store.inter_simple_supertype(new.clone());
//         new
//     }
// {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-enum"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum SimpleSupertype {
    SimpleSubtypeA(Uuid),
    SimpleSubtypeB(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_supertype-implementation"}}}
impl SimpleSupertype {
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
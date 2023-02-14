// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"simple_subtype_a-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-use-statements"}}}
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"simple_subtype_a-struct-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"simple_subtype_a-const-documentation"}}}
/// Simple [`Subtype`] A
///
/// This is represented as a singleton.
///
/// ❗️{ "singleton_object": true }
// {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-struct-definition"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-struct-implementation"}}}
// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"simple_subtype_a-struct-impl-new"}}}
/// Inter a new SimpleSubtypeA in the store, and return it's `id`.
//     pub fn new(store: &mut IsaDomainStore) -> SimpleSubtypeA {
//         let id = Uuid::new_v5(&UUID_NS, format!("",).as_bytes());
//         let new = SimpleSubtypeA { id };
//         store.inter_simple_subtype_a(new.clone());
//         new
//     }
// {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-const-definition"}}}
pub const SIMPLE_SUBTYPE_A: Uuid = uuid!["e484270c-ec90-5c3c-9371-f7b716be6ffc"];
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

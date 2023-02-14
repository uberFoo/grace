// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"super_t-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::isa_domain::store::ObjectStore as IsaDomainStore;
use crate::isa_domain::UUID_NS;

// Subtype imports
use crate::isa_domain::types::subtype_a::SubtypeA;
use crate::isa_domain::types::subtype_b::SubtypeB;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"super_t-struct-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"super_t-enum-documentation"}}}
/// A [`Supertype`] with normal [`Subtype`]s
///
/// This was called "Super". Rust didn't like it when it became "super". There needs to be a way of fixing keywords.
// {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-struct-definition"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-struct-implementation"}}}
// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"super_t-struct-impl-new"}}}
/// Inter a new SuperT in the store, and return it's `id`.
//     pub fn new(store: &mut IsaDomainStore) -> SuperT {
//         let id = Uuid::new_v5(&UUID_NS, format!("",).as_bytes());
//         let new = SuperT { id };
//         store.inter_super_t(new.clone());
//         new
//     }
// {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-enum"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum SuperT {
    SubtypeA(Uuid),
    SubtypeB(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-implementation"}}}
impl SuperT {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            SuperT::SubtypeA(id) => *id,
            SuperT::SubtypeB(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"subtype_a-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_a-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::isa_domain::UUID_NS;

use crate::isa_domain::store::ObjectStore as IsaDomainStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_a-struct-documentation"}}}
/// This [`Subtype`][s] has [`Attribute`][a]s
///
/// [a]: nut::sarzak::Attribute
/// [s]: nut::sarzak::Subtype
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_a-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct SubtypeA {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_a-implementation"}}}
impl SubtypeA {
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"subtype_a-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_a-struct-impl-new"}}}
    /// Inter a new SubtypeA in the store, and return it's `id`.
    pub fn new(name: String, store: &mut IsaDomainStore) -> SubtypeA {
        let id = Uuid::new_v5(&UUID_NS, format!("{}", name).as_bytes());
        let new = SubtypeA { name: name, id };
        store.inter_subtype_a(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"subtype_a-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_a-use-statements"}}}
use uuid::Uuid;

use crate::domain::isa_clone::types::super_t::SuperT;
use crate::domain::isa_clone::UUID_NS;
use serde::{Deserialize, Serialize};

use crate::domain::isa_clone::store::ObjectStore as IsaCloneStore;
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
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_a-struct-impl-new"}}}
    /// Inter a new 'Subtype A' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut IsaCloneStore) -> SubtypeA {
        let id = Uuid::new_v5(&UUID_NS, format!("{}", name).as_bytes());
        let new = SubtypeA { id: id, name: name };
        store.inter_subtype_a(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_a-impl-nav-subtype-to-supertype-super_t"}}}
    // Navigate to [`SuperT`] across R2(isa)
    pub fn r2_super_t<'a>(&'a self, store: &'a IsaCloneStore) -> Vec<&SuperT> {
        vec![store.exhume_super_t(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"reference-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::domain::isa::UUID_NS;

// Referent imports
use crate::domain::isa::types::super_t::SuperT;

use crate::domain::isa::store::ObjectStore as IsaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-documentation"}}}
/// Something to Refer To
///
/// I'm the guy you need to keep track of.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Reference {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-implementation"}}}
impl Reference {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-impl-new"}}}
    /// Inter a new Reference in the store, and return it's `id`.
    pub fn new(name: String, store: &mut IsaStore) -> Reference {
        let id = Uuid::new_v5(&UUID_NS, format!("{}", name).as_bytes());
        let new = Reference { name: name, id };
        store.inter_reference(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-impl-nav-backward-one-to-super_t"}}}
    /// Navigate to [`SuperT`] across R88(1-1)
    pub fn r88_super_t<'a>(&'a self, store: &'a IsaStore) -> Vec<&SuperT> {
        vec![store
            .iter_super_t()
            .find(|super_t| super_t.pointer == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"reference-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-use-statements"}}}
use uuid::Uuid;

use crate::domain::isa_clone::types::super_t::SuperT;
use serde::{Deserialize, Serialize};

use crate::domain::isa_clone::store::ObjectStore as IsaCloneStore;
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
    /// Inter a new 'Reference' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut IsaCloneStore) -> Reference {
        let id = Uuid::new_v4();
        let new = Reference { id: id, name: name };
        store.inter_reference(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-impl-new_"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-impl-nav-backward-one-to-super_t"}}}
    /// Navigate to [`SuperT`] across R88(1-1)
    pub fn r88_super_t<'a>(&'a self, store: &'a IsaCloneStore) -> Vec<&SuperT> {
        vec![store
            .iter_super_t()
            .find(|super_t| super_t.pointer == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

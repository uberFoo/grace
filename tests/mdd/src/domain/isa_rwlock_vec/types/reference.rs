// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"reference-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::isa_rwlock_vec::types::super_t::SuperT;
use serde::{Deserialize, Serialize};

use crate::domain::isa_rwlock_vec::store::ObjectStore as IsaRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-documentation"}}}
/// Something to Refer To
///
/// I'm the guy you need to keep track of.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Reference {
    pub id: usize,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-implementation"}}}
impl Reference {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-impl-new"}}}
    /// Inter a new 'Reference' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut IsaRwlockVecStore) -> Arc<RwLock<Reference>> {
        store.inter_reference(|id| {
            Arc::new(RwLock::new(Reference {
                id,
                name: name.to_owned(),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-struct-impl-nav-backward-one-to-super_t"}}}
    /// Navigate to [`SuperT`] across R88(1-1)
    pub fn r88_super_t<'a>(&'a self, store: &'a IsaRwlockVecStore) -> Vec<Arc<RwLock<SuperT>>> {
        vec![store
            .iter_super_t()
            .find(|super_t| super_t.read().unwrap().pointer == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"reference-implementation"}}}
impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

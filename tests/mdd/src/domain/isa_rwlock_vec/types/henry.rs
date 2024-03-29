// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"henry-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"henry-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::isa_rwlock_vec::types::simple_subtype_a::SimpleSubtypeA;
use serde::{Deserialize, Serialize};

use crate::domain::isa_rwlock_vec::store::ObjectStore as IsaRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"henry-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Henry {
    pub id: usize,
    pub last_name: String,
    /// R3: [`Henry`] 'foo' [`SimpleSubtypeA`]
    pub bar: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"henry-implementation"}}}
impl Henry {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"henry-struct-impl-new"}}}
    /// Inter a new 'Henry' in the store, and return it's `id`.
    pub fn new(
        last_name: String,
        bar: &Arc<RwLock<SimpleSubtypeA>>,
        store: &mut IsaRwlockVecStore,
    ) -> Arc<RwLock<Henry>> {
        store.inter_henry(|id| {
            Arc::new(RwLock::new(Henry {
                id,
                last_name: last_name.to_owned(),
                bar: bar.read().unwrap().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"henry-struct-impl-nav-forward-to-bar"}}}
    /// Navigate to [`SimpleSubtypeA`] across R3(1-*)
    pub fn r3_simple_subtype_a<'a>(
        &'a self,
        store: &'a IsaRwlockVecStore,
    ) -> Vec<Arc<RwLock<SimpleSubtypeA>>> {
        vec![store.exhume_simple_subtype_a(&self.bar).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"henry-implementation"}}}
impl PartialEq for Henry {
    fn eq(&self, other: &Self) -> bool {
        self.last_name == other.last_name && self.bar == other.bar
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

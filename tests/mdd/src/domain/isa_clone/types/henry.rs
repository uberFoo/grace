// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"henry-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"henry-use-statements"}}}
use uuid::Uuid;

use crate::domain::isa_clone::types::simple_subtype_a::SimpleSubtypeA;
use crate::domain::isa_clone::UUID_NS;
use serde::{Deserialize, Serialize};

use crate::domain::isa_clone::store::ObjectStore as IsaCloneStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"henry-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Henry {
    pub id: Uuid,
    pub last_name: String,
    /// R3: [`Henry`] 'foo' [`SimpleSubtypeA`]
    pub bar: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"henry-implementation"}}}
impl Henry {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"henry-struct-impl-new"}}}
    /// Inter a new 'Henry' in the store, and return it's `id`.
    pub fn new(last_name: String, bar: &SimpleSubtypeA, store: &mut IsaCloneStore) -> Henry {
        let id = Uuid::new_v5(&UUID_NS, format!("{}:{:?}", last_name, bar).as_bytes());
        let new = Henry {
            bar: bar.id(),
            last_name: last_name,
            id: id,
        };
        store.inter_henry(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"henry-struct-impl-nav-forward-to-bar"}}}
    /// Navigate to [`SimpleSubtypeA`] across R3(1-*)
    pub fn r3_simple_subtype_a<'a>(&'a self, store: &'a IsaCloneStore) -> Vec<&SimpleSubtypeA> {
        vec![store.exhume_simple_subtype_a(&self.bar).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

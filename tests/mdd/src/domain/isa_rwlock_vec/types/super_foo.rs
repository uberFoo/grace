// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"super_foo-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::isa_rwlock_vec::types::gamma::Gamma;
use serde::{Deserialize, Serialize};

use crate::domain::isa_rwlock_vec::store::ObjectStore as IsaRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-hybrid-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SuperFoo {
    pub subtype: SuperFooEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-hybrid-enum-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum SuperFooEnum {
    Gamma(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-implementation"}}}
impl SuperFoo {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-struct-impl-new_gamma"}}}
    /// Inter a new SuperFoo in the store, and return it's `id`.
    pub fn new_gamma(
        subtype: &Arc<RwLock<Gamma>>,
        store: &mut IsaRwlockVecStore,
    ) -> Arc<RwLock<SuperFoo>> {
        store.inter_super_foo(|id| {
            Arc::new(RwLock::new(SuperFoo {
                subtype: SuperFooEnum::Gamma(subtype.read().unwrap().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-implementation"}}}
impl PartialEq for SuperFoo {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"super_foo-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-use-statements"}}}
use crate::domain::isa_rwlock::store::ObjectStore as IsaRwlockStore;
use crate::domain::isa_rwlock::types::gamma::Gamma;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-hybrid-struct-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-enum-definition"}}}
#[derive(Copy, Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum SuperFoo {
    Gamma(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-hybrid-enum-definition"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-implementation"}}}
impl SuperFoo {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-struct-impl-new_gamma"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-new-impl"}}}
    /// Create a new instance of SuperFoo::Gamma
    pub fn new_gamma(gamma: &Arc<RwLock<Gamma>>, store: &mut IsaRwlockStore) -> Arc<RwLock<Self>> {
        let id = gamma.read().unwrap().id;
        if let Some(gamma) = store.exhume_super_foo(&id) {
            gamma
        } else {
            let new = Arc::new(RwLock::new(Self::Gamma(id)));
            store.inter_super_foo(new.clone());
            new
        }
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::Gamma(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

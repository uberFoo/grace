// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"simple_subtype_a-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-use-statements"}}}
use crate::domain::isa_rwlock::store::ObjectStore as IsaRwlockStore;
use crate::domain::isa_rwlock::types::henry::Henry;
use crate::domain::isa_rwlock::types::oh_boy::OhBoy;
use crate::domain::isa_rwlock::types::simple_supertype::SimpleSupertype;
use crate::domain::isa_rwlock::types::simple_supertype::SimpleSupertypeEnum;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-hybrid-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-enum-documentation"}}}
/// Simple [`Subtype`] A
///
/// This is represented as a singleton.
///
/// ❗️{ "singleton_object": true }
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-hybrid-struct-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-enum-definition"}}}
#[derive(Copy, Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum SimpleSubtypeA {
    OhBoy(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-hybrid-enum-definition"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-implementation"}}}
impl SimpleSubtypeA {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-struct-impl-new_oh_boy"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-new-impl"}}}
    /// Create a new instance of SimpleSubtypeA::OhBoy
    pub fn new_oh_boy(
        oh_boy: &Arc<RwLock<OhBoy>>,
        store: &mut IsaRwlockStore,
    ) -> Arc<RwLock<Self>> {
        let id = oh_boy.read().unwrap().id;
        if let Some(oh_boy) = store.exhume_simple_subtype_a(&id) {
            oh_boy
        } else {
            let new = Arc::new(RwLock::new(Self::OhBoy(id)));
            store.inter_simple_subtype_a(new.clone());
            new
        }
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::OhBoy(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-struct-impl-nav-backward-one-to-henry"}}}
    /// Navigate to [`Henry`] across R3(1-1)
    pub fn r3_henry<'a>(&'a self, store: &'a IsaRwlockStore) -> Vec<Arc<RwLock<Henry>>> {
        span!("r3_henry");
        vec![store
            .iter_henry()
            .find(|henry| henry.read().unwrap().bar == self.id())
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-impl-nav-subtype-to-supertype-simple_supertype"}}}
    // Navigate to [`SimpleSupertype`] across R1(isa)
    pub fn r1_simple_supertype<'a>(
        &'a self,
        store: &'a IsaRwlockStore,
    ) -> Vec<Arc<RwLock<SimpleSupertype>>> {
        span!("r1_simple_supertype");
        vec![store
            .iter_simple_supertype()
            .find(|simple_supertype| {
                if let SimpleSupertypeEnum::SimpleSubtypeA(id) =
                    simple_supertype.read().unwrap().subtype
                {
                    id == self.id()
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

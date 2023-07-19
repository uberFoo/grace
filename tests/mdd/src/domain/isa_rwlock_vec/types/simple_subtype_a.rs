// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"simple_subtype_a-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::isa_rwlock_vec::types::henry::Henry;
use crate::domain::isa_rwlock_vec::types::oh_boy::OhBoy;
use crate::domain::isa_rwlock_vec::types::simple_supertype::SimpleSupertype;
use crate::domain::isa_rwlock_vec::types::simple_supertype::SimpleSupertypeEnum;
use serde::{Deserialize, Serialize};

use crate::domain::isa_rwlock_vec::store::ObjectStore as IsaRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-hybrid-documentation"}}}
/// Simple [`Subtype`] A
///
/// This is represented as a singleton.
///
/// ❗️{ "singleton_object": true }
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-hybrid-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimpleSubtypeA {
    pub subtype: SimpleSubtypeAEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-hybrid-enum-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum SimpleSubtypeAEnum {
    OhBoy(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-implementation"}}}
impl SimpleSubtypeA {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-struct-impl-new_oh_boy"}}}
    /// Inter a new SimpleSubtypeA in the store, and return it's `id`.
    pub fn new_oh_boy(
        subtype: &Arc<RwLock<OhBoy>>,
        store: &mut IsaRwlockVecStore,
    ) -> Arc<RwLock<SimpleSubtypeA>> {
        store.inter_simple_subtype_a(|id| {
            Arc::new(RwLock::new(SimpleSubtypeA {
                subtype: SimpleSubtypeAEnum::OhBoy(subtype.read().unwrap().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-struct-impl-nav-backward-one-to-henry"}}}
    /// Navigate to [`Henry`] across R3(1-1)
    pub fn r3_henry<'a>(&'a self, store: &'a IsaRwlockVecStore) -> Vec<Arc<RwLock<Henry>>> {
        span!("r3_henry");
        vec![store
            .iter_henry()
            .find(|henry| henry.read().unwrap().bar == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-impl-nav-subtype-to-supertype-simple_supertype"}}}
    // Navigate to [`SimpleSupertype`] across R1(isa)
    pub fn r1_simple_supertype<'a>(
        &'a self,
        store: &'a IsaRwlockVecStore,
    ) -> Vec<Arc<RwLock<SimpleSupertype>>> {
        span!("r1_simple_supertype");
        vec![store
            .iter_simple_supertype()
            .find(|simple_supertype| {
                if let SimpleSupertypeEnum::SimpleSubtypeA(id) =
                    simple_supertype.read().unwrap().subtype
                {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-implementation"}}}
impl PartialEq for SimpleSubtypeA {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

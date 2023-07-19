// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"super_bar-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::isa_rwlock_vec::types::beta::Beta;
use crate::domain::isa_rwlock_vec::types::beta::BetaEnum;
use crate::domain::isa_rwlock_vec::types::gamma::Gamma;
use serde::{Deserialize, Serialize};

use crate::domain::isa_rwlock_vec::store::ObjectStore as IsaRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-hybrid-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SuperBar {
    pub subtype: SuperBarEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-hybrid-enum-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum SuperBarEnum {
    Gamma(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-implementation"}}}
impl SuperBar {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-struct-impl-new_gamma"}}}
    /// Inter a new SuperBar in the store, and return it's `id`.
    pub fn new_gamma(
        subtype: &Arc<RwLock<Gamma>>,
        store: &mut IsaRwlockVecStore,
    ) -> Arc<RwLock<SuperBar>> {
        store.inter_super_bar(|id| {
            Arc::new(RwLock::new(SuperBar {
                subtype: SuperBarEnum::Gamma(subtype.read().unwrap().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-impl-nav-subtype-to-supertype-beta"}}}
    // Navigate to [`Beta`] across R11(isa)
    pub fn r11_beta<'a>(&'a self, store: &'a IsaRwlockVecStore) -> Vec<Arc<RwLock<Beta>>> {
        span!("r11_beta");
        vec![store
            .iter_beta()
            .find(|beta| {
                if let BetaEnum::SuperBar(id) = beta.read().unwrap().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-implementation"}}}
impl PartialEq for SuperBar {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

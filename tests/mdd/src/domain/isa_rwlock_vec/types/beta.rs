// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"beta-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::isa_rwlock_vec::types::gamma::Gamma;
use crate::domain::isa_rwlock_vec::types::super_bar::SuperBar;
use serde::{Deserialize, Serialize};

use crate::domain::isa_rwlock_vec::store::ObjectStore as IsaRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-hybrid-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Beta {
    pub subtype: BetaEnum,
    pub id: usize,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-hybrid-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum BetaEnum {
    Gamma(usize),
    SuperBar(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-implementation"}}}
impl Beta {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-struct-impl-new_gamma"}}}
    /// Inter a new Beta in the store, and return it's `id`.
    pub fn new_gamma(
        name: String,
        subtype: &Arc<RwLock<Gamma>>,
        store: &mut IsaRwlockVecStore,
    ) -> Arc<RwLock<Beta>> {
        store.inter_beta(|id| {
            Arc::new(RwLock::new(Beta {
                name: name.to_owned(),
                subtype: BetaEnum::Gamma(subtype.read().unwrap().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-struct-impl-new_super_bar"}}}
    /// Inter a new Beta in the store, and return it's `id`.
    pub fn new_super_bar(
        name: String,
        subtype: &Arc<RwLock<SuperBar>>,
        store: &mut IsaRwlockVecStore,
    ) -> Arc<RwLock<Beta>> {
        store.inter_beta(|id| {
            Arc::new(RwLock::new(Beta {
                name: name.to_owned(),
                subtype: BetaEnum::SuperBar(subtype.read().unwrap().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

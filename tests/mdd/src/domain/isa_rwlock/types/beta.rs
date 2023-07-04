// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"beta-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::isa_rwlock::types::gamma::Gamma;
use crate::domain::isa_rwlock::types::super_bar::SuperBar;
use serde::{Deserialize, Serialize};

use crate::domain::isa_rwlock::store::ObjectStore as IsaRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-hybrid-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Beta {
    pub subtype: BetaEnum,
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-hybrid-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum BetaEnum {
    Gamma(Uuid),
    SuperBar(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-implementation"}}}
impl Beta {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-struct-impl-new_gamma"}}}
    /// Inter a new Beta in the store, and return it's `id`.
    pub fn new_gamma(
        name: String,
        subtype: &Arc<RwLock<Gamma>>,
        store: &mut IsaRwlockStore,
    ) -> Arc<RwLock<Beta>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Beta {
            name: name,
            subtype: BetaEnum::Gamma(subtype.read().unwrap().id),
            id,
        }));
        store.inter_beta(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-struct-impl-new_super_bar"}}}
    /// Inter a new Beta in the store, and return it's `id`.
    pub fn new_super_bar(
        name: String,
        subtype: &Arc<RwLock<SuperBar>>,
        store: &mut IsaRwlockStore,
    ) -> Arc<RwLock<Beta>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Beta {
            name: name,
            subtype: BetaEnum::SuperBar(subtype.read().unwrap().id()),
            id,
        }));
        store.inter_beta(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

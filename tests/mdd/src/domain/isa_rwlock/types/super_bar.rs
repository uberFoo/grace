// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"super_bar-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-use-statements"}}}
use crate::domain::isa_rwlock::store::ObjectStore as IsaRwlockStore;
use crate::domain::isa_rwlock::types::beta::Beta;
use crate::domain::isa_rwlock::types::beta::BetaEnum;
use crate::domain::isa_rwlock::types::gamma::Gamma;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-hybrid-struct-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-enum-definition"}}}
#[derive(Copy, Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum SuperBar {
    Gamma(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-hybrid-enum-definition"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-implementation"}}}
impl SuperBar {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-struct-impl-new_gamma"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-new-impl"}}}
    /// Create a new instance of SuperBar::Gamma
    pub fn new_gamma(gamma: &Arc<RwLock<Gamma>>, store: &mut IsaRwlockStore) -> Arc<RwLock<Self>> {
        let id = gamma.read().unwrap().id;
        if let Some(gamma) = store.exhume_super_bar(&id) {
            gamma
        } else {
            let new = Arc::new(RwLock::new(Self::Gamma(id)));
            store.inter_super_bar(new.clone());
            new
        }
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::Gamma(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-impl-nav-subtype-to-supertype-beta"}}}
    // Navigate to [`Beta`] across R11(isa)
    pub fn r11_beta<'a>(&'a self, store: &'a IsaRwlockStore) -> Vec<Arc<RwLock<Beta>>> {
        span!("r11_beta");
        vec![store
            .iter_beta()
            .find(|beta| {
                if let BetaEnum::SuperBar(id) = beta.read().unwrap().subtype {
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

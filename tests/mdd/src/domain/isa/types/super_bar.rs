// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"super_bar-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-use-statements"}}}
use crate::domain::isa::store::ObjectStore as IsaStore;
use crate::domain::isa::types::beta::Beta;
use crate::domain::isa::types::beta::BetaEnum;
use crate::domain::isa::types::gamma::Gamma;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-enum-definition"}}}
#[derive(Copy, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum SuperBar {
    Gamma(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-implementation"}}}
impl SuperBar {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-new-impl"}}}
    /// Create a new instance of SuperBar::Gamma
    pub fn new_gamma(gamma: &Gamma, store: &mut IsaStore) -> Self {
        let new = Self::Gamma(gamma.id);
        store.inter_super_bar(new.clone());
        new
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
    pub fn r11_beta<'a>(&'a self, store: &'a IsaStore) -> Vec<&Beta> {
        vec![store
            .iter_beta()
            .find(|beta| {
                if let BetaEnum::SuperBar(id) = beta.subtype {
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

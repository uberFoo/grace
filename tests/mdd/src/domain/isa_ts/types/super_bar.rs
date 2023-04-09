// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"super_bar-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-use-statements"}}}
use crate::domain::isa_ts::store::ObjectStore as IsaTsStore;
use crate::domain::isa_ts::types::beta::Beta;
use crate::domain::isa_ts::types::gamma::Gamma;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum SuperBar {
    Gamma(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-implementation"}}}
impl SuperBar {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-new-impl"}}}
    /// Create a new instance of SuperBar::Gamma
    pub fn new_gamma(gamma: &Gamma, store: &mut IsaTsStore) -> Self {
        let new = Self::Gamma(gamma.id);
        store.inter_super_bar(new.clone());
        new
    }

    pub fn new_gamma_(gamma: &Gamma) -> Self {
        let new = Self::Gamma(gamma.id);
        new
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            SuperBar::Gamma(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-impl-nav-subtype-to-supertype-beta"}}}
    // Navigate to [`Beta`] across R11(isa)
    pub fn r11_beta<'a>(&'a self, store: &'a IsaTsStore) -> Vec<&Beta> {
        vec![store.exhume_beta(&self.id()).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

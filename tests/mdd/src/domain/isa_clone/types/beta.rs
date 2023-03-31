// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"beta-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-use-statements"}}}
use crate::domain::isa_clone::store::ObjectStore as IsaCloneStore;
use crate::domain::isa_clone::types::gamma::Gamma;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-enum-documentation"}}}
/// This test is not complete.
///
/// To complete this test add an attribute to this object.
///
/// See grace#58.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum Beta {
    Gamma(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-implementation"}}}
impl Beta {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-new-impl"}}}
    /// Create a new instance of Beta::Gamma
    pub fn new_gamma(gamma: &Gamma, store: &mut IsaCloneStore) -> Self {
        let new = Self::Gamma(gamma.id);
        store.inter_beta(new.clone());
        new
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Beta::Gamma(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

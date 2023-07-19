// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"super_foo-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-use-statements"}}}
use crate::domain::isa_ts::store::ObjectStore as IsaTsStore;
use crate::domain::isa_ts::types::gamma::Gamma;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-enum-definition"}}}
#[derive(Copy, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum SuperFoo {
    Gamma(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-implementation"}}}
impl SuperFoo {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-new-impl"}}}
    /// Create a new instance of SuperFoo::Gamma
    pub fn new_gamma(gamma: &Gamma, store: &mut IsaTsStore) -> Self {
        let new = Self::Gamma(gamma.id);
        store.inter_super_foo(new.clone());
        new
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

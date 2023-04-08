// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"simple_subtype_a-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-use-statements"}}}
use crate::domain::isa::store::ObjectStore as IsaStore;
use crate::domain::isa::types::henry::Henry;
use crate::domain::isa::types::oh_boy::OhBoy;
use crate::domain::isa::types::simple_supertype::SimpleSupertype;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-enum-documentation"}}}
/// Simple [`Subtype`] A
///
/// This is represented as a singleton.
///
/// ❗️{ "singleton_object": true }
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum SimpleSubtypeA {
    OhBoy(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-implementation"}}}
impl SimpleSubtypeA {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-new-impl"}}}
    /// Create a new instance of SimpleSubtypeA::OhBoy
    pub fn new_oh_boy(oh_boy: &OhBoy, store: &mut IsaStore) -> Self {
        let new = Self::OhBoy(oh_boy.id);
        store.inter_simple_subtype_a(new.clone());
        new
    }

    pub fn new_oh_boy_(oh_boy: &OhBoy) -> Self {
        let new = Self::OhBoy(oh_boy.id);
        new
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            SimpleSubtypeA::OhBoy(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-struct-impl-nav-backward-one-to-henry"}}}
    /// Navigate to [`Henry`] across R3(1-1)
    pub fn r3_henry<'a>(&'a self, store: &'a IsaStore) -> Vec<&Henry> {
        vec![store
            .iter_henry()
            .find(|henry| henry.bar == self.id())
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"simple_subtype_a-impl-nav-subtype-to-supertype-simple_supertype"}}}
    // Navigate to [`SimpleSupertype`] across R1(isa)
    pub fn r1_simple_supertype<'a>(&'a self, store: &'a IsaStore) -> Vec<&SimpleSupertype> {
        vec![store.exhume_simple_supertype(&self.id()).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

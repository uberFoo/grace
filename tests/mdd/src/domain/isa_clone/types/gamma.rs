// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"gamma-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-use-statements"}}}
use uuid::Uuid;

use crate::domain::isa_clone::types::alpha::Alpha;
use crate::domain::isa_clone::types::beta::Beta;
use crate::domain::isa_clone::types::super_bar::SuperBar;
use crate::domain::isa_clone::types::super_foo::SuperFoo;
use crate::domain::isa_clone::UUID_NS;
use serde::{Deserialize, Serialize};

use crate::domain::isa_clone::store::ObjectStore as IsaCloneStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-struct-documentation"}}}
/// This object has two supertypes.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Gamma {
    pub id: Uuid,
    pub value: f64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-implementation"}}}
impl Gamma {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-struct-impl-new"}}}
    /// Inter a new 'Gamma' in the store, and return it's `id`.
    pub fn new(value: f64, store: &mut IsaCloneStore) -> Gamma {
        let id = Uuid::new_v5(&UUID_NS, format!("{}", value).as_bytes());
        let new = Gamma {
            id: id,
            value: value,
        };
        store.inter_gamma(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-super_foo"}}}
    // Navigate to [`SuperFoo`] across R13(isa)
    pub fn r13_super_foo<'a>(&'a self, store: &'a IsaCloneStore) -> Vec<&SuperFoo> {
        vec![store.exhume_super_foo(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-super_bar"}}}
    // Navigate to [`SuperBar`] across R12(isa)
    pub fn r12_super_bar<'a>(&'a self, store: &'a IsaCloneStore) -> Vec<&SuperBar> {
        vec![store.exhume_super_bar(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-beta"}}}
    // Navigate to [`Beta`] across R11(isa)
    pub fn r11_beta<'a>(&'a self, store: &'a IsaCloneStore) -> Vec<&Beta> {
        vec![store.exhume_beta(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-alpha"}}}
    // Navigate to [`Alpha`] across R10(isa)
    pub fn r10_alpha<'a>(&'a self, store: &'a IsaCloneStore) -> Vec<&Alpha> {
        vec![store.exhume_alpha(&self.id).unwrap()]
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-beta"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-beta"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-beta"}}}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

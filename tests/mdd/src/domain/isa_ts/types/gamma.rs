// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"gamma-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-use-statements"}}}
use uuid::Uuid;

use crate::domain::isa_ts::types::alpha::Alpha;
use crate::domain::isa_ts::types::alpha::AlphaEnum;
use crate::domain::isa_ts::types::beta::Beta;
use crate::domain::isa_ts::types::beta::BetaEnum;
use crate::domain::isa_ts::types::super_bar::SuperBar;
use crate::domain::isa_ts::types::super_foo::SuperFoo;
use serde::{Deserialize, Serialize};

use crate::domain::isa_ts::store::ObjectStore as IsaTsStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-struct-documentation"}}}
/// This object has two supertypes.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-struct-definition"}}}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Gamma {
    pub id: Uuid,
    pub x_value: f64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-implementation"}}}
impl Gamma {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-struct-impl-new"}}}
    /// Inter a new 'Gamma' in the store, and return it's `id`.
    pub fn new(x_value: f64, store: &mut IsaTsStore) -> Gamma {
        let id = Uuid::new_v4();
        let new = Gamma { id, x_value };
        store.inter_gamma(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-beta"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-struct-impl-new_"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-alpha"}}}
    // Navigate to [`Alpha`] across R10(isa)
    pub fn r10_alpha<'a>(&'a self, store: &'a IsaTsStore) -> Vec<&Alpha> {
        vec![store
            .iter_alpha()
            .find(|alpha| {
                if let AlphaEnum::Gamma(id) = alpha.subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-beta"}}}
    // Navigate to [`Beta`] across R11(isa)
    pub fn r11_beta<'a>(&'a self, store: &'a IsaTsStore) -> Vec<&Beta> {
        vec![store
            .iter_beta()
            .find(|beta| {
                if let BetaEnum::Gamma(id) = beta.subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-super_foo"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-super_bar"}}}
    // Navigate to [`SuperBar`] across R12(isa)
    pub fn r12_super_bar<'a>(&'a self, store: &'a IsaTsStore) -> Vec<&SuperBar> {
        vec![store.exhume_super_bar(&self.id).unwrap()]
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-beta"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-alpha"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-beta"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-beta"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-beta"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-beta"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-alpha"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-beta"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-alpha"}}}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-beta"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-alpha"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-beta"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-beta"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-super_foo"}}}
    // Navigate to [`SuperFoo`] across R13(isa)
    pub fn r13_super_foo<'a>(&'a self, store: &'a IsaTsStore) -> Vec<&SuperFoo> {
        vec![store.exhume_super_foo(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

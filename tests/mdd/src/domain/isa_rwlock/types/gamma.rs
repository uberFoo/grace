// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"gamma-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::isa_rwlock::types::alpha::Alpha;
use crate::domain::isa_rwlock::types::alpha::AlphaEnum;
use crate::domain::isa_rwlock::types::beta::Beta;
use crate::domain::isa_rwlock::types::beta::BetaEnum;
use crate::domain::isa_rwlock::types::super_bar::SuperBar;
use crate::domain::isa_rwlock::types::super_foo::SuperFoo;
use serde::{Deserialize, Serialize};

use crate::domain::isa_rwlock::store::ObjectStore as IsaRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-struct-documentation"}}}
/// This object has two supertypes.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
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
    pub fn new(x_value: f64, store: &mut IsaRwlockStore) -> Arc<RwLock<Gamma>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Gamma { id, x_value }));
        store.inter_gamma(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-alpha"}}}
    // Navigate to [`Alpha`] across R10(isa)
    pub fn r10_alpha<'a>(&'a self, store: &'a IsaRwlockStore) -> Vec<Arc<RwLock<Alpha>>> {
        span!("r10_alpha");
        vec![store
            .iter_alpha()
            .find(|alpha| {
                if let AlphaEnum::Gamma(id) = alpha.read().unwrap().subtype {
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
    pub fn r11_beta<'a>(&'a self, store: &'a IsaRwlockStore) -> Vec<Arc<RwLock<Beta>>> {
        span!("r11_beta");
        vec![store
            .iter_beta()
            .find(|beta| {
                if let BetaEnum::Gamma(id) = beta.read().unwrap().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-super_bar"}}}
    // Navigate to [`SuperBar`] across R12(isa)
    pub fn r12_super_bar<'a>(&'a self, store: &'a IsaRwlockStore) -> Vec<Arc<RwLock<SuperBar>>> {
        span!("r12_super_bar");
        vec![store.exhume_super_bar(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"gamma-impl-nav-subtype-to-supertype-super_foo"}}}
    // Navigate to [`SuperFoo`] across R13(isa)
    pub fn r13_super_foo<'a>(&'a self, store: &'a IsaRwlockStore) -> Vec<Arc<RwLock<SuperFoo>>> {
        span!("r13_super_foo");
        vec![store.exhume_super_foo(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"super_bar-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::isa_vec::types::beta::Beta;
use crate::domain::isa_vec::types::beta::BetaEnum;
use crate::domain::isa_vec::types::gamma::Gamma;
use serde::{Deserialize, Serialize};

use crate::domain::isa_vec::store::ObjectStore as IsaVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-hybrid-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct SuperBar {
    pub subtype: SuperBarEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-hybrid-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum SuperBarEnum {
    Gamma(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-implementation"}}}
impl SuperBar {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-struct-impl-new_gamma"}}}
    /// Inter a new SuperBar in the store, and return it's `id`.
    pub fn new_gamma(
        subtype: &Rc<RefCell<Gamma>>,
        store: &mut IsaVecStore,
    ) -> Rc<RefCell<SuperBar>> {
        store.inter_super_bar(|id| {
            Rc::new(RefCell::new(SuperBar {
                subtype: SuperBarEnum::Gamma(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_bar-impl-nav-subtype-to-supertype-beta"}}}
    // Navigate to [`Beta`] across R11(isa)
    pub fn r11_beta<'a>(&'a self, store: &'a IsaVecStore) -> Vec<Rc<RefCell<Beta>>> {
        span!("r11_beta");
        vec![store
            .iter_beta()
            .find(|beta| {
                if let BetaEnum::SuperBar(id) = beta.borrow().subtype {
                    id == self.id
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

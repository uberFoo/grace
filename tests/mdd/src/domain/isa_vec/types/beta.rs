// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"beta-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::isa_vec::types::gamma::Gamma;
use crate::domain::isa_vec::types::super_bar::SuperBar;
use serde::{Deserialize, Serialize};

use crate::domain::isa_vec::store::ObjectStore as IsaVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-hybrid-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Beta {
    pub subtype: BetaEnum,
    pub id: usize,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-hybrid-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum BetaEnum {
    Gamma(usize),
    SuperBar(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-implementation"}}}
impl Beta {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-struct-impl-new_gamma"}}}
    /// Inter a new Beta in the store, and return it's `id`.
    pub fn new_gamma(
        name: String,
        subtype: &Rc<RefCell<Gamma>>,
        store: &mut IsaVecStore,
    ) -> Rc<RefCell<Beta>> {
        store.inter_beta(|id| {
            Rc::new(RefCell::new(Beta {
                name: name.to_owned(),
                subtype: BetaEnum::Gamma(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-struct-impl-new_super_bar"}}}
    /// Inter a new Beta in the store, and return it's `id`.
    pub fn new_super_bar(
        name: String,
        subtype: &Rc<RefCell<SuperBar>>,
        store: &mut IsaVecStore,
    ) -> Rc<RefCell<Beta>> {
        store.inter_beta(|id| {
            Rc::new(RefCell::new(Beta {
                name: name.to_owned(),
                subtype: BetaEnum::SuperBar(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

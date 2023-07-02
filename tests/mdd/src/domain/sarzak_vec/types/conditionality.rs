// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"conditionality-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::sarzak_vec::types::associative_referent::AssociativeReferent;
use crate::domain::sarzak_vec::types::conditional::CONDITIONAL;
use crate::domain::sarzak_vec::types::referent::Referent;
use crate::domain::sarzak_vec::types::referrer::Referrer;
use crate::domain::sarzak_vec::types::unconditional::UNCONDITIONAL;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak_vec::store::ObjectStore as SarzakVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-hybrid-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Conditionality {
    pub subtype: ConditionalityEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-hybrid-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum ConditionalityEnum {
    Conditional(Uuid),
    Unconditional(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-implementation"}}}
impl Conditionality {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-struct-impl-new_conditional"}}}
    /// Inter a new Conditionality in the store, and return it's `id`.
    pub fn new_conditional(store: &mut SarzakVecStore) -> Rc<RefCell<Conditionality>> {
        store.inter_conditionality(|id| {
            Rc::new(RefCell::new(Conditionality {
                subtype: ConditionalityEnum::Conditional(CONDITIONAL),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-struct-impl-new_unconditional"}}}
    /// Inter a new Conditionality in the store, and return it's `id`.
    pub fn new_unconditional(store: &mut SarzakVecStore) -> Rc<RefCell<Conditionality>> {
        store.inter_conditionality(|id| {
            Rc::new(RefCell::new(Conditionality {
                subtype: ConditionalityEnum::Unconditional(UNCONDITIONAL),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-struct-impl-nav-backward-1_M-to-associative_referent"}}}
    /// Navigate to [`AssociativeReferent`] across R77(1-M)
    pub fn r77_associative_referent<'a>(
        &'a self,
        store: &'a SarzakVecStore,
    ) -> Vec<Rc<RefCell<AssociativeReferent>>> {
        span!("r77_associative_referent");
        store
            .iter_associative_referent()
            .filter(|associative_referent| associative_referent.borrow().conditionality == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-struct-impl-nav-backward-1_M-to-referent"}}}
    /// Navigate to [`Referent`] across R12(1-M)
    pub fn r12_referent<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Referent>>> {
        span!("r12_referent");
        store
            .iter_referent()
            .filter(|referent| referent.borrow().conditionality == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"conditionality-struct-impl-nav-backward-1_M-to-referrer"}}}
    /// Navigate to [`Referrer`] across R11(1-M)
    pub fn r11_referrer<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Referrer>>> {
        span!("r11_referrer");
        store
            .iter_referrer()
            .filter(|referrer| referrer.borrow().conditionality == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

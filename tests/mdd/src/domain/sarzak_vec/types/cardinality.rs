// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"cardinality-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"cardinality-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::sarzak_vec::types::associative_referent::AssociativeReferent;
use crate::domain::sarzak_vec::types::associative_referrer::AssociativeReferrer;
use crate::domain::sarzak_vec::types::many::MANY;
use crate::domain::sarzak_vec::types::one::ONE;
use crate::domain::sarzak_vec::types::referent::Referent;
use crate::domain::sarzak_vec::types::referrer::Referrer;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak_vec::store::ObjectStore as SarzakVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"cardinality-hybrid-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Cardinality {
    pub subtype: CardinalityEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"cardinality-hybrid-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum CardinalityEnum {
    Many(Uuid),
    One(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"cardinality-implementation"}}}
impl Cardinality {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"cardinality-struct-impl-new_many"}}}
    /// Inter a new Cardinality in the store, and return it's `id`.
    pub fn new_many(store: &mut SarzakVecStore) -> Rc<RefCell<Cardinality>> {
        store.inter_cardinality(|id| {
            Rc::new(RefCell::new(Cardinality {
                subtype: CardinalityEnum::Many(MANY),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"cardinality-struct-impl-new_one"}}}
    /// Inter a new Cardinality in the store, and return it's `id`.
    pub fn new_one(store: &mut SarzakVecStore) -> Rc<RefCell<Cardinality>> {
        store.inter_cardinality(|id| {
            Rc::new(RefCell::new(Cardinality {
                subtype: CardinalityEnum::One(ONE),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"cardinality-struct-impl-nav-backward-1_M-to-associative_referent"}}}
    /// Navigate to [`AssociativeReferent`] across R88(1-M)
    pub fn r88_associative_referent<'a>(
        &'a self,
        store: &'a SarzakVecStore,
    ) -> Vec<Rc<RefCell<AssociativeReferent>>> {
        span!("r88_associative_referent");
        store
            .iter_associative_referent()
            .filter(|associative_referent| associative_referent.borrow().cardinality == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"cardinality-struct-impl-nav-backward-1_M-to-associative_referrer"}}}
    /// Navigate to [`AssociativeReferrer`] across R89(1-M)
    pub fn r89_associative_referrer<'a>(
        &'a self,
        store: &'a SarzakVecStore,
    ) -> Vec<Rc<RefCell<AssociativeReferrer>>> {
        span!("r89_associative_referrer");
        store
            .iter_associative_referrer()
            .filter(|associative_referrer| associative_referrer.borrow().cardinality == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"cardinality-struct-impl-nav-backward-1_M-to-referent"}}}
    /// Navigate to [`Referent`] across R8(1-M)
    pub fn r8_referent<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Referent>>> {
        span!("r8_referent");
        store
            .iter_referent()
            .filter(|referent| referent.borrow().cardinality == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"cardinality-struct-impl-nav-backward-1_M-to-referrer"}}}
    /// Navigate to [`Referrer`] across R9(1-M)
    pub fn r9_referrer<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Referrer>>> {
        span!("r9_referrer");
        store
            .iter_referrer()
            .filter(|referrer| referrer.borrow().cardinality == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

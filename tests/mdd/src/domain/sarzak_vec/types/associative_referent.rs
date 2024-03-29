// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"associative_referent-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::sarzak_vec::types::an_associative_referent::AnAssociativeReferent;
use crate::domain::sarzak_vec::types::cardinality::Cardinality;
use crate::domain::sarzak_vec::types::conditionality::Conditionality;
use crate::domain::sarzak_vec::types::object::Object;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak_vec::store::ObjectStore as SarzakVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-documentation"}}}
/// The other objects in an Associative Relationship
///
/// This represents one of the two objects that are related in an [`Associative`] relationhip
/// .
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct AssociativeReferent {
    pub description: String,
    pub id: usize,
    /// R88: [`AssociativeReferent`] 'has' [`Cardinality`]
    pub cardinality: usize,
    /// R77: [`AssociativeReferent`] 'has' [`Conditionality`]
    pub conditionality: usize,
    /// R25: [`AssociativeReferent`] 'has other' [`Object`]
    pub obj_id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-implementation"}}}
impl AssociativeReferent {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-new"}}}
    /// Inter a new 'Associative Referent' in the store, and return it's `id`.
    pub fn new(
        description: String,
        cardinality: &Rc<RefCell<Cardinality>>,
        conditionality: &Rc<RefCell<Conditionality>>,
        obj_id: &Rc<RefCell<Object>>,
        store: &mut SarzakVecStore,
    ) -> Rc<RefCell<AssociativeReferent>> {
        store.inter_associative_referent(|id| {
            Rc::new(RefCell::new(AssociativeReferent {
                description: description.to_owned(),
                id,
                cardinality: cardinality.borrow().id,
                conditionality: conditionality.borrow().id,
                obj_id: obj_id.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-nav-forward-to-cardinality"}}}
    /// Navigate to [`Cardinality`] across R88(1-*)
    pub fn r88_cardinality<'a>(
        &'a self,
        store: &'a SarzakVecStore,
    ) -> Vec<Rc<RefCell<Cardinality>>> {
        span!("r88_cardinality");
        vec![store.exhume_cardinality(&self.cardinality).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-nav-forward-to-conditionality"}}}
    /// Navigate to [`Conditionality`] across R77(1-*)
    pub fn r77_conditionality<'a>(
        &'a self,
        store: &'a SarzakVecStore,
    ) -> Vec<Rc<RefCell<Conditionality>>> {
        span!("r77_conditionality");
        vec![store.exhume_conditionality(&self.conditionality).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-nav-forward-to-obj_id"}}}
    /// Navigate to [`Object`] across R25(1-*)
    pub fn r25_object<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Object>>> {
        span!("r25_object");
        vec![store.exhume_object(&self.obj_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-nav-backward-assoc-one-to-an_associative_referent"}}}
    /// Navigate to [`AnAssociativeReferent`] across R22(1-1)
    pub fn r22_an_associative_referent<'a>(
        &'a self,
        store: &'a SarzakVecStore,
    ) -> Vec<Rc<RefCell<AnAssociativeReferent>>> {
        span!("r22_an_associative_referent");
        vec![store
            .iter_an_associative_referent()
            .find(|an_associative_referent| an_associative_referent.borrow().referent == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

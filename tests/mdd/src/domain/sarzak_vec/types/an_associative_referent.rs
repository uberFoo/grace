// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"an_associative_referent-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"an_associative_referent-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::sarzak_vec::types::associative::Associative;
use crate::domain::sarzak_vec::types::associative_referent::AssociativeReferent;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak_vec::store::ObjectStore as SarzakVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"an_associative_referent-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct AnAssociativeReferent {
    pub id: usize,
    pub referential_attribute: String,
    /// R22: [`Associative`] '🚧 Out of order — see sarzak#14.' [`Associative`]
    pub associative: usize,
    /// R22: [`AssociativeReferent`] '🚧 Out of order — see sarzak#14.' [`AssociativeReferent`]
    pub referent: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"an_associative_referent-implementation"}}}
impl AnAssociativeReferent {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"an_associative_referent-struct-impl-new"}}}
    /// Inter a new 'An Associative Referent' in the store, and return it's `id`.
    pub fn new(
        referential_attribute: String,
        associative: &Rc<RefCell<Associative>>,
        referent: &Rc<RefCell<AssociativeReferent>>,
        store: &mut SarzakVecStore,
    ) -> Rc<RefCell<AnAssociativeReferent>> {
        store.inter_an_associative_referent(|id| {
            Rc::new(RefCell::new(AnAssociativeReferent {
                id,
                referential_attribute: referential_attribute.to_owned(),
                associative: associative.borrow().id,
                referent: referent.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"an_associative_referent-struct-impl-nav-forward-assoc-to-associative"}}}
    /// Navigate to [`Associative`] across R22(1-*)
    pub fn r22_associative<'a>(
        &'a self,
        store: &'a SarzakVecStore,
    ) -> Vec<Rc<RefCell<Associative>>> {
        span!("r22_associative");
        vec![store.exhume_associative(&self.associative).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"an_associative_referent-struct-impl-nav-forward-assoc-to-referent"}}}
    /// Navigate to [`AssociativeReferent`] across R22(1-*)
    pub fn r22_associative_referent<'a>(
        &'a self,
        store: &'a SarzakVecStore,
    ) -> Vec<Rc<RefCell<AssociativeReferent>>> {
        span!("r22_associative_referent");
        vec![store.exhume_associative_referent(&self.referent).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

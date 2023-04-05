// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"associative_referent-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-use-statements"}}}
use uuid::Uuid;

use crate::domain::sarzak::types::an_associative_referent::AnAssociativeReferent;
use crate::domain::sarzak::types::cardinality::Cardinality;
use crate::domain::sarzak::types::conditionality::Conditionality;
use crate::domain::sarzak::types::object::Object;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-documentation"}}}
/// The other objects in an Associative Relationship
///
/// This represents one of the two objects that are related in an [`Associative`] relationhip
///.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct AssociativeReferent {
    pub description: String,
    pub id: Uuid,
    /// R88: [`AssociativeReferent`] 'has' [`Cardinality`]
    pub cardinality: Uuid,
    /// R77: [`AssociativeReferent`] 'has' [`Conditionality`]
    pub conditionality: Uuid,
    /// R25: [`AssociativeReferent`] 'has other' [`Object`]
    pub obj_id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-implementation"}}}
impl AssociativeReferent {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-new"}}}
    /// Inter a new 'Associative Referent' in the store, and return it's `id`.
    pub fn new(
        description: String,
        cardinality: &Cardinality,
        conditionality: &Conditionality,
        obj_id: &Object,
        store: &mut SarzakStore,
    ) -> AssociativeReferent {
        let id = Uuid::new_v4();
        let new = AssociativeReferent {
            description: description,
            id: id,
            cardinality: cardinality.id(),
            conditionality: conditionality.id(),
            obj_id: obj_id.id,
        };
        store.inter_associative_referent(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-nav-forward-to-cardinality"}}}
    /// Navigate to [`Cardinality`] across R88(1-*)
    pub fn r88_cardinality<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Cardinality> {
        vec![store.exhume_cardinality(&self.cardinality).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-nav-forward-to-conditionality"}}}
    /// Navigate to [`Conditionality`] across R77(1-*)
    pub fn r77_conditionality<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Conditionality> {
        vec![store.exhume_conditionality(&self.conditionality).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-nav-forward-to-obj_id"}}}
    /// Navigate to [`Object`] across R25(1-*)
    pub fn r25_object<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Object> {
        vec![store.exhume_object(&self.obj_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-nav-backward-one-to-associative"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-nav-backward-cond-to-associative"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-nav-backward-one-to-associative"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-nav-backward-cond-to-associative"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-nav-backward-assoc-one-to-an_associative_referent"}}}
    /// Navigate to [`AnAssociativeReferent`] across R22(1-1)
    pub fn r22_an_associative_referent<'a>(
        &'a self,
        store: &'a SarzakStore,
    ) -> Vec<&AnAssociativeReferent> {
        vec![store
            .iter_an_associative_referent()
            .find(|an_associative_referent| an_associative_referent.referent == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

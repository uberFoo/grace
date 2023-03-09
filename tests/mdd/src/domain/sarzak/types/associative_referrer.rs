// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"associative_referrer-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-use-statements"}}}
use uuid::Uuid;

use crate::domain::sarzak::types::associative::Associative;
use crate::domain::sarzak::types::cardinality::Cardinality;
use crate::domain::sarzak::types::object::Object;
use crate::domain::sarzak::UUID_NS;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-struct-documentation"}}}
/// Associative Object
///
/// This is used in an [`Associative`] relationship to point to the Associative object itself
///. It's the box with the line pointing at another line.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct AssociativeReferrer {
    pub id: Uuid,
    pub one_referential_attribute: String,
    pub other_referential_attribute: String,
    /// R89: [`AssociativeReferrer`] 'has' [`Cardinality`]
    pub cardinality: Uuid,
    /// R26: [`AssociativeReferrer`] 'is also an' [`Object`]
    pub obj_id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-implementation"}}}
impl AssociativeReferrer {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-struct-impl-new"}}}
    /// Inter a new 'Associative Referrer' in the store, and return it's `id`.
    pub fn new(
        one_referential_attribute: String,
        other_referential_attribute: String,
        cardinality: &Cardinality,
        obj_id: &Object,
        store: &mut SarzakStore,
    ) -> AssociativeReferrer {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!(
                "{}:{}:{:?}:{:?}",
                one_referential_attribute, other_referential_attribute, cardinality, obj_id
            )
            .as_bytes(),
        );
        let new = AssociativeReferrer {
            id: id,
            one_referential_attribute: one_referential_attribute,
            cardinality: cardinality.id(),
            obj_id: obj_id.id,
            other_referential_attribute: other_referential_attribute,
        };
        store.inter_associative_referrer(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-struct-impl-nav-forward-to-cardinality"}}}
    /// Navigate to [`Cardinality`] across R89(1-*)
    pub fn r89_cardinality<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Cardinality> {
        vec![store.exhume_cardinality(&self.cardinality).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-struct-impl-nav-forward-to-obj_id"}}}
    /// Navigate to [`Object`] across R26(1-*)
    pub fn r26_object<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Object> {
        vec![store.exhume_object(&self.obj_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-struct-impl-nav-backward-one-to-associative"}}}
    /// Navigate to [`Associative`] across R21(1-1)
    pub fn r21_associative<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Associative> {
        vec![store
            .iter_associative()
            .find(|associative| associative.from == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

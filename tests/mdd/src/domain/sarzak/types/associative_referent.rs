// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"associative_referent-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::domain::sarzak::UUID_NS;

// Referrer imports
use crate::domain::sarzak::types::object::Object;

// Referent imports
use crate::domain::sarzak::types::associative::Associative;

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
    pub id: Uuid,
    /// R25: [`AssociativeReferent`] 'has other' [`Object`]
    pub obj_id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-implementation"}}}
impl AssociativeReferent {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-new"}}}
    /// Inter a new AssociativeReferent in the store, and return it's `id`.
    pub fn new(obj_id: &Object, store: &mut SarzakStore) -> AssociativeReferent {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}", obj_id).as_bytes());
        let new = AssociativeReferent {
            obj_id: obj_id.id,
            id,
        };
        store.inter_associative_referent(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-nav-forward-to-obj_id"}}}
    /// Navigate to [`Object`] across R25(1-?)
    pub fn r25_object<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Object> {
        vec![store.exhume_object(&self.obj_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-nav-backward-one-to-associative"}}}
    /// Navigate to [`Associative`] across R23(1-1)
    pub fn r23_associative<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Associative> {
        vec![
            store
                .iter_associative()
                .find(|associative| associative.1.one == self.id)
                .unwrap()
                .1,
        ]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referent-struct-impl-nav-backward-one-to-associative"}}}
    /// Navigate to [`Associative`] across R22(1-1)
    pub fn r22_associative<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Associative> {
        vec![
            store
                .iter_associative()
                .find(|associative| associative.1.other == self.id)
                .unwrap()
                .1,
        ]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

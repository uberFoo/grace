// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"associative_referrer-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::sarzak_domain::UUID_NS;

// Referrer imports
use crate::sarzak_domain::types::object::Object;

// Referent imports
use crate::sarzak_domain::types::associative::Associative;

use crate::sarzak_domain::store::ObjectStore as SarzakDomainStore;
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
    /// R26: [`AssociativeReferrer`] 'is also an' [`Object`]
    pub obj_id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-implementation"}}}
impl AssociativeReferrer {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-struct-impl-new"}}}
    /// Inter a new AssociativeReferrer in the store, and return it's `id`.
    pub fn new(obj_id: &Object, store: &mut SarzakDomainStore) -> AssociativeReferrer {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}", obj_id).as_bytes());
        let new = AssociativeReferrer {
            obj_id: obj_id.id,
            id,
        };
        store.inter_associative_referrer(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-struct-impl-nav-forward-to-obj_id"}}}
    /// Navigate to [`Object`] across R26(1-?)
    pub fn r26_object<'a>(&'a self, store: &'a SarzakDomainStore) -> Vec<&Object> {
        vec![store.exhume_object(&self.obj_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-struct-impl-nav-backward-one-to-associative"}}}
    /// Navigate to [`Associative`] across R21(1-1)
    pub fn r21_associative<'a>(&'a self, store: &'a SarzakDomainStore) -> Vec<&Associative> {
        vec![
            store
                .iter_associative()
                .find(|associative| associative.1.from == self.id)
                .unwrap()
                .1,
        ]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

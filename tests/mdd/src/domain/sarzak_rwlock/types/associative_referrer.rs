// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"associative_referrer-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::sarzak_rwlock::types::associative::Associative;
use crate::domain::sarzak_rwlock::types::cardinality::Cardinality;
use crate::domain::sarzak_rwlock::types::object::Object;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak_rwlock::store::ObjectStore as SarzakRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-struct-documentation"}}}
/// Associative Object
///
/// This is used in an [`Associative`] relationship to point to the Associative object itself
/// . It's the box with the line pointing at another line.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-struct-definition"}}}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AssociativeReferrer {
    pub id: Uuid,
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
        cardinality: &Arc<RwLock<Cardinality>>,
        obj_id: &Arc<RwLock<Object>>,
        store: &mut SarzakRwlockStore,
    ) -> Arc<RwLock<AssociativeReferrer>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(AssociativeReferrer {
            id,
            cardinality: cardinality.read().unwrap().id(),
            obj_id: obj_id.read().unwrap().id,
        }));
        store.inter_associative_referrer(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-struct-impl-nav-forward-to-cardinality"}}}
    /// Navigate to [`Cardinality`] across R89(1-*)
    pub fn r89_cardinality<'a>(
        &'a self,
        store: &'a SarzakRwlockStore,
    ) -> Vec<Arc<RwLock<Cardinality>>> {
        vec![store.exhume_cardinality(&self.cardinality).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-struct-impl-nav-forward-to-obj_id"}}}
    /// Navigate to [`Object`] across R26(1-*)
    pub fn r26_object<'a>(&'a self, store: &'a SarzakRwlockStore) -> Vec<Arc<RwLock<Object>>> {
        vec![store.exhume_object(&self.obj_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_referrer-struct-impl-nav-backward-one-to-associative"}}}
    /// Navigate to [`Associative`] across R21(1-1)
    pub fn r21_associative<'a>(
        &'a self,
        store: &'a SarzakRwlockStore,
    ) -> Vec<Arc<RwLock<Associative>>> {
        vec![store
            .iter_associative()
            .find(|associative| associative.read().unwrap().from == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

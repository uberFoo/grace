// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"state-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"state-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::sarzak_rwlock::types::acknowledged_event::AcknowledgedEvent;
use crate::domain::sarzak_rwlock::types::object::Object;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak_rwlock::store::ObjectStore as SarzakRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"state-struct-documentation"}}}
/// An [Object] state, more precisely, a set of states, is where all the action happens.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"state-struct-definition"}}}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct State {
    pub id: Uuid,
    pub name: String,
    /// R18: [`State`] 'performs actions on behalf of' [`Object`]
    pub obj_id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"state-implementation"}}}
impl State {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"state-struct-impl-new"}}}
    /// Inter a new 'State' in the store, and return it's `id`.
    pub fn new(
        name: String,
        obj_id: &Arc<RwLock<Object>>,
        store: &mut SarzakRwlockStore,
    ) -> Arc<RwLock<State>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(State {
            id,
            name,
            obj_id: obj_id.read().unwrap().id,
        }));
        store.inter_state(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"state-struct-impl-nav-forward-to-obj_id"}}}
    /// Navigate to [`Object`] across R18(1-*)
    pub fn r18_object<'a>(&'a self, store: &'a SarzakRwlockStore) -> Vec<Arc<RwLock<Object>>> {
        vec![store.exhume_object(&self.obj_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"state-struct-impl-nav-backward-assoc-many-to-acknowledged_event"}}}
    /// Navigate to [`AcknowledgedEvent`] across R20(1-M)
    pub fn r20_acknowledged_event<'a>(
        &'a self,
        store: &'a SarzakRwlockStore,
    ) -> Vec<Arc<RwLock<AcknowledgedEvent>>> {
        store
            .iter_acknowledged_event()
            .filter(|acknowledged_event| acknowledged_event.read().unwrap().state_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

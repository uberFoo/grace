// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"state-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"state-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::associative_rwlock_vec::types::acknowledged_event::AcknowledgedEvent;
use serde::{Deserialize, Serialize};

use crate::domain::associative_rwlock_vec::store::ObjectStore as AssociativeRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"state-struct-documentation"}}}
/// An [Object] state, more precisely, a set of states, is where all the action happens.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"state-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct State {
    pub id: usize,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"state-implementation"}}}
impl State {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"state-struct-impl-new"}}}
    /// Inter a new 'State' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut AssociativeRwlockVecStore) -> Arc<RwLock<State>> {
        store.inter_state(|id| {
            Arc::new(RwLock::new(State {
                id,
                name: name.to_owned(),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"state-struct-impl-nav-backward-assoc-many-to-acknowledged_event"}}}
    /// Navigate to [`AcknowledgedEvent`] across R20(1-M)
    pub fn r20_acknowledged_event<'a>(
        &'a self,
        store: &'a AssociativeRwlockVecStore,
    ) -> Vec<Arc<RwLock<AcknowledgedEvent>>> {
        store
            .iter_acknowledged_event()
            .filter(|acknowledged_event| acknowledged_event.read().unwrap().state_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"state-implementation"}}}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

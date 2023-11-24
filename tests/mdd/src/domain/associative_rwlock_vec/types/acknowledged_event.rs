// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"acknowledged_event-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::associative_rwlock_vec::types::event::Event;
use crate::domain::associative_rwlock_vec::types::state::State;
use serde::{Deserialize, Serialize};

use crate::domain::associative_rwlock_vec::store::ObjectStore as AssociativeRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-struct-documentation"}}}
/// An Event that Does Something
///
/// An acknowledged event is an event that a [`State`] knows how to handle.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AcknowledgedEvent {
    pub id: usize,
    /// R20: [`Event`] '🚧 Comments are out of order — see sarzak#14.' [`Event`]
    pub event_id: usize,
    /// R20: [`State`] '🚧 Comments are out of order — see sarzak#14.' [`State`]
    pub state_id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-implementation"}}}
impl AcknowledgedEvent {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-struct-impl-new"}}}
    /// Inter a new 'Acknowledged Event' in the store, and return it's `id`.
    pub fn new(
        event_id: &Arc<RwLock<Event>>,
        state_id: &Arc<RwLock<State>>,
        store: &mut AssociativeRwlockVecStore,
    ) -> Arc<RwLock<AcknowledgedEvent>> {
        store.inter_acknowledged_event(|id| {
            Arc::new(RwLock::new(AcknowledgedEvent {
                id,
                event_id: event_id.read().unwrap().id,
                state_id: state_id.read().unwrap().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-struct-impl-nav-forward-assoc-to-event_id"}}}
    /// Navigate to [`Event`] across R20(1-*)
    pub fn r20_event<'a>(
        &'a self,
        store: &'a AssociativeRwlockVecStore,
    ) -> Vec<Arc<RwLock<Event>>> {
        vec![store.exhume_event(&self.event_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-struct-impl-nav-forward-assoc-to-state_id"}}}
    /// Navigate to [`State`] across R20(1-*)
    pub fn r20_state<'a>(
        &'a self,
        store: &'a AssociativeRwlockVecStore,
    ) -> Vec<Arc<RwLock<State>>> {
        vec![store.exhume_state(&self.state_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-implementation"}}}
impl PartialEq for AcknowledgedEvent {
    fn eq(&self, other: &Self) -> bool {
        self.event_id == other.event_id && self.state_id == other.state_id
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

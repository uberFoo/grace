// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"acknowledged_event-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::domain::associative_vec::types::event::Event;
use crate::domain::associative_vec::types::state::State;
use serde::{Deserialize, Serialize};

use crate::domain::associative_vec::store::ObjectStore as AssociativeVecStore;
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
        event_id: &Rc<RefCell<Event>>,
        state_id: &Rc<RefCell<State>>,
        store: &mut AssociativeVecStore,
    ) -> Rc<RefCell<AcknowledgedEvent>> {
        store.inter_acknowledged_event(|id| {
            Rc::new(RefCell::new(AcknowledgedEvent {
                id,
                event_id: event_id.borrow().id,
                state_id: state_id.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-struct-impl-nav-forward-assoc-to-event_id"}}}
    /// Navigate to [`Event`] across R20(1-*)
    pub fn r20_event<'a>(&'a self, store: &'a AssociativeVecStore) -> Vec<Rc<RefCell<Event>>> {
        vec![store.exhume_event(&self.event_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-struct-impl-nav-forward-assoc-to-state_id"}}}
    /// Navigate to [`State`] across R20(1-*)
    pub fn r20_state<'a>(&'a self, store: &'a AssociativeVecStore) -> Vec<Rc<RefCell<State>>> {
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

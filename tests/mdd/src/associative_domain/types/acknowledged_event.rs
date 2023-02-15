// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"acknowledged_event-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::associative_domain::UUID_NS;

// Referent imports
use crate::associative_domain::types::event::Event;
use crate::associative_domain::types::state::State;

use crate::associative_domain::store::ObjectStore as AssociativeDomainStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"acknowledged_event-const-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"acknowledged_event-struct-documentation"}}}
/// An Event that Does Something
///
/// An acknowledged event is an event that a [`State`] knows how to handle.
// {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-const-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct AcknowledgedEvent {
    pub id: Uuid,
    /// R20: [`Event`] '🚧 Out of order — see sarzak#14.' [`Event`]
    pub event_id: Uuid,
    /// R20: [`State`] '🚧 Out of order — see sarzak#14.' [`State`]
    pub state_id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"acknowledged_event-implementation"}}}
impl AcknowledgedEvent {
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"acknowledged_event-struct-impl-new"}}}
    /// Inter a new AcknowledgedEvent in the store, and return it's `id`.
    //     pub fn new(store: &mut AssociativeDomainStore) -> AcknowledgedEvent {
    //         let id = Uuid::new_v5(&UUID_NS, format!("",).as_bytes());
    //         let new = AcknowledgedEvent { id };
    pub fn new(
        //         event_id: Option<&Event>,
        event_id: &Event,
        state_id: &State,
        store: &mut AssociativeDomainStore,
    ) -> AcknowledgedEvent {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}:{:?}", event_id, state_id).as_bytes(),
        );
        let new = AcknowledgedEvent {
            //             event_id: event_id.map(|event| event.id),
            event_id: event_id.id,
            state_id: state_id.id,
            id,
        };
        store.inter_acknowledged_event(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"acknowledged_event-struct-impl-nav-forward-assoc-to-event_id"}}}
    /// Navigate to [`Event`] across R20(1-?)
    pub fn event<'a>(&'a self, store: &'a AssociativeDomainStore) -> Vec<&Event> {
        vec![store.exhume_event(&self.event_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"acknowledged_event-struct-impl-nav-forward-assoc-to-state_id"}}}
    /// Navigate to [`State`] across R20(1-?)
    pub fn state<'a>(&'a self, store: &'a AssociativeDomainStore) -> Vec<&State> {
        vec![store.exhume_state(&self.state_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
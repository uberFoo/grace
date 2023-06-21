// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"event-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-use-statements"}}}
use uuid::Uuid;

use crate::domain::associative::types::acknowledged_event::AcknowledgedEvent;
use serde::{Deserialize, Serialize};

use crate::domain::associative::store::ObjectStore as AssociativeStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-struct-documentation"}}}
/// An event is sent to an object, and processed by the current state. Assuming it accepts the
///  event. Otherwise it’s dropped on the floor.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-implementation"}}}
impl Event {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-struct-impl-new_"}}}
    /// Inter a new 'Event' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut AssociativeStore) -> Event {
        let id = Uuid::new_v4();
        let new = Event { id, name };
        store.inter_event(new.clone());
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-struct-impl-new"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-struct-impl-new_"}}}
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-struct-impl-nav-backward-assoc_many-to-acknowledged_event"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-struct-impl-nav-backward-assoc-many-to-acknowledged_event"}}}
    /// Navigate to [`AcknowledgedEvent`] across R20(1-M)
    pub fn r20_acknowledged_event<'a>(
        &'a self,
        store: &'a AssociativeStore,
    ) -> Vec<&AcknowledgedEvent> {
        store
            .iter_acknowledged_event()
            .filter(|acknowledged_event| acknowledged_event.event_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

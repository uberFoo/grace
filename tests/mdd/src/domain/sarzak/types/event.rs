// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"event-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-use-statements"}}}
use uuid::Uuid;

use crate::domain::sarzak::types::acknowledged_event::AcknowledgedEvent;
use crate::domain::sarzak::types::object::Object;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-struct-documentation"}}}
/// An event is sent to an object, and processed by the current state. Assuming it accepts the
/// event. Otherwise it’s dropped on the floor.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    /// R19: [`Event`] 'triggers state transitions on' [`Object`]
    pub obj_id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-implementation"}}}
impl Event {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-struct-impl-new_"}}}
    /// Inter a new 'Event' in the store, and return it's `id`.
    pub fn new(name: String, obj_id: &Object, store: &mut SarzakStore) -> Event {
        let id = Uuid::new_v4();
        let new = Event {
            id: id,
            name: name,
            obj_id: obj_id.id,
        };
        store.inter_event(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-struct-impl-new_"}}}
    /// Inter a new 'Event' in the store, and return it's `id`.
    pub fn new_(name: String, obj_id: &Object) -> Event {
        let id = Uuid::new_v4();
        let new = Event {
            id: id,
            name: name,
            obj_id: obj_id.id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-struct-impl-nav-forward-to-obj_id"}}}
    /// Navigate to [`Object`] across R19(1-*)
    pub fn r19_object<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Object> {
        vec![store.exhume_object(&self.obj_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-struct-impl-nav-backward-assoc_many-to-acknowledged_event"}}}
    /// Navigate to [`AcknowledgedEvent`] across R20(1-M)
    pub fn r20_acknowledged_event<'a>(&'a self, store: &'a SarzakStore) -> Vec<&AcknowledgedEvent> {
        store
            .iter_acknowledged_event()
            .filter_map(|acknowledged_event| {
                if acknowledged_event.event_id == self.id {
                    Some(acknowledged_event)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

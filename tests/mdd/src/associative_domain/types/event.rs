// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"event-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::associative_domain::UUID_NS;

// Referrer imports
use crate::associative_domain::types::acknowledged_event::AcknowledgedEvent;

use crate::associative_domain::store::ObjectStore as AssociativeDomainStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"event-struct-documentation"}}}
/// An event is sent to an object, and processed by the current state. Assuming it accepts the event. Otherwise it’s dropped on the floor.
// {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"event-implementation"}}}
impl Event {
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"event-struct-impl-new"}}}
    /// Inter a new Event in the store, and return it's `id`.
    pub fn new(name: String, store: &mut AssociativeDomainStore) -> Event {
        let id = Uuid::new_v5(&UUID_NS, format!("{}", name).as_bytes());
        let new = Event { name: name, id };
        store.inter_event(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"event-struct-impl-nav-backward-assoc-one-to-acknowledged_event"}}}
    /// Navigate to [`AcknowledgedEvent`] across R20(1-1)
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"event-struct-impl-nav-backward-cond-to-acknowledged_event"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"event-struct-impl-nav-backward-assoc-one-cond-to-acknowledged_event"}}}
    /// Navigate to [`AcknowledgedEvent`] across R20(1-1c)
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"event-struct-impl-nav-backward-assoc-many-cond-to-acknowledged_event"}}}
    /// Navigate to [`AcknowledgedEvent`] across R20(1-Mc)
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"event-struct-impl-nav-backward-assoc_many-to-acknowledged_event"}}}
    /// Navigate to [`AcknowledgedEvent`] across R20(1-M)
    pub fn acknowledged_event<'a>(
        &'a self,
        store: &'a AssociativeDomainStore,
    ) -> Vec<&AcknowledgedEvent> {
        //         vec![
        //             store
        //                 .iter_acknowledged_event()
        //                 .find(|acknowledged_event| acknowledged_event.1.event_id == self.id)
        //                 .unwrap()
        //                 .1,
        //         ]
        //         let acknowledged_event = store
        //             .iter_acknowledged_event()
        //             .find(|acknowledged_event| acknowledged_event.1.event_id == self.id);
        //         match acknowledged_event {
        //             Some(ref acknowledged_event) => vec![acknowledged_event.1],
        //             None => Vec::new(),
        //         }
        //         vec![
        //             store
        //                 .iter_acknowledged_event()
        //                 .find(|acknowledged_event| acknowledged_event.1.event_id == self.id)
        //                 .unwrap()
        //                 .1,
        //         ]
        store
            .iter_acknowledged_event()
            .filter_map(|acknowledged_event| {
                //                 if acknowledged_event.1.event_id == Some(self.id) {
                if acknowledged_event.1.event_id == self.id {
                    Some(acknowledged_event.1)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

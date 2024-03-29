// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"state-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"state-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::sarzak_vec::types::acknowledged_event::AcknowledgedEvent;
use crate::domain::sarzak_vec::types::object::Object;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak_vec::store::ObjectStore as SarzakVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"state-struct-documentation"}}}
/// An [Object] state, more precisely, a set of states, is where all the action happens.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"state-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct State {
    pub id: usize,
    pub name: String,
    /// R18: [`State`] 'performs actions on behalf of' [`Object`]
    pub obj_id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"state-implementation"}}}
impl State {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"state-struct-impl-new"}}}
    /// Inter a new 'State' in the store, and return it's `id`.
    pub fn new(
        name: String,
        obj_id: &Rc<RefCell<Object>>,
        store: &mut SarzakVecStore,
    ) -> Rc<RefCell<State>> {
        store.inter_state(|id| {
            Rc::new(RefCell::new(State {
                id,
                name: name.to_owned(),
                obj_id: obj_id.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"state-struct-impl-nav-forward-to-obj_id"}}}
    /// Navigate to [`Object`] across R18(1-*)
    pub fn r18_object<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Object>>> {
        span!("r18_object");
        vec![store.exhume_object(&self.obj_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"state-struct-impl-nav-backward-assoc-many-to-acknowledged_event"}}}
    /// Navigate to [`AcknowledgedEvent`] across R20(1-M)
    pub fn r20_acknowledged_event<'a>(
        &'a self,
        store: &'a SarzakVecStore,
    ) -> Vec<Rc<RefCell<AcknowledgedEvent>>> {
        span!("r20_acknowledged_event");
        store
            .iter_acknowledged_event()
            .filter(|acknowledged_event| acknowledged_event.borrow().state_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

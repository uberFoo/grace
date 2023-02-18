// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"rando_object-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::domain::everything::UUID_NS;

// Referent imports
use crate::domain::everything::types::everything::Everything;

use crate::domain::everything::store::ObjectStore as EverythingStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-struct-documentation"}}}
/// Just some random object with which we wish to relate
///
/// How tawdry.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct RandoObject {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-implementation"}}}
impl RandoObject {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-struct-impl-new"}}}
    /// Inter a new RandoObject in the store, and return it's `id`.
    pub fn new(name: String, store: &mut EverythingStore) -> RandoObject {
        let id = Uuid::new_v5(&UUID_NS, format!("{}", name).as_bytes());
        let new = RandoObject { name: name, id };
        store.inter_rando_object(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-struct-impl-nav-backward-one-to-everything"}}}
    /// Navigate to [`Everything`] across R1(1-1)
    pub fn r1_everything<'a>(&'a self, store: &'a EverythingStore) -> Vec<&Everything> {
        vec![
            store
                .iter_everything()
                .find(|everything| everything.1.rando == self.id)
                .unwrap()
                .1,
        ]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

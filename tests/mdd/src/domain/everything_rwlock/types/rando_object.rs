// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"rando_object-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::everything_rwlock::types::everything::Everything;
use serde::{Deserialize, Serialize};

use crate::domain::everything_rwlock::store::ObjectStore as EverythingRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-struct-documentation"}}}
/// Just some random object with which we wish to relate
///
/// How tawdry.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-struct-definition"}}}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RandoObject {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-implementation"}}}
impl RandoObject {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-struct-impl-new"}}}
    /// Inter a new 'Rando Object' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut EverythingRwlockStore) -> Arc<RwLock<RandoObject>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(RandoObject { id, name }));
        store.inter_rando_object(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-struct-impl-nav-backward-one-to-everything"}}}
    /// Navigate to [`Everything`] across R1(1-1)
    pub fn r1_everything<'a>(
        &'a self,
        store: &'a EverythingRwlockStore,
    ) -> Vec<Arc<RwLock<Everything>>> {
        span!("r1_everything");
        vec![store
            .iter_everything()
            .find(|everything| everything.read().unwrap().rando == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

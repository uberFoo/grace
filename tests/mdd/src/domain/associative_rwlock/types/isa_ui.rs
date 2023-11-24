// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"isa_ui-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::associative_rwlock::types::subtype_anchor::SubtypeAnchor;
use serde::{Deserialize, Serialize};

use crate::domain::associative_rwlock::store::ObjectStore as AssociativeRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-documentation"}}}
/// This represents additional data necessary to render an `Isa` relationship in the user interface
/// .
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-definition"}}}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IsaUi {
    pub id: Uuid,
    pub number: i64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-implementation"}}}
impl IsaUi {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-impl-new"}}}
    /// Inter a new 'IsaUI' in the store, and return it's `id`.
    pub fn new(number: i64, store: &mut AssociativeRwlockStore) -> Arc<RwLock<IsaUi>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(IsaUi { id, number }));
        store.inter_isa_ui(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-impl-nav-backward-assoc-many-to-subtype_anchor"}}}
    /// Navigate to [`SubtypeAnchor`] across R10(1-M)
    pub fn r10_subtype_anchor<'a>(
        &'a self,
        store: &'a AssociativeRwlockStore,
    ) -> Vec<Arc<RwLock<SubtypeAnchor>>> {
        store
            .iter_subtype_anchor()
            .filter(|subtype_anchor| subtype_anchor.read().unwrap().isaui_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

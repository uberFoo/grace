// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"isa_ui-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::associative_rwlock_vec::types::subtype_anchor::SubtypeAnchor;
use serde::{Deserialize, Serialize};

use crate::domain::associative_rwlock_vec::store::ObjectStore as AssociativeRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-documentation"}}}
/// This represents additional data necessary to render an `Isa` relationship in the user interface
/// .
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IsaUi {
    pub id: usize,
    pub number: i64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-implementation"}}}
impl IsaUi {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-impl-new"}}}
    /// Inter a new 'IsaUI' in the store, and return it's `id`.
    pub fn new(number: i64, store: &mut AssociativeRwlockVecStore) -> Arc<RwLock<IsaUi>> {
        store.inter_isa_ui(|id| Arc::new(RwLock::new(IsaUi { id, number })))
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-impl-nav-backward-assoc-many-to-subtype_anchor"}}}
    /// Navigate to [`SubtypeAnchor`] across R10(1-M)
    pub fn r10_subtype_anchor<'a>(
        &'a self,
        store: &'a AssociativeRwlockVecStore,
    ) -> Vec<Arc<RwLock<SubtypeAnchor>>> {
        span!("r10_subtype_anchor");
        store
            .iter_subtype_anchor()
            .filter(|subtype_anchor| subtype_anchor.read().unwrap().isaui_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-implementation"}}}
impl PartialEq for IsaUi {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

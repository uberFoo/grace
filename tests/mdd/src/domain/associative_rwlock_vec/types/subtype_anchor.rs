// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"subtype_anchor-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::associative_rwlock_vec::types::anchor::Anchor;
use crate::domain::associative_rwlock_vec::types::isa_ui::IsaUi;
use serde::{Deserialize, Serialize};

use crate::domain::associative_rwlock_vec::store::ObjectStore as AssociativeRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-struct-documentation"}}}
/// Subtype Anchor
///
/// Just as it sounds, these are [`Anchor`]s used by [`Subtype`]s in an [`Isa`] relationship
/// .
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubtypeAnchor {
    pub id: usize,
    /// R10: [`Anchor`] '🚧 Comments are out of order — see sarzak#14.' [`Anchor`]
    pub anchor_id: usize,
    /// R10: [`IsaUi`] '🚧 Comments are out of order — see sarzak#14.' [`IsaUi`]
    pub isaui_id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-implementation"}}}
impl SubtypeAnchor {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-struct-impl-new"}}}
    /// Inter a new 'Subtype Anchor' in the store, and return it's `id`.
    pub fn new(
        anchor_id: &Arc<RwLock<Anchor>>,
        isaui_id: &Arc<RwLock<IsaUi>>,
        store: &mut AssociativeRwlockVecStore,
    ) -> Arc<RwLock<SubtypeAnchor>> {
        store.inter_subtype_anchor(|id| {
            Arc::new(RwLock::new(SubtypeAnchor {
                id,
                anchor_id: anchor_id.read().unwrap().id,
                isaui_id: isaui_id.read().unwrap().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-struct-impl-nav-forward-assoc-to-anchor_id"}}}
    /// Navigate to [`Anchor`] across R10(1-*)
    pub fn r10_anchor<'a>(
        &'a self,
        store: &'a AssociativeRwlockVecStore,
    ) -> Vec<Arc<RwLock<Anchor>>> {
        span!("r10_anchor");
        vec![store.exhume_anchor(&self.anchor_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-struct-impl-nav-forward-assoc-to-isaui_id"}}}
    /// Navigate to [`IsaUi`] across R10(1-*)
    pub fn r10_isa_ui<'a>(
        &'a self,
        store: &'a AssociativeRwlockVecStore,
    ) -> Vec<Arc<RwLock<IsaUi>>> {
        span!("r10_isa_ui");
        vec![store.exhume_isa_ui(&self.isaui_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-implementation"}}}
impl PartialEq for SubtypeAnchor {
    fn eq(&self, other: &Self) -> bool {
        self.anchor_id == other.anchor_id && self.isaui_id == other.isaui_id
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

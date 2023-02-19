// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"subtype_anchor-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::domain::associative::UUID_NS;

// Referent imports
use crate::domain::associative::types::anchor::Anchor;
use crate::domain::associative::types::isa_ui::IsaUi;

use crate::domain::associative::store::ObjectStore as AssociativeStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-struct-documentation"}}}
/// Subtype Anchor
///
/// Just as it sounds, these are [`Anchor`]s used by [`Subtype`]s in an [`Isa`] relationship
///.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct SubtypeAnchor {
    pub id: Uuid,
    /// R10: [`IsaUi`] '🚧 Out of order — see sarzak#14.' [`IsaUi`]
    pub isaui_id: Uuid,
    /// R10: [`Anchor`] '🚧 Out of order — see sarzak#14.' [`Anchor`]
    pub anchor_id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-implementation"}}}
impl SubtypeAnchor {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-struct-impl-new"}}}
    /// Inter a new SubtypeAnchor in the store, and return it's `id`.
    pub fn new(
        isaui_id: &IsaUi,
        anchor_id: &Anchor,
        store: &mut AssociativeStore,
    ) -> SubtypeAnchor {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}:{:?}", isaui_id, anchor_id).as_bytes(),
        );
        let new = SubtypeAnchor {
            isaui_id: isaui_id.id,
            anchor_id: anchor_id.id,
            id,
        };
        store.inter_subtype_anchor(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-struct-impl-nav-forward-assoc-to-isaui_id"}}}
    /// Navigate to [`IsaUi`] across R10(1-?)
    pub fn r10_isa_ui<'a>(&'a self, store: &'a AssociativeStore) -> Vec<&IsaUi> {
        vec![store.exhume_isa_ui(&self.isaui_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-struct-impl-nav-forward-assoc-to-anchor_id"}}}
    /// Navigate to [`Anchor`] across R10(1-?)
    pub fn r10_anchor<'a>(&'a self, store: &'a AssociativeStore) -> Vec<&Anchor> {
        vec![store.exhume_anchor(&self.anchor_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"subtype_anchor-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::associative_vec::types::anchor::Anchor;
use crate::domain::associative_vec::types::isa_ui::IsaUi;
use serde::{Deserialize, Serialize};

use crate::domain::associative_vec::store::ObjectStore as AssociativeVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-struct-documentation"}}}
/// Subtype Anchor
///
/// Just as it sounds, these are [`Anchor`]s used by [`Subtype`]s in an [`Isa`] relationship
/// .
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct SubtypeAnchor {
    pub id: usize,
    /// R10: [`Anchor`] '🚧 Out of order — see sarzak#14.' [`Anchor`]
    pub anchor_id: usize,
    /// R10: [`IsaUi`] '🚧 Out of order — see sarzak#14.' [`IsaUi`]
    pub isaui_id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-implementation"}}}
impl SubtypeAnchor {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-struct-impl-new"}}}
    /// Inter a new 'Subtype Anchor' in the store, and return it's `id`.
    pub fn new(
        anchor_id: &Rc<RefCell<Anchor>>,
        isaui_id: &Rc<RefCell<IsaUi>>,
        store: &mut AssociativeVecStore,
    ) -> Rc<RefCell<SubtypeAnchor>> {
        store.inter_subtype_anchor(|id| {
            Rc::new(RefCell::new(SubtypeAnchor {
                id,
                anchor_id: anchor_id.borrow().id,
                isaui_id: isaui_id.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-struct-impl-nav-forward-assoc-to-anchor_id"}}}
    /// Navigate to [`Anchor`] across R10(1-*)
    pub fn r10_anchor<'a>(&'a self, store: &'a AssociativeVecStore) -> Vec<Rc<RefCell<Anchor>>> {
        span!("r10_anchor");
        vec![store.exhume_anchor(self.anchor_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_anchor-struct-impl-nav-forward-assoc-to-isaui_id"}}}
    /// Navigate to [`IsaUi`] across R10(1-*)
    pub fn r10_isa_ui<'a>(&'a self, store: &'a AssociativeVecStore) -> Vec<Rc<RefCell<IsaUi>>> {
        span!("r10_isa_ui");
        vec![store.exhume_isa_ui(self.isaui_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

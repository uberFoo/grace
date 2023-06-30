// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"anchor-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::associative_vec::types::subtype_anchor::SubtypeAnchor;
use serde::{Deserialize, Serialize};

use crate::domain::associative_vec::store::ObjectStore as AssociativeVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-documentation"}}}
/// An anchor, or anchor point, is the location where an arrow from a relationship attached
///  to an object.
///
/// Rather than storing the `x` and `y` coordinates of where the anchor attaches, we are related
///  to an [Edge], which is related to a box, which is related to the [Object] to which we are
///  attached. This of course completes the circuit from the [Relationship] for which we are
///  drawing the lines in the first place.
///
/// Anchor also contains a direction, so that we know the orientation to draw the arrows. Finally
/// , there is an offset, which is a point that describes the offset from the anchor for the
///  relationship phrase.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Anchor {
    pub id: usize,
    pub number: i64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-implementation"}}}
impl Anchor {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-new"}}}
    /// Inter a new 'Anchor' in the store, and return it's `id`.
    pub fn new(number: i64, store: &mut AssociativeVecStore) -> Rc<RefCell<Anchor>> {
        store.inter_anchor(|id| Rc::new(RefCell::new(Anchor { id, number })))
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"anchor-struct-impl-nav-backward-assoc-one-cond-to-subtype_anchor"}}}
    /// Navigate to [`SubtypeAnchor`] across R10(1-1c)
    pub fn r10_subtype_anchor<'a>(
        &'a self,
        store: &'a AssociativeVecStore,
    ) -> Vec<Rc<RefCell<SubtypeAnchor>>> {
        span!("r10_subtype_anchor");
        let subtype_anchor = store
            .iter_subtype_anchor()
            .find(|subtype_anchor| subtype_anchor.borrow().anchor_id == self.id);
        match subtype_anchor {
            Some(subtype_anchor) => vec![subtype_anchor],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

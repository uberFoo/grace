// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"isa_ui-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::associative_domain::UUID_NS;

// Referrer imports
use crate::associative_domain::types::subtype_anchor::SubtypeAnchor;

use crate::associative_domain::store::ObjectStore as AssociativeDomainStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-documentation"}}}
/// This represents additional data necessary to render an `Isa` relationship in the user interface
///.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct IsaUi {
    pub id: Uuid,
    pub number: i64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-implementation"}}}
impl IsaUi {
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"isa_ui-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-impl-new"}}}
    /// Inter a new IsaUi in the store, and return it's `id`.
    pub fn new(number: i64, store: &mut AssociativeDomainStore) -> IsaUi {
        let id = Uuid::new_v5(&UUID_NS, format!("{}", number).as_bytes());
        let new = IsaUi { number: number, id };
        store.inter_isa_ui(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"isa_ui-struct-impl-nav-backward-assoc_many-to-subtype_anchor"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_ui-struct-impl-nav-backward-assoc_many-to-subtype_anchor"}}}
    /// Navigate to [`SubtypeAnchor`] across R10(1-M)
    pub fn subtype_anchor<'a>(&'a self, store: &'a AssociativeDomainStore) -> Vec<&SubtypeAnchor> {
        store
            .iter_subtype_anchor()
            .filter_map(|subtype_anchor| {
                if subtype_anchor.1.isaui_id == self.id {
                    Some(subtype_anchor.1)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

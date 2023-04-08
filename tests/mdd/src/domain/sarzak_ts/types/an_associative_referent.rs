// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"an_associative_referent-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"an_associative_referent-use-statements"}}}
use uuid::Uuid;

use crate::domain::sarzak_ts::types::associative::Associative;
use crate::domain::sarzak_ts::types::associative_referent::AssociativeReferent;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak_ts::store::ObjectStore as SarzakTsStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"an_associative_referent-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct AnAssociativeReferent {
    pub id: Uuid,
    pub referential_attribute: String,
    /// R22: [`Associative`] '🚧 Out of order — see sarzak#14.' [`Associative`]
    pub associative: Uuid,
    /// R22: [`AssociativeReferent`] '🚧 Out of order — see sarzak#14.' [`AssociativeReferent`]
    pub referent: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"an_associative_referent-implementation"}}}
impl AnAssociativeReferent {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"an_associative_referent-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"an_associative_referent-struct-impl-new_"}}}
    /// Inter a new 'An Associative Referent' in the store, and return it's `id`.
    pub fn new(
        referential_attribute: String,
        associative: &Associative,
        referent: &AssociativeReferent,
        store: &mut SarzakTsStore,
    ) -> AnAssociativeReferent {
        let id = Uuid::new_v4();
        let new = AnAssociativeReferent {
            id: id,
            referential_attribute: referential_attribute,
            associative: associative.id,
            referent: referent.id,
        };
        store.inter_an_associative_referent(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"an_associative_referent-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"an_associative_referent-struct-impl-new_"}}}
    /// Inter a new 'An Associative Referent' in the store, and return it's `id`.
    pub fn new_(
        referential_attribute: String,
        associative: &Associative,
        referent: &AssociativeReferent,
    ) -> AnAssociativeReferent {
        let id = Uuid::new_v4();
        let new = AnAssociativeReferent {
            id: id,
            referential_attribute: referential_attribute,
            associative: associative.id,
            referent: referent.id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"an_associative_referent-struct-impl-nav-forward-assoc-to-associative"}}}
    /// Navigate to [`Associative`] across R22(1-*)
    pub fn r22_associative<'a>(&'a self, store: &'a SarzakTsStore) -> Vec<&Associative> {
        vec![store.exhume_associative(&self.associative).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"an_associative_referent-struct-impl-nav-forward-assoc-to-referent"}}}
    /// Navigate to [`AssociativeReferent`] across R22(1-*)
    pub fn r22_associative_referent<'a>(
        &'a self,
        store: &'a SarzakTsStore,
    ) -> Vec<&AssociativeReferent> {
        vec![store.exhume_associative_referent(&self.referent).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

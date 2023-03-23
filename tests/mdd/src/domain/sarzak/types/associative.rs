// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"associative-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-use-statements"}}}
use uuid::Uuid;

use crate::domain::sarzak::types::an_associative_referent::AnAssociativeReferent;
use crate::domain::sarzak::types::associative_referrer::AssociativeReferrer;
use crate::domain::sarzak::types::relationship::Relationship;
use crate::domain::sarzak::UUID_NS;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Associative {
    pub id: Uuid,
    pub number: i64,
    /// R21: [`Associative`] 'is formalized by' [`AssociativeReferrer`]
    pub from: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-implementation"}}}
impl Associative {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-impl-new"}}}
    /// Inter a new 'Associative' in the store, and return it's `id`.
    pub fn new(number: i64, from: &AssociativeReferrer, store: &mut SarzakStore) -> Associative {
        let id = Uuid::new_v5(&UUID_NS, format!("{}:{:?}", number, from).as_bytes());
        let new = Associative {
            id: id,
            number: number,
            from: from.id,
        };
        store.inter_associative(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-impl-nav-forward-to-one"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-impl-nav-forward-to-other"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-impl-nav-forward-to-other"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-impl-nav-forward-to-one"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-impl-nav-forward-to-from"}}}
    /// Navigate to [`AssociativeReferrer`] across R21(1-*)
    pub fn r21_associative_referrer<'a>(
        &'a self,
        store: &'a SarzakStore,
    ) -> Vec<&AssociativeReferrer> {
        vec![store.exhume_associative_referrer(&self.from).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-impl-nav-backward-assoc_many-to-an_associative_referent"}}}
    /// Navigate to [`AnAssociativeReferent`] across R22(1-M)
    pub fn r22_an_associative_referent<'a>(
        &'a self,
        store: &'a SarzakStore,
    ) -> Vec<&AnAssociativeReferent> {
        store
            .iter_an_associative_referent()
            .filter_map(|an_associative_referent| {
                if an_associative_referent.associative == self.id {
                    Some(an_associative_referent)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-impl-nav-subtype-to-supertype-relationship"}}}
    // Navigate to [`Relationship`] across R4(isa)
    pub fn r4_relationship<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Relationship> {
        vec![store.exhume_relationship(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

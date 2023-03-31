// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"cardinality-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"cardinality-use-statements"}}}
use crate::domain::sarzak_ts::store::ObjectStore as SarzakTsStore;
use crate::domain::sarzak_ts::types::associative_referent::AssociativeReferent;
use crate::domain::sarzak_ts::types::associative_referrer::AssociativeReferrer;
use crate::domain::sarzak_ts::types::many::MANY;
use crate::domain::sarzak_ts::types::one::ONE;
use crate::domain::sarzak_ts::types::referent::Referent;
use crate::domain::sarzak_ts::types::referrer::Referrer;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"cardinality-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum Cardinality {
    Many(Uuid),
    One(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"cardinality-implementation"}}}
impl Cardinality {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"cardinality-new-impl"}}}
    /// Create a new instance of Cardinality::Many
    pub fn new_many() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Many(MANY)
    }

    /// Create a new instance of Cardinality::One
    pub fn new_one() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::One(ONE)
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"cardinality-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Cardinality::Many(id) => *id,
            Cardinality::One(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"cardinality-struct-impl-nav-backward-1_M-to-associative_referent"}}}
    /// Navigate to [`AssociativeReferent`] across R88(1-M)
    pub fn r88_associative_referent<'a>(
        &'a self,
        store: &'a SarzakTsStore,
    ) -> Vec<&AssociativeReferent> {
        store
            .iter_associative_referent()
            .filter_map(|associative_referent| {
                if associative_referent.cardinality == self.id() {
                    Some(associative_referent)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"cardinality-struct-impl-nav-backward-1_M-to-associative_referrer"}}}
    /// Navigate to [`AssociativeReferrer`] across R89(1-M)
    pub fn r89_associative_referrer<'a>(
        &'a self,
        store: &'a SarzakTsStore,
    ) -> Vec<&AssociativeReferrer> {
        store
            .iter_associative_referrer()
            .filter_map(|associative_referrer| {
                if associative_referrer.cardinality == self.id() {
                    Some(associative_referrer)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"cardinality-struct-impl-nav-backward-1_M-to-referent"}}}
    /// Navigate to [`Referent`] across R8(1-M)
    pub fn r8_referent<'a>(&'a self, store: &'a SarzakTsStore) -> Vec<&Referent> {
        store
            .iter_referent()
            .filter_map(|referent| {
                if referent.cardinality == self.id() {
                    Some(referent)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"cardinality-struct-impl-nav-backward-1_M-to-referrer"}}}
    /// Navigate to [`Referrer`] across R9(1-M)
    pub fn r9_referrer<'a>(&'a self, store: &'a SarzakTsStore) -> Vec<&Referrer> {
        store
            .iter_referrer()
            .filter_map(|referrer| {
                if referrer.cardinality == self.id() {
                    Some(referrer)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
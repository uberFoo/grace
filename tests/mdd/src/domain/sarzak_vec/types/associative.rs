// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"associative-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::sarzak_vec::types::an_associative_referent::AnAssociativeReferent;
use crate::domain::sarzak_vec::types::associative_referrer::AssociativeReferrer;
use crate::domain::sarzak_vec::types::relationship::Relationship;
use crate::domain::sarzak_vec::types::relationship::RelationshipEnum;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak_vec::store::ObjectStore as SarzakVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Associative {
    pub id: usize,
    pub number: i64,
    /// R21: [`Associative`] 'is formalized by' [`AssociativeReferrer`]
    pub from: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-implementation"}}}
impl Associative {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-impl-new"}}}
    /// Inter a new 'Associative' in the store, and return it's `id`.
    pub fn new(
        number: i64,
        from: &Rc<RefCell<AssociativeReferrer>>,
        store: &mut SarzakVecStore,
    ) -> Rc<RefCell<Associative>> {
        store.inter_associative(|id| {
            Rc::new(RefCell::new(Associative {
                id,
                number,
                from: from.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-impl-nav-forward-to-from"}}}
    /// Navigate to [`AssociativeReferrer`] across R21(1-*)
    pub fn r21_associative_referrer<'a>(
        &'a self,
        store: &'a SarzakVecStore,
    ) -> Vec<Rc<RefCell<AssociativeReferrer>>> {
        span!("r21_associative_referrer");
        vec![store.exhume_associative_referrer(&self.from).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-struct-impl-nav-backward-assoc-many-to-an_associative_referent"}}}
    /// Navigate to [`AnAssociativeReferent`] across R22(1-M)
    pub fn r22_an_associative_referent<'a>(
        &'a self,
        store: &'a SarzakVecStore,
    ) -> Vec<Rc<RefCell<AnAssociativeReferent>>> {
        span!("r22_an_associative_referent");
        store
            .iter_an_associative_referent()
            .filter(|an_associative_referent| {
                an_associative_referent.borrow().associative == self.id
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative-impl-nav-subtype-to-supertype-relationship"}}}
    // Navigate to [`Relationship`] across R4(isa)
    pub fn r4_relationship<'a>(
        &'a self,
        store: &'a SarzakVecStore,
    ) -> Vec<Rc<RefCell<Relationship>>> {
        span!("r4_relationship");
        vec![store
            .iter_relationship()
            .find(|relationship| {
                if let RelationshipEnum::Associative(id) = relationship.borrow().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

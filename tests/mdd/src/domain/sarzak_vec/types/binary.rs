// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"binary-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::sarzak_vec::types::referent::Referent;
use crate::domain::sarzak_vec::types::referrer::Referrer;
use crate::domain::sarzak_vec::types::relationship::Relationship;
use crate::domain::sarzak_vec::types::relationship::RelationshipEnum;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak_vec::store::ObjectStore as SarzakVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-documentation"}}}
/// A `Binary` relationship, as it’s name implies, is a relationship between
/// two objects. It consists of two parts, the `Dependent` end of the
/// relationship and the `Independent` end.
///
/// The former is so named because it has the job of formalizing the
/// relationship. It stores a pointer to the independent object as an attribute.
///
/// The latter is aware of the relationship, but it does not store any
/// information about the relationship. That said, there are means of
/// traversing the relationship from the `Independent` object.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Binary {
    pub id: usize,
    pub number: i64,
    /// R5: [`Binary`] 'loops in the' [`Referent`]
    pub to: usize,
    /// R6: [`Binary`] 'is formalized by' [`Referrer`]
    pub from: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-implementation"}}}
impl Binary {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-new"}}}
    /// Inter a new 'Binary' in the store, and return it's `id`.
    pub fn new(
        number: i64,
        to: &Rc<RefCell<Referent>>,
        from: &Rc<RefCell<Referrer>>,
        store: &mut SarzakVecStore,
    ) -> Rc<RefCell<Binary>> {
        store.inter_binary(|id| {
            Rc::new(RefCell::new(Binary {
                id,
                number,
                to: to.borrow().id,
                from: from.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-nav-forward-to-to"}}}
    /// Navigate to [`Referent`] across R5(1-*)
    pub fn r5_referent<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Referent>>> {
        span!("r5_referent");
        vec![store.exhume_referent(&self.to).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-struct-impl-nav-forward-to-from"}}}
    /// Navigate to [`Referrer`] across R6(1-*)
    pub fn r6_referrer<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Referrer>>> {
        span!("r6_referrer");
        vec![store.exhume_referrer(&self.from).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"binary-impl-nav-subtype-to-supertype-relationship"}}}
    // Navigate to [`Relationship`] across R4(isa)
    pub fn r4_relationship<'a>(
        &'a self,
        store: &'a SarzakVecStore,
    ) -> Vec<Rc<RefCell<Relationship>>> {
        span!("r4_relationship");
        vec![store
            .iter_relationship()
            .find(|relationship| {
                if let RelationshipEnum::Binary(id) = relationship.borrow().subtype {
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

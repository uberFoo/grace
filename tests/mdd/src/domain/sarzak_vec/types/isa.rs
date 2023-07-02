// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"isa-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::sarzak_vec::types::relationship::Relationship;
use crate::domain::sarzak_vec::types::relationship::RelationshipEnum;
use crate::domain::sarzak_vec::types::subtype::Subtype;
use crate::domain::sarzak_vec::types::supertype::Supertype;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak_vec::store::ObjectStore as SarzakVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Isa {
    pub id: usize,
    pub number: i64,
    /// R13: [`Isa`] 'has one' [`Supertype`]
    pub supertype: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-implementation"}}}
impl Isa {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-struct-impl-new"}}}
    /// Inter a new 'Isa' in the store, and return it's `id`.
    pub fn new(
        number: i64,
        supertype: &Rc<RefCell<Supertype>>,
        store: &mut SarzakVecStore,
    ) -> Rc<RefCell<Isa>> {
        store.inter_isa(|id| {
            Rc::new(RefCell::new(Isa {
                id,
                number,
                supertype: supertype.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-struct-impl-nav-forward-to-supertype"}}}
    /// Navigate to [`Supertype`] across R13(1-*)
    pub fn r13_supertype<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Supertype>>> {
        span!("r13_supertype");
        vec![store.exhume_supertype(&self.supertype).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-struct-impl-nav-backward-1_M-to-subtype"}}}
    /// Navigate to [`Subtype`] across R27(1-M)
    pub fn r27_subtype<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Subtype>>> {
        span!("r27_subtype");
        store
            .iter_subtype()
            .filter(|subtype| subtype.borrow().isa == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-impl-nav-subtype-to-supertype-relationship"}}}
    // Navigate to [`Relationship`] across R4(isa)
    pub fn r4_relationship<'a>(
        &'a self,
        store: &'a SarzakVecStore,
    ) -> Vec<Rc<RefCell<Relationship>>> {
        span!("r4_relationship");
        vec![store
            .iter_relationship()
            .find(|relationship| {
                if let RelationshipEnum::Isa(id) = relationship.borrow().subtype {
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

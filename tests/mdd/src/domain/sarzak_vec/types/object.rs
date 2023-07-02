// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"object-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::sarzak_vec::types::associative_referent::AssociativeReferent;
use crate::domain::sarzak_vec::types::associative_referrer::AssociativeReferrer;
use crate::domain::sarzak_vec::types::attribute::Attribute;
use crate::domain::sarzak_vec::types::event::Event;
use crate::domain::sarzak_vec::types::referent::Referent;
use crate::domain::sarzak_vec::types::referrer::Referrer;
use crate::domain::sarzak_vec::types::state::State;
use crate::domain::sarzak_vec::types::subtype::Subtype;
use crate::domain::sarzak_vec::types::supertype::Supertype;
use crate::domain::sarzak_vec::types::ty::Ty;
use crate::domain::sarzak_vec::types::ty::TyEnum;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak_vec::store::ObjectStore as SarzakVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-documentation"}}}
/// An `Object` is a collection of related data. By creating `Object`s, and
/// connecting them with `Relationships` we build a powerful abstraction.
///
/// `Object`s contain [Attribute]s that represent the data that the
/// `Object`encapsulates. All `Object`s have an attribute called `id`, which
/// is a unique identifier for each class of `Object`. The `id` attribute is a
/// version 5 UUID.
///
/// 🐶 {"derive": ["Clone", "Debug", "Deserialize", "Eq", "Hash", "PartialEq", "Serialize
/// "]}
///
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Object {
    pub description: String,
    pub id: usize,
    pub key_letters: String,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-implementation"}}}
impl Object {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-new"}}}
    /// Inter a new 'Object' in the store, and return it's `id`.
    pub fn new(
        description: String,
        key_letters: String,
        name: String,
        store: &mut SarzakVecStore,
    ) -> Rc<RefCell<Object>> {
        store.inter_object(|id| {
            Rc::new(RefCell::new(Object {
                description: description.to_owned(),
                id,
                key_letters: key_letters.to_owned(),
                name: name.to_owned(),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-associative_referent"}}}
    /// Navigate to [`AssociativeReferent`] across R25(1-M)
    pub fn r25_associative_referent<'a>(
        &'a self,
        store: &'a SarzakVecStore,
    ) -> Vec<Rc<RefCell<AssociativeReferent>>> {
        span!("r25_associative_referent");
        store
            .iter_associative_referent()
            .filter(|associative_referent| associative_referent.borrow().obj_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-associative_referrer"}}}
    /// Navigate to [`AssociativeReferrer`] across R26(1-M)
    pub fn r26_associative_referrer<'a>(
        &'a self,
        store: &'a SarzakVecStore,
    ) -> Vec<Rc<RefCell<AssociativeReferrer>>> {
        span!("r26_associative_referrer");
        store
            .iter_associative_referrer()
            .filter(|associative_referrer| associative_referrer.borrow().obj_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-attribute"}}}
    /// Navigate to [`Attribute`] across R1(1-M)
    pub fn r1_attribute<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Attribute>>> {
        span!("r1_attribute");
        store
            .iter_attribute()
            .filter(|attribute| attribute.borrow().obj_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-event"}}}
    /// Navigate to [`Event`] across R19(1-M)
    pub fn r19_event<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Event>>> {
        span!("r19_event");
        store
            .iter_event()
            .filter(|event| event.borrow().obj_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-referent"}}}
    /// Navigate to [`Referent`] across R16(1-M)
    pub fn r16_referent<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Referent>>> {
        span!("r16_referent");
        store
            .iter_referent()
            .filter(|referent| referent.borrow().obj_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-referrer"}}}
    /// Navigate to [`Referrer`] across R17(1-M)
    pub fn r17_referrer<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Referrer>>> {
        span!("r17_referrer");
        store
            .iter_referrer()
            .filter(|referrer| referrer.borrow().obj_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-state"}}}
    /// Navigate to [`State`] across R18(1-M)
    pub fn r18_state<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<State>>> {
        span!("r18_state");
        store
            .iter_state()
            .filter(|state| state.borrow().obj_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-subtype"}}}
    /// Navigate to [`Subtype`] across R15(1-M)
    pub fn r15_subtype<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Subtype>>> {
        span!("r15_subtype");
        store
            .iter_subtype()
            .filter(|subtype| subtype.borrow().obj_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-supertype"}}}
    /// Navigate to [`Supertype`] across R14(1-M)
    pub fn r14_supertype<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Supertype>>> {
        span!("r14_supertype");
        store
            .iter_supertype()
            .filter(|supertype| supertype.borrow().obj_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-impl-nav-subtype-to-supertype-ty"}}}
    // Navigate to [`Ty`] across R3(isa)
    pub fn r3_ty<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Ty>>> {
        span!("r3_ty");
        vec![store
            .iter_ty()
            .find(|ty| {
                if let TyEnum::Object(id) = ty.borrow().subtype {
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

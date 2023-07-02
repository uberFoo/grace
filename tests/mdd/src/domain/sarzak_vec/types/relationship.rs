// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"relationship-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::sarzak_vec::types::associative::Associative;
use crate::domain::sarzak_vec::types::binary::Binary;
use crate::domain::sarzak_vec::types::isa::Isa;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak_vec::store::ObjectStore as SarzakVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship-hybrid-documentation"}}}
/// A `Relationship` indicates that a set of objects are connected to each other in some manner
/// . Typically it is a _real world_ relationship. In the
/// case of this model it is strictly an abstraction.
///
/// There are three types of `Relationship`: [`Isa`], [`Binary`], and [`Associative`]. Thus
///  `Relationship` is itself the *supertype* in an [`Isa`] relationship. It is a partitioning
///  *supertype-subtype* relationship, rather one of inheritance. As such, it’s  perfectly
///  suited to a rust `enum`! 😃
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship-hybrid-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Relationship {
    pub subtype: RelationshipEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship-hybrid-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum RelationshipEnum {
    Associative(usize),
    Binary(usize),
    Isa(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship-implementation"}}}
impl Relationship {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship-struct-impl-new_associative"}}}
    /// Inter a new Relationship in the store, and return it's `id`.
    pub fn new_associative(
        subtype: &Rc<RefCell<Associative>>,
        store: &mut SarzakVecStore,
    ) -> Rc<RefCell<Relationship>> {
        store.inter_relationship(|id| {
            Rc::new(RefCell::new(Relationship {
                subtype: RelationshipEnum::Associative(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship-struct-impl-new_binary"}}}
    /// Inter a new Relationship in the store, and return it's `id`.
    pub fn new_binary(
        subtype: &Rc<RefCell<Binary>>,
        store: &mut SarzakVecStore,
    ) -> Rc<RefCell<Relationship>> {
        store.inter_relationship(|id| {
            Rc::new(RefCell::new(Relationship {
                subtype: RelationshipEnum::Binary(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship-struct-impl-new_isa"}}}
    /// Inter a new Relationship in the store, and return it's `id`.
    pub fn new_isa(
        subtype: &Rc<RefCell<Isa>>,
        store: &mut SarzakVecStore,
    ) -> Rc<RefCell<Relationship>> {
        store.inter_relationship(|id| {
            Rc::new(RefCell::new(Relationship {
                subtype: RelationshipEnum::Isa(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

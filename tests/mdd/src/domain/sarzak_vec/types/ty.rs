// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"ty-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::sarzak_vec::types::attribute::Attribute;
use crate::domain::sarzak_vec::types::boolean::BOOLEAN;
use crate::domain::sarzak_vec::types::external::External;
use crate::domain::sarzak_vec::types::float::FLOAT;
use crate::domain::sarzak_vec::types::integer::INTEGER;
use crate::domain::sarzak_vec::types::object::Object;
use crate::domain::sarzak_vec::types::s_string::S_STRING;
use crate::domain::sarzak_vec::types::s_uuid::S_UUID;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak_vec::store::ObjectStore as SarzakVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-hybrid-documentation"}}}
/// The type of a value
///
/// There are several values available: [Integer], [Boolean], [Float], [String], and [UUID]
/// .
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-hybrid-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Ty {
    pub subtype: TyEnum,
    pub id: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-hybrid-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum TyEnum {
    Boolean(Uuid),
    External(usize),
    Float(Uuid),
    Integer(Uuid),
    Object(usize),
    SString(Uuid),
    SUuid(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-implementation"}}}
impl Ty {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-struct-impl-new_boolean"}}}
    /// Inter a new Ty in the store, and return it's `id`.
    pub fn new_boolean(store: &mut SarzakVecStore) -> Rc<RefCell<Ty>> {
        store.inter_ty(|id| {
            Rc::new(RefCell::new(Ty {
                subtype: TyEnum::Boolean(BOOLEAN),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-struct-impl-new_external"}}}
    /// Inter a new Ty in the store, and return it's `id`.
    pub fn new_external(
        subtype: &Rc<RefCell<External>>,
        store: &mut SarzakVecStore,
    ) -> Rc<RefCell<Ty>> {
        store.inter_ty(|id| {
            Rc::new(RefCell::new(Ty {
                subtype: TyEnum::External(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-struct-impl-new_float"}}}
    /// Inter a new Ty in the store, and return it's `id`.
    pub fn new_float(store: &mut SarzakVecStore) -> Rc<RefCell<Ty>> {
        store.inter_ty(|id| {
            Rc::new(RefCell::new(Ty {
                subtype: TyEnum::Float(FLOAT),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-struct-impl-new_integer"}}}
    /// Inter a new Ty in the store, and return it's `id`.
    pub fn new_integer(store: &mut SarzakVecStore) -> Rc<RefCell<Ty>> {
        store.inter_ty(|id| {
            Rc::new(RefCell::new(Ty {
                subtype: TyEnum::Integer(INTEGER),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-struct-impl-new_object"}}}
    /// Inter a new Ty in the store, and return it's `id`.
    pub fn new_object(
        subtype: &Rc<RefCell<Object>>,
        store: &mut SarzakVecStore,
    ) -> Rc<RefCell<Ty>> {
        store.inter_ty(|id| {
            Rc::new(RefCell::new(Ty {
                subtype: TyEnum::Object(subtype.borrow().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-struct-impl-new_s_string"}}}
    /// Inter a new Ty in the store, and return it's `id`.
    pub fn new_s_string(store: &mut SarzakVecStore) -> Rc<RefCell<Ty>> {
        store.inter_ty(|id| {
            Rc::new(RefCell::new(Ty {
                subtype: TyEnum::SString(S_STRING),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-struct-impl-new_s_uuid"}}}
    /// Inter a new Ty in the store, and return it's `id`.
    pub fn new_s_uuid(store: &mut SarzakVecStore) -> Rc<RefCell<Ty>> {
        store.inter_ty(|id| {
            Rc::new(RefCell::new(Ty {
                subtype: TyEnum::SUuid(S_UUID),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-struct-impl-nav-backward-one-to-attribute"}}}
    /// Navigate to [`Attribute`] across R2(1-1)
    pub fn r2_attribute<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Attribute>>> {
        span!("r2_attribute");
        vec![store
            .iter_attribute()
            .find(|attribute| attribute.borrow().ty == self.id)
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

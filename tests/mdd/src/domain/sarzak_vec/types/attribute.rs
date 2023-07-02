// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"attribute-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"attribute-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::sarzak_vec::types::object::Object;
use crate::domain::sarzak_vec::types::ty::Ty;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak_vec::store::ObjectStore as SarzakVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"attribute-struct-documentation"}}}
/// An `Attribute` represents a single value. Each value must have a
/// [`Type`], which constrains the values of data that may be assigned to
/// an `Attribute`.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"attribute-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Attribute {
    pub id: usize,
    pub name: String,
    /// R1: [`Attribute`] 'lives in an' [`Object`]
    pub obj_id: usize,
    /// R2: [`Attribute`] 'has a' [`Ty`]
    pub ty: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"attribute-implementation"}}}
impl Attribute {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"attribute-struct-impl-new"}}}
    /// Inter a new 'Attribute' in the store, and return it's `id`.
    pub fn new(
        name: String,
        obj_id: &Rc<RefCell<Object>>,
        ty: &Rc<RefCell<Ty>>,
        store: &mut SarzakVecStore,
    ) -> Rc<RefCell<Attribute>> {
        store.inter_attribute(|id| {
            Rc::new(RefCell::new(Attribute {
                id,
                name: name.to_owned(),
                obj_id: obj_id.borrow().id,
                ty: ty.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"attribute-struct-impl-nav-forward-to-obj_id"}}}
    /// Navigate to [`Object`] across R1(1-*)
    pub fn r1_object<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Object>>> {
        span!("r1_object");
        vec![store.exhume_object(&self.obj_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"attribute-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`Ty`] across R2(1-*)
    pub fn r2_ty<'a>(&'a self, store: &'a SarzakVecStore) -> Vec<Rc<RefCell<Ty>>> {
        span!("r2_ty");
        vec![store.exhume_ty(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

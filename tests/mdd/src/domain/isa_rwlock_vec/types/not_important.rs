// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"not_important-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"not_important-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::isa_rwlock_vec::types::super_t::SuperT;
use serde::{Deserialize, Serialize};

use crate::domain::isa_rwlock_vec::store::ObjectStore as IsaRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"not_important-struct-documentation"}}}
/// Optional Enum Attribute
///
/// This is testing code generation as specified in the description. The specific issue is in
///  the grace Domain code. Here's what it's currently generating. The problem is commented inline
/// .
///
/// ```ignore
///     pub fn new(
///         description: String,
///         name: String,
///         object: Option<&Object>,
///         param: &Parameter,
///         ty: Option<&Ty>,
///         visibility: Option<&Visibility>,
///         store: &mut WoogStore,
///     ) -> ObjectMethod {
///         let id = Uuid::new_v5(
///             &UUID_NS,
///             format!(
///                 "{}:{}:{:?}:{:?}:{:?}:{:?}",
///                 description, name, object, param, ty, visibility
///             )
///             .as_bytes(),
///         );
///         let new = ObjectMethod {
///             description: description,
///             name: name,
///             object: object.map(|object| object.id),
///             param: param.id,
///             ty: ty.map(|ty| ty.id),
///             // Should be visibility.id()
///             visibility: visibility.map(|visibility| visibility.id),
///             id,
///         };
///         store.inter_object_method(new.clone());
///         new
///     }
/// ```
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"not_important-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotImportant {
    pub id: usize,
    pub name: Uuid,
    /// R888: [`NotImportant`] 'is referring to' [`SuperT`]
    pub x_ref: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"not_important-implementation"}}}
impl NotImportant {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"not_important-struct-impl-new"}}}
    /// Inter a new 'Not Important' in the store, and return it's `id`.
    pub fn new(
        name: Uuid,
        x_ref: &Arc<RwLock<SuperT>>,
        store: &mut IsaRwlockVecStore,
    ) -> Arc<RwLock<NotImportant>> {
        store.inter_not_important(|id| {
            Arc::new(RwLock::new(NotImportant {
                id,
                name,
                x_ref: x_ref.read().unwrap().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"not_important-struct-impl-nav-forward-to-x_ref"}}}
    /// Navigate to [`SuperT`] across R888(1-*)
    pub fn r888_super_t<'a>(&'a self, store: &'a IsaRwlockVecStore) -> Vec<Arc<RwLock<SuperT>>> {
        span!("r888_super_t");
        vec![store.exhume_super_t(&self.x_ref).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"not_important-implementation"}}}
impl PartialEq for NotImportant {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.x_ref == other.x_ref
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

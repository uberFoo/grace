// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"not_important-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"not_important-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::isa_domain::UUID_NS;

// Referrer imports
use crate::isa_domain::types::super_t::SuperT;

use crate::isa_domain::store::ObjectStore as IsaDomainStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"not_important-struct-documentation"}}}
/// Optional Enum Attribute
///
/// This is testing code generation as specified in the description. The specific issue is in
/// the grace Domain code. Here's what it's currently generating. The problem is commented inline
///.
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
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct NotImportant {
    pub id: Uuid,
    pub name: Uuid,
    /// R888: [`NotImportant`] 'is referring to' [`SuperT`]
    pub x_ref: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"not_important-implementation"}}}
impl NotImportant {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"not_important-struct-impl-new"}}}
    /// Inter a new NotImportant in the store, and return it's `id`.
    pub fn new(name: Uuid, x_ref: Option<&SuperT>, store: &mut IsaDomainStore) -> NotImportant {
        let id = Uuid::new_v5(&UUID_NS, format!("{}:{:?}", name, x_ref).as_bytes());
        let new = NotImportant {
            name: name,
            x_ref: x_ref.map(|super_t| super_t.id()),
            id,
        };
        store.inter_not_important(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"not_important-struct-impl-nav-forward-cond-to-x_ref"}}}
    /// Navigate to [`SuperT`] across R888(1-?c)
    pub fn r888_super_t<'a>(&'a self, store: &'a IsaDomainStore) -> Vec<&SuperT> {
        match self.x_ref {
            Some(ref x_ref) => vec![store.exhume_super_t(x_ref).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

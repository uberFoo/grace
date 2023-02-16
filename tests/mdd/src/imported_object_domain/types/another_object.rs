// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"another_object-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"another_object-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::imported_object_domain::UUID_NS;

// Referrer imports
use crate::sarzak::types::object::Object;

use crate::imported_object_domain::store::ObjectStore as ImportedObjectDomainStore;
use crate::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"another_object-struct-documentation"}}}
/// This is another object, but different.
///
/// As a side effect, this is going to test being able to collapse a type with a space. It will
/// break, and I’ll have a new feature.
///
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"another_object-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct AnotherObject {
    pub id: Uuid,
    /// R1: [`AnotherObject`] 'points at' [`Object`]
    pub ptr: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"another_object-implementation"}}}
impl AnotherObject {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"another_object-struct-impl-new"}}}
    /// Inter a new AnotherObject in the store, and return it's `id`.
    pub fn new(ptr: &Object, store: &mut ImportedObjectDomainStore) -> AnotherObject {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}", ptr).as_bytes());
        let new = AnotherObject { ptr: ptr.id, id };
        store.inter_another_object(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"another_object-struct-impl-nav-forward-to-ptr"}}}
    /// Navigate to [`Object`] across R1(1-?)
    pub fn r1_object<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Object> {
        vec![store.exhume_object(&self.ptr).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

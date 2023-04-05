// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"another_object-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"another_object-use-statements"}}}
use uuid::Uuid;

use crate::domain::isa::types::simple_supertype::SimpleSupertype;
use crate::domain::sarzak::types::object::Object;
use serde::{Deserialize, Serialize};

use crate::domain::imported_object::store::ObjectStore as ImportedObjectStore;
use crate::domain::isa::store::ObjectStore as IsaStore;
use crate::domain::sarzak::store::ObjectStore as SarzakStore;
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
    /// R2: [`AnotherObject`] 'has a' [`SimpleSupertype`]
    pub edge: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"another_object-implementation"}}}
impl AnotherObject {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"another_object-struct-impl-new"}}}
    /// Inter a new 'Another Object' in the store, and return it's `id`.
    pub fn new(
        ptr: &Object,
        edge: &SimpleSupertype,
        store: &mut ImportedObjectStore,
    ) -> AnotherObject {
        let id = Uuid::new_v4();
        let new = AnotherObject {
            id: id,
            ptr: ptr.id,
            edge: edge.id,
        };
        store.inter_another_object(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"another_object-struct-impl-nav-forward-to-edge"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"another_object-struct-impl-nav-forward-to-ptr"}}}
    /// Navigate to [`Object`] across R1(1-*)
    pub fn r1_object<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Object> {
        vec![store.exhume_object(&self.ptr).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"another_object-struct-impl-nav-forward-to-edge"}}}
    /// Navigate to [`SimpleSupertype`] across R2(1-*)
    pub fn r2_simple_supertype<'a>(&'a self, store: &'a IsaStore) -> Vec<&SimpleSupertype> {
        vec![store.exhume_simple_supertype(&self.edge).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

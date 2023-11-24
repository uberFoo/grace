// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"another_object-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"another_object-use-statements"}}}
use uuid::Uuid;

use domain::isa::types::simple_supertype::SimpleSupertype;
use domain::sarzak::types::object::Object;
use serde::{Deserialize, Serialize};

use crate::domain::imported_object_vec::store::ObjectStore as ImportedObjectVecStore;
use domain::isa::store::ObjectStore as IsaStore;
use domain::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"another_object-struct-documentation"}}}
/// This is another object, but different.
///
/// As a side effect, this is going to test being able to collapse a type with a space. It will
///  break, and I’ll have a new feature.
///
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"another_object-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnotherObject {
    pub id: Uuid,
    /// R2: [`AnotherObject`] 'has a' [`SimpleSupertype`]
    pub edge: Uuid,
    /// R1: [`AnotherObject`] 'points at' [`Object`]
    pub ptr: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"another_object-implementation"}}}
impl AnotherObject {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"another_object-struct-impl-new"}}}
    /// Inter a new 'Another Object' in the store, and return it's `id`.
    pub fn new(
        edge: &SimpleSupertype,
        ptr: &Object,
        store: &mut ImportedObjectVecStore,
    ) -> AnotherObject {
        let id = Uuid::new_v4();
        let new = AnotherObject {
            id,
            edge: edge.id,
            ptr: ptr.id,
        };
        store.inter_another_object(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"another_object-struct-impl-nav-forward-to-ptr"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"another_object-struct-impl-nav-forward-to-edge"}}}
    /// Navigate to [`SimpleSupertype`] across R2(1-*)
    pub fn r2_simple_supertype<'a>(&'a self, store: &'a IsaStore) -> Vec<&SimpleSupertype> {
        vec![store.exhume_simple_supertype(&self.edge).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"another_object-struct-impl-nav-forward-to-ptr"}}}
    /// Navigate to [`Object`] across R1(1-*)
    pub fn r1_object<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Object> {
        vec![store.exhume_object(&self.ptr).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

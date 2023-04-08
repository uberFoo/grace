// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"baz-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"baz-use-statements"}}}
use uuid::Uuid;

use crate::domain::isa_clone::types::simple_supertype::SimpleSupertype;
use serde::{Deserialize, Serialize};

use crate::domain::isa_clone::store::ObjectStore as IsaCloneStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"baz-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Baz {
    pub id: Uuid,
    pub insanity: f64,
    /// R4: [`Baz`] 'chord' [`SimpleSupertype`]
    pub fugue: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"baz-implementation"}}}
impl Baz {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"baz-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"baz-struct-impl-new_"}}}
    /// Inter a new 'Baz' in the store, and return it's `id`.
    pub fn new(insanity: f64, fugue: &SimpleSupertype, store: &mut IsaCloneStore) -> Baz {
        let id = Uuid::new_v4();
        let new = Baz {
            id: id,
            insanity: insanity,
            fugue: fugue.id,
        };
        store.inter_baz(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"baz-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"baz-struct-impl-new_"}}}
    /// Inter a new 'Baz' in the store, and return it's `id`.
    pub fn new_(insanity: f64, fugue: &SimpleSupertype) -> Baz {
        let id = Uuid::new_v4();
        let new = Baz {
            id: id,
            insanity: insanity,
            fugue: fugue.id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"baz-struct-impl-nav-forward-to-fugue"}}}
    /// Navigate to [`SimpleSupertype`] across R4(1-*)
    pub fn r4_simple_supertype<'a>(&'a self, store: &'a IsaCloneStore) -> Vec<&SimpleSupertype> {
        vec![store.exhume_simple_supertype(&self.fugue).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

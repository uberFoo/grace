// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"everything-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-use-statements"}}}
use uuid::Uuid;

use crate::domain::everything::types::rando_object::RandoObject;
use serde::{Deserialize, Serialize};

use crate::domain::everything::store::ObjectStore as EverythingStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-struct-documentation"}}}
/// An object, with everything on it!
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Everything {
    pub bool: bool,
    pub float: f64,
    pub id: Uuid,
    pub int: i64,
    pub string: String,
    /// R1: [`Everything`] 'points at' [`RandoObject`]
    pub rando: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-implementation"}}}
impl Everything {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-struct-impl-new_"}}}
    /// Inter a new 'Everything' in the store, and return it's `id`.
    pub fn new(
        bool: bool,
        float: f64,
        int: i64,
        string: String,
        rando: &RandoObject,
        store: &mut EverythingStore,
    ) -> Everything {
        let id = Uuid::new_v4();
        let new = Everything {
            bool: bool,
            float: float,
            id: id,
            int: int,
            string: string,
            rando: rando.id,
        };
        store.inter_everything(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-struct-impl-new_"}}}
    /// Inter a new 'Everything' in the store, and return it's `id`.
    pub fn new_(
        bool: bool,
        float: f64,
        int: i64,
        string: String,
        rando: &RandoObject,
    ) -> Everything {
        let id = Uuid::new_v4();
        let new = Everything {
            bool: bool,
            float: float,
            id: id,
            int: int,
            string: string,
            rando: rando.id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-struct-impl-nav-forward-to-rando"}}}
    /// Navigate to [`RandoObject`] across R1(1-*)
    pub fn r1_rando_object<'a>(&'a self, store: &'a EverythingStore) -> Vec<&RandoObject> {
        vec![store.exhume_rando_object(&self.rando).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

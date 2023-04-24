// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"subtype_b-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-use-statements"}}}
use uuid::Uuid;

use crate::domain::isa::types::super_t::SuperT;
use serde::{Deserialize, Serialize};

use crate::domain::isa::store::ObjectStore as IsaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-struct-documentation"}}}
/// This [`Subtype`][s] has a number
///
/// [s]: nut::sarzak::Subtype
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct SubtypeB {
    pub id: Uuid,
    pub number: i64,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-implementation"}}}
impl SubtypeB {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-struct-impl-new_"}}}
    /// Inter a new 'Subtype B' in the store, and return it's `id`.
    pub fn new(number: i64, store: &mut IsaStore) -> SubtypeB {
        let id = Uuid::new_v4();
        let new = SubtypeB {
            id: id,
            number: number,
        };
        store.inter_subtype_b(new.clone());
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-struct-impl-new"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-struct-impl-new_"}}}
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-impl-nav-subtype-to-supertype-super_t"}}}
    // Navigate to [`SuperT`] across R2(isa)
    pub fn r2_super_t<'a>(&'a self, store: &'a IsaStore) -> Vec<&SuperT> {
        vec![store.exhume_super_t(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

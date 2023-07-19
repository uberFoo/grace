// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"subtype_b-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-use-statements"}}}
use uuid::Uuid;

use crate::domain::isa_clone::types::super_t::SuperT;
use crate::domain::isa_clone::types::super_t::SuperTEnum;
use serde::{Deserialize, Serialize};

use crate::domain::isa_clone::store::ObjectStore as IsaCloneStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-struct-documentation"}}}
/// This [`Subtype`][s] has a number
///
/// [s]: nut::sarzak::Subtype
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-struct-definition"}}}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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
    pub fn new(number: i64, store: &mut IsaCloneStore) -> SubtypeB {
        let id = Uuid::new_v4();
        let new = SubtypeB { id, number };
        store.inter_subtype_b(new.clone());
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-struct-impl-new"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-struct-impl-new_"}}}
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-impl-nav-subtype-to-supertype-super_t"}}}
    // Navigate to [`SuperT`] across R2(isa)
    pub fn r2_super_t<'a>(&'a self, store: &'a IsaCloneStore) -> Vec<&SuperT> {
        vec![store
            .iter_super_t()
            .find(|super_t| {
                if let SuperTEnum::SubtypeB(id) = super_t.subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

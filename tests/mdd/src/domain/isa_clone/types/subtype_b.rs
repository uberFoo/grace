// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"subtype_b-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::domain::isa_clone::UUID_NS;

use crate::domain::isa_clone::types::super_t::SuperT;

use crate::domain::isa_clone::store::ObjectStore as IsaCloneStore;
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
    /// Inter a new 'Subtype B' in the store, and return it's `id`.
    pub fn new(number: i64, store: &mut IsaCloneStore) -> SubtypeB {
        let id = Uuid::new_v5(&UUID_NS, format!("{}", number).as_bytes());
        let new = SubtypeB { number: number, id };
        store.inter_subtype_b(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"subtype_b-impl-nav-subtype-to-supertype-super_t"}}}
    // Navigate to [`SuperT`] across R2(isa)
    pub fn r2_super_t<'a>(&'a self, store: &'a IsaCloneStore) -> Vec<&SuperT> {
        vec![store.exhume_super_t(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

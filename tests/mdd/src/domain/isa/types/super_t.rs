// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"super_t-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-use-statements"}}}
use uuid::Uuid;

use crate::domain::isa::UUID_NS;

use serde::{Deserialize, Serialize};

// Subtype imports
use crate::domain::isa::types::subtype_a::SubtypeA;
use crate::domain::isa::types::subtype_b::SubtypeB;

// Referrer imports
use crate::domain::isa::types::reference::Reference;

// Referent imports
use crate::domain::isa::types::not_important::NotImportant;

use crate::domain::isa::store::ObjectStore as IsaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-enum-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-hybrid-documentation"}}}
/// A [`Supertype`] with normal [`Subtype`]s
///
/// This was called "Super". Rust didn't like it when it became "super". There needs to be
///a way of fixing keywords.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-enum-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-hybrid-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum SuperTEnum {
    SubtypeA(Uuid),
    SubtypeB(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-hybrid-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct SuperT {
    pub subtype: SuperTEnum,
    pub id: Uuid,
    /// R88: [`SuperT`] 'refers to' [`Reference`]
    pub pointer: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-implementation"}}}
impl SuperT {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-hybrid-new-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-struct-impl-new"}}}
    /// Inter a new SuperT in the store, and return it's `id`.
    pub fn new_subtype_a(pointer: &Reference, subtype: &SubtypeA, store: &mut IsaStore) -> SuperT {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}:{:?}", pointer, subtype).as_bytes());
        let new = SuperT {
            pointer: pointer.id,
            subtype: SuperTEnum::SubtypeA(subtype.id),
            id,
        };
        store.inter_super_t(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-struct-impl-new"}}}
    /// Inter a new SuperT in the store, and return it's `id`.
    pub fn new_subtype_b(pointer: &Reference, subtype: &SubtypeB, store: &mut IsaStore) -> SuperT {
        let id = Uuid::new_v5(&UUID_NS, format!("{:?}:{:?}", pointer, subtype).as_bytes());
        let new = SuperT {
            pointer: pointer.id,
            subtype: SuperTEnum::SubtypeB(subtype.id),
            id,
        };
        store.inter_super_t(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-struct-impl-nav-forward-to-pointer"}}}
    /// Navigate to [`Reference`] across R88(1-*)
    pub fn r88_reference<'a>(&'a self, store: &'a IsaStore) -> Vec<&Reference> {
        vec![store.exhume_reference(&self.pointer).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-struct-impl-nav-backward-cond-to-not_important"}}}
    /// Navigate to [`NotImportant`] across R888(1-1c)
    pub fn r888c_not_important<'a>(&'a self, store: &'a IsaStore) -> Vec<&NotImportant> {
        let not_important = store
            .iter_not_important()
            .find(|not_important| not_important.x_ref == self.id);
        match not_important {
            Some(ref not_important) => vec![not_important],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

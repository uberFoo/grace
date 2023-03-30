// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"super_t-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-use-statements"}}}
use uuid::Uuid;

use crate::domain::isa_ts::types::not_important::NotImportant;
use crate::domain::isa_ts::types::reference::Reference;
use crate::domain::isa_ts::types::subtype_a::SubtypeA;
use crate::domain::isa_ts::types::subtype_b::SubtypeB;
use serde::{Deserialize, Serialize};

use crate::domain::isa_ts::store::ObjectStore as IsaTsStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-hybrid-documentation"}}}
/// A [`Supertype`] with normal [`Subtype`]s
///
/// This was called "Super". Rust didn't like it when it became "super". There needs to be
///a way of fixing keywords.
///
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-hybrid-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum SuperTEnum {
    SubtypeA(Uuid),
    SubtypeB(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-implementation"}}}
impl SuperT {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-struct-impl-new"}}}
    /// Inter a new SuperT in the store, and return it's `id`.
    pub fn new_subtype_a(
        pointer: &Reference,
        subtype: &SubtypeA,
        store: &mut IsaTsStore,
    ) -> SuperT {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
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
    pub fn new_subtype_b(
        pointer: &Reference,
        subtype: &SubtypeB,
        store: &mut IsaTsStore,
    ) -> SuperT {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
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
    pub fn r88_reference<'a>(&'a self, store: &'a IsaTsStore) -> Vec<&Reference> {
        vec![store.exhume_reference(&self.pointer).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-struct-impl-nav-backward-cond-to-not_important"}}}
    /// Navigate to [`NotImportant`] across R888(1-1c)
    pub fn r888c_not_important<'a>(&'a self, store: &'a IsaTsStore) -> Vec<&NotImportant> {
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

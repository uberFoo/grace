// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"super_t-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::isa_rwlock_vec::types::not_important::NotImportant;
use crate::domain::isa_rwlock_vec::types::reference::Reference;
use crate::domain::isa_rwlock_vec::types::subtype_a::SubtypeA;
use crate::domain::isa_rwlock_vec::types::subtype_b::SubtypeB;
use serde::{Deserialize, Serialize};

use crate::domain::isa_rwlock_vec::store::ObjectStore as IsaRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-hybrid-documentation"}}}
/// A [`Supertype`] with normal [`Subtype`]s
///
/// This was called "Super". Rust didn't like it when it became "super". There needs to be
/// a way of fixing keywords.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-hybrid-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct SuperT {
    pub subtype: SuperTEnum,
    pub id: usize,
    /// R88: [`SuperT`] 'refers to' [`Reference`]
    pub pointer: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-hybrid-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum SuperTEnum {
    SubtypeA(usize),
    SubtypeB(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-implementation"}}}
impl SuperT {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-struct-impl-new_subtype_a"}}}
    /// Inter a new SuperT in the store, and return it's `id`.
    pub fn new_subtype_a(
        pointer: &Arc<RwLock<Reference>>,
        subtype: &Arc<RwLock<SubtypeA>>,
        store: &mut IsaRwlockVecStore,
    ) -> Arc<RwLock<SuperT>> {
        store.inter_super_t(|id| {
            Arc::new(RwLock::new(SuperT {
                pointer: pointer.read().unwrap().id,
                subtype: SuperTEnum::SubtypeA(subtype.read().unwrap().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-struct-impl-new_subtype_b"}}}
    /// Inter a new SuperT in the store, and return it's `id`.
    pub fn new_subtype_b(
        pointer: &Arc<RwLock<Reference>>,
        subtype: &Arc<RwLock<SubtypeB>>,
        store: &mut IsaRwlockVecStore,
    ) -> Arc<RwLock<SuperT>> {
        store.inter_super_t(|id| {
            Arc::new(RwLock::new(SuperT {
                pointer: pointer.read().unwrap().id,
                subtype: SuperTEnum::SubtypeB(subtype.read().unwrap().id),
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-struct-impl-nav-forward-to-pointer"}}}
    /// Navigate to [`Reference`] across R88(1-*)
    pub fn r88_reference<'a>(
        &'a self,
        store: &'a IsaRwlockVecStore,
    ) -> Vec<Arc<RwLock<Reference>>> {
        span!("r88_reference");
        vec![store.exhume_reference(&self.pointer).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_t-struct-impl-nav-backward-cond-to-not_important"}}}
    /// Navigate to [`NotImportant`] across R888(1-1c)
    pub fn r888c_not_important<'a>(
        &'a self,
        store: &'a IsaRwlockVecStore,
    ) -> Vec<Arc<RwLock<NotImportant>>> {
        span!("r888_not_important");
        let not_important = store
            .iter_not_important()
            .find(|not_important| not_important.read().unwrap().x_ref == self.id);
        match not_important {
            Some(ref not_important) => vec![not_important.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

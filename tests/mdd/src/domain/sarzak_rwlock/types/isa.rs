// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"isa-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::sarzak_rwlock::types::relationship::Relationship;
use crate::domain::sarzak_rwlock::types::subtype::Subtype;
use crate::domain::sarzak_rwlock::types::supertype::Supertype;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak_rwlock::store::ObjectStore as SarzakRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-struct-definition"}}}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Isa {
    pub id: Uuid,
    pub number: i64,
    /// R13: [`Isa`] 'has one' [`Supertype`]
    pub supertype: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-implementation"}}}
impl Isa {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-struct-impl-new"}}}
    /// Inter a new 'Isa' in the store, and return it's `id`.
    pub fn new(
        number: i64,
        supertype: &Arc<RwLock<Supertype>>,
        store: &mut SarzakRwlockStore,
    ) -> Arc<RwLock<Isa>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Isa {
            id,
            number,
            supertype: supertype.read().unwrap().id,
        }));
        store.inter_isa(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-struct-impl-nav-forward-to-supertype"}}}
    /// Navigate to [`Supertype`] across R13(1-*)
    pub fn r13_supertype<'a>(
        &'a self,
        store: &'a SarzakRwlockStore,
    ) -> Vec<Arc<RwLock<Supertype>>> {
        vec![store.exhume_supertype(&self.supertype).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-struct-impl-nav-backward-1_M-to-subtype"}}}
    /// Navigate to [`Subtype`] across R27(1-M)
    pub fn r27_subtype<'a>(&'a self, store: &'a SarzakRwlockStore) -> Vec<Arc<RwLock<Subtype>>> {
        store
            .iter_subtype()
            .filter(|subtype| subtype.read().unwrap().isa == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa-impl-nav-subtype-to-supertype-relationship"}}}
    // Navigate to [`Relationship`] across R4(isa)
    pub fn r4_relationship<'a>(
        &'a self,
        store: &'a SarzakRwlockStore,
    ) -> Vec<Arc<RwLock<Relationship>>> {
        vec![store.exhume_relationship(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

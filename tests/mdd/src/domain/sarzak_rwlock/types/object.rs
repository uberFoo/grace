// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"object-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::sarzak_rwlock::types::associative_referent::AssociativeReferent;
use crate::domain::sarzak_rwlock::types::associative_referrer::AssociativeReferrer;
use crate::domain::sarzak_rwlock::types::attribute::Attribute;
use crate::domain::sarzak_rwlock::types::event::Event;
use crate::domain::sarzak_rwlock::types::referent::Referent;
use crate::domain::sarzak_rwlock::types::referrer::Referrer;
use crate::domain::sarzak_rwlock::types::state::State;
use crate::domain::sarzak_rwlock::types::subtype::Subtype;
use crate::domain::sarzak_rwlock::types::supertype::Supertype;
use crate::domain::sarzak_rwlock::types::ty::Ty;
use serde::{Deserialize, Serialize};

use crate::domain::sarzak_rwlock::store::ObjectStore as SarzakRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-documentation"}}}
/// An `Object` is a collection of related data. By creating `Object`s, and
/// connecting them with `Relationships` we build a powerful abstraction.
///
/// `Object`s contain [Attribute]s that represent the data that the
/// `Object`encapsulates. All `Object`s have an attribute called `id`, which
/// is a unique identifier for each class of `Object`. The `id` attribute is a
/// version 5 UUID.
///
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-definition"}}}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Object {
    pub description: String,
    pub id: Uuid,
    pub key_letters: String,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-implementation"}}}
impl Object {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-new"}}}
    /// Inter a new 'Object' in the store, and return it's `id`.
    pub fn new(
        description: String,
        key_letters: String,
        name: String,
        store: &mut SarzakRwlockStore,
    ) -> Arc<RwLock<Object>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Object {
            description,
            id,
            key_letters,
            name,
        }));
        store.inter_object(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-associative_referent"}}}
    /// Navigate to [`AssociativeReferent`] across R25(1-M)
    pub fn r25_associative_referent<'a>(
        &'a self,
        store: &'a SarzakRwlockStore,
    ) -> Vec<Arc<RwLock<AssociativeReferent>>> {
        store
            .iter_associative_referent()
            .filter(|associative_referent| associative_referent.read().unwrap().obj_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-associative_referrer"}}}
    /// Navigate to [`AssociativeReferrer`] across R26(1-M)
    pub fn r26_associative_referrer<'a>(
        &'a self,
        store: &'a SarzakRwlockStore,
    ) -> Vec<Arc<RwLock<AssociativeReferrer>>> {
        store
            .iter_associative_referrer()
            .filter(|associative_referrer| associative_referrer.read().unwrap().obj_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-attribute"}}}
    /// Navigate to [`Attribute`] across R1(1-M)
    pub fn r1_attribute<'a>(&'a self, store: &'a SarzakRwlockStore) -> Vec<Arc<RwLock<Attribute>>> {
        store
            .iter_attribute()
            .filter(|attribute| attribute.read().unwrap().obj_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-event"}}}
    /// Navigate to [`Event`] across R19(1-M)
    pub fn r19_event<'a>(&'a self, store: &'a SarzakRwlockStore) -> Vec<Arc<RwLock<Event>>> {
        store
            .iter_event()
            .filter(|event| event.read().unwrap().obj_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-referent"}}}
    /// Navigate to [`Referent`] across R16(1-M)
    pub fn r16_referent<'a>(&'a self, store: &'a SarzakRwlockStore) -> Vec<Arc<RwLock<Referent>>> {
        store
            .iter_referent()
            .filter(|referent| referent.read().unwrap().obj_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-referrer"}}}
    /// Navigate to [`Referrer`] across R17(1-M)
    pub fn r17_referrer<'a>(&'a self, store: &'a SarzakRwlockStore) -> Vec<Arc<RwLock<Referrer>>> {
        store
            .iter_referrer()
            .filter(|referrer| referrer.read().unwrap().obj_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-state"}}}
    /// Navigate to [`State`] across R18(1-M)
    pub fn r18_state<'a>(&'a self, store: &'a SarzakRwlockStore) -> Vec<Arc<RwLock<State>>> {
        store
            .iter_state()
            .filter(|state| state.read().unwrap().obj_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-subtype"}}}
    /// Navigate to [`Subtype`] across R15(1-M)
    pub fn r15_subtype<'a>(&'a self, store: &'a SarzakRwlockStore) -> Vec<Arc<RwLock<Subtype>>> {
        store
            .iter_subtype()
            .filter(|subtype| subtype.read().unwrap().obj_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-supertype"}}}
    /// Navigate to [`Supertype`] across R14(1-M)
    pub fn r14_supertype<'a>(
        &'a self,
        store: &'a SarzakRwlockStore,
    ) -> Vec<Arc<RwLock<Supertype>>> {
        store
            .iter_supertype()
            .filter(|supertype| supertype.read().unwrap().obj_id == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-impl-nav-subtype-to-supertype-ty"}}}
    // Navigate to [`Ty`] across R3(isa)
    pub fn r3_ty<'a>(&'a self, store: &'a SarzakRwlockStore) -> Vec<Arc<RwLock<Ty>>> {
        vec![store.exhume_ty(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"object-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::sarzak_domain::UUID_NS;

// Referent imports
use crate::sarzak_domain::types::associative_referent::AssociativeReferent;
use crate::sarzak_domain::types::associative_referrer::AssociativeReferrer;
use crate::sarzak_domain::types::attribute::Attribute;
use crate::sarzak_domain::types::event::Event;
use crate::sarzak_domain::types::referent::Referent;
use crate::sarzak_domain::types::referrer::Referrer;
use crate::sarzak_domain::types::state::State;
use crate::sarzak_domain::types::subtype::Subtype;
use crate::sarzak_domain::types::supertype::Supertype;

use crate::sarzak_domain::store::ObjectStore as SarzakDomainStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-documentation"}}}
/// An `Object` is a collection of related data. By creating `Object`s, and
/// connecting them with `Relationships` we build a powerful abstraction.
///
/// `Object`s contain [Attribute]s that represent the data that the
/// `Object`encapsulates. All `Object`s have an attribute called `id`, which
/// is a unique idenifier for each class of `Object`. The `id` attribute is a
/// version 5 UUID.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
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
    /// Inter a new Object in the store, and return it's `id`.
    pub fn new(
        description: String,
        key_letters: String,
        name: String,
        store: &mut SarzakDomainStore,
    ) -> Object {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{}:{}:{}", description, key_letters, name).as_bytes(),
        );
        let new = Object {
            description: description,
            key_letters: key_letters,
            name: name,
            id,
        };
        store.inter_object(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-associative_referent"}}}
    /// Navigate to [`AssociativeReferent`] across R25(1-M)
    pub fn r25_associative_referent<'a>(
        &'a self,
        store: &'a SarzakDomainStore,
    ) -> Vec<&AssociativeReferent> {
        store
            .iter_associative_referent()
            .filter_map(|associative_referent| {
                if associative_referent.1.obj_id == self.id {
                    Some(associative_referent.1)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-associative_referrer"}}}
    /// Navigate to [`AssociativeReferrer`] across R26(1-M)
    pub fn r26_associative_referrer<'a>(
        &'a self,
        store: &'a SarzakDomainStore,
    ) -> Vec<&AssociativeReferrer> {
        store
            .iter_associative_referrer()
            .filter_map(|associative_referrer| {
                if associative_referrer.1.obj_id == self.id {
                    Some(associative_referrer.1)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_Mc-to-attribute"}}}
    /// Navigate to [`Attribute`] across R1(1-Mc)
    pub fn r1_attribute<'a>(&'a self, store: &'a SarzakDomainStore) -> Vec<&Attribute> {
        store
            .iter_attribute()
            .filter_map(|attribute| {
                if attribute.1.obj_id == Some(self.id) {
                    Some(attribute.1)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-event"}}}
    /// Navigate to [`Event`] across R19(1-M)
    pub fn r19_event<'a>(&'a self, store: &'a SarzakDomainStore) -> Vec<&Event> {
        store
            .iter_event()
            .filter_map(|event| {
                if event.1.obj_id == self.id {
                    Some(event.1)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-referent"}}}
    /// Navigate to [`Referent`] across R16(1-M)
    pub fn r16_referent<'a>(&'a self, store: &'a SarzakDomainStore) -> Vec<&Referent> {
        store
            .iter_referent()
            .filter_map(|referent| {
                if referent.1.obj_id == self.id {
                    Some(referent.1)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-referrer"}}}
    /// Navigate to [`Referrer`] across R17(1-M)
    pub fn r17_referrer<'a>(&'a self, store: &'a SarzakDomainStore) -> Vec<&Referrer> {
        store
            .iter_referrer()
            .filter_map(|referrer| {
                if referrer.1.obj_id == self.id {
                    Some(referrer.1)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-state"}}}
    /// Navigate to [`State`] across R18(1-M)
    pub fn r18_state<'a>(&'a self, store: &'a SarzakDomainStore) -> Vec<&State> {
        store
            .iter_state()
            .filter_map(|state| {
                if state.1.obj_id == self.id {
                    Some(state.1)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-subtype"}}}
    /// Navigate to [`Subtype`] across R15(1-M)
    pub fn r15_subtype<'a>(&'a self, store: &'a SarzakDomainStore) -> Vec<&Subtype> {
        store
            .iter_subtype()
            .filter_map(|subtype| {
                if subtype.1.obj_id == self.id {
                    Some(subtype.1)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"object-struct-impl-nav-backward-1_M-to-supertype"}}}
    /// Navigate to [`Supertype`] across R14(1-M)
    pub fn r14_supertype<'a>(&'a self, store: &'a SarzakDomainStore) -> Vec<&Supertype> {
        store
            .iter_supertype()
            .filter_map(|supertype| {
                if supertype.1.obj_id == self.id {
                    Some(supertype.1)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

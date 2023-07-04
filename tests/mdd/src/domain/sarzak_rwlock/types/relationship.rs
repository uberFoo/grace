// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"relationship-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship-use-statements"}}}
use crate::domain::sarzak_rwlock::store::ObjectStore as SarzakRwlockStore;
use crate::domain::sarzak_rwlock::types::associative::Associative;
use crate::domain::sarzak_rwlock::types::binary::Binary;
use crate::domain::sarzak_rwlock::types::isa::Isa;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship-enum-documentation"}}}
/// A `Relationship` indicates that a set of objects are connected to each other in some manner
/// . Typically it is a _real world_ relationship. In the
/// case of this model it is strictly an abstraction.
///
/// There are three types of `Relationship`: [`Isa`], [`Binary`], and [`Associative`]. Thus
///  `Relationship` is itself the *supertype* in an [`Isa`] relationship. It is a partitioning
///  *supertype-subtype* relationship, rather one of inheritance. As such, it’s  perfectly
///  suited to a rust `enum`! 😃
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship-enum-definition"}}}
#[derive(Copy, Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum Relationship {
    Associative(Uuid),
    Binary(Uuid),
    Isa(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship-implementation"}}}
impl Relationship {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship-new-impl"}}}
    /// Create a new instance of Relationship::Associative
    pub fn new_associative(
        associative: &Arc<RwLock<Associative>>,
        store: &mut SarzakRwlockStore,
    ) -> Arc<RwLock<Self>> {
        let id = associative.read().unwrap().id;
        if let Some(associative) = store.exhume_relationship(&id) {
            associative
        } else {
            let new = Arc::new(RwLock::new(Self::Associative(id)));
            store.inter_relationship(new.clone());
            new
        }
    }

    /// Create a new instance of Relationship::Binary
    pub fn new_binary(
        binary: &Arc<RwLock<Binary>>,
        store: &mut SarzakRwlockStore,
    ) -> Arc<RwLock<Self>> {
        let id = binary.read().unwrap().id;
        if let Some(binary) = store.exhume_relationship(&id) {
            binary
        } else {
            let new = Arc::new(RwLock::new(Self::Binary(id)));
            store.inter_relationship(new.clone());
            new
        }
    }

    /// Create a new instance of Relationship::Isa
    pub fn new_isa(isa: &Arc<RwLock<Isa>>, store: &mut SarzakRwlockStore) -> Arc<RwLock<Self>> {
        let id = isa.read().unwrap().id;
        if let Some(isa) = store.exhume_relationship(&id) {
            isa
        } else {
            let new = Arc::new(RwLock::new(Self::Isa(id)));
            store.inter_relationship(new.clone());
            new
        }
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"relationship-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::Associative(id) => *id,
            Self::Binary(id) => *id,
            Self::Isa(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

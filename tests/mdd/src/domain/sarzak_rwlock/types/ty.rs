// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"ty-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-use-statements"}}}
use crate::domain::sarzak_rwlock::store::ObjectStore as SarzakRwlockStore;
use crate::domain::sarzak_rwlock::types::attribute::Attribute;
use crate::domain::sarzak_rwlock::types::boolean::BOOLEAN;
use crate::domain::sarzak_rwlock::types::external::External;
use crate::domain::sarzak_rwlock::types::float::FLOAT;
use crate::domain::sarzak_rwlock::types::integer::INTEGER;
use crate::domain::sarzak_rwlock::types::object::Object;
use crate::domain::sarzak_rwlock::types::s_string::S_STRING;
use crate::domain::sarzak_rwlock::types::s_uuid::S_UUID;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-enum-documentation"}}}
/// The type of a value
///
/// There are several values available: [Integer], [Boolean], [Float], [String], and [UUID]
/// .
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-enum-definition"}}}
#[derive(Copy, Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum Ty {
    Boolean(Uuid),
    External(Uuid),
    Float(Uuid),
    Integer(Uuid),
    Object(Uuid),
    SString(Uuid),
    SUuid(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-implementation"}}}
impl Ty {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-new-impl"}}}
    /// Create a new instance of Ty::Boolean
    pub fn new_boolean(store: &SarzakRwlockStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_ty(&BOOLEAN).unwrap()
    }

    /// Create a new instance of Ty::External
    pub fn new_external(
        external: &Arc<RwLock<External>>,
        store: &mut SarzakRwlockStore,
    ) -> Arc<RwLock<Self>> {
        let id = external.read().unwrap().id;
        if let Some(external) = store.exhume_ty(&id) {
            external
        } else {
            let new = Arc::new(RwLock::new(Self::External(id)));
            store.inter_ty(new.clone());
            new
        }
    }

    /// Create a new instance of Ty::Float
    pub fn new_float(store: &SarzakRwlockStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_ty(&FLOAT).unwrap()
    }

    /// Create a new instance of Ty::Integer
    pub fn new_integer(store: &SarzakRwlockStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_ty(&INTEGER).unwrap()
    }

    /// Create a new instance of Ty::Object
    pub fn new_object(
        object: &Arc<RwLock<Object>>,
        store: &mut SarzakRwlockStore,
    ) -> Arc<RwLock<Self>> {
        let id = object.read().unwrap().id;
        if let Some(object) = store.exhume_ty(&id) {
            object
        } else {
            let new = Arc::new(RwLock::new(Self::Object(id)));
            store.inter_ty(new.clone());
            new
        }
    }

    /// Create a new instance of Ty::SString
    pub fn new_s_string(store: &SarzakRwlockStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_ty(&S_STRING).unwrap()
    }

    /// Create a new instance of Ty::SUuid
    pub fn new_s_uuid(store: &SarzakRwlockStore) -> Arc<RwLock<Self>> {
        // This is already in the store.
        store.exhume_ty(&S_UUID).unwrap()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::Boolean(id) => *id,
            Self::External(id) => *id,
            Self::Float(id) => *id,
            Self::Integer(id) => *id,
            Self::Object(id) => *id,
            Self::SString(id) => *id,
            Self::SUuid(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-struct-impl-nav-backward-one-to-attribute"}}}
    /// Navigate to [`Attribute`] across R2(1-1)
    pub fn r2_attribute<'a>(&'a self, store: &'a SarzakRwlockStore) -> Vec<Arc<RwLock<Attribute>>> {
        span!("r2_attribute");
        vec![store
            .iter_attribute()
            .find(|attribute| attribute.read().unwrap().ty == self.id())
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

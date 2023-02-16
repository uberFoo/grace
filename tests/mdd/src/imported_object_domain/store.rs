//! imported_object_domain Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"imported_object_domain-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`AnotherObject`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"imported_object_domain-object-store-definition"}}}
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::imported_object_domain::types::AnotherObject;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    another_object: HashMap<Uuid, AnotherObject>,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            another_object: HashMap::new(),
        }
    }

    /// Inter [`AnotherObject`] into the store.
    ///
    pub fn inter_another_object(&mut self, another_object: AnotherObject) {
        self.another_object
            .insert(another_object.id, another_object);
    }

    /// Exhume [`AnotherObject`] from the store.
    ///
    pub fn exhume_another_object(&self, id: &Uuid) -> Option<&AnotherObject> {
        self.another_object.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, AnotherObject>`.
    //
    pub fn iter_another_object(&self) -> impl Iterator<Item = (&Uuid, &AnotherObject)> {
        self.another_object.iter()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

//! everything_domain Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
//!
//! # Contents:
//!
//! * [`Everything`]
//! * [`RandoObject`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything_domain-object-store-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything_domain-object-store-definition"}}}
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::everything_domain::types::{Everything, RandoObject};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    everything: HashMap<Uuid, Everything>,
    rando_object: HashMap<Uuid, RandoObject>,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            everything: HashMap::new(),
            rando_object: HashMap::new(),
        }
    }

    /// Inter [`Everything`] into the store.
    ///
    pub fn inter_everything(&mut self, everything: Everything) {
        self.everything.insert(everything.id, everything);
    }

    /// Exhume [`Everything`] from the store.
    ///
    pub fn exhume_everything(&self, id: &Uuid) -> Option<&Everything> {
        self.everything.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Everything>`.
    //
    pub fn iter_everything(&self) -> impl Iterator<Item = (&Uuid, &Everything)> {
        self.everything.iter()
    }
    /// Inter [`RandoObject`] into the store.
    ///
    pub fn inter_rando_object(&mut self, rando_object: RandoObject) {
        self.rando_object.insert(rando_object.id, rando_object);
    }

    /// Exhume [`RandoObject`] from the store.
    ///
    pub fn exhume_rando_object(&self, id: &Uuid) -> Option<&RandoObject> {
        self.rando_object.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, RandoObject>`.
    //
    pub fn iter_rando_object(&self) -> impl Iterator<Item = (&Uuid, &RandoObject)> {
        self.rando_object.iter()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}


//! domain::imported_object Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::imported_object-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`AnotherObject`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::imported_object-object-store-definition"}}}
use std::collections::HashMap;
use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::imported_object::types::AnotherObject;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    another_object: HashMap<Uuid, AnotherObject>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            another_object: HashMap::new(),
        };

        // Initialize Singleton Subtypes

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::imported_object-object-store-methods"}}}
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
    /// Exhume [`AnotherObject`] from the store — mutably.
    ///
    pub fn exhume_another_object_mut(&mut self, id: &Uuid) -> Option<&mut AnotherObject> {
        self.another_object.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, AnotherObject>`.
    ///
    pub fn iter_another_object(&self) -> impl Iterator<Item = (&Uuid, &AnotherObject)> {
        self.another_object.iter()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::imported_object-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let path = path.join("imported_object.json");
        fs::create_dir_all(&path)?;

        // Persist another_object.
        {
            let path = path.join("another_object.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.another_object.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        Ok(())
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

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
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use fnv::FnvHashMap as HashMap;
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
            another_object: HashMap::default(),
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
    pub fn iter_another_object(&self) -> impl Iterator<Item = &AnotherObject> {
        self.another_object.values()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::imported_object-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        fs::create_dir_all(&path)?;

        let bin_path = path.clone().join("imported_object.bin");
        let mut bin_file = fs::File::create(bin_path)?;
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        bin_file.write_all(&encoded)?;

        let path = path.join("imported_object.json");
        fs::create_dir_all(&path)?;

        // Persist Another Object.
        {
            let path = path.join("another_object");
            fs::create_dir_all(&path)?;
            for another_object in self.another_object.values() {
                let path = path.join(format!("{}.json", another_object.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &another_object)?;
            }
        }

        Ok(())
    }

    /// Load the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let path = path.join("imported_object.json");

        let mut store = Self::new();

        // Load Another Object.
        {
            let path = path.join("another_object");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let another_object: AnotherObject = serde_json::from_reader(reader)?;
                store
                    .another_object
                    .insert(another_object.id, another_object);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

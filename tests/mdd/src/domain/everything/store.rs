//! domain::everything Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::everything-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`Everything`]
//! * [`RandoObject`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::everything-object-store-definition"}}}
use std::collections::HashMap;
use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::everything::types::{Everything, RandoObject};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    everything: HashMap<Uuid, Everything>,
    rando_object: HashMap<Uuid, RandoObject>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            everything: HashMap::new(),
            rando_object: HashMap::new(),
        };

        // Initialize Singleton Subtypes

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::everything-object-store-methods"}}}
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

    /// Exhume [`Everything`] from the store — mutably.
    ///
    pub fn exhume_everything_mut(&mut self, id: &Uuid) -> Option<&mut Everything> {
        self.everything.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Everything>`.
    ///
    pub fn iter_everything(&self) -> impl Iterator<Item = &Everything> {
        self.everything.values()
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

    /// Exhume [`RandoObject`] from the store — mutably.
    ///
    pub fn exhume_rando_object_mut(&mut self, id: &Uuid) -> Option<&mut RandoObject> {
        self.rando_object.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RandoObject>`.
    ///
    pub fn iter_rando_object(&self) -> impl Iterator<Item = &RandoObject> {
        self.rando_object.values()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::everything-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let path = path.join("everything.json");
        fs::create_dir_all(&path)?;

        // Persist Everything.
        {
            let path = path.join("everything.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.everything.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Rando Object.
        {
            let path = path.join("rando_object.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.rando_object.values().map(|x| x).collect::<Vec<_>>(),
            )?;
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
        let path = path.join("everything.json");

        let mut store = Self::new();

        // Load Everything.
        {
            let path = path.join("everything.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let everything: Vec<Everything> = serde_json::from_reader(reader)?;
            store.everything = everything.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Rando Object.
        {
            let path = path.join("rando_object.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let rando_object: Vec<RandoObject> = serde_json::from_reader(reader)?;
            store.rando_object = rando_object.into_iter().map(|道| (道.id, 道)).collect();
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

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
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use rustc_hash::FxHashMap as HashMap;
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
            everything: HashMap::default(),
            rando_object: HashMap::default(),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::everything-object-store-methods"}}}
    /// Inter (insert) [`Everything`] into the store.
    ///
    pub fn inter_everything(&mut self, everything: Everything) {
        self.everything.insert(everything.id, everything);
    }

    /// Exhume (get) [`Everything`] from the store.
    ///
    pub fn exhume_everything(&self, id: &Uuid) -> Option<&Everything> {
        self.everything.get(id)
    }

    /// Exorcise (remove) [`Everything`] from the store.
    ///
    pub fn exorcise_everything(&mut self, id: &Uuid) -> Option<Everything> {
        self.everything.remove(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Everything>`.
    ///
    pub fn iter_everything(&self) -> impl Iterator<Item = &Everything> {
        self.everything.values()
    }

    /// Inter (insert) [`RandoObject`] into the store.
    ///
    pub fn inter_rando_object(&mut self, rando_object: RandoObject) {
        self.rando_object.insert(rando_object.id, rando_object);
    }

    /// Exhume (get) [`RandoObject`] from the store.
    ///
    pub fn exhume_rando_object(&self, id: &Uuid) -> Option<&RandoObject> {
        self.rando_object.get(id)
    }

    /// Exorcise (remove) [`RandoObject`] from the store.
    ///
    pub fn exorcise_rando_object(&mut self, id: &Uuid) -> Option<RandoObject> {
        self.rando_object.remove(id)
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
    /// The store is persisted as a a bincode file.
    pub fn persist_bincode<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        let mut bin_file = fs::File::create(path)?;
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        bin_file.write_all(&encoded)?;
        Ok(())
    }

    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        fs::create_dir_all(path)?;

        let path = path.join("everything.json");
        fs::create_dir_all(&path)?;

        // Persist Everything.
        {
            let path = path.join("everything");
            fs::create_dir_all(&path)?;
            for everything in self.everything.values() {
                let path = path.join(format!("{}.json", everything.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &everything)?;
            }
        }

        // Persist Rando Object.
        {
            let path = path.join("rando_object");
            fs::create_dir_all(&path)?;
            for rando_object in self.rando_object.values() {
                let path = path.join(format!("{}.json", rando_object.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &rando_object)?;
            }
        }

        Ok(())
    }

    /// Load the store.
    ///
    pub fn from_bincode(code: &[u8]) -> io::Result<Self> {
        Ok(bincode::deserialize(code).unwrap())
    }

    /// The store is as a bincode file.
    pub fn load_bincode<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let bin_file = fs::File::open(path)?;
        Ok(bincode::deserialize_from(bin_file).unwrap())
    }

    /// Load the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automagic git integration as an option.
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let path = path.join("everything.json");

        let mut store = Self::new();

        // Load Everything.
        {
            let path = path.join("everything");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let everything: Everything = serde_json::from_reader(reader)?;
                store.everything.insert(everything.id, everything);
            }
        }

        // Load Rando Object.
        {
            let path = path.join("rando_object");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let rando_object: RandoObject = serde_json::from_reader(reader)?;
                store.rando_object.insert(rando_object.id, rando_object);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

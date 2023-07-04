//! domain::everything_rwlock Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::everything_rwlock-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`Everything`]
//! * [`RandoObject`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::everything_rwlock-object-store-definition"}}}
use std::sync::Arc;
use std::sync::RwLock;
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use fnv::FnvHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::everything_rwlock::types::{Everything, RandoObject};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    everything: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Everything>>>>>,
    rando_object: Arc<RwLock<HashMap<Uuid, Arc<RwLock<RandoObject>>>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            everything: Arc::new(RwLock::new(HashMap::default())),
            rando_object: Arc::new(RwLock::new(HashMap::default())),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::everything_rwlock-object-store-methods"}}}
    /// Inter (insert) [`Everything`] into the store.
    ///
    pub fn inter_everything(&mut self, everything: Arc<RwLock<Everything>>) {
        let read = everything.read().unwrap();
        self.everything
            .write()
            .unwrap()
            .insert(read.id, everything.clone());
    }

    /// Exhume (get) [`Everything`] from the store.
    ///
    pub fn exhume_everything(&self, id: &Uuid) -> Option<Arc<RwLock<Everything>>> {
        self.everything
            .read()
            .unwrap()
            .get(id)
            .map(|everything| everything.clone())
    }

    /// Exorcise (remove) [`Everything`] from the store.
    ///
    pub fn exorcise_everything(&mut self, id: &Uuid) -> Option<Arc<RwLock<Everything>>> {
        self.everything
            .write()
            .unwrap()
            .remove(id)
            .map(|everything| everything.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Everything>`.
    ///
    pub fn iter_everything(&self) -> impl Iterator<Item = Arc<RwLock<Everything>>> + '_ {
        let values: Vec<Arc<RwLock<Everything>>> = self
            .everything
            .read()
            .unwrap()
            .values()
            .map(|everything| everything.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`RandoObject`] into the store.
    ///
    pub fn inter_rando_object(&mut self, rando_object: Arc<RwLock<RandoObject>>) {
        let read = rando_object.read().unwrap();
        self.rando_object
            .write()
            .unwrap()
            .insert(read.id, rando_object.clone());
    }

    /// Exhume (get) [`RandoObject`] from the store.
    ///
    pub fn exhume_rando_object(&self, id: &Uuid) -> Option<Arc<RwLock<RandoObject>>> {
        self.rando_object
            .read()
            .unwrap()
            .get(id)
            .map(|rando_object| rando_object.clone())
    }

    /// Exorcise (remove) [`RandoObject`] from the store.
    ///
    pub fn exorcise_rando_object(&mut self, id: &Uuid) -> Option<Arc<RwLock<RandoObject>>> {
        self.rando_object
            .write()
            .unwrap()
            .remove(id)
            .map(|rando_object| rando_object.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RandoObject>`.
    ///
    pub fn iter_rando_object(&self) -> impl Iterator<Item = Arc<RwLock<RandoObject>>> + '_ {
        let values: Vec<Arc<RwLock<RandoObject>>> = self
            .rando_object
            .read()
            .unwrap()
            .values()
            .map(|rando_object| rando_object.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::everything_rwlock-object-store-persistence"}}}
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
            for everything in self.everything.read().unwrap().values() {
                let path = path.join(format!("{}.json", everything.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &everything)?;
            }
        }

        // Persist Rando Object.
        {
            let path = path.join("rando_object");
            fs::create_dir_all(&path)?;
            for rando_object in self.rando_object.read().unwrap().values() {
                let path = path.join(format!("{}.json", rando_object.read().unwrap().id));
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

        let store = Self::new();

        // Load Everything.
        {
            let path = path.join("everything");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let everything: Arc<RwLock<Everything>> = serde_json::from_reader(reader)?;
                store
                    .everything
                    .write()
                    .unwrap()
                    .insert(everything.read().unwrap().id, everything.clone());
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
                let rando_object: Arc<RwLock<RandoObject>> = serde_json::from_reader(reader)?;
                store
                    .rando_object
                    .write()
                    .unwrap()
                    .insert(rando_object.read().unwrap().id, rando_object.clone());
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

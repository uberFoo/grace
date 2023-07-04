//! domain::external_rwlock Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::external_rwlock-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`Nunchuck`]
//! * [`Timestamp`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::external_rwlock-object-store-definition"}}}
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

use crate::domain::external_rwlock::types::{Nunchuck, Timestamp};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    nunchuck: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Nunchuck>>>>>,
    timestamp: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Timestamp>>>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            nunchuck: Arc::new(RwLock::new(HashMap::default())),
            timestamp: Arc::new(RwLock::new(HashMap::default())),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::external_rwlock-object-store-methods"}}}
    /// Inter (insert) [`Nunchuck`] into the store.
    ///
    pub fn inter_nunchuck(&mut self, nunchuck: Arc<RwLock<Nunchuck>>) {
        let read = nunchuck.read().unwrap();
        self.nunchuck
            .write()
            .unwrap()
            .insert(read.id, nunchuck.clone());
    }

    /// Exhume (get) [`Nunchuck`] from the store.
    ///
    pub fn exhume_nunchuck(&self, id: &Uuid) -> Option<Arc<RwLock<Nunchuck>>> {
        self.nunchuck
            .read()
            .unwrap()
            .get(id)
            .map(|nunchuck| nunchuck.clone())
    }

    /// Exorcise (remove) [`Nunchuck`] from the store.
    ///
    pub fn exorcise_nunchuck(&mut self, id: &Uuid) -> Option<Arc<RwLock<Nunchuck>>> {
        self.nunchuck
            .write()
            .unwrap()
            .remove(id)
            .map(|nunchuck| nunchuck.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Nunchuck>`.
    ///
    pub fn iter_nunchuck(&self) -> impl Iterator<Item = Arc<RwLock<Nunchuck>>> + '_ {
        let values: Vec<Arc<RwLock<Nunchuck>>> = self
            .nunchuck
            .read()
            .unwrap()
            .values()
            .map(|nunchuck| nunchuck.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Timestamp`] into the store.
    ///
    pub fn inter_timestamp(&mut self, timestamp: Arc<RwLock<Timestamp>>) {
        let read = timestamp.read().unwrap();
        self.timestamp
            .write()
            .unwrap()
            .insert(read.id, timestamp.clone());
    }

    /// Exhume (get) [`Timestamp`] from the store.
    ///
    pub fn exhume_timestamp(&self, id: &Uuid) -> Option<Arc<RwLock<Timestamp>>> {
        self.timestamp
            .read()
            .unwrap()
            .get(id)
            .map(|timestamp| timestamp.clone())
    }

    /// Exorcise (remove) [`Timestamp`] from the store.
    ///
    pub fn exorcise_timestamp(&mut self, id: &Uuid) -> Option<Arc<RwLock<Timestamp>>> {
        self.timestamp
            .write()
            .unwrap()
            .remove(id)
            .map(|timestamp| timestamp.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Timestamp>`.
    ///
    pub fn iter_timestamp(&self) -> impl Iterator<Item = Arc<RwLock<Timestamp>>> + '_ {
        let values: Vec<Arc<RwLock<Timestamp>>> = self
            .timestamp
            .read()
            .unwrap()
            .values()
            .map(|timestamp| timestamp.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::external_rwlock-object-store-persistence"}}}
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

        let path = path.join("External Entity.json");
        fs::create_dir_all(&path)?;

        // Persist Nunchuck.
        {
            let path = path.join("nunchuck");
            fs::create_dir_all(&path)?;
            for nunchuck in self.nunchuck.read().unwrap().values() {
                let path = path.join(format!("{}.json", nunchuck.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &nunchuck)?;
            }
        }

        // Persist Timestamp.
        {
            let path = path.join("timestamp");
            fs::create_dir_all(&path)?;
            for timestamp in self.timestamp.read().unwrap().values() {
                let path = path.join(format!("{}.json", timestamp.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &timestamp)?;
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
        let path = path.join("External Entity.json");

        let store = Self::new();

        // Load Nunchuck.
        {
            let path = path.join("nunchuck");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let nunchuck: Arc<RwLock<Nunchuck>> = serde_json::from_reader(reader)?;
                store
                    .nunchuck
                    .write()
                    .unwrap()
                    .insert(nunchuck.read().unwrap().id, nunchuck.clone());
            }
        }

        // Load Timestamp.
        {
            let path = path.join("timestamp");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let timestamp: Arc<RwLock<Timestamp>> = serde_json::from_reader(reader)?;
                store
                    .timestamp
                    .write()
                    .unwrap()
                    .insert(timestamp.read().unwrap().id, timestamp.clone());
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

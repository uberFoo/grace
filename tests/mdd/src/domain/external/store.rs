//! domain::external Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::external-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`Nunchuck`]
//! * [`Timestamp`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::external-object-store-definition"}}}
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use fnv::FnvHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::external::types::{Nunchuck, Timestamp};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    nunchuck: HashMap<Uuid, Nunchuck>,
    timestamp: HashMap<Uuid, Timestamp>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            nunchuck: HashMap::default(),
            timestamp: HashMap::default(),
        };

        // Initialize Singleton Subtypes

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::external-object-store-methods"}}}
    /// Inter [`Nunchuck`] into the store.
    ///
    pub fn inter_nunchuck(&mut self, nunchuck: Nunchuck) {
        self.nunchuck.insert(nunchuck.id, nunchuck);
    }

    /// Exhume [`Nunchuck`] from the store.
    ///
    pub fn exhume_nunchuck(&self, id: &Uuid) -> Option<&Nunchuck> {
        self.nunchuck.get(id)
    }

    /// Exhume [`Nunchuck`] from the store — mutably.
    ///
    pub fn exhume_nunchuck_mut(&mut self, id: &Uuid) -> Option<&mut Nunchuck> {
        self.nunchuck.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Nunchuck>`.
    ///
    pub fn iter_nunchuck(&self) -> impl Iterator<Item = &Nunchuck> {
        self.nunchuck.values()
    }

    /// Inter [`Timestamp`] into the store.
    ///
    pub fn inter_timestamp(&mut self, timestamp: Timestamp) {
        self.timestamp.insert(timestamp.id, timestamp);
    }

    /// Exhume [`Timestamp`] from the store.
    ///
    pub fn exhume_timestamp(&self, id: &Uuid) -> Option<&Timestamp> {
        self.timestamp.get(id)
    }

    /// Exhume [`Timestamp`] from the store — mutably.
    ///
    pub fn exhume_timestamp_mut(&mut self, id: &Uuid) -> Option<&mut Timestamp> {
        self.timestamp.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Timestamp>`.
    ///
    pub fn iter_timestamp(&self) -> impl Iterator<Item = &Timestamp> {
        self.timestamp.values()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::external-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();

        let bin_path = path.clone().join("External Entitiy.bin");
        let mut bin_file = fs::File::create(bin_path)?;
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        bin_file.write_all(&encoded)?;

        let path = path.join("External Entitiy.json");
        fs::create_dir_all(&path)?;

        // Persist Nunchuck.
        {
            let path = path.join("nunchuck");
            fs::create_dir_all(&path)?;
            for nunchuck in self.nunchuck.values() {
                let path = path.join(format!("{}.json", nunchuck.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &nunchuck)?;
            }
        }

        // Persist Timestamp.
        {
            let path = path.join("timestamp");
            fs::create_dir_all(&path)?;
            for timestamp in self.timestamp.values() {
                let path = path.join(format!("{}.json", timestamp.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &timestamp)?;
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
        let path = path.join("External Entitiy.json");

        let mut store = Self::new();

        // Load Nunchuck.
        {
            let path = path.join("nunchuck");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let nunchuck: Nunchuck = serde_json::from_reader(reader)?;
                store.nunchuck.insert(nunchuck.id, nunchuck);
            }
        }

        // Load Timestamp.
        {
            let path = path.join("timestamp");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let timestamp: Timestamp = serde_json::from_reader(reader)?;
                store.timestamp.insert(timestamp.id, timestamp);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

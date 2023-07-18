//! domain::external_vec Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::external_vec-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`Nunchuck`]
//! * [`Timestamp`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::external_vec-object-store-definition"}}}
use std::cell::RefCell;
use std::rc::Rc;
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::external_vec::types::{Nunchuck, Timestamp};

#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    nunchuck_free_list: Vec<usize>,
    nunchuck: Vec<Option<Rc<RefCell<Nunchuck>>>>,
    timestamp_free_list: Vec<usize>,
    timestamp: Vec<Option<Rc<RefCell<Timestamp>>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            nunchuck_free_list: Vec::new(),
            nunchuck: Vec::new(),
            timestamp_free_list: Vec::new(),
            timestamp: Vec::new(),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::external_vec-object-store-methods"}}}
    /// Inter (insert) [`Nunchuck`] into the store.
    ///
    pub fn inter_nunchuck<F>(&mut self, nunchuck: F) -> Rc<RefCell<Nunchuck>>
    where
        F: Fn(usize) -> Rc<RefCell<Nunchuck>>,
    {
        let _index = if let Some(_index) = self.nunchuck_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.nunchuck.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.nunchuck.push(None);
            _index
        };
        let nunchuck = nunchuck(_index);
        if let Some(Some(nunchuck)) = self.nunchuck.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *nunchuck.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {nunchuck:?}.");
            self.nunchuck_free_list.push(_index);
            nunchuck.clone()
        } else {
            log::debug!(target: "store", "interring {nunchuck:?}.");
            self.nunchuck[_index] = Some(nunchuck.clone());
            nunchuck
        }
    }

    /// Exhume (get) [`Nunchuck`] from the store.
    ///
    pub fn exhume_nunchuck(&self, id: &usize) -> Option<Rc<RefCell<Nunchuck>>> {
        match self.nunchuck.get(*id) {
            Some(nunchuck) => nunchuck.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Nunchuck`] from the store.
    ///
    pub fn exorcise_nunchuck(&mut self, id: &usize) -> Option<Rc<RefCell<Nunchuck>>> {
        let result = self.nunchuck[*id].take();
        self.nunchuck_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Nunchuck>`.
    ///
    pub fn iter_nunchuck(&self) -> impl Iterator<Item = Rc<RefCell<Nunchuck>>> + '_ {
        let len = self.nunchuck.len();
        (0..len)
            .filter(|i| self.nunchuck[*i].is_some())
            .map(move |i| {
                self.nunchuck[i]
                    .as_ref()
                    .map(|nunchuck| nunchuck.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Timestamp`] into the store.
    ///
    pub fn inter_timestamp<F>(&mut self, timestamp: F) -> Rc<RefCell<Timestamp>>
    where
        F: Fn(usize) -> Rc<RefCell<Timestamp>>,
    {
        let _index = if let Some(_index) = self.timestamp_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.timestamp.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.timestamp.push(None);
            _index
        };
        let timestamp = timestamp(_index);
        if let Some(Some(timestamp)) = self.timestamp.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *timestamp.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {timestamp:?}.");
            self.timestamp_free_list.push(_index);
            timestamp.clone()
        } else {
            log::debug!(target: "store", "interring {timestamp:?}.");
            self.timestamp[_index] = Some(timestamp.clone());
            timestamp
        }
    }

    /// Exhume (get) [`Timestamp`] from the store.
    ///
    pub fn exhume_timestamp(&self, id: &usize) -> Option<Rc<RefCell<Timestamp>>> {
        match self.timestamp.get(*id) {
            Some(timestamp) => timestamp.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Timestamp`] from the store.
    ///
    pub fn exorcise_timestamp(&mut self, id: &usize) -> Option<Rc<RefCell<Timestamp>>> {
        let result = self.timestamp[*id].take();
        self.timestamp_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Timestamp>`.
    ///
    pub fn iter_timestamp(&self) -> impl Iterator<Item = Rc<RefCell<Timestamp>>> + '_ {
        let len = self.timestamp.len();
        (0..len)
            .filter(|i| self.timestamp[*i].is_some())
            .map(move |i| {
                self.timestamp[i]
                    .as_ref()
                    .map(|timestamp| timestamp.clone())
                    .unwrap()
            })
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::external_vec-object-store-persistence"}}}
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
            for nunchuck in &self.nunchuck {
                if let Some(nunchuck) = nunchuck {
                    let path = path.join(format!("{}.json", nunchuck.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &nunchuck)?;
                }
            }
        }

        // Persist Timestamp.
        {
            let path = path.join("timestamp");
            fs::create_dir_all(&path)?;
            for timestamp in &self.timestamp {
                if let Some(timestamp) = timestamp {
                    let path = path.join(format!("{}.json", timestamp.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &timestamp)?;
                }
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

        let mut store = Self::new();

        // Load Nunchuck.
        {
            let path = path.join("nunchuck");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let nunchuck: Rc<RefCell<Nunchuck>> = serde_json::from_reader(reader)?;
                store
                    .nunchuck
                    .insert(nunchuck.borrow().id, Some(nunchuck.clone()));
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
                let timestamp: Rc<RefCell<Timestamp>> = serde_json::from_reader(reader)?;
                store
                    .timestamp
                    .insert(timestamp.borrow().id, Some(timestamp.clone()));
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

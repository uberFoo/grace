//! domain::everything_vec Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::everything_vec-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`Everything`]
//! * [`RandoObject`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::everything_vec-object-store-definition"}}}
use std::cell::RefCell;
use std::rc::Rc;
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use fnv::FnvHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::everything_vec::types::{Everything, RandoObject};

#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    everything_free_list: std::sync::Mutex<Vec<usize>>,
    everything: Vec<Option<Rc<RefCell<Everything>>>>,
    rando_object_free_list: std::sync::Mutex<Vec<usize>>,
    rando_object: Vec<Option<Rc<RefCell<RandoObject>>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            everything_free_list: std::sync::Mutex::new(Vec::new()),
            everything: Vec::new(),
            rando_object_free_list: std::sync::Mutex::new(Vec::new()),
            rando_object: Vec::new(),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::everything_vec-object-store-methods"}}}
    /// Inter (insert) [`Everything`] into the store.
    ///
    pub fn inter_everything<F>(&mut self, everything: F) -> Rc<RefCell<Everything>>
    where
        F: Fn(usize) -> Rc<RefCell<Everything>>,
    {
        if let Some(index) = self.everything_free_list.lock().unwrap().pop() {
            let everything = everything(index);
            self.everything[index] = Some(everything.clone());
            everything
        } else {
            let index = self.everything.len();
            let everything = everything(index);
            self.everything.push(Some(everything.clone()));
            everything
        }
    }

    /// Exhume (get) [`Everything`] from the store.
    ///
    pub fn exhume_everything(&self, id: usize) -> Option<Rc<RefCell<Everything>>> {
        match self.everything.get(id) {
            Some(everything) => everything.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Everything`] from the store.
    ///
    pub fn exorcise_everything(&mut self, id: usize) -> Option<Rc<RefCell<Everything>>> {
        let result = self.everything[id].take();
        self.everything_free_list.lock().unwrap().push(id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Everything>`.
    ///
    pub fn iter_everything(&self) -> impl Iterator<Item = Rc<RefCell<Everything>>> + '_ {
        let len = self.everything.len();
        (0..len).map(move |i| {
            self.everything[i]
                .as_ref()
                .map(|everything| everything.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`RandoObject`] into the store.
    ///
    pub fn inter_rando_object<F>(&mut self, rando_object: F) -> Rc<RefCell<RandoObject>>
    where
        F: Fn(usize) -> Rc<RefCell<RandoObject>>,
    {
        if let Some(index) = self.rando_object_free_list.lock().unwrap().pop() {
            let rando_object = rando_object(index);
            self.rando_object[index] = Some(rando_object.clone());
            rando_object
        } else {
            let index = self.rando_object.len();
            let rando_object = rando_object(index);
            self.rando_object.push(Some(rando_object.clone()));
            rando_object
        }
    }

    /// Exhume (get) [`RandoObject`] from the store.
    ///
    pub fn exhume_rando_object(&self, id: usize) -> Option<Rc<RefCell<RandoObject>>> {
        match self.rando_object.get(id) {
            Some(rando_object) => rando_object.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`RandoObject`] from the store.
    ///
    pub fn exorcise_rando_object(&mut self, id: usize) -> Option<Rc<RefCell<RandoObject>>> {
        let result = self.rando_object[id].take();
        self.rando_object_free_list.lock().unwrap().push(id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RandoObject>`.
    ///
    pub fn iter_rando_object(&self) -> impl Iterator<Item = Rc<RefCell<RandoObject>>> + '_ {
        let len = self.rando_object.len();
        (0..len).map(move |i| {
            self.rando_object[i]
                .as_ref()
                .map(|rando_object| rando_object.clone())
                .unwrap()
        })
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::everything_vec-object-store-persistence"}}}
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
            for everything in &self.everything {
                if let Some(everything) = everything {
                    let path = path.join(format!("{}.json", everything.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &everything)?;
                }
            }
        }

        // Persist Rando Object.
        {
            let path = path.join("rando_object");
            fs::create_dir_all(&path)?;
            for rando_object in &self.rando_object {
                if let Some(rando_object) = rando_object {
                    let path = path.join(format!("{}.json", rando_object.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &rando_object)?;
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
                let everything: Rc<RefCell<Everything>> = serde_json::from_reader(reader)?;
                store.inter_everything(|id| {
                    everything.borrow_mut().id = id;
                    everything.clone()
                });
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
                let rando_object: Rc<RefCell<RandoObject>> = serde_json::from_reader(reader)?;
                store.inter_rando_object(|id| {
                    rando_object.borrow_mut().id = id;
                    rando_object.clone()
                });
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

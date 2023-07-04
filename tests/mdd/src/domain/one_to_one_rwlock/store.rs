//! domain::one_to_one_rwlock Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_one_rwlock-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`A`]
//! * [`B`]
//! * [`C`]
//! * [`Parameter`]
//! * [`Referent`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_one_rwlock-object-store-definition"}}}
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

use crate::domain::one_to_one_rwlock::types::{Parameter, Referent, A, B, C};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    a: Arc<RwLock<HashMap<Uuid, Arc<RwLock<A>>>>>,
    b: Arc<RwLock<HashMap<Uuid, Arc<RwLock<B>>>>>,
    c: Arc<RwLock<HashMap<Uuid, Arc<RwLock<C>>>>>,
    parameter: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Parameter>>>>>,
    referent: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Referent>>>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            a: Arc::new(RwLock::new(HashMap::default())),
            b: Arc::new(RwLock::new(HashMap::default())),
            c: Arc::new(RwLock::new(HashMap::default())),
            parameter: Arc::new(RwLock::new(HashMap::default())),
            referent: Arc::new(RwLock::new(HashMap::default())),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_one_rwlock-object-store-methods"}}}
    /// Inter (insert) [`A`] into the store.
    ///
    pub fn inter_a(&mut self, a: Arc<RwLock<A>>) {
        let read = a.read().unwrap();
        self.a.write().unwrap().insert(read.id, a.clone());
    }

    /// Exhume (get) [`A`] from the store.
    ///
    pub fn exhume_a(&self, id: &Uuid) -> Option<Arc<RwLock<A>>> {
        self.a.read().unwrap().get(id).map(|a| a.clone())
    }

    /// Exorcise (remove) [`A`] from the store.
    ///
    pub fn exorcise_a(&mut self, id: &Uuid) -> Option<Arc<RwLock<A>>> {
        self.a.write().unwrap().remove(id).map(|a| a.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, A>`.
    ///
    pub fn iter_a(&self) -> impl Iterator<Item = Arc<RwLock<A>>> + '_ {
        let values: Vec<Arc<RwLock<A>>> =
            self.a.read().unwrap().values().map(|a| a.clone()).collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`B`] into the store.
    ///
    pub fn inter_b(&mut self, b: Arc<RwLock<B>>) {
        let read = b.read().unwrap();
        self.b.write().unwrap().insert(read.id, b.clone());
    }

    /// Exhume (get) [`B`] from the store.
    ///
    pub fn exhume_b(&self, id: &Uuid) -> Option<Arc<RwLock<B>>> {
        self.b.read().unwrap().get(id).map(|b| b.clone())
    }

    /// Exorcise (remove) [`B`] from the store.
    ///
    pub fn exorcise_b(&mut self, id: &Uuid) -> Option<Arc<RwLock<B>>> {
        self.b.write().unwrap().remove(id).map(|b| b.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, B>`.
    ///
    pub fn iter_b(&self) -> impl Iterator<Item = Arc<RwLock<B>>> + '_ {
        let values: Vec<Arc<RwLock<B>>> =
            self.b.read().unwrap().values().map(|b| b.clone()).collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`C`] into the store.
    ///
    pub fn inter_c(&mut self, c: Arc<RwLock<C>>) {
        let read = c.read().unwrap();
        self.c.write().unwrap().insert(read.id, c.clone());
    }

    /// Exhume (get) [`C`] from the store.
    ///
    pub fn exhume_c(&self, id: &Uuid) -> Option<Arc<RwLock<C>>> {
        self.c.read().unwrap().get(id).map(|c| c.clone())
    }

    /// Exorcise (remove) [`C`] from the store.
    ///
    pub fn exorcise_c(&mut self, id: &Uuid) -> Option<Arc<RwLock<C>>> {
        self.c.write().unwrap().remove(id).map(|c| c.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, C>`.
    ///
    pub fn iter_c(&self) -> impl Iterator<Item = Arc<RwLock<C>>> + '_ {
        let values: Vec<Arc<RwLock<C>>> =
            self.c.read().unwrap().values().map(|c| c.clone()).collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Parameter`] into the store.
    ///
    pub fn inter_parameter(&mut self, parameter: Arc<RwLock<Parameter>>) {
        let read = parameter.read().unwrap();
        self.parameter
            .write()
            .unwrap()
            .insert(read.id, parameter.clone());
    }

    /// Exhume (get) [`Parameter`] from the store.
    ///
    pub fn exhume_parameter(&self, id: &Uuid) -> Option<Arc<RwLock<Parameter>>> {
        self.parameter
            .read()
            .unwrap()
            .get(id)
            .map(|parameter| parameter.clone())
    }

    /// Exorcise (remove) [`Parameter`] from the store.
    ///
    pub fn exorcise_parameter(&mut self, id: &Uuid) -> Option<Arc<RwLock<Parameter>>> {
        self.parameter
            .write()
            .unwrap()
            .remove(id)
            .map(|parameter| parameter.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    pub fn iter_parameter(&self) -> impl Iterator<Item = Arc<RwLock<Parameter>>> + '_ {
        let values: Vec<Arc<RwLock<Parameter>>> = self
            .parameter
            .read()
            .unwrap()
            .values()
            .map(|parameter| parameter.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Referent`] into the store.
    ///
    pub fn inter_referent(&mut self, referent: Arc<RwLock<Referent>>) {
        let read = referent.read().unwrap();
        self.referent
            .write()
            .unwrap()
            .insert(read.id, referent.clone());
    }

    /// Exhume (get) [`Referent`] from the store.
    ///
    pub fn exhume_referent(&self, id: &Uuid) -> Option<Arc<RwLock<Referent>>> {
        self.referent
            .read()
            .unwrap()
            .get(id)
            .map(|referent| referent.clone())
    }

    /// Exorcise (remove) [`Referent`] from the store.
    ///
    pub fn exorcise_referent(&mut self, id: &Uuid) -> Option<Arc<RwLock<Referent>>> {
        self.referent
            .write()
            .unwrap()
            .remove(id)
            .map(|referent| referent.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Referent>`.
    ///
    pub fn iter_referent(&self) -> impl Iterator<Item = Arc<RwLock<Referent>>> + '_ {
        let values: Vec<Arc<RwLock<Referent>>> = self
            .referent
            .read()
            .unwrap()
            .values()
            .map(|referent| referent.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_one_rwlock-object-store-persistence"}}}
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

        let path = path.join("one_to_one.json");
        fs::create_dir_all(&path)?;

        // Persist A.
        {
            let path = path.join("a");
            fs::create_dir_all(&path)?;
            for a in self.a.read().unwrap().values() {
                let path = path.join(format!("{}.json", a.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &a)?;
            }
        }

        // Persist B.
        {
            let path = path.join("b");
            fs::create_dir_all(&path)?;
            for b in self.b.read().unwrap().values() {
                let path = path.join(format!("{}.json", b.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &b)?;
            }
        }

        // Persist C.
        {
            let path = path.join("c");
            fs::create_dir_all(&path)?;
            for c in self.c.read().unwrap().values() {
                let path = path.join(format!("{}.json", c.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &c)?;
            }
        }

        // Persist Parameter.
        {
            let path = path.join("parameter");
            fs::create_dir_all(&path)?;
            for parameter in self.parameter.read().unwrap().values() {
                let path = path.join(format!("{}.json", parameter.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &parameter)?;
            }
        }

        // Persist Referent.
        {
            let path = path.join("referent");
            fs::create_dir_all(&path)?;
            for referent in self.referent.read().unwrap().values() {
                let path = path.join(format!("{}.json", referent.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &referent)?;
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
        let path = path.join("one_to_one.json");

        let store = Self::new();

        // Load A.
        {
            let path = path.join("a");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let a: Arc<RwLock<A>> = serde_json::from_reader(reader)?;
                store
                    .a
                    .write()
                    .unwrap()
                    .insert(a.read().unwrap().id, a.clone());
            }
        }

        // Load B.
        {
            let path = path.join("b");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let b: Arc<RwLock<B>> = serde_json::from_reader(reader)?;
                store
                    .b
                    .write()
                    .unwrap()
                    .insert(b.read().unwrap().id, b.clone());
            }
        }

        // Load C.
        {
            let path = path.join("c");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let c: Arc<RwLock<C>> = serde_json::from_reader(reader)?;
                store
                    .c
                    .write()
                    .unwrap()
                    .insert(c.read().unwrap().id, c.clone());
            }
        }

        // Load Parameter.
        {
            let path = path.join("parameter");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let parameter: Arc<RwLock<Parameter>> = serde_json::from_reader(reader)?;
                store
                    .parameter
                    .write()
                    .unwrap()
                    .insert(parameter.read().unwrap().id, parameter.clone());
            }
        }

        // Load Referent.
        {
            let path = path.join("referent");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let referent: Arc<RwLock<Referent>> = serde_json::from_reader(reader)?;
                store
                    .referent
                    .write()
                    .unwrap()
                    .insert(referent.read().unwrap().id, referent.clone());
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

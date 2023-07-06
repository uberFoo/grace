//! domain::one_to_many_rwlock_vec Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_many_rwlock_vec-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`A`]
//! * [`B`]
//! * [`C`]
//! * [`D`]
//! * [`Referent`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_many_rwlock_vec-object-store-definition"}}}
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

use crate::domain::one_to_many_rwlock_vec::types::{Referent, A, B, C, D};

#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    a_free_list: std::sync::Mutex<Vec<usize>>,
    a: Arc<RwLock<Vec<Option<Arc<RwLock<A>>>>>>,
    b_free_list: std::sync::Mutex<Vec<usize>>,
    b: Arc<RwLock<Vec<Option<Arc<RwLock<B>>>>>>,
    c_free_list: std::sync::Mutex<Vec<usize>>,
    c: Arc<RwLock<Vec<Option<Arc<RwLock<C>>>>>>,
    d_free_list: std::sync::Mutex<Vec<usize>>,
    d: Arc<RwLock<Vec<Option<Arc<RwLock<D>>>>>>,
    referent_free_list: std::sync::Mutex<Vec<usize>>,
    referent: Arc<RwLock<Vec<Option<Arc<RwLock<Referent>>>>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            a_free_list: std::sync::Mutex::new(Vec::new()),
            a: Arc::new(RwLock::new(Vec::new())),
            b_free_list: std::sync::Mutex::new(Vec::new()),
            b: Arc::new(RwLock::new(Vec::new())),
            c_free_list: std::sync::Mutex::new(Vec::new()),
            c: Arc::new(RwLock::new(Vec::new())),
            d_free_list: std::sync::Mutex::new(Vec::new()),
            d: Arc::new(RwLock::new(Vec::new())),
            referent_free_list: std::sync::Mutex::new(Vec::new()),
            referent: Arc::new(RwLock::new(Vec::new())),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_many_rwlock_vec-object-store-methods"}}}
    /// Inter (insert) [`A`] into the store.
    ///
    pub fn inter_a<F>(&mut self, a: F) -> Arc<RwLock<A>>
    where
        F: Fn(usize) -> Arc<RwLock<A>>,
    {
        if let Some(_index) = self.a_free_list.lock().unwrap().pop() {
            let a = a(_index);
            self.a.write().unwrap()[_index] = Some(a.clone());
            a
        } else {
            let _index = self.a.read().unwrap().len();
            let a = a(_index);
            self.a.write().unwrap().push(Some(a.clone()));
            a
        }
    }

    /// Exhume (get) [`A`] from the store.
    ///
    pub fn exhume_a(&self, id: &usize) -> Option<Arc<RwLock<A>>> {
        match self.a.read().unwrap().get(*id) {
            Some(a) => a.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`A`] from the store.
    ///
    pub fn exorcise_a(&mut self, id: &usize) -> Option<Arc<RwLock<A>>> {
        let result = self.a.write().unwrap()[*id].take();
        self.a_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, A>`.
    ///
    pub fn iter_a(&self) -> impl Iterator<Item = Arc<RwLock<A>>> + '_ {
        let len = self.a.read().unwrap().len();
        (0..len).map(move |i| {
            self.a.read().unwrap()[i]
                .as_ref()
                .map(|a| a.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`B`] into the store.
    ///
    pub fn inter_b<F>(&mut self, b: F) -> Arc<RwLock<B>>
    where
        F: Fn(usize) -> Arc<RwLock<B>>,
    {
        if let Some(_index) = self.b_free_list.lock().unwrap().pop() {
            let b = b(_index);
            self.b.write().unwrap()[_index] = Some(b.clone());
            b
        } else {
            let _index = self.b.read().unwrap().len();
            let b = b(_index);
            self.b.write().unwrap().push(Some(b.clone()));
            b
        }
    }

    /// Exhume (get) [`B`] from the store.
    ///
    pub fn exhume_b(&self, id: &usize) -> Option<Arc<RwLock<B>>> {
        match self.b.read().unwrap().get(*id) {
            Some(b) => b.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`B`] from the store.
    ///
    pub fn exorcise_b(&mut self, id: &usize) -> Option<Arc<RwLock<B>>> {
        let result = self.b.write().unwrap()[*id].take();
        self.b_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, B>`.
    ///
    pub fn iter_b(&self) -> impl Iterator<Item = Arc<RwLock<B>>> + '_ {
        let len = self.b.read().unwrap().len();
        (0..len).map(move |i| {
            self.b.read().unwrap()[i]
                .as_ref()
                .map(|b| b.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`C`] into the store.
    ///
    pub fn inter_c<F>(&mut self, c: F) -> Arc<RwLock<C>>
    where
        F: Fn(usize) -> Arc<RwLock<C>>,
    {
        if let Some(_index) = self.c_free_list.lock().unwrap().pop() {
            let c = c(_index);
            self.c.write().unwrap()[_index] = Some(c.clone());
            c
        } else {
            let _index = self.c.read().unwrap().len();
            let c = c(_index);
            self.c.write().unwrap().push(Some(c.clone()));
            c
        }
    }

    /// Exhume (get) [`C`] from the store.
    ///
    pub fn exhume_c(&self, id: &usize) -> Option<Arc<RwLock<C>>> {
        match self.c.read().unwrap().get(*id) {
            Some(c) => c.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`C`] from the store.
    ///
    pub fn exorcise_c(&mut self, id: &usize) -> Option<Arc<RwLock<C>>> {
        let result = self.c.write().unwrap()[*id].take();
        self.c_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, C>`.
    ///
    pub fn iter_c(&self) -> impl Iterator<Item = Arc<RwLock<C>>> + '_ {
        let len = self.c.read().unwrap().len();
        (0..len).map(move |i| {
            self.c.read().unwrap()[i]
                .as_ref()
                .map(|c| c.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`D`] into the store.
    ///
    pub fn inter_d<F>(&mut self, d: F) -> Arc<RwLock<D>>
    where
        F: Fn(usize) -> Arc<RwLock<D>>,
    {
        if let Some(_index) = self.d_free_list.lock().unwrap().pop() {
            let d = d(_index);
            self.d.write().unwrap()[_index] = Some(d.clone());
            d
        } else {
            let _index = self.d.read().unwrap().len();
            let d = d(_index);
            self.d.write().unwrap().push(Some(d.clone()));
            d
        }
    }

    /// Exhume (get) [`D`] from the store.
    ///
    pub fn exhume_d(&self, id: &usize) -> Option<Arc<RwLock<D>>> {
        match self.d.read().unwrap().get(*id) {
            Some(d) => d.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`D`] from the store.
    ///
    pub fn exorcise_d(&mut self, id: &usize) -> Option<Arc<RwLock<D>>> {
        let result = self.d.write().unwrap()[*id].take();
        self.d_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, D>`.
    ///
    pub fn iter_d(&self) -> impl Iterator<Item = Arc<RwLock<D>>> + '_ {
        let len = self.d.read().unwrap().len();
        (0..len).map(move |i| {
            self.d.read().unwrap()[i]
                .as_ref()
                .map(|d| d.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Referent`] into the store.
    ///
    pub fn inter_referent<F>(&mut self, referent: F) -> Arc<RwLock<Referent>>
    where
        F: Fn(usize) -> Arc<RwLock<Referent>>,
    {
        if let Some(_index) = self.referent_free_list.lock().unwrap().pop() {
            let referent = referent(_index);
            self.referent.write().unwrap()[_index] = Some(referent.clone());
            referent
        } else {
            let _index = self.referent.read().unwrap().len();
            let referent = referent(_index);
            self.referent.write().unwrap().push(Some(referent.clone()));
            referent
        }
    }

    /// Exhume (get) [`Referent`] from the store.
    ///
    pub fn exhume_referent(&self, id: &usize) -> Option<Arc<RwLock<Referent>>> {
        match self.referent.read().unwrap().get(*id) {
            Some(referent) => referent.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Referent`] from the store.
    ///
    pub fn exorcise_referent(&mut self, id: &usize) -> Option<Arc<RwLock<Referent>>> {
        let result = self.referent.write().unwrap()[*id].take();
        self.referent_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Referent>`.
    ///
    pub fn iter_referent(&self) -> impl Iterator<Item = Arc<RwLock<Referent>>> + '_ {
        let len = self.referent.read().unwrap().len();
        (0..len).map(move |i| {
            self.referent.read().unwrap()[i]
                .as_ref()
                .map(|referent| referent.clone())
                .unwrap()
        })
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_many_rwlock_vec-object-store-persistence"}}}
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

        let path = path.join("one_to_many.json");
        fs::create_dir_all(&path)?;

        // Persist A.
        {
            let path = path.join("a");
            fs::create_dir_all(&path)?;
            for a in &*self.a.read().unwrap() {
                if let Some(a) = a {
                    let path = path.join(format!("{}.json", a.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &a)?;
                }
            }
        }

        // Persist B.
        {
            let path = path.join("b");
            fs::create_dir_all(&path)?;
            for b in &*self.b.read().unwrap() {
                if let Some(b) = b {
                    let path = path.join(format!("{}.json", b.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &b)?;
                }
            }
        }

        // Persist C.
        {
            let path = path.join("c");
            fs::create_dir_all(&path)?;
            for c in &*self.c.read().unwrap() {
                if let Some(c) = c {
                    let path = path.join(format!("{}.json", c.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &c)?;
                }
            }
        }

        // Persist D.
        {
            let path = path.join("d");
            fs::create_dir_all(&path)?;
            for d in &*self.d.read().unwrap() {
                if let Some(d) = d {
                    let path = path.join(format!("{}.json", d.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &d)?;
                }
            }
        }

        // Persist Referent.
        {
            let path = path.join("referent");
            fs::create_dir_all(&path)?;
            for referent in &*self.referent.read().unwrap() {
                if let Some(referent) = referent {
                    let path = path.join(format!("{}.json", referent.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &referent)?;
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
        let path = path.join("one_to_many.json");

        let mut store = Self::new();

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
                    .insert(a.read().unwrap().id, Some(a.clone()));
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
                    .insert(b.read().unwrap().id, Some(b.clone()));
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
                    .insert(c.read().unwrap().id, Some(c.clone()));
            }
        }

        // Load D.
        {
            let path = path.join("d");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let d: Arc<RwLock<D>> = serde_json::from_reader(reader)?;
                store
                    .d
                    .write()
                    .unwrap()
                    .insert(d.read().unwrap().id, Some(d.clone()));
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
                    .insert(referent.read().unwrap().id, Some(referent.clone()));
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

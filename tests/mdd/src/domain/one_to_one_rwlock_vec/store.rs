//! domain::one_to_one_rwlock_vec Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_one_rwlock_vec-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`A`]
//! * [`B`]
//! * [`C`]
//! * [`Parameter`]
//! * [`Referent`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_one_rwlock_vec-object-store-definition"}}}
use std::sync::Arc;
use std::sync::RwLock;
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::one_to_one_rwlock_vec::types::{Parameter, Referent, A, B, C};

#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    a_free_list: std::sync::Mutex<Vec<usize>>,
    a: Arc<RwLock<Vec<Option<Arc<RwLock<A>>>>>>,
    b_free_list: std::sync::Mutex<Vec<usize>>,
    b: Arc<RwLock<Vec<Option<Arc<RwLock<B>>>>>>,
    c_free_list: std::sync::Mutex<Vec<usize>>,
    c: Arc<RwLock<Vec<Option<Arc<RwLock<C>>>>>>,
    parameter_free_list: std::sync::Mutex<Vec<usize>>,
    parameter: Arc<RwLock<Vec<Option<Arc<RwLock<Parameter>>>>>>,
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
            parameter_free_list: std::sync::Mutex::new(Vec::new()),
            parameter: Arc::new(RwLock::new(Vec::new())),
            referent_free_list: std::sync::Mutex::new(Vec::new()),
            referent: Arc::new(RwLock::new(Vec::new())),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_one_rwlock_vec-object-store-methods"}}}
    /// Inter (insert) [`A`] into the store.
    ///
    #[inline]
    pub fn inter_a<F>(&mut self, a: F) -> Arc<RwLock<A>>
    where
        F: Fn(usize) -> Arc<RwLock<A>>,
    {
        let _index = if let Some(_index) = self.a_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.a.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.a.write().unwrap().push(None);
            _index
        };

        let a = a(_index);

        let found = if let Some(a) = self.a.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *a.read().unwrap()
            } else {
                false
            }
        }) {
            a.clone()
        } else {
            None
        };

        if let Some(a) = found {
            log::debug!(target: "store", "found duplicate {a:?}.");
            self.a_free_list.lock().unwrap().push(_index);
            a.clone()
        } else {
            log::debug!(target: "store", "interring {a:?}.");
            self.a.write().unwrap()[_index] = Some(a.clone());
            a
        }
    }

    /// Exhume (get) [`A`] from the store.
    ///
    #[inline]
    pub fn exhume_a(&self, id: &usize) -> Option<Arc<RwLock<A>>> {
        match self.a.read().unwrap().get(*id) {
            Some(a) => a.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`A`] from the store.
    ///
    #[inline]
    pub fn exorcise_a(&mut self, id: &usize) -> Option<Arc<RwLock<A>>> {
        log::debug!(target: "store", "exorcising a slot: {id}.");
        let result = self.a.write().unwrap()[*id].take();
        self.a_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, A>`.
    ///
    #[inline]
    pub fn iter_a(&self) -> impl Iterator<Item = Arc<RwLock<A>>> + '_ {
        let len = self.a.read().unwrap().len();
        (0..len)
            .filter(|i| self.a.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.a.read().unwrap()[i]
                    .as_ref()
                    .map(|a| a.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`B`] into the store.
    ///
    #[inline]
    pub fn inter_b<F>(&mut self, b: F) -> Arc<RwLock<B>>
    where
        F: Fn(usize) -> Arc<RwLock<B>>,
    {
        let _index = if let Some(_index) = self.b_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.b.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.b.write().unwrap().push(None);
            _index
        };

        let b = b(_index);

        let found = if let Some(b) = self.b.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *b.read().unwrap()
            } else {
                false
            }
        }) {
            b.clone()
        } else {
            None
        };

        if let Some(b) = found {
            log::debug!(target: "store", "found duplicate {b:?}.");
            self.b_free_list.lock().unwrap().push(_index);
            b.clone()
        } else {
            log::debug!(target: "store", "interring {b:?}.");
            self.b.write().unwrap()[_index] = Some(b.clone());
            b
        }
    }

    /// Exhume (get) [`B`] from the store.
    ///
    #[inline]
    pub fn exhume_b(&self, id: &usize) -> Option<Arc<RwLock<B>>> {
        match self.b.read().unwrap().get(*id) {
            Some(b) => b.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`B`] from the store.
    ///
    #[inline]
    pub fn exorcise_b(&mut self, id: &usize) -> Option<Arc<RwLock<B>>> {
        log::debug!(target: "store", "exorcising b slot: {id}.");
        let result = self.b.write().unwrap()[*id].take();
        self.b_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, B>`.
    ///
    #[inline]
    pub fn iter_b(&self) -> impl Iterator<Item = Arc<RwLock<B>>> + '_ {
        let len = self.b.read().unwrap().len();
        (0..len)
            .filter(|i| self.b.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.b.read().unwrap()[i]
                    .as_ref()
                    .map(|b| b.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`C`] into the store.
    ///
    #[inline]
    pub fn inter_c<F>(&mut self, c: F) -> Arc<RwLock<C>>
    where
        F: Fn(usize) -> Arc<RwLock<C>>,
    {
        let _index = if let Some(_index) = self.c_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.c.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.c.write().unwrap().push(None);
            _index
        };

        let c = c(_index);

        let found = if let Some(c) = self.c.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *c.read().unwrap()
            } else {
                false
            }
        }) {
            c.clone()
        } else {
            None
        };

        if let Some(c) = found {
            log::debug!(target: "store", "found duplicate {c:?}.");
            self.c_free_list.lock().unwrap().push(_index);
            c.clone()
        } else {
            log::debug!(target: "store", "interring {c:?}.");
            self.c.write().unwrap()[_index] = Some(c.clone());
            c
        }
    }

    /// Exhume (get) [`C`] from the store.
    ///
    #[inline]
    pub fn exhume_c(&self, id: &usize) -> Option<Arc<RwLock<C>>> {
        match self.c.read().unwrap().get(*id) {
            Some(c) => c.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`C`] from the store.
    ///
    #[inline]
    pub fn exorcise_c(&mut self, id: &usize) -> Option<Arc<RwLock<C>>> {
        log::debug!(target: "store", "exorcising c slot: {id}.");
        let result = self.c.write().unwrap()[*id].take();
        self.c_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, C>`.
    ///
    #[inline]
    pub fn iter_c(&self) -> impl Iterator<Item = Arc<RwLock<C>>> + '_ {
        let len = self.c.read().unwrap().len();
        (0..len)
            .filter(|i| self.c.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.c.read().unwrap()[i]
                    .as_ref()
                    .map(|c| c.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Parameter`] into the store.
    ///
    #[inline]
    pub fn inter_parameter<F>(&mut self, parameter: F) -> Arc<RwLock<Parameter>>
    where
        F: Fn(usize) -> Arc<RwLock<Parameter>>,
    {
        let _index = if let Some(_index) = self.parameter_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.parameter.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.parameter.write().unwrap().push(None);
            _index
        };

        let parameter = parameter(_index);

        let found = if let Some(parameter) = self.parameter.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *parameter.read().unwrap()
            } else {
                false
            }
        }) {
            parameter.clone()
        } else {
            None
        };

        if let Some(parameter) = found {
            log::debug!(target: "store", "found duplicate {parameter:?}.");
            self.parameter_free_list.lock().unwrap().push(_index);
            parameter.clone()
        } else {
            log::debug!(target: "store", "interring {parameter:?}.");
            self.parameter.write().unwrap()[_index] = Some(parameter.clone());
            parameter
        }
    }

    /// Exhume (get) [`Parameter`] from the store.
    ///
    #[inline]
    pub fn exhume_parameter(&self, id: &usize) -> Option<Arc<RwLock<Parameter>>> {
        match self.parameter.read().unwrap().get(*id) {
            Some(parameter) => parameter.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Parameter`] from the store.
    ///
    #[inline]
    pub fn exorcise_parameter(&mut self, id: &usize) -> Option<Arc<RwLock<Parameter>>> {
        log::debug!(target: "store", "exorcising parameter slot: {id}.");
        let result = self.parameter.write().unwrap()[*id].take();
        self.parameter_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    #[inline]
    pub fn iter_parameter(&self) -> impl Iterator<Item = Arc<RwLock<Parameter>>> + '_ {
        let len = self.parameter.read().unwrap().len();
        (0..len)
            .filter(|i| self.parameter.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.parameter.read().unwrap()[i]
                    .as_ref()
                    .map(|parameter| parameter.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Referent`] into the store.
    ///
    #[inline]
    pub fn inter_referent<F>(&mut self, referent: F) -> Arc<RwLock<Referent>>
    where
        F: Fn(usize) -> Arc<RwLock<Referent>>,
    {
        let _index = if let Some(_index) = self.referent_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.referent.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.referent.write().unwrap().push(None);
            _index
        };

        let referent = referent(_index);

        let found = if let Some(referent) = self.referent.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *referent.read().unwrap()
            } else {
                false
            }
        }) {
            referent.clone()
        } else {
            None
        };

        if let Some(referent) = found {
            log::debug!(target: "store", "found duplicate {referent:?}.");
            self.referent_free_list.lock().unwrap().push(_index);
            referent.clone()
        } else {
            log::debug!(target: "store", "interring {referent:?}.");
            self.referent.write().unwrap()[_index] = Some(referent.clone());
            referent
        }
    }

    /// Exhume (get) [`Referent`] from the store.
    ///
    #[inline]
    pub fn exhume_referent(&self, id: &usize) -> Option<Arc<RwLock<Referent>>> {
        match self.referent.read().unwrap().get(*id) {
            Some(referent) => referent.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Referent`] from the store.
    ///
    #[inline]
    pub fn exorcise_referent(&mut self, id: &usize) -> Option<Arc<RwLock<Referent>>> {
        log::debug!(target: "store", "exorcising referent slot: {id}.");
        let result = self.referent.write().unwrap()[*id].take();
        self.referent_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Referent>`.
    ///
    #[inline]
    pub fn iter_referent(&self) -> impl Iterator<Item = Arc<RwLock<Referent>>> + '_ {
        let len = self.referent.read().unwrap().len();
        (0..len)
            .filter(|i| self.referent.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.referent.read().unwrap()[i]
                    .as_ref()
                    .map(|referent| referent.clone())
                    .unwrap()
            })
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_one_rwlock_vec-object-store-persistence"}}}
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

        // Persist Parameter.
        {
            let path = path.join("parameter");
            fs::create_dir_all(&path)?;
            for parameter in &*self.parameter.read().unwrap() {
                if let Some(parameter) = parameter {
                    let path = path.join(format!("{}.json", parameter.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &parameter)?;
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
        let path = path.join("one_to_one.json");

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
                    .insert(parameter.read().unwrap().id, Some(parameter.clone()));
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

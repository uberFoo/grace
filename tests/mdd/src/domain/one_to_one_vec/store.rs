//! domain::one_to_one_vec Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_one_vec-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`A`]
//! * [`B`]
//! * [`C`]
//! * [`Parameter`]
//! * [`Referent`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_one_vec-object-store-definition"}}}
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

use crate::domain::one_to_one_vec::types::{Parameter, Referent, A, B, C};

#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    a_free_list: Vec<usize>,
    a: Vec<Option<Rc<RefCell<A>>>>,
    b_free_list: Vec<usize>,
    b: Vec<Option<Rc<RefCell<B>>>>,
    c_free_list: Vec<usize>,
    c: Vec<Option<Rc<RefCell<C>>>>,
    parameter_free_list: Vec<usize>,
    parameter: Vec<Option<Rc<RefCell<Parameter>>>>,
    referent_free_list: Vec<usize>,
    referent: Vec<Option<Rc<RefCell<Referent>>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            a_free_list: Vec::new(),
            a: Vec::new(),
            b_free_list: Vec::new(),
            b: Vec::new(),
            c_free_list: Vec::new(),
            c: Vec::new(),
            parameter_free_list: Vec::new(),
            parameter: Vec::new(),
            referent_free_list: Vec::new(),
            referent: Vec::new(),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_one_vec-object-store-methods"}}}
    /// Inter (insert) [`A`] into the store.
    ///
    pub fn inter_a<F>(&mut self, a: F) -> Rc<RefCell<A>>
    where
        F: Fn(usize) -> Rc<RefCell<A>>,
    {
        if let Some(_index) = self.a_free_list.pop() {
            let a = a(_index);
            self.a[_index] = Some(a.clone());
            a
        } else {
            let _index = self.a.len();
            let a = a(_index);
            self.a.push(Some(a.clone()));
            a
        }
    }

    /// Exhume (get) [`A`] from the store.
    ///
    pub fn exhume_a(&self, id: &usize) -> Option<Rc<RefCell<A>>> {
        match self.a.get(*id) {
            Some(a) => a.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`A`] from the store.
    ///
    pub fn exorcise_a(&mut self, id: &usize) -> Option<Rc<RefCell<A>>> {
        let result = self.a[*id].take();
        self.a_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, A>`.
    ///
    pub fn iter_a(&self) -> impl Iterator<Item = Rc<RefCell<A>>> + '_ {
        let len = self.a.len();
        (0..len).map(move |i| self.a[i].as_ref().map(|a| a.clone()).unwrap())
    }

    /// Inter (insert) [`B`] into the store.
    ///
    pub fn inter_b<F>(&mut self, b: F) -> Rc<RefCell<B>>
    where
        F: Fn(usize) -> Rc<RefCell<B>>,
    {
        if let Some(_index) = self.b_free_list.pop() {
            let b = b(_index);
            self.b[_index] = Some(b.clone());
            b
        } else {
            let _index = self.b.len();
            let b = b(_index);
            self.b.push(Some(b.clone()));
            b
        }
    }

    /// Exhume (get) [`B`] from the store.
    ///
    pub fn exhume_b(&self, id: &usize) -> Option<Rc<RefCell<B>>> {
        match self.b.get(*id) {
            Some(b) => b.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`B`] from the store.
    ///
    pub fn exorcise_b(&mut self, id: &usize) -> Option<Rc<RefCell<B>>> {
        let result = self.b[*id].take();
        self.b_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, B>`.
    ///
    pub fn iter_b(&self) -> impl Iterator<Item = Rc<RefCell<B>>> + '_ {
        let len = self.b.len();
        (0..len).map(move |i| self.b[i].as_ref().map(|b| b.clone()).unwrap())
    }

    /// Inter (insert) [`C`] into the store.
    ///
    pub fn inter_c<F>(&mut self, c: F) -> Rc<RefCell<C>>
    where
        F: Fn(usize) -> Rc<RefCell<C>>,
    {
        if let Some(_index) = self.c_free_list.pop() {
            let c = c(_index);
            self.c[_index] = Some(c.clone());
            c
        } else {
            let _index = self.c.len();
            let c = c(_index);
            self.c.push(Some(c.clone()));
            c
        }
    }

    /// Exhume (get) [`C`] from the store.
    ///
    pub fn exhume_c(&self, id: &usize) -> Option<Rc<RefCell<C>>> {
        match self.c.get(*id) {
            Some(c) => c.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`C`] from the store.
    ///
    pub fn exorcise_c(&mut self, id: &usize) -> Option<Rc<RefCell<C>>> {
        let result = self.c[*id].take();
        self.c_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, C>`.
    ///
    pub fn iter_c(&self) -> impl Iterator<Item = Rc<RefCell<C>>> + '_ {
        let len = self.c.len();
        (0..len).map(move |i| self.c[i].as_ref().map(|c| c.clone()).unwrap())
    }

    /// Inter (insert) [`Parameter`] into the store.
    ///
    pub fn inter_parameter<F>(&mut self, parameter: F) -> Rc<RefCell<Parameter>>
    where
        F: Fn(usize) -> Rc<RefCell<Parameter>>,
    {
        if let Some(_index) = self.parameter_free_list.pop() {
            let parameter = parameter(_index);
            self.parameter[_index] = Some(parameter.clone());
            parameter
        } else {
            let _index = self.parameter.len();
            let parameter = parameter(_index);
            self.parameter.push(Some(parameter.clone()));
            parameter
        }
    }

    /// Exhume (get) [`Parameter`] from the store.
    ///
    pub fn exhume_parameter(&self, id: &usize) -> Option<Rc<RefCell<Parameter>>> {
        match self.parameter.get(*id) {
            Some(parameter) => parameter.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Parameter`] from the store.
    ///
    pub fn exorcise_parameter(&mut self, id: &usize) -> Option<Rc<RefCell<Parameter>>> {
        let result = self.parameter[*id].take();
        self.parameter_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    ///
    pub fn iter_parameter(&self) -> impl Iterator<Item = Rc<RefCell<Parameter>>> + '_ {
        let len = self.parameter.len();
        (0..len).map(move |i| {
            self.parameter[i]
                .as_ref()
                .map(|parameter| parameter.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Referent`] into the store.
    ///
    pub fn inter_referent<F>(&mut self, referent: F) -> Rc<RefCell<Referent>>
    where
        F: Fn(usize) -> Rc<RefCell<Referent>>,
    {
        if let Some(_index) = self.referent_free_list.pop() {
            let referent = referent(_index);
            self.referent[_index] = Some(referent.clone());
            referent
        } else {
            let _index = self.referent.len();
            let referent = referent(_index);
            self.referent.push(Some(referent.clone()));
            referent
        }
    }

    /// Exhume (get) [`Referent`] from the store.
    ///
    pub fn exhume_referent(&self, id: &usize) -> Option<Rc<RefCell<Referent>>> {
        match self.referent.get(*id) {
            Some(referent) => referent.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Referent`] from the store.
    ///
    pub fn exorcise_referent(&mut self, id: &usize) -> Option<Rc<RefCell<Referent>>> {
        let result = self.referent[*id].take();
        self.referent_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Referent>`.
    ///
    pub fn iter_referent(&self) -> impl Iterator<Item = Rc<RefCell<Referent>>> + '_ {
        let len = self.referent.len();
        (0..len).map(move |i| {
            self.referent[i]
                .as_ref()
                .map(|referent| referent.clone())
                .unwrap()
        })
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_one_vec-object-store-persistence"}}}
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
            for a in &self.a {
                if let Some(a) = a {
                    let path = path.join(format!("{}.json", a.borrow().id));
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
            for b in &self.b {
                if let Some(b) = b {
                    let path = path.join(format!("{}.json", b.borrow().id));
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
            for c in &self.c {
                if let Some(c) = c {
                    let path = path.join(format!("{}.json", c.borrow().id));
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
            for parameter in &self.parameter {
                if let Some(parameter) = parameter {
                    let path = path.join(format!("{}.json", parameter.borrow().id));
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
            for referent in &self.referent {
                if let Some(referent) = referent {
                    let path = path.join(format!("{}.json", referent.borrow().id));
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
                let a: Rc<RefCell<A>> = serde_json::from_reader(reader)?;
                store.inter_a(|id| {
                    a.borrow_mut().id = id;
                    a.clone()
                });
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
                let b: Rc<RefCell<B>> = serde_json::from_reader(reader)?;
                store.inter_b(|id| {
                    b.borrow_mut().id = id;
                    b.clone()
                });
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
                let c: Rc<RefCell<C>> = serde_json::from_reader(reader)?;
                store.inter_c(|id| {
                    c.borrow_mut().id = id;
                    c.clone()
                });
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
                let parameter: Rc<RefCell<Parameter>> = serde_json::from_reader(reader)?;
                store.inter_parameter(|id| {
                    parameter.borrow_mut().id = id;
                    parameter.clone()
                });
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
                let referent: Rc<RefCell<Referent>> = serde_json::from_reader(reader)?;
                store.inter_referent(|id| {
                    referent.borrow_mut().id = id;
                    referent.clone()
                });
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

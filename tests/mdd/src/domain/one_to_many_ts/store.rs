//! domain::one_to_many_ts Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_many_ts-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`A`]
//! * [`B`]
//! * [`C`]
//! * [`D`]
//! * [`Referent`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_many_ts-object-store-definition"}}}
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
    time::SystemTime,
};

use fnv::FnvHashMap as HashMap;
use heck::ToUpperCamelCase;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::one_to_many_ts::types::{Referent, A, B, C, D};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    a: HashMap<Uuid, (A, SystemTime)>,
    a_by_name: HashMap<String, (A, SystemTime)>,
    b: HashMap<Uuid, (B, SystemTime)>,
    c: HashMap<Uuid, (C, SystemTime)>,
    d: HashMap<Uuid, (D, SystemTime)>,
    referent: HashMap<Uuid, (Referent, SystemTime)>,
    referent_by_name: HashMap<String, (Referent, SystemTime)>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            a: HashMap::default(),
            a_by_name: HashMap::default(),
            b: HashMap::default(),
            c: HashMap::default(),
            d: HashMap::default(),
            referent: HashMap::default(),
            referent_by_name: HashMap::default(),
        };

        // Initialize Singleton Subtypes

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_many_ts-object-store-methods"}}}
    /// Inter [`A`] into the store.
    ///
    pub fn inter_a(&mut self, a: A) {
        let value = (a, SystemTime::now());
        self.a.insert(value.0.id, value.clone());
        self.a_by_name
            .insert(value.0.name.to_upper_camel_case(), value);
    }

    /// Exhume [`A`] from the store.
    ///
    pub fn exhume_a(&self, id: &Uuid) -> Option<&A> {
        self.a.get(id).map(|a| &a.0)
    }

    /// Exhume [`A`] from the store — mutably.
    ///
    pub fn exhume_a_mut(&mut self, id: &Uuid) -> Option<&mut A> {
        self.a.get_mut(id).map(|a| &mut a.0)
    }

    /// Exhume [`A`] from the store by name.
    ///
    pub fn exhume_a_by_name(&self, name: &str) -> Option<&A> {
        self.a_by_name.get(name).map(|a| &a.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, A>`.
    ///
    pub fn iter_a(&self) -> impl Iterator<Item = &A> {
        self.a.values().map(|a| &a.0)
    }

    /// Get the timestamp for A.
    ///
    pub fn a_timestamp(&self, a: &A) -> SystemTime {
        self.a.get(&a.id).map(|a| a.1).unwrap_or(SystemTime::now())
    }

    /// Inter [`B`] into the store.
    ///
    pub fn inter_b(&mut self, b: B) {
        self.b.insert(b.id, (b, SystemTime::now()));
    }

    /// Exhume [`B`] from the store.
    ///
    pub fn exhume_b(&self, id: &Uuid) -> Option<&B> {
        self.b.get(id).map(|b| &b.0)
    }

    /// Exhume [`B`] from the store — mutably.
    ///
    pub fn exhume_b_mut(&mut self, id: &Uuid) -> Option<&mut B> {
        self.b.get_mut(id).map(|b| &mut b.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, B>`.
    ///
    pub fn iter_b(&self) -> impl Iterator<Item = &B> {
        self.b.values().map(|b| &b.0)
    }

    /// Get the timestamp for B.
    ///
    pub fn b_timestamp(&self, b: &B) -> SystemTime {
        self.b.get(&b.id).map(|b| b.1).unwrap_or(SystemTime::now())
    }

    /// Inter [`C`] into the store.
    ///
    pub fn inter_c(&mut self, c: C) {
        self.c.insert(c.id, (c, SystemTime::now()));
    }

    /// Exhume [`C`] from the store.
    ///
    pub fn exhume_c(&self, id: &Uuid) -> Option<&C> {
        self.c.get(id).map(|c| &c.0)
    }

    /// Exhume [`C`] from the store — mutably.
    ///
    pub fn exhume_c_mut(&mut self, id: &Uuid) -> Option<&mut C> {
        self.c.get_mut(id).map(|c| &mut c.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, C>`.
    ///
    pub fn iter_c(&self) -> impl Iterator<Item = &C> {
        self.c.values().map(|c| &c.0)
    }

    /// Get the timestamp for C.
    ///
    pub fn c_timestamp(&self, c: &C) -> SystemTime {
        self.c.get(&c.id).map(|c| c.1).unwrap_or(SystemTime::now())
    }

    /// Inter [`D`] into the store.
    ///
    pub fn inter_d(&mut self, d: D) {
        self.d.insert(d.id, (d, SystemTime::now()));
    }

    /// Exhume [`D`] from the store.
    ///
    pub fn exhume_d(&self, id: &Uuid) -> Option<&D> {
        self.d.get(id).map(|d| &d.0)
    }

    /// Exhume [`D`] from the store — mutably.
    ///
    pub fn exhume_d_mut(&mut self, id: &Uuid) -> Option<&mut D> {
        self.d.get_mut(id).map(|d| &mut d.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, D>`.
    ///
    pub fn iter_d(&self) -> impl Iterator<Item = &D> {
        self.d.values().map(|d| &d.0)
    }

    /// Get the timestamp for D.
    ///
    pub fn d_timestamp(&self, d: &D) -> SystemTime {
        self.d.get(&d.id).map(|d| d.1).unwrap_or(SystemTime::now())
    }

    /// Inter [`Referent`] into the store.
    ///
    pub fn inter_referent(&mut self, referent: Referent) {
        let value = (referent, SystemTime::now());
        self.referent.insert(value.0.id, value.clone());
        self.referent_by_name
            .insert(value.0.name.to_upper_camel_case(), value);
    }

    /// Exhume [`Referent`] from the store.
    ///
    pub fn exhume_referent(&self, id: &Uuid) -> Option<&Referent> {
        self.referent.get(id).map(|referent| &referent.0)
    }

    /// Exhume [`Referent`] from the store — mutably.
    ///
    pub fn exhume_referent_mut(&mut self, id: &Uuid) -> Option<&mut Referent> {
        self.referent.get_mut(id).map(|referent| &mut referent.0)
    }

    /// Exhume [`Referent`] from the store by name.
    ///
    pub fn exhume_referent_by_name(&self, name: &str) -> Option<&Referent> {
        self.referent_by_name.get(name).map(|referent| &referent.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Referent>`.
    ///
    pub fn iter_referent(&self) -> impl Iterator<Item = &Referent> {
        self.referent.values().map(|referent| &referent.0)
    }

    /// Get the timestamp for Referent.
    ///
    pub fn referent_timestamp(&self, referent: &Referent) -> SystemTime {
        self.referent
            .get(&referent.id)
            .map(|referent| referent.1)
            .unwrap_or(SystemTime::now())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_many_ts-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        fs::create_dir_all(&path)?;

        let bin_path = path.clone().join("one_to_many.bin");
        let mut bin_file = fs::File::create(bin_path)?;
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        bin_file.write_all(&encoded)?;

        let path = path.join("one_to_many.json");
        fs::create_dir_all(&path)?;

        // Persist A.
        {
            let path = path.join("a");
            fs::create_dir_all(&path)?;
            for a_tuple in self.a.values() {
                let path = path.join(format!("{}.json", a_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (A, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != a_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &a_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &a_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.a.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist B.
        {
            let path = path.join("b");
            fs::create_dir_all(&path)?;
            for b_tuple in self.b.values() {
                let path = path.join(format!("{}.json", b_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (B, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != b_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &b_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &b_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.b.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist C.
        {
            let path = path.join("c");
            fs::create_dir_all(&path)?;
            for c_tuple in self.c.values() {
                let path = path.join(format!("{}.json", c_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (C, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != c_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &c_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &c_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.c.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist D.
        {
            let path = path.join("d");
            fs::create_dir_all(&path)?;
            for d_tuple in self.d.values() {
                let path = path.join(format!("{}.json", d_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (D, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != d_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &d_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &d_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.d.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Referent.
        {
            let path = path.join("referent");
            fs::create_dir_all(&path)?;
            for referent_tuple in self.referent.values() {
                let path = path.join(format!("{}.json", referent_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Referent, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != referent_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &referent_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &referent_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.referent.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
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
        let path = path.join("one_to_many.json");

        let mut store = Self::new();

        // Load A.
        {
            let path = path.join("a");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let a: (A, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .a_by_name
                    .insert(a.0.name.to_upper_camel_case(), a.clone());
                store.a.insert(a.0.id, a);
            }
        }

        // Load B.
        {
            let path = path.join("b");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let b: (B, SystemTime) = serde_json::from_reader(reader)?;
                store.b.insert(b.0.id, b);
            }
        }

        // Load C.
        {
            let path = path.join("c");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let c: (C, SystemTime) = serde_json::from_reader(reader)?;
                store.c.insert(c.0.id, c);
            }
        }

        // Load D.
        {
            let path = path.join("d");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let d: (D, SystemTime) = serde_json::from_reader(reader)?;
                store.d.insert(d.0.id, d);
            }
        }

        // Load Referent.
        {
            let path = path.join("referent");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let referent: (Referent, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .referent_by_name
                    .insert(referent.0.name.to_upper_camel_case(), referent.clone());
                store.referent.insert(referent.0.id, referent);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

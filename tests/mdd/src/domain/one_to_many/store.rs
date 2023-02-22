//! domain::one_to_many Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_many-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`A`]
//! * [`B`]
//! * [`C`]
//! * [`D`]
//! * [`Referent`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_many-object-store-definition"}}}
use std::collections::HashMap;
use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::one_to_many::types::{Referent, A, B, C, D};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    a: HashMap<Uuid, A>,
    b: HashMap<Uuid, B>,
    c: HashMap<Uuid, C>,
    d: HashMap<Uuid, D>,
    referent: HashMap<Uuid, Referent>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            a: HashMap::new(),
            b: HashMap::new(),
            c: HashMap::new(),
            d: HashMap::new(),
            referent: HashMap::new(),
        };

        // Initialize Singleton Subtypes

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_many-object-store-methods"}}}
    /// Inter [`A`] into the store.
    ///
    pub fn inter_a(&mut self, a: A) {
        self.a.insert(a.id, a);
    }

    /// Exhume [`A`] from the store.
    ///
    pub fn exhume_a(&self, id: &Uuid) -> Option<&A> {
        self.a.get(id)
    }
    /// Exhume [`A`] from the store — mutably.
    ///
    pub fn exhume_a_mut(&mut self, id: &Uuid) -> Option<&mut A> {
        self.a.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, A>`.
    ///
    pub fn iter_a(&self) -> impl Iterator<Item = &A> {
        self.a.values()
    }
    /// Inter [`B`] into the store.
    ///
    pub fn inter_b(&mut self, b: B) {
        self.b.insert(b.id, b);
    }

    /// Exhume [`B`] from the store.
    ///
    pub fn exhume_b(&self, id: &Uuid) -> Option<&B> {
        self.b.get(id)
    }
    /// Exhume [`B`] from the store — mutably.
    ///
    pub fn exhume_b_mut(&mut self, id: &Uuid) -> Option<&mut B> {
        self.b.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, B>`.
    ///
    pub fn iter_b(&self) -> impl Iterator<Item = &B> {
        self.b.values()
    }
    /// Inter [`C`] into the store.
    ///
    pub fn inter_c(&mut self, c: C) {
        self.c.insert(c.id, c);
    }

    /// Exhume [`C`] from the store.
    ///
    pub fn exhume_c(&self, id: &Uuid) -> Option<&C> {
        self.c.get(id)
    }
    /// Exhume [`C`] from the store — mutably.
    ///
    pub fn exhume_c_mut(&mut self, id: &Uuid) -> Option<&mut C> {
        self.c.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, C>`.
    ///
    pub fn iter_c(&self) -> impl Iterator<Item = &C> {
        self.c.values()
    }
    /// Inter [`D`] into the store.
    ///
    pub fn inter_d(&mut self, d: D) {
        self.d.insert(d.id, d);
    }

    /// Exhume [`D`] from the store.
    ///
    pub fn exhume_d(&self, id: &Uuid) -> Option<&D> {
        self.d.get(id)
    }
    /// Exhume [`D`] from the store — mutably.
    ///
    pub fn exhume_d_mut(&mut self, id: &Uuid) -> Option<&mut D> {
        self.d.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, D>`.
    ///
    pub fn iter_d(&self) -> impl Iterator<Item = &D> {
        self.d.values()
    }
    /// Inter [`Referent`] into the store.
    ///
    pub fn inter_referent(&mut self, referent: Referent) {
        self.referent.insert(referent.id, referent);
    }

    /// Exhume [`Referent`] from the store.
    ///
    pub fn exhume_referent(&self, id: &Uuid) -> Option<&Referent> {
        self.referent.get(id)
    }
    /// Exhume [`Referent`] from the store — mutably.
    ///
    pub fn exhume_referent_mut(&mut self, id: &Uuid) -> Option<&mut Referent> {
        self.referent.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Referent>`.
    ///
    pub fn iter_referent(&self) -> impl Iterator<Item = &Referent> {
        self.referent.values()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_many-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let path = path.join("one_to_many.json");
        fs::create_dir_all(&path)?;

        // Persist a.
        {
            let path = path.join("a.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.a.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist b.
        {
            let path = path.join("b.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.b.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist c.
        {
            let path = path.join("c.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.c.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist d.
        {
            let path = path.join("d.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.d.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist referent.
        {
            let path = path.join("referent.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.referent.values().map(|x| x).collect::<Vec<_>>(),
            )?;
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

        // Load a.
        {
            let path = path.join("a.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let a: Vec<A> = serde_json::from_reader(reader)?;
            store.a = a.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load b.
        {
            let path = path.join("b.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let b: Vec<B> = serde_json::from_reader(reader)?;
            store.b = b.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load c.
        {
            let path = path.join("c.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let c: Vec<C> = serde_json::from_reader(reader)?;
            store.c = c.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load d.
        {
            let path = path.join("d.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let d: Vec<D> = serde_json::from_reader(reader)?;
            store.d = d.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load referent.
        {
            let path = path.join("referent.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let referent: Vec<Referent> = serde_json::from_reader(reader)?;
            store.referent = referent.into_iter().map(|道| (道.id, 道)).collect();
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

//! domain::isa Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`NotImportant`]
//! * [`SimpleSupertype`]
//! * [`SubtypeA`]
//! * [`SubtypeB`]
//! * [`SuperT`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa-object-store-definition"}}}
use std::collections::HashMap;
use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::isa::types::{
    NotImportant, SimpleSupertype, SubtypeA, SubtypeB, SuperT, SIMPLE_SUBTYPE_A, SIMPLE_SUBTYPE_B,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    not_important: HashMap<Uuid, NotImportant>,
    simple_supertype: HashMap<Uuid, SimpleSupertype>,
    subtype_a: HashMap<Uuid, SubtypeA>,
    subtype_b: HashMap<Uuid, SubtypeB>,
    super_t: HashMap<Uuid, SuperT>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            not_important: HashMap::new(),
            simple_supertype: HashMap::new(),
            subtype_a: HashMap::new(),
            subtype_b: HashMap::new(),
            super_t: HashMap::new(),
        };

        // Initialize Singleton Subtypes
        store.inter_simple_supertype(SimpleSupertype::SimpleSubtypeA(SIMPLE_SUBTYPE_A));
        store.inter_simple_supertype(SimpleSupertype::SimpleSubtypeB(SIMPLE_SUBTYPE_B));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa-object-store-methods"}}}
    /// Inter [`NotImportant`] into the store.
    ///
    pub fn inter_not_important(&mut self, not_important: NotImportant) {
        self.not_important.insert(not_important.id, not_important);
    }

    /// Exhume [`NotImportant`] from the store.
    ///
    pub fn exhume_not_important(&self, id: &Uuid) -> Option<&NotImportant> {
        self.not_important.get(id)
    }
    /// Exhume [`NotImportant`] from the store — mutably.
    ///
    pub fn exhume_not_important_mut(&mut self, id: &Uuid) -> Option<&mut NotImportant> {
        self.not_important.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, NotImportant>`.
    ///
    pub fn iter_not_important(&self) -> impl Iterator<Item = (&Uuid, &NotImportant)> {
        self.not_important.iter()
    }
    /// Inter [`SimpleSupertype`] into the store.
    ///
    pub fn inter_simple_supertype(&mut self, simple_supertype: SimpleSupertype) {
        self.simple_supertype
            .insert(simple_supertype.id(), simple_supertype);
    }

    /// Exhume [`SimpleSupertype`] from the store.
    ///
    pub fn exhume_simple_supertype(&self, id: &Uuid) -> Option<&SimpleSupertype> {
        self.simple_supertype.get(id)
    }
    /// Exhume [`SimpleSupertype`] from the store — mutably.
    ///
    pub fn exhume_simple_supertype_mut(&mut self, id: &Uuid) -> Option<&mut SimpleSupertype> {
        self.simple_supertype.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, SimpleSupertype>`.
    ///
    pub fn iter_simple_supertype(&self) -> impl Iterator<Item = (&Uuid, &SimpleSupertype)> {
        self.simple_supertype.iter()
    }
    /// Inter [`SubtypeA`] into the store.
    ///
    pub fn inter_subtype_a(&mut self, subtype_a: SubtypeA) {
        self.subtype_a.insert(subtype_a.id, subtype_a);
    }

    /// Exhume [`SubtypeA`] from the store.
    ///
    pub fn exhume_subtype_a(&self, id: &Uuid) -> Option<&SubtypeA> {
        self.subtype_a.get(id)
    }
    /// Exhume [`SubtypeA`] from the store — mutably.
    ///
    pub fn exhume_subtype_a_mut(&mut self, id: &Uuid) -> Option<&mut SubtypeA> {
        self.subtype_a.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeA>`.
    ///
    pub fn iter_subtype_a(&self) -> impl Iterator<Item = (&Uuid, &SubtypeA)> {
        self.subtype_a.iter()
    }
    /// Inter [`SubtypeB`] into the store.
    ///
    pub fn inter_subtype_b(&mut self, subtype_b: SubtypeB) {
        self.subtype_b.insert(subtype_b.id, subtype_b);
    }

    /// Exhume [`SubtypeB`] from the store.
    ///
    pub fn exhume_subtype_b(&self, id: &Uuid) -> Option<&SubtypeB> {
        self.subtype_b.get(id)
    }
    /// Exhume [`SubtypeB`] from the store — mutably.
    ///
    pub fn exhume_subtype_b_mut(&mut self, id: &Uuid) -> Option<&mut SubtypeB> {
        self.subtype_b.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeB>`.
    ///
    pub fn iter_subtype_b(&self) -> impl Iterator<Item = (&Uuid, &SubtypeB)> {
        self.subtype_b.iter()
    }
    /// Inter [`SuperT`] into the store.
    ///
    pub fn inter_super_t(&mut self, super_t: SuperT) {
        self.super_t.insert(super_t.id(), super_t);
    }

    /// Exhume [`SuperT`] from the store.
    ///
    pub fn exhume_super_t(&self, id: &Uuid) -> Option<&SuperT> {
        self.super_t.get(id)
    }
    /// Exhume [`SuperT`] from the store — mutably.
    ///
    pub fn exhume_super_t_mut(&mut self, id: &Uuid) -> Option<&mut SuperT> {
        self.super_t.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, SuperT>`.
    ///
    pub fn iter_super_t(&self) -> impl Iterator<Item = (&Uuid, &SuperT)> {
        self.super_t.iter()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let path = path.join("Isa Relationship.json");
        fs::create_dir_all(&path)?;

        // Persist not_important.
        {
            let path = path.join("not_important.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.not_important.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist simple_supertype.
        {
            let path = path.join("simple_supertype.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self
                    .simple_supertype
                    .values()
                    .map(|x| x)
                    .collect::<Vec<_>>(),
            )?;
        }
        // Persist subtype_a.
        {
            let path = path.join("subtype_a.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.subtype_a.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist subtype_b.
        {
            let path = path.join("subtype_b.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.subtype_b.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist super_t.
        {
            let path = path.join("super_t.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.super_t.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        Ok(())
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

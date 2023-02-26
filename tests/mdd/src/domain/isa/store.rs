//! domain::isa Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`Henry`]
//! * [`NotImportant`]
//! * [`OhBoy`]
//! * [`Reference`]
//! * [`SimpleSubtypeA`]
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
    Henry, NotImportant, OhBoy, Reference, SimpleSubtypeA, SimpleSupertype, SubtypeA, SubtypeB,
    SuperT,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    henry: HashMap<Uuid, Henry>,
    not_important: HashMap<Uuid, NotImportant>,
    oh_boy: HashMap<Uuid, OhBoy>,
    reference: HashMap<Uuid, Reference>,
    simple_subtype_a: HashMap<Uuid, SimpleSubtypeA>,
    simple_supertype: HashMap<Uuid, SimpleSupertype>,
    subtype_a: HashMap<Uuid, SubtypeA>,
    subtype_b: HashMap<Uuid, SubtypeB>,
    super_t: HashMap<Uuid, SuperT>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            henry: HashMap::new(),
            not_important: HashMap::new(),
            oh_boy: HashMap::new(),
            reference: HashMap::new(),
            simple_subtype_a: HashMap::new(),
            simple_supertype: HashMap::new(),
            subtype_a: HashMap::new(),
            subtype_b: HashMap::new(),
            super_t: HashMap::new(),
        };

        // Initialize Singleton Subtypes

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa-object-store-methods"}}}
    /// Inter [`Henry`] into the store.
    ///
    pub fn inter_henry(&mut self, henry: Henry) {
        self.henry.insert(henry.id, henry);
    }

    /// Exhume [`Henry`] from the store.
    ///
    pub fn exhume_henry(&self, id: &Uuid) -> Option<&Henry> {
        self.henry.get(id)
    }
    /// Exhume [`Henry`] from the store — mutably.
    ///
    pub fn exhume_henry_mut(&mut self, id: &Uuid) -> Option<&mut Henry> {
        self.henry.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Henry>`.
    ///
    pub fn iter_henry(&self) -> impl Iterator<Item = &Henry> {
        self.henry.values()
    }
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
    pub fn iter_not_important(&self) -> impl Iterator<Item = &NotImportant> {
        self.not_important.values()
    }
    /// Inter [`OhBoy`] into the store.
    ///
    pub fn inter_oh_boy(&mut self, oh_boy: OhBoy) {
        self.oh_boy.insert(oh_boy.id, oh_boy);
    }

    /// Exhume [`OhBoy`] from the store.
    ///
    pub fn exhume_oh_boy(&self, id: &Uuid) -> Option<&OhBoy> {
        self.oh_boy.get(id)
    }
    /// Exhume [`OhBoy`] from the store — mutably.
    ///
    pub fn exhume_oh_boy_mut(&mut self, id: &Uuid) -> Option<&mut OhBoy> {
        self.oh_boy.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, OhBoy>`.
    ///
    pub fn iter_oh_boy(&self) -> impl Iterator<Item = &OhBoy> {
        self.oh_boy.values()
    }
    /// Inter [`Reference`] into the store.
    ///
    pub fn inter_reference(&mut self, reference: Reference) {
        self.reference.insert(reference.id, reference);
    }

    /// Exhume [`Reference`] from the store.
    ///
    pub fn exhume_reference(&self, id: &Uuid) -> Option<&Reference> {
        self.reference.get(id)
    }
    /// Exhume [`Reference`] from the store — mutably.
    ///
    pub fn exhume_reference_mut(&mut self, id: &Uuid) -> Option<&mut Reference> {
        self.reference.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Reference>`.
    ///
    pub fn iter_reference(&self) -> impl Iterator<Item = &Reference> {
        self.reference.values()
    }
    /// Inter [`SimpleSubtypeA`] into the store.
    ///
    pub fn inter_simple_subtype_a(&mut self, simple_subtype_a: SimpleSubtypeA) {
        self.simple_subtype_a
            .insert(simple_subtype_a.id(), simple_subtype_a);
    }

    /// Exhume [`SimpleSubtypeA`] from the store.
    ///
    pub fn exhume_simple_subtype_a(&self, id: &Uuid) -> Option<&SimpleSubtypeA> {
        self.simple_subtype_a.get(id)
    }
    /// Exhume [`SimpleSubtypeA`] from the store — mutably.
    ///
    pub fn exhume_simple_subtype_a_mut(&mut self, id: &Uuid) -> Option<&mut SimpleSubtypeA> {
        self.simple_subtype_a.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, SimpleSubtypeA>`.
    ///
    pub fn iter_simple_subtype_a(&self) -> impl Iterator<Item = &SimpleSubtypeA> {
        self.simple_subtype_a.values()
    }
    /// Inter [`SimpleSupertype`] into the store.
    ///
    pub fn inter_simple_supertype(&mut self, simple_supertype: SimpleSupertype) {
        self.simple_supertype
            .insert(simple_supertype.id, simple_supertype);
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
    pub fn iter_simple_supertype(&self) -> impl Iterator<Item = &SimpleSupertype> {
        self.simple_supertype.values()
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
    pub fn iter_subtype_a(&self) -> impl Iterator<Item = &SubtypeA> {
        self.subtype_a.values()
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
    pub fn iter_subtype_b(&self) -> impl Iterator<Item = &SubtypeB> {
        self.subtype_b.values()
    }
    /// Inter [`SuperT`] into the store.
    ///
    pub fn inter_super_t(&mut self, super_t: SuperT) {
        self.super_t.insert(super_t.id, super_t);
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
    pub fn iter_super_t(&self) -> impl Iterator<Item = &SuperT> {
        self.super_t.values()
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

        // Persist henry.
        {
            let path = path.join("henry.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.henry.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
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
        // Persist oh_boy.
        {
            let path = path.join("oh_boy.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.oh_boy.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist reference.
        {
            let path = path.join("reference.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.reference.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist simple_subtype_a.
        {
            let path = path.join("simple_subtype_a.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self
                    .simple_subtype_a
                    .values()
                    .map(|x| x)
                    .collect::<Vec<_>>(),
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

    /// Load the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let path = path.join("Isa Relationship.json");

        let mut store = Self::new();

        // Load henry.
        {
            let path = path.join("henry.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let henry: Vec<Henry> = serde_json::from_reader(reader)?;
            store.henry = henry.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load not_important.
        {
            let path = path.join("not_important.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let not_important: Vec<NotImportant> = serde_json::from_reader(reader)?;
            store.not_important = not_important.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load oh_boy.
        {
            let path = path.join("oh_boy.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let oh_boy: Vec<OhBoy> = serde_json::from_reader(reader)?;
            store.oh_boy = oh_boy.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load reference.
        {
            let path = path.join("reference.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let reference: Vec<Reference> = serde_json::from_reader(reader)?;
            store.reference = reference.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load simple_subtype_a.
        {
            let path = path.join("simple_subtype_a.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let simple_subtype_a: Vec<SimpleSubtypeA> = serde_json::from_reader(reader)?;
            store.simple_subtype_a = simple_subtype_a
                .into_iter()
                .map(|道| (道.id(), 道))
                .collect();
        }
        // Load simple_supertype.
        {
            let path = path.join("simple_supertype.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let simple_supertype: Vec<SimpleSupertype> = serde_json::from_reader(reader)?;
            store.simple_supertype = simple_supertype.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load subtype_a.
        {
            let path = path.join("subtype_a.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let subtype_a: Vec<SubtypeA> = serde_json::from_reader(reader)?;
            store.subtype_a = subtype_a.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load subtype_b.
        {
            let path = path.join("subtype_b.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let subtype_b: Vec<SubtypeB> = serde_json::from_reader(reader)?;
            store.subtype_b = subtype_b.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load super_t.
        {
            let path = path.join("super_t.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let super_t: Vec<SuperT> = serde_json::from_reader(reader)?;
            store.super_t = super_t.into_iter().map(|道| (道.id, 道)).collect();
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

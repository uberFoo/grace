//! domain::isa Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`Borrowed`]
//! * [`Henry`]
//! * [`NotImportant`]
//! * [`OhBoy`]
//! * [`Ownership`]
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
    Borrowed, Henry, NotImportant, OhBoy, Ownership, Reference, SimpleSubtypeA, SimpleSupertype,
    SubtypeA, SubtypeB, SuperT, MUTABLE, OWNED, SHARED,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    borrowed: HashMap<Uuid, Borrowed>,
    henry: HashMap<Uuid, Henry>,
    not_important: HashMap<Uuid, NotImportant>,
    oh_boy: HashMap<Uuid, OhBoy>,
    ownership: HashMap<Uuid, Ownership>,
    reference: HashMap<Uuid, Reference>,
    simple_subtype_a: HashMap<Uuid, SimpleSubtypeA>,
    simple_supertype: HashMap<Uuid, SimpleSupertype>,
    subtype_a: HashMap<Uuid, SubtypeA>,
    subtype_b: HashMap<Uuid, SubtypeB>,
    super_t: HashMap<Uuid, SuperT>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            borrowed: HashMap::new(),
            henry: HashMap::new(),
            not_important: HashMap::new(),
            oh_boy: HashMap::new(),
            ownership: HashMap::new(),
            reference: HashMap::new(),
            simple_subtype_a: HashMap::new(),
            simple_supertype: HashMap::new(),
            subtype_a: HashMap::new(),
            subtype_b: HashMap::new(),
            super_t: HashMap::new(),
        };

        // Initialize Singleton Subtypes
        store.inter_borrowed(Borrowed::Mutable(MUTABLE));
        store.inter_borrowed(Borrowed::Shared(SHARED));
        store.inter_ownership(Ownership::Owned(OWNED));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa-object-store-methods"}}}
    /// Inter [`Borrowed`] into the store.
    ///
    pub fn inter_borrowed(&mut self, borrowed: Borrowed) {
        self.borrowed.insert(borrowed.id(), borrowed);
    }

    /// Exhume [`Borrowed`] from the store.
    ///
    pub fn exhume_borrowed(&self, id: &Uuid) -> Option<&Borrowed> {
        self.borrowed.get(id)
    }

    /// Exhume [`Borrowed`] from the store — mutably.
    ///
    pub fn exhume_borrowed_mut(&mut self, id: &Uuid) -> Option<&mut Borrowed> {
        self.borrowed.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Borrowed>`.
    ///
    pub fn iter_borrowed(&self) -> impl Iterator<Item = &Borrowed> {
        self.borrowed.values()
    }

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

    /// Inter [`Ownership`] into the store.
    ///
    pub fn inter_ownership(&mut self, ownership: Ownership) {
        self.ownership.insert(ownership.id(), ownership);
    }

    /// Exhume [`Ownership`] from the store.
    ///
    pub fn exhume_ownership(&self, id: &Uuid) -> Option<&Ownership> {
        self.ownership.get(id)
    }

    /// Exhume [`Ownership`] from the store — mutably.
    ///
    pub fn exhume_ownership_mut(&mut self, id: &Uuid) -> Option<&mut Ownership> {
        self.ownership.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Ownership>`.
    ///
    pub fn iter_ownership(&self) -> impl Iterator<Item = &Ownership> {
        self.ownership.values()
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
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        let path = path.join("Isa Relationship.json");
        fs::create_dir_all(&path)?;

        // Persist Borrowed.
        {
            let path = path.join("borrowed");
            fs::create_dir_all(&path)?;
            for borrowed in self.borrowed.values() {
                let path = path.join(format!("{}.json", borrowed.id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &borrowed)?;
            }
        }

        // Persist Henry.
        {
            let path = path.join("henry");
            fs::create_dir_all(&path)?;
            for henry in self.henry.values() {
                let path = path.join(format!("{}.json", henry.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &henry)?;
            }
        }

        // Persist Not Important.
        {
            let path = path.join("not_important");
            fs::create_dir_all(&path)?;
            for not_important in self.not_important.values() {
                let path = path.join(format!("{}.json", not_important.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &not_important)?;
            }
        }

        // Persist Oh Boy!.
        {
            let path = path.join("oh_boy");
            fs::create_dir_all(&path)?;
            for oh_boy in self.oh_boy.values() {
                let path = path.join(format!("{}.json", oh_boy.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &oh_boy)?;
            }
        }

        // Persist Ownership.
        {
            let path = path.join("ownership");
            fs::create_dir_all(&path)?;
            for ownership in self.ownership.values() {
                let path = path.join(format!("{}.json", ownership.id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &ownership)?;
            }
        }

        // Persist Reference.
        {
            let path = path.join("reference");
            fs::create_dir_all(&path)?;
            for reference in self.reference.values() {
                let path = path.join(format!("{}.json", reference.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &reference)?;
            }
        }

        // Persist Simple Subtype A.
        {
            let path = path.join("simple_subtype_a");
            fs::create_dir_all(&path)?;
            for simple_subtype_a in self.simple_subtype_a.values() {
                let path = path.join(format!("{}.json", simple_subtype_a.id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &simple_subtype_a)?;
            }
        }

        // Persist Simple Supertype.
        {
            let path = path.join("simple_supertype");
            fs::create_dir_all(&path)?;
            for simple_supertype in self.simple_supertype.values() {
                let path = path.join(format!("{}.json", simple_supertype.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &simple_supertype)?;
            }
        }

        // Persist Subtype A.
        {
            let path = path.join("subtype_a");
            fs::create_dir_all(&path)?;
            for subtype_a in self.subtype_a.values() {
                let path = path.join(format!("{}.json", subtype_a.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &subtype_a)?;
            }
        }

        // Persist Subtype B.
        {
            let path = path.join("subtype_b");
            fs::create_dir_all(&path)?;
            for subtype_b in self.subtype_b.values() {
                let path = path.join(format!("{}.json", subtype_b.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &subtype_b)?;
            }
        }

        // Persist Super T.
        {
            let path = path.join("super_t");
            fs::create_dir_all(&path)?;
            for super_t in self.super_t.values() {
                let path = path.join(format!("{}.json", super_t.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &super_t)?;
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
        let path = path.join("Isa Relationship.json");

        let mut store = Self::new();

        // Load Borrowed.
        {
            let path = path.join("borrowed");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let borrowed: Borrowed = serde_json::from_reader(reader)?;
                store.borrowed.insert(borrowed.id(), borrowed);
            }
        }

        // Load Henry.
        {
            let path = path.join("henry");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let henry: Henry = serde_json::from_reader(reader)?;
                store.henry.insert(henry.id, henry);
            }
        }

        // Load Not Important.
        {
            let path = path.join("not_important");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let not_important: NotImportant = serde_json::from_reader(reader)?;
                store.not_important.insert(not_important.id, not_important);
            }
        }

        // Load Oh Boy!.
        {
            let path = path.join("oh_boy");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let oh_boy: OhBoy = serde_json::from_reader(reader)?;
                store.oh_boy.insert(oh_boy.id, oh_boy);
            }
        }

        // Load Ownership.
        {
            let path = path.join("ownership");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let ownership: Ownership = serde_json::from_reader(reader)?;
                store.ownership.insert(ownership.id(), ownership);
            }
        }

        // Load Reference.
        {
            let path = path.join("reference");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let reference: Reference = serde_json::from_reader(reader)?;
                store.reference.insert(reference.id, reference);
            }
        }

        // Load Simple Subtype A.
        {
            let path = path.join("simple_subtype_a");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let simple_subtype_a: SimpleSubtypeA = serde_json::from_reader(reader)?;
                store
                    .simple_subtype_a
                    .insert(simple_subtype_a.id(), simple_subtype_a);
            }
        }

        // Load Simple Supertype.
        {
            let path = path.join("simple_supertype");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let simple_supertype: SimpleSupertype = serde_json::from_reader(reader)?;
                store
                    .simple_supertype
                    .insert(simple_supertype.id, simple_supertype);
            }
        }

        // Load Subtype A.
        {
            let path = path.join("subtype_a");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let subtype_a: SubtypeA = serde_json::from_reader(reader)?;
                store.subtype_a.insert(subtype_a.id, subtype_a);
            }
        }

        // Load Subtype B.
        {
            let path = path.join("subtype_b");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let subtype_b: SubtypeB = serde_json::from_reader(reader)?;
                store.subtype_b.insert(subtype_b.id, subtype_b);
            }
        }

        // Load Super T.
        {
            let path = path.join("super_t");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let super_t: SuperT = serde_json::from_reader(reader)?;
                store.super_t.insert(super_t.id, super_t);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

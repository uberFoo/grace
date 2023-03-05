//! domain::isa_clone Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa_clone-object-store-file"}}}
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa_clone-object-store-definition"}}}
use std::collections::HashMap;
use std::{fs, io, path::Path, time::SystemTime};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::isa_clone::types::{
    Henry, NotImportant, OhBoy, Reference, SimpleSubtypeA, SimpleSupertype, SubtypeA, SubtypeB,
    SuperT,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    henry: HashMap<Uuid, (Henry, SystemTime)>,
    not_important: HashMap<Uuid, (NotImportant, SystemTime)>,
    oh_boy: HashMap<Uuid, (OhBoy, SystemTime)>,
    reference: HashMap<Uuid, (Reference, SystemTime)>,
    simple_subtype_a: HashMap<Uuid, (SimpleSubtypeA, SystemTime)>,
    simple_supertype: HashMap<Uuid, (SimpleSupertype, SystemTime)>,
    subtype_a: HashMap<Uuid, (SubtypeA, SystemTime)>,
    subtype_b: HashMap<Uuid, (SubtypeB, SystemTime)>,
    super_t: HashMap<Uuid, (SuperT, SystemTime)>,
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

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa_clone-object-store-methods"}}}
    /// Inter [`Henry`] into the store.
    ///
    pub fn inter_henry(&mut self, henry: Henry) {
        self.henry.insert(henry.id, (henry, SystemTime::now()));
    }

    /// Exhume [`Henry`] from the store.
    ///
    pub fn exhume_henry(&self, id: &Uuid) -> Option<&Henry> {
        self.henry.get(id).map(|henry| &henry.0)
    }

    /// Exhume [`Henry`] from the store — mutably.
    ///
    pub fn exhume_henry_mut(&mut self, id: &Uuid) -> Option<&mut Henry> {
        self.henry.get_mut(id).map(|henry| &mut henry.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Henry>`.
    ///
    pub fn iter_henry(&self) -> impl Iterator<Item = &Henry> {
        self.henry.values().map(|henry| &henry.0)
    }

    /// Get the timestamp for Henry.
    ///
    pub fn henry_timestamp(&self, henry: &Henry) -> SystemTime {
        self.henry
            .get(&henry.id)
            .map(|henry| henry.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`NotImportant`] into the store.
    ///
    pub fn inter_not_important(&mut self, not_important: NotImportant) {
        self.not_important
            .insert(not_important.id, (not_important, SystemTime::now()));
    }

    /// Exhume [`NotImportant`] from the store.
    ///
    pub fn exhume_not_important(&self, id: &Uuid) -> Option<&NotImportant> {
        self.not_important
            .get(id)
            .map(|not_important| &not_important.0)
    }

    /// Exhume [`NotImportant`] from the store — mutably.
    ///
    pub fn exhume_not_important_mut(&mut self, id: &Uuid) -> Option<&mut NotImportant> {
        self.not_important
            .get_mut(id)
            .map(|not_important| &mut not_important.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, NotImportant>`.
    ///
    pub fn iter_not_important(&self) -> impl Iterator<Item = &NotImportant> {
        self.not_important
            .values()
            .map(|not_important| &not_important.0)
    }

    /// Get the timestamp for NotImportant.
    ///
    pub fn not_important_timestamp(&self, not_important: &NotImportant) -> SystemTime {
        self.not_important
            .get(&not_important.id)
            .map(|not_important| not_important.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`OhBoy`] into the store.
    ///
    pub fn inter_oh_boy(&mut self, oh_boy: OhBoy) {
        self.oh_boy.insert(oh_boy.id, (oh_boy, SystemTime::now()));
    }

    /// Exhume [`OhBoy`] from the store.
    ///
    pub fn exhume_oh_boy(&self, id: &Uuid) -> Option<&OhBoy> {
        self.oh_boy.get(id).map(|oh_boy| &oh_boy.0)
    }

    /// Exhume [`OhBoy`] from the store — mutably.
    ///
    pub fn exhume_oh_boy_mut(&mut self, id: &Uuid) -> Option<&mut OhBoy> {
        self.oh_boy.get_mut(id).map(|oh_boy| &mut oh_boy.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, OhBoy>`.
    ///
    pub fn iter_oh_boy(&self) -> impl Iterator<Item = &OhBoy> {
        self.oh_boy.values().map(|oh_boy| &oh_boy.0)
    }

    /// Get the timestamp for OhBoy.
    ///
    pub fn oh_boy_timestamp(&self, oh_boy: &OhBoy) -> SystemTime {
        self.oh_boy
            .get(&oh_boy.id)
            .map(|oh_boy| oh_boy.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Reference`] into the store.
    ///
    pub fn inter_reference(&mut self, reference: Reference) {
        self.reference
            .insert(reference.id, (reference, SystemTime::now()));
    }

    /// Exhume [`Reference`] from the store.
    ///
    pub fn exhume_reference(&self, id: &Uuid) -> Option<&Reference> {
        self.reference.get(id).map(|reference| &reference.0)
    }

    /// Exhume [`Reference`] from the store — mutably.
    ///
    pub fn exhume_reference_mut(&mut self, id: &Uuid) -> Option<&mut Reference> {
        self.reference.get_mut(id).map(|reference| &mut reference.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Reference>`.
    ///
    pub fn iter_reference(&self) -> impl Iterator<Item = &Reference> {
        self.reference.values().map(|reference| &reference.0)
    }

    /// Get the timestamp for Reference.
    ///
    pub fn reference_timestamp(&self, reference: &Reference) -> SystemTime {
        self.reference
            .get(&reference.id)
            .map(|reference| reference.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`SimpleSubtypeA`] into the store.
    ///
    pub fn inter_simple_subtype_a(&mut self, simple_subtype_a: SimpleSubtypeA) {
        self.simple_subtype_a
            .insert(simple_subtype_a.id(), (simple_subtype_a, SystemTime::now()));
    }

    /// Exhume [`SimpleSubtypeA`] from the store.
    ///
    pub fn exhume_simple_subtype_a(&self, id: &Uuid) -> Option<&SimpleSubtypeA> {
        self.simple_subtype_a
            .get(id)
            .map(|simple_subtype_a| &simple_subtype_a.0)
    }

    /// Exhume [`SimpleSubtypeA`] from the store — mutably.
    ///
    pub fn exhume_simple_subtype_a_mut(&mut self, id: &Uuid) -> Option<&mut SimpleSubtypeA> {
        self.simple_subtype_a
            .get_mut(id)
            .map(|simple_subtype_a| &mut simple_subtype_a.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SimpleSubtypeA>`.
    ///
    pub fn iter_simple_subtype_a(&self) -> impl Iterator<Item = &SimpleSubtypeA> {
        self.simple_subtype_a
            .values()
            .map(|simple_subtype_a| &simple_subtype_a.0)
    }

    /// Get the timestamp for SimpleSubtypeA.
    ///
    pub fn simple_subtype_a_timestamp(&self, simple_subtype_a: &SimpleSubtypeA) -> SystemTime {
        self.simple_subtype_a
            .get(&simple_subtype_a.id())
            .map(|simple_subtype_a| simple_subtype_a.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`SimpleSupertype`] into the store.
    ///
    pub fn inter_simple_supertype(&mut self, simple_supertype: SimpleSupertype) {
        self.simple_supertype
            .insert(simple_supertype.id, (simple_supertype, SystemTime::now()));
    }

    /// Exhume [`SimpleSupertype`] from the store.
    ///
    pub fn exhume_simple_supertype(&self, id: &Uuid) -> Option<&SimpleSupertype> {
        self.simple_supertype
            .get(id)
            .map(|simple_supertype| &simple_supertype.0)
    }

    /// Exhume [`SimpleSupertype`] from the store — mutably.
    ///
    pub fn exhume_simple_supertype_mut(&mut self, id: &Uuid) -> Option<&mut SimpleSupertype> {
        self.simple_supertype
            .get_mut(id)
            .map(|simple_supertype| &mut simple_supertype.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SimpleSupertype>`.
    ///
    pub fn iter_simple_supertype(&self) -> impl Iterator<Item = &SimpleSupertype> {
        self.simple_supertype
            .values()
            .map(|simple_supertype| &simple_supertype.0)
    }

    /// Get the timestamp for SimpleSupertype.
    ///
    pub fn simple_supertype_timestamp(&self, simple_supertype: &SimpleSupertype) -> SystemTime {
        self.simple_supertype
            .get(&simple_supertype.id)
            .map(|simple_supertype| simple_supertype.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`SubtypeA`] into the store.
    ///
    pub fn inter_subtype_a(&mut self, subtype_a: SubtypeA) {
        self.subtype_a
            .insert(subtype_a.id, (subtype_a, SystemTime::now()));
    }

    /// Exhume [`SubtypeA`] from the store.
    ///
    pub fn exhume_subtype_a(&self, id: &Uuid) -> Option<&SubtypeA> {
        self.subtype_a.get(id).map(|subtype_a| &subtype_a.0)
    }

    /// Exhume [`SubtypeA`] from the store — mutably.
    ///
    pub fn exhume_subtype_a_mut(&mut self, id: &Uuid) -> Option<&mut SubtypeA> {
        self.subtype_a.get_mut(id).map(|subtype_a| &mut subtype_a.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeA>`.
    ///
    pub fn iter_subtype_a(&self) -> impl Iterator<Item = &SubtypeA> {
        self.subtype_a.values().map(|subtype_a| &subtype_a.0)
    }

    /// Get the timestamp for SubtypeA.
    ///
    pub fn subtype_a_timestamp(&self, subtype_a: &SubtypeA) -> SystemTime {
        self.subtype_a
            .get(&subtype_a.id)
            .map(|subtype_a| subtype_a.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`SubtypeB`] into the store.
    ///
    pub fn inter_subtype_b(&mut self, subtype_b: SubtypeB) {
        self.subtype_b
            .insert(subtype_b.id, (subtype_b, SystemTime::now()));
    }

    /// Exhume [`SubtypeB`] from the store.
    ///
    pub fn exhume_subtype_b(&self, id: &Uuid) -> Option<&SubtypeB> {
        self.subtype_b.get(id).map(|subtype_b| &subtype_b.0)
    }

    /// Exhume [`SubtypeB`] from the store — mutably.
    ///
    pub fn exhume_subtype_b_mut(&mut self, id: &Uuid) -> Option<&mut SubtypeB> {
        self.subtype_b.get_mut(id).map(|subtype_b| &mut subtype_b.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeB>`.
    ///
    pub fn iter_subtype_b(&self) -> impl Iterator<Item = &SubtypeB> {
        self.subtype_b.values().map(|subtype_b| &subtype_b.0)
    }

    /// Get the timestamp for SubtypeB.
    ///
    pub fn subtype_b_timestamp(&self, subtype_b: &SubtypeB) -> SystemTime {
        self.subtype_b
            .get(&subtype_b.id)
            .map(|subtype_b| subtype_b.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`SuperT`] into the store.
    ///
    pub fn inter_super_t(&mut self, super_t: SuperT) {
        self.super_t
            .insert(super_t.id, (super_t, SystemTime::now()));
    }

    /// Exhume [`SuperT`] from the store.
    ///
    pub fn exhume_super_t(&self, id: &Uuid) -> Option<&SuperT> {
        self.super_t.get(id).map(|super_t| &super_t.0)
    }

    /// Exhume [`SuperT`] from the store — mutably.
    ///
    pub fn exhume_super_t_mut(&mut self, id: &Uuid) -> Option<&mut SuperT> {
        self.super_t.get_mut(id).map(|super_t| &mut super_t.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SuperT>`.
    ///
    pub fn iter_super_t(&self) -> impl Iterator<Item = &SuperT> {
        self.super_t.values().map(|super_t| &super_t.0)
    }

    /// Get the timestamp for SuperT.
    ///
    pub fn super_t_timestamp(&self, super_t: &SuperT) -> SystemTime {
        self.super_t
            .get(&super_t.id)
            .map(|super_t| super_t.1)
            .unwrap_or(SystemTime::now())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa_clone-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        let path = path.join("Isa Relationship.json");
        fs::create_dir_all(&path)?;

        // Persist Henry.
        {
            let path = path.join("henry");
            fs::create_dir_all(&path)?;
            for henry_tuple in self.henry.values() {
                let path = path.join(format!("{}.json", henry_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Henry, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != henry_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &henry_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &henry_tuple)?;
                }
            }
        }

        // Persist Not Important.
        {
            let path = path.join("not_important");
            fs::create_dir_all(&path)?;
            for not_important_tuple in self.not_important.values() {
                let path = path.join(format!("{}.json", not_important_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (NotImportant, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != not_important_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &not_important_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &not_important_tuple)?;
                }
            }
        }

        // Persist Oh Boy!.
        {
            let path = path.join("oh_boy");
            fs::create_dir_all(&path)?;
            for oh_boy_tuple in self.oh_boy.values() {
                let path = path.join(format!("{}.json", oh_boy_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (OhBoy, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != oh_boy_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &oh_boy_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &oh_boy_tuple)?;
                }
            }
        }

        // Persist Reference.
        {
            let path = path.join("reference");
            fs::create_dir_all(&path)?;
            for reference_tuple in self.reference.values() {
                let path = path.join(format!("{}.json", reference_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Reference, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != reference_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &reference_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &reference_tuple)?;
                }
            }
        }

        // Persist Simple Subtype A.
        {
            let path = path.join("simple_subtype_a");
            fs::create_dir_all(&path)?;
            for simple_subtype_a_tuple in self.simple_subtype_a.values() {
                let path = path.join(format!("{}.json", simple_subtype_a_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (SimpleSubtypeA, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != simple_subtype_a_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &simple_subtype_a_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &simple_subtype_a_tuple)?;
                }
            }
        }

        // Persist Simple Supertype.
        {
            let path = path.join("simple_supertype");
            fs::create_dir_all(&path)?;
            for simple_supertype_tuple in self.simple_supertype.values() {
                let path = path.join(format!("{}.json", simple_supertype_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (SimpleSupertype, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != simple_supertype_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &simple_supertype_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &simple_supertype_tuple)?;
                }
            }
        }

        // Persist Subtype A.
        {
            let path = path.join("subtype_a");
            fs::create_dir_all(&path)?;
            for subtype_a_tuple in self.subtype_a.values() {
                let path = path.join(format!("{}.json", subtype_a_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (SubtypeA, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != subtype_a_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &subtype_a_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &subtype_a_tuple)?;
                }
            }
        }

        // Persist Subtype B.
        {
            let path = path.join("subtype_b");
            fs::create_dir_all(&path)?;
            for subtype_b_tuple in self.subtype_b.values() {
                let path = path.join(format!("{}.json", subtype_b_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (SubtypeB, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != subtype_b_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &subtype_b_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &subtype_b_tuple)?;
                }
            }
        }

        // Persist Super T.
        {
            let path = path.join("super_t");
            fs::create_dir_all(&path)?;
            for super_t_tuple in self.super_t.values() {
                let path = path.join(format!("{}.json", super_t_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (SuperT, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != super_t_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &super_t_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &super_t_tuple)?;
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
        let path = path.join("Isa Relationship.json");

        let mut store = Self::new();

        // Load Henry.
        {
            let path = path.join("henry");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let henry: (Henry, SystemTime) = serde_json::from_reader(reader)?;
                store.henry.insert(henry.0.id, henry);
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
                let not_important: (NotImportant, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .not_important
                    .insert(not_important.0.id, not_important);
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
                let oh_boy: (OhBoy, SystemTime) = serde_json::from_reader(reader)?;
                store.oh_boy.insert(oh_boy.0.id, oh_boy);
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
                let reference: (Reference, SystemTime) = serde_json::from_reader(reader)?;
                store.reference.insert(reference.0.id, reference);
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
                let simple_subtype_a: (SimpleSubtypeA, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .simple_subtype_a
                    .insert(simple_subtype_a.0.id(), simple_subtype_a);
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
                let simple_supertype: (SimpleSupertype, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .simple_supertype
                    .insert(simple_supertype.0.id, simple_supertype);
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
                let subtype_a: (SubtypeA, SystemTime) = serde_json::from_reader(reader)?;
                store.subtype_a.insert(subtype_a.0.id, subtype_a);
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
                let subtype_b: (SubtypeB, SystemTime) = serde_json::from_reader(reader)?;
                store.subtype_b.insert(subtype_b.0.id, subtype_b);
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
                let super_t: (SuperT, SystemTime) = serde_json::from_reader(reader)?;
                store.super_t.insert(super_t.0.id, super_t);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

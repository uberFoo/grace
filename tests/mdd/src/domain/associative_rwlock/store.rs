//! domain::associative_rwlock Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::associative_rwlock-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`AcknowledgedEvent`]
//! * [`Anchor`]
//! * [`Event`]
//! * [`IsaUi`]
//! * [`State`]
//! * [`SubtypeAnchor`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::associative_rwlock-object-store-definition"}}}
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

use crate::domain::associative_rwlock::types::{
    AcknowledgedEvent, Anchor, Event, IsaUi, State, SubtypeAnchor,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    acknowledged_event: Arc<RwLock<HashMap<Uuid, Arc<RwLock<AcknowledgedEvent>>>>>,
    anchor: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Anchor>>>>>,
    event: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Event>>>>>,
    isa_ui: Arc<RwLock<HashMap<Uuid, Arc<RwLock<IsaUi>>>>>,
    state: Arc<RwLock<HashMap<Uuid, Arc<RwLock<State>>>>>,
    subtype_anchor: Arc<RwLock<HashMap<Uuid, Arc<RwLock<SubtypeAnchor>>>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            acknowledged_event: Arc::new(RwLock::new(HashMap::default())),
            anchor: Arc::new(RwLock::new(HashMap::default())),
            event: Arc::new(RwLock::new(HashMap::default())),
            isa_ui: Arc::new(RwLock::new(HashMap::default())),
            state: Arc::new(RwLock::new(HashMap::default())),
            subtype_anchor: Arc::new(RwLock::new(HashMap::default())),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::associative_rwlock-object-store-methods"}}}
    /// Inter (insert) [`AcknowledgedEvent`] into the store.
    ///
    pub fn inter_acknowledged_event(&mut self, acknowledged_event: Arc<RwLock<AcknowledgedEvent>>) {
        let read = acknowledged_event.read().unwrap();
        self.acknowledged_event
            .write()
            .unwrap()
            .insert(read.id, acknowledged_event.clone());
    }

    /// Exhume (get) [`AcknowledgedEvent`] from the store.
    ///
    pub fn exhume_acknowledged_event(&self, id: &Uuid) -> Option<Arc<RwLock<AcknowledgedEvent>>> {
        self.acknowledged_event
            .read()
            .unwrap()
            .get(id)
            .map(|acknowledged_event| acknowledged_event.clone())
    }

    /// Exorcise (remove) [`AcknowledgedEvent`] from the store.
    ///
    pub fn exorcise_acknowledged_event(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<AcknowledgedEvent>>> {
        self.acknowledged_event
            .write()
            .unwrap()
            .remove(id)
            .map(|acknowledged_event| acknowledged_event.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AcknowledgedEvent>`.
    ///
    pub fn iter_acknowledged_event(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<AcknowledgedEvent>>> + '_ {
        let values: Vec<Arc<RwLock<AcknowledgedEvent>>> = self
            .acknowledged_event
            .read()
            .unwrap()
            .values()
            .map(|acknowledged_event| acknowledged_event.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Anchor`] into the store.
    ///
    pub fn inter_anchor(&mut self, anchor: Arc<RwLock<Anchor>>) {
        let read = anchor.read().unwrap();
        self.anchor.write().unwrap().insert(read.id, anchor.clone());
    }

    /// Exhume (get) [`Anchor`] from the store.
    ///
    pub fn exhume_anchor(&self, id: &Uuid) -> Option<Arc<RwLock<Anchor>>> {
        self.anchor
            .read()
            .unwrap()
            .get(id)
            .map(|anchor| anchor.clone())
    }

    /// Exorcise (remove) [`Anchor`] from the store.
    ///
    pub fn exorcise_anchor(&mut self, id: &Uuid) -> Option<Arc<RwLock<Anchor>>> {
        self.anchor
            .write()
            .unwrap()
            .remove(id)
            .map(|anchor| anchor.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Anchor>`.
    ///
    pub fn iter_anchor(&self) -> impl Iterator<Item = Arc<RwLock<Anchor>>> + '_ {
        let values: Vec<Arc<RwLock<Anchor>>> = self
            .anchor
            .read()
            .unwrap()
            .values()
            .map(|anchor| anchor.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Event`] into the store.
    ///
    pub fn inter_event(&mut self, event: Arc<RwLock<Event>>) {
        let read = event.read().unwrap();
        self.event.write().unwrap().insert(read.id, event.clone());
    }

    /// Exhume (get) [`Event`] from the store.
    ///
    pub fn exhume_event(&self, id: &Uuid) -> Option<Arc<RwLock<Event>>> {
        self.event
            .read()
            .unwrap()
            .get(id)
            .map(|event| event.clone())
    }

    /// Exorcise (remove) [`Event`] from the store.
    ///
    pub fn exorcise_event(&mut self, id: &Uuid) -> Option<Arc<RwLock<Event>>> {
        self.event
            .write()
            .unwrap()
            .remove(id)
            .map(|event| event.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Event>`.
    ///
    pub fn iter_event(&self) -> impl Iterator<Item = Arc<RwLock<Event>>> + '_ {
        let values: Vec<Arc<RwLock<Event>>> = self
            .event
            .read()
            .unwrap()
            .values()
            .map(|event| event.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`IsaUi`] into the store.
    ///
    pub fn inter_isa_ui(&mut self, isa_ui: Arc<RwLock<IsaUi>>) {
        let read = isa_ui.read().unwrap();
        self.isa_ui.write().unwrap().insert(read.id, isa_ui.clone());
    }

    /// Exhume (get) [`IsaUi`] from the store.
    ///
    pub fn exhume_isa_ui(&self, id: &Uuid) -> Option<Arc<RwLock<IsaUi>>> {
        self.isa_ui
            .read()
            .unwrap()
            .get(id)
            .map(|isa_ui| isa_ui.clone())
    }

    /// Exorcise (remove) [`IsaUi`] from the store.
    ///
    pub fn exorcise_isa_ui(&mut self, id: &Uuid) -> Option<Arc<RwLock<IsaUi>>> {
        self.isa_ui
            .write()
            .unwrap()
            .remove(id)
            .map(|isa_ui| isa_ui.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IsaUi>`.
    ///
    pub fn iter_isa_ui(&self) -> impl Iterator<Item = Arc<RwLock<IsaUi>>> + '_ {
        let values: Vec<Arc<RwLock<IsaUi>>> = self
            .isa_ui
            .read()
            .unwrap()
            .values()
            .map(|isa_ui| isa_ui.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`State`] into the store.
    ///
    pub fn inter_state(&mut self, state: Arc<RwLock<State>>) {
        let read = state.read().unwrap();
        self.state.write().unwrap().insert(read.id, state.clone());
    }

    /// Exhume (get) [`State`] from the store.
    ///
    pub fn exhume_state(&self, id: &Uuid) -> Option<Arc<RwLock<State>>> {
        self.state
            .read()
            .unwrap()
            .get(id)
            .map(|state| state.clone())
    }

    /// Exorcise (remove) [`State`] from the store.
    ///
    pub fn exorcise_state(&mut self, id: &Uuid) -> Option<Arc<RwLock<State>>> {
        self.state
            .write()
            .unwrap()
            .remove(id)
            .map(|state| state.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, State>`.
    ///
    pub fn iter_state(&self) -> impl Iterator<Item = Arc<RwLock<State>>> + '_ {
        let values: Vec<Arc<RwLock<State>>> = self
            .state
            .read()
            .unwrap()
            .values()
            .map(|state| state.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`SubtypeAnchor`] into the store.
    ///
    pub fn inter_subtype_anchor(&mut self, subtype_anchor: Arc<RwLock<SubtypeAnchor>>) {
        let read = subtype_anchor.read().unwrap();
        self.subtype_anchor
            .write()
            .unwrap()
            .insert(read.id, subtype_anchor.clone());
    }

    /// Exhume (get) [`SubtypeAnchor`] from the store.
    ///
    pub fn exhume_subtype_anchor(&self, id: &Uuid) -> Option<Arc<RwLock<SubtypeAnchor>>> {
        self.subtype_anchor
            .read()
            .unwrap()
            .get(id)
            .map(|subtype_anchor| subtype_anchor.clone())
    }

    /// Exorcise (remove) [`SubtypeAnchor`] from the store.
    ///
    pub fn exorcise_subtype_anchor(&mut self, id: &Uuid) -> Option<Arc<RwLock<SubtypeAnchor>>> {
        self.subtype_anchor
            .write()
            .unwrap()
            .remove(id)
            .map(|subtype_anchor| subtype_anchor.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeAnchor>`.
    ///
    pub fn iter_subtype_anchor(&self) -> impl Iterator<Item = Arc<RwLock<SubtypeAnchor>>> + '_ {
        let values: Vec<Arc<RwLock<SubtypeAnchor>>> = self
            .subtype_anchor
            .read()
            .unwrap()
            .values()
            .map(|subtype_anchor| subtype_anchor.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::associative_rwlock-object-store-persistence"}}}
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

        let path = path.join("associative.json");
        fs::create_dir_all(&path)?;

        // Persist Acknowledged Event.
        {
            let path = path.join("acknowledged_event");
            fs::create_dir_all(&path)?;
            for acknowledged_event in self.acknowledged_event.read().unwrap().values() {
                let path = path.join(format!("{}.json", acknowledged_event.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &acknowledged_event)?;
            }
        }

        // Persist Anchor.
        {
            let path = path.join("anchor");
            fs::create_dir_all(&path)?;
            for anchor in self.anchor.read().unwrap().values() {
                let path = path.join(format!("{}.json", anchor.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &anchor)?;
            }
        }

        // Persist Event.
        {
            let path = path.join("event");
            fs::create_dir_all(&path)?;
            for event in self.event.read().unwrap().values() {
                let path = path.join(format!("{}.json", event.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &event)?;
            }
        }

        // Persist IsaUI.
        {
            let path = path.join("isa_ui");
            fs::create_dir_all(&path)?;
            for isa_ui in self.isa_ui.read().unwrap().values() {
                let path = path.join(format!("{}.json", isa_ui.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &isa_ui)?;
            }
        }

        // Persist State.
        {
            let path = path.join("state");
            fs::create_dir_all(&path)?;
            for state in self.state.read().unwrap().values() {
                let path = path.join(format!("{}.json", state.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &state)?;
            }
        }

        // Persist Subtype Anchor.
        {
            let path = path.join("subtype_anchor");
            fs::create_dir_all(&path)?;
            for subtype_anchor in self.subtype_anchor.read().unwrap().values() {
                let path = path.join(format!("{}.json", subtype_anchor.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &subtype_anchor)?;
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
        let path = path.join("associative.json");

        let store = Self::new();

        // Load Acknowledged Event.
        {
            let path = path.join("acknowledged_event");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let acknowledged_event: Arc<RwLock<AcknowledgedEvent>> =
                    serde_json::from_reader(reader)?;
                store.acknowledged_event.write().unwrap().insert(
                    acknowledged_event.read().unwrap().id,
                    acknowledged_event.clone(),
                );
            }
        }

        // Load Anchor.
        {
            let path = path.join("anchor");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let anchor: Arc<RwLock<Anchor>> = serde_json::from_reader(reader)?;
                store
                    .anchor
                    .write()
                    .unwrap()
                    .insert(anchor.read().unwrap().id, anchor.clone());
            }
        }

        // Load Event.
        {
            let path = path.join("event");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let event: Arc<RwLock<Event>> = serde_json::from_reader(reader)?;
                store
                    .event
                    .write()
                    .unwrap()
                    .insert(event.read().unwrap().id, event.clone());
            }
        }

        // Load IsaUI.
        {
            let path = path.join("isa_ui");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let isa_ui: Arc<RwLock<IsaUi>> = serde_json::from_reader(reader)?;
                store
                    .isa_ui
                    .write()
                    .unwrap()
                    .insert(isa_ui.read().unwrap().id, isa_ui.clone());
            }
        }

        // Load State.
        {
            let path = path.join("state");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let state: Arc<RwLock<State>> = serde_json::from_reader(reader)?;
                store
                    .state
                    .write()
                    .unwrap()
                    .insert(state.read().unwrap().id, state.clone());
            }
        }

        // Load Subtype Anchor.
        {
            let path = path.join("subtype_anchor");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let subtype_anchor: Arc<RwLock<SubtypeAnchor>> = serde_json::from_reader(reader)?;
                store
                    .subtype_anchor
                    .write()
                    .unwrap()
                    .insert(subtype_anchor.read().unwrap().id, subtype_anchor.clone());
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

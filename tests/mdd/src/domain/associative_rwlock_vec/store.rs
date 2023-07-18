//! domain::associative_rwlock_vec Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::associative_rwlock_vec-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`AcknowledgedEvent`]
//! * [`Anchor`]
//! * [`Event`]
//! * [`IsaUi`]
//! * [`State`]
//! * [`SubtypeAnchor`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::associative_rwlock_vec-object-store-definition"}}}
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

use crate::domain::associative_rwlock_vec::types::{
    AcknowledgedEvent, Anchor, Event, IsaUi, State, SubtypeAnchor,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    acknowledged_event_free_list: std::sync::Mutex<Vec<usize>>,
    acknowledged_event: Arc<RwLock<Vec<Option<Arc<RwLock<AcknowledgedEvent>>>>>>,
    anchor_free_list: std::sync::Mutex<Vec<usize>>,
    anchor: Arc<RwLock<Vec<Option<Arc<RwLock<Anchor>>>>>>,
    event_free_list: std::sync::Mutex<Vec<usize>>,
    event: Arc<RwLock<Vec<Option<Arc<RwLock<Event>>>>>>,
    isa_ui_free_list: std::sync::Mutex<Vec<usize>>,
    isa_ui: Arc<RwLock<Vec<Option<Arc<RwLock<IsaUi>>>>>>,
    state_free_list: std::sync::Mutex<Vec<usize>>,
    state: Arc<RwLock<Vec<Option<Arc<RwLock<State>>>>>>,
    subtype_anchor_free_list: std::sync::Mutex<Vec<usize>>,
    subtype_anchor: Arc<RwLock<Vec<Option<Arc<RwLock<SubtypeAnchor>>>>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            acknowledged_event_free_list: std::sync::Mutex::new(Vec::new()),
            acknowledged_event: Arc::new(RwLock::new(Vec::new())),
            anchor_free_list: std::sync::Mutex::new(Vec::new()),
            anchor: Arc::new(RwLock::new(Vec::new())),
            event_free_list: std::sync::Mutex::new(Vec::new()),
            event: Arc::new(RwLock::new(Vec::new())),
            isa_ui_free_list: std::sync::Mutex::new(Vec::new()),
            isa_ui: Arc::new(RwLock::new(Vec::new())),
            state_free_list: std::sync::Mutex::new(Vec::new()),
            state: Arc::new(RwLock::new(Vec::new())),
            subtype_anchor_free_list: std::sync::Mutex::new(Vec::new()),
            subtype_anchor: Arc::new(RwLock::new(Vec::new())),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::associative_rwlock_vec-object-store-methods"}}}
    /// Inter (insert) [`AcknowledgedEvent`] into the store.
    ///
    pub fn inter_acknowledged_event<F>(
        &mut self,
        acknowledged_event: F,
    ) -> Arc<RwLock<AcknowledgedEvent>>
    where
        F: Fn(usize) -> Arc<RwLock<AcknowledgedEvent>>,
    {
        let _index = if let Some(_index) = self.acknowledged_event_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.acknowledged_event.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.acknowledged_event.write().unwrap().push(None);
            _index
        };

        let acknowledged_event = acknowledged_event(_index);

        let found = if let Some(acknowledged_event) = self
            .acknowledged_event
            .read()
            .unwrap()
            .iter()
            .find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *acknowledged_event.read().unwrap()
                } else {
                    false
                }
            }) {
            acknowledged_event.clone()
        } else {
            None
        };

        if let Some(acknowledged_event) = found {
            log::debug!(target: "store", "found duplicate {acknowledged_event:?}.");
            self.acknowledged_event_free_list
                .lock()
                .unwrap()
                .push(_index);
            acknowledged_event.clone()
        } else {
            log::debug!(target: "store", "interring {acknowledged_event:?}.");
            self.acknowledged_event.write().unwrap()[_index] = Some(acknowledged_event.clone());
            acknowledged_event
        }
    }

    /// Exhume (get) [`AcknowledgedEvent`] from the store.
    ///
    pub fn exhume_acknowledged_event(&self, id: &usize) -> Option<Arc<RwLock<AcknowledgedEvent>>> {
        match self.acknowledged_event.read().unwrap().get(*id) {
            Some(acknowledged_event) => acknowledged_event.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`AcknowledgedEvent`] from the store.
    ///
    pub fn exorcise_acknowledged_event(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<AcknowledgedEvent>>> {
        let result = self.acknowledged_event.write().unwrap()[*id].take();
        self.acknowledged_event_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AcknowledgedEvent>`.
    ///
    pub fn iter_acknowledged_event(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<AcknowledgedEvent>>> + '_ {
        let len = self.acknowledged_event.read().unwrap().len();
        (0..len)
            .filter(|i| self.acknowledged_event.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.acknowledged_event.read().unwrap()[i]
                    .as_ref()
                    .map(|acknowledged_event| acknowledged_event.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Anchor`] into the store.
    ///
    pub fn inter_anchor<F>(&mut self, anchor: F) -> Arc<RwLock<Anchor>>
    where
        F: Fn(usize) -> Arc<RwLock<Anchor>>,
    {
        let _index = if let Some(_index) = self.anchor_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.anchor.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.anchor.write().unwrap().push(None);
            _index
        };

        let anchor = anchor(_index);

        let found = if let Some(anchor) = self.anchor.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *anchor.read().unwrap()
            } else {
                false
            }
        }) {
            anchor.clone()
        } else {
            None
        };

        if let Some(anchor) = found {
            log::debug!(target: "store", "found duplicate {anchor:?}.");
            self.anchor_free_list.lock().unwrap().push(_index);
            anchor.clone()
        } else {
            log::debug!(target: "store", "interring {anchor:?}.");
            self.anchor.write().unwrap()[_index] = Some(anchor.clone());
            anchor
        }
    }

    /// Exhume (get) [`Anchor`] from the store.
    ///
    pub fn exhume_anchor(&self, id: &usize) -> Option<Arc<RwLock<Anchor>>> {
        match self.anchor.read().unwrap().get(*id) {
            Some(anchor) => anchor.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Anchor`] from the store.
    ///
    pub fn exorcise_anchor(&mut self, id: &usize) -> Option<Arc<RwLock<Anchor>>> {
        let result = self.anchor.write().unwrap()[*id].take();
        self.anchor_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Anchor>`.
    ///
    pub fn iter_anchor(&self) -> impl Iterator<Item = Arc<RwLock<Anchor>>> + '_ {
        let len = self.anchor.read().unwrap().len();
        (0..len)
            .filter(|i| self.anchor.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.anchor.read().unwrap()[i]
                    .as_ref()
                    .map(|anchor| anchor.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Event`] into the store.
    ///
    pub fn inter_event<F>(&mut self, event: F) -> Arc<RwLock<Event>>
    where
        F: Fn(usize) -> Arc<RwLock<Event>>,
    {
        let _index = if let Some(_index) = self.event_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.event.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.event.write().unwrap().push(None);
            _index
        };

        let event = event(_index);

        let found = if let Some(event) = self.event.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *event.read().unwrap()
            } else {
                false
            }
        }) {
            event.clone()
        } else {
            None
        };

        if let Some(event) = found {
            log::debug!(target: "store", "found duplicate {event:?}.");
            self.event_free_list.lock().unwrap().push(_index);
            event.clone()
        } else {
            log::debug!(target: "store", "interring {event:?}.");
            self.event.write().unwrap()[_index] = Some(event.clone());
            event
        }
    }

    /// Exhume (get) [`Event`] from the store.
    ///
    pub fn exhume_event(&self, id: &usize) -> Option<Arc<RwLock<Event>>> {
        match self.event.read().unwrap().get(*id) {
            Some(event) => event.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Event`] from the store.
    ///
    pub fn exorcise_event(&mut self, id: &usize) -> Option<Arc<RwLock<Event>>> {
        let result = self.event.write().unwrap()[*id].take();
        self.event_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Event>`.
    ///
    pub fn iter_event(&self) -> impl Iterator<Item = Arc<RwLock<Event>>> + '_ {
        let len = self.event.read().unwrap().len();
        (0..len)
            .filter(|i| self.event.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.event.read().unwrap()[i]
                    .as_ref()
                    .map(|event| event.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`IsaUi`] into the store.
    ///
    pub fn inter_isa_ui<F>(&mut self, isa_ui: F) -> Arc<RwLock<IsaUi>>
    where
        F: Fn(usize) -> Arc<RwLock<IsaUi>>,
    {
        let _index = if let Some(_index) = self.isa_ui_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.isa_ui.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.isa_ui.write().unwrap().push(None);
            _index
        };

        let isa_ui = isa_ui(_index);

        let found = if let Some(isa_ui) = self.isa_ui.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *isa_ui.read().unwrap()
            } else {
                false
            }
        }) {
            isa_ui.clone()
        } else {
            None
        };

        if let Some(isa_ui) = found {
            log::debug!(target: "store", "found duplicate {isa_ui:?}.");
            self.isa_ui_free_list.lock().unwrap().push(_index);
            isa_ui.clone()
        } else {
            log::debug!(target: "store", "interring {isa_ui:?}.");
            self.isa_ui.write().unwrap()[_index] = Some(isa_ui.clone());
            isa_ui
        }
    }

    /// Exhume (get) [`IsaUi`] from the store.
    ///
    pub fn exhume_isa_ui(&self, id: &usize) -> Option<Arc<RwLock<IsaUi>>> {
        match self.isa_ui.read().unwrap().get(*id) {
            Some(isa_ui) => isa_ui.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`IsaUi`] from the store.
    ///
    pub fn exorcise_isa_ui(&mut self, id: &usize) -> Option<Arc<RwLock<IsaUi>>> {
        let result = self.isa_ui.write().unwrap()[*id].take();
        self.isa_ui_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IsaUi>`.
    ///
    pub fn iter_isa_ui(&self) -> impl Iterator<Item = Arc<RwLock<IsaUi>>> + '_ {
        let len = self.isa_ui.read().unwrap().len();
        (0..len)
            .filter(|i| self.isa_ui.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.isa_ui.read().unwrap()[i]
                    .as_ref()
                    .map(|isa_ui| isa_ui.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`State`] into the store.
    ///
    pub fn inter_state<F>(&mut self, state: F) -> Arc<RwLock<State>>
    where
        F: Fn(usize) -> Arc<RwLock<State>>,
    {
        let _index = if let Some(_index) = self.state_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.state.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.state.write().unwrap().push(None);
            _index
        };

        let state = state(_index);

        let found = if let Some(state) = self.state.read().unwrap().iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.read().unwrap() == *state.read().unwrap()
            } else {
                false
            }
        }) {
            state.clone()
        } else {
            None
        };

        if let Some(state) = found {
            log::debug!(target: "store", "found duplicate {state:?}.");
            self.state_free_list.lock().unwrap().push(_index);
            state.clone()
        } else {
            log::debug!(target: "store", "interring {state:?}.");
            self.state.write().unwrap()[_index] = Some(state.clone());
            state
        }
    }

    /// Exhume (get) [`State`] from the store.
    ///
    pub fn exhume_state(&self, id: &usize) -> Option<Arc<RwLock<State>>> {
        match self.state.read().unwrap().get(*id) {
            Some(state) => state.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`State`] from the store.
    ///
    pub fn exorcise_state(&mut self, id: &usize) -> Option<Arc<RwLock<State>>> {
        let result = self.state.write().unwrap()[*id].take();
        self.state_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, State>`.
    ///
    pub fn iter_state(&self) -> impl Iterator<Item = Arc<RwLock<State>>> + '_ {
        let len = self.state.read().unwrap().len();
        (0..len)
            .filter(|i| self.state.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.state.read().unwrap()[i]
                    .as_ref()
                    .map(|state| state.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`SubtypeAnchor`] into the store.
    ///
    pub fn inter_subtype_anchor<F>(&mut self, subtype_anchor: F) -> Arc<RwLock<SubtypeAnchor>>
    where
        F: Fn(usize) -> Arc<RwLock<SubtypeAnchor>>,
    {
        let _index = if let Some(_index) = self.subtype_anchor_free_list.lock().unwrap().pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.subtype_anchor.read().unwrap().len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.subtype_anchor.write().unwrap().push(None);
            _index
        };

        let subtype_anchor = subtype_anchor(_index);

        let found = if let Some(subtype_anchor) =
            self.subtype_anchor.read().unwrap().iter().find(|stored| {
                if let Some(stored) = stored {
                    *stored.read().unwrap() == *subtype_anchor.read().unwrap()
                } else {
                    false
                }
            }) {
            subtype_anchor.clone()
        } else {
            None
        };

        if let Some(subtype_anchor) = found {
            log::debug!(target: "store", "found duplicate {subtype_anchor:?}.");
            self.subtype_anchor_free_list.lock().unwrap().push(_index);
            subtype_anchor.clone()
        } else {
            log::debug!(target: "store", "interring {subtype_anchor:?}.");
            self.subtype_anchor.write().unwrap()[_index] = Some(subtype_anchor.clone());
            subtype_anchor
        }
    }

    /// Exhume (get) [`SubtypeAnchor`] from the store.
    ///
    pub fn exhume_subtype_anchor(&self, id: &usize) -> Option<Arc<RwLock<SubtypeAnchor>>> {
        match self.subtype_anchor.read().unwrap().get(*id) {
            Some(subtype_anchor) => subtype_anchor.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`SubtypeAnchor`] from the store.
    ///
    pub fn exorcise_subtype_anchor(&mut self, id: &usize) -> Option<Arc<RwLock<SubtypeAnchor>>> {
        let result = self.subtype_anchor.write().unwrap()[*id].take();
        self.subtype_anchor_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeAnchor>`.
    ///
    pub fn iter_subtype_anchor(&self) -> impl Iterator<Item = Arc<RwLock<SubtypeAnchor>>> + '_ {
        let len = self.subtype_anchor.read().unwrap().len();
        (0..len)
            .filter(|i| self.subtype_anchor.read().unwrap()[*i].is_some())
            .map(move |i| {
                self.subtype_anchor.read().unwrap()[i]
                    .as_ref()
                    .map(|subtype_anchor| subtype_anchor.clone())
                    .unwrap()
            })
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::associative_rwlock_vec-object-store-persistence"}}}
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
            for acknowledged_event in &*self.acknowledged_event.read().unwrap() {
                if let Some(acknowledged_event) = acknowledged_event {
                    let path = path.join(format!("{}.json", acknowledged_event.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &acknowledged_event)?;
                }
            }
        }

        // Persist Anchor.
        {
            let path = path.join("anchor");
            fs::create_dir_all(&path)?;
            for anchor in &*self.anchor.read().unwrap() {
                if let Some(anchor) = anchor {
                    let path = path.join(format!("{}.json", anchor.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &anchor)?;
                }
            }
        }

        // Persist Event.
        {
            let path = path.join("event");
            fs::create_dir_all(&path)?;
            for event in &*self.event.read().unwrap() {
                if let Some(event) = event {
                    let path = path.join(format!("{}.json", event.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &event)?;
                }
            }
        }

        // Persist IsaUI.
        {
            let path = path.join("isa_ui");
            fs::create_dir_all(&path)?;
            for isa_ui in &*self.isa_ui.read().unwrap() {
                if let Some(isa_ui) = isa_ui {
                    let path = path.join(format!("{}.json", isa_ui.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &isa_ui)?;
                }
            }
        }

        // Persist State.
        {
            let path = path.join("state");
            fs::create_dir_all(&path)?;
            for state in &*self.state.read().unwrap() {
                if let Some(state) = state {
                    let path = path.join(format!("{}.json", state.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &state)?;
                }
            }
        }

        // Persist Subtype Anchor.
        {
            let path = path.join("subtype_anchor");
            fs::create_dir_all(&path)?;
            for subtype_anchor in &*self.subtype_anchor.read().unwrap() {
                if let Some(subtype_anchor) = subtype_anchor {
                    let path = path.join(format!("{}.json", subtype_anchor.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &subtype_anchor)?;
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
        let path = path.join("associative.json");

        let mut store = Self::new();

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
                    Some(acknowledged_event.clone()),
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
                    .insert(anchor.read().unwrap().id, Some(anchor.clone()));
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
                    .insert(event.read().unwrap().id, Some(event.clone()));
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
                    .insert(isa_ui.read().unwrap().id, Some(isa_ui.clone()));
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
                    .insert(state.read().unwrap().id, Some(state.clone()));
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
                store.subtype_anchor.write().unwrap().insert(
                    subtype_anchor.read().unwrap().id,
                    Some(subtype_anchor.clone()),
                );
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

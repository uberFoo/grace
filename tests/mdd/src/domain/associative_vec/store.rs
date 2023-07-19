//! domain::associative_vec Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::associative_vec-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`AcknowledgedEvent`]
//! * [`Anchor`]
//! * [`Event`]
//! * [`IsaUi`]
//! * [`State`]
//! * [`SubtypeAnchor`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::associative_vec-object-store-definition"}}}
use std::cell::RefCell;
use std::rc::Rc;
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::associative_vec::types::{
    AcknowledgedEvent, Anchor, Event, IsaUi, State, SubtypeAnchor,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    acknowledged_event_free_list: Vec<usize>,
    acknowledged_event: Vec<Option<Rc<RefCell<AcknowledgedEvent>>>>,
    anchor_free_list: Vec<usize>,
    anchor: Vec<Option<Rc<RefCell<Anchor>>>>,
    event_free_list: Vec<usize>,
    event: Vec<Option<Rc<RefCell<Event>>>>,
    isa_ui_free_list: Vec<usize>,
    isa_ui: Vec<Option<Rc<RefCell<IsaUi>>>>,
    state_free_list: Vec<usize>,
    state: Vec<Option<Rc<RefCell<State>>>>,
    subtype_anchor_free_list: Vec<usize>,
    subtype_anchor: Vec<Option<Rc<RefCell<SubtypeAnchor>>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            acknowledged_event_free_list: Vec::new(),
            acknowledged_event: Vec::new(),
            anchor_free_list: Vec::new(),
            anchor: Vec::new(),
            event_free_list: Vec::new(),
            event: Vec::new(),
            isa_ui_free_list: Vec::new(),
            isa_ui: Vec::new(),
            state_free_list: Vec::new(),
            state: Vec::new(),
            subtype_anchor_free_list: Vec::new(),
            subtype_anchor: Vec::new(),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::associative_vec-object-store-methods"}}}
    /// Inter (insert) [`AcknowledgedEvent`] into the store.
    ///
    pub fn inter_acknowledged_event<F>(
        &mut self,
        acknowledged_event: F,
    ) -> Rc<RefCell<AcknowledgedEvent>>
    where
        F: Fn(usize) -> Rc<RefCell<AcknowledgedEvent>>,
    {
        let _index = if let Some(_index) = self.acknowledged_event_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.acknowledged_event.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.acknowledged_event.push(None);
            _index
        };

        let acknowledged_event = acknowledged_event(_index);

        if let Some(Some(acknowledged_event)) = self.acknowledged_event.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *acknowledged_event.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {acknowledged_event:?}.");
            self.acknowledged_event_free_list.push(_index);
            acknowledged_event.clone()
        } else {
            log::debug!(target: "store", "interring {acknowledged_event:?}.");
            self.acknowledged_event[_index] = Some(acknowledged_event.clone());
            acknowledged_event
        }
    }

    /// Exhume (get) [`AcknowledgedEvent`] from the store.
    ///
    pub fn exhume_acknowledged_event(&self, id: &usize) -> Option<Rc<RefCell<AcknowledgedEvent>>> {
        match self.acknowledged_event.get(*id) {
            Some(acknowledged_event) => acknowledged_event.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`AcknowledgedEvent`] from the store.
    ///
    pub fn exorcise_acknowledged_event(
        &mut self,
        id: &usize,
    ) -> Option<Rc<RefCell<AcknowledgedEvent>>> {
        let result = self.acknowledged_event[*id].take();
        self.acknowledged_event_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AcknowledgedEvent>`.
    ///
    pub fn iter_acknowledged_event(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<AcknowledgedEvent>>> + '_ {
        let len = self.acknowledged_event.len();
        (0..len)
            .filter(|i| self.acknowledged_event[*i].is_some())
            .map(move |i| {
                self.acknowledged_event[i]
                    .as_ref()
                    .map(|acknowledged_event| acknowledged_event.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Anchor`] into the store.
    ///
    pub fn inter_anchor<F>(&mut self, anchor: F) -> Rc<RefCell<Anchor>>
    where
        F: Fn(usize) -> Rc<RefCell<Anchor>>,
    {
        let _index = if let Some(_index) = self.anchor_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.anchor.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.anchor.push(None);
            _index
        };

        let anchor = anchor(_index);

        if let Some(Some(anchor)) = self.anchor.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *anchor.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {anchor:?}.");
            self.anchor_free_list.push(_index);
            anchor.clone()
        } else {
            log::debug!(target: "store", "interring {anchor:?}.");
            self.anchor[_index] = Some(anchor.clone());
            anchor
        }
    }

    /// Exhume (get) [`Anchor`] from the store.
    ///
    pub fn exhume_anchor(&self, id: &usize) -> Option<Rc<RefCell<Anchor>>> {
        match self.anchor.get(*id) {
            Some(anchor) => anchor.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Anchor`] from the store.
    ///
    pub fn exorcise_anchor(&mut self, id: &usize) -> Option<Rc<RefCell<Anchor>>> {
        let result = self.anchor[*id].take();
        self.anchor_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Anchor>`.
    ///
    pub fn iter_anchor(&self) -> impl Iterator<Item = Rc<RefCell<Anchor>>> + '_ {
        let len = self.anchor.len();
        (0..len)
            .filter(|i| self.anchor[*i].is_some())
            .map(move |i| {
                self.anchor[i]
                    .as_ref()
                    .map(|anchor| anchor.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Event`] into the store.
    ///
    pub fn inter_event<F>(&mut self, event: F) -> Rc<RefCell<Event>>
    where
        F: Fn(usize) -> Rc<RefCell<Event>>,
    {
        let _index = if let Some(_index) = self.event_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.event.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.event.push(None);
            _index
        };

        let event = event(_index);

        if let Some(Some(event)) = self.event.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *event.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {event:?}.");
            self.event_free_list.push(_index);
            event.clone()
        } else {
            log::debug!(target: "store", "interring {event:?}.");
            self.event[_index] = Some(event.clone());
            event
        }
    }

    /// Exhume (get) [`Event`] from the store.
    ///
    pub fn exhume_event(&self, id: &usize) -> Option<Rc<RefCell<Event>>> {
        match self.event.get(*id) {
            Some(event) => event.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Event`] from the store.
    ///
    pub fn exorcise_event(&mut self, id: &usize) -> Option<Rc<RefCell<Event>>> {
        let result = self.event[*id].take();
        self.event_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Event>`.
    ///
    pub fn iter_event(&self) -> impl Iterator<Item = Rc<RefCell<Event>>> + '_ {
        let len = self.event.len();
        (0..len)
            .filter(|i| self.event[*i].is_some())
            .map(move |i| self.event[i].as_ref().map(|event| event.clone()).unwrap())
    }

    /// Inter (insert) [`IsaUi`] into the store.
    ///
    pub fn inter_isa_ui<F>(&mut self, isa_ui: F) -> Rc<RefCell<IsaUi>>
    where
        F: Fn(usize) -> Rc<RefCell<IsaUi>>,
    {
        let _index = if let Some(_index) = self.isa_ui_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.isa_ui.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.isa_ui.push(None);
            _index
        };

        let isa_ui = isa_ui(_index);

        if let Some(Some(isa_ui)) = self.isa_ui.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *isa_ui.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {isa_ui:?}.");
            self.isa_ui_free_list.push(_index);
            isa_ui.clone()
        } else {
            log::debug!(target: "store", "interring {isa_ui:?}.");
            self.isa_ui[_index] = Some(isa_ui.clone());
            isa_ui
        }
    }

    /// Exhume (get) [`IsaUi`] from the store.
    ///
    pub fn exhume_isa_ui(&self, id: &usize) -> Option<Rc<RefCell<IsaUi>>> {
        match self.isa_ui.get(*id) {
            Some(isa_ui) => isa_ui.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`IsaUi`] from the store.
    ///
    pub fn exorcise_isa_ui(&mut self, id: &usize) -> Option<Rc<RefCell<IsaUi>>> {
        let result = self.isa_ui[*id].take();
        self.isa_ui_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IsaUi>`.
    ///
    pub fn iter_isa_ui(&self) -> impl Iterator<Item = Rc<RefCell<IsaUi>>> + '_ {
        let len = self.isa_ui.len();
        (0..len)
            .filter(|i| self.isa_ui[*i].is_some())
            .map(move |i| {
                self.isa_ui[i]
                    .as_ref()
                    .map(|isa_ui| isa_ui.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`State`] into the store.
    ///
    pub fn inter_state<F>(&mut self, state: F) -> Rc<RefCell<State>>
    where
        F: Fn(usize) -> Rc<RefCell<State>>,
    {
        let _index = if let Some(_index) = self.state_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.state.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.state.push(None);
            _index
        };

        let state = state(_index);

        if let Some(Some(state)) = self.state.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *state.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {state:?}.");
            self.state_free_list.push(_index);
            state.clone()
        } else {
            log::debug!(target: "store", "interring {state:?}.");
            self.state[_index] = Some(state.clone());
            state
        }
    }

    /// Exhume (get) [`State`] from the store.
    ///
    pub fn exhume_state(&self, id: &usize) -> Option<Rc<RefCell<State>>> {
        match self.state.get(*id) {
            Some(state) => state.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`State`] from the store.
    ///
    pub fn exorcise_state(&mut self, id: &usize) -> Option<Rc<RefCell<State>>> {
        let result = self.state[*id].take();
        self.state_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, State>`.
    ///
    pub fn iter_state(&self) -> impl Iterator<Item = Rc<RefCell<State>>> + '_ {
        let len = self.state.len();
        (0..len)
            .filter(|i| self.state[*i].is_some())
            .map(move |i| self.state[i].as_ref().map(|state| state.clone()).unwrap())
    }

    /// Inter (insert) [`SubtypeAnchor`] into the store.
    ///
    pub fn inter_subtype_anchor<F>(&mut self, subtype_anchor: F) -> Rc<RefCell<SubtypeAnchor>>
    where
        F: Fn(usize) -> Rc<RefCell<SubtypeAnchor>>,
    {
        let _index = if let Some(_index) = self.subtype_anchor_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.subtype_anchor.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.subtype_anchor.push(None);
            _index
        };

        let subtype_anchor = subtype_anchor(_index);

        if let Some(Some(subtype_anchor)) = self.subtype_anchor.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *subtype_anchor.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {subtype_anchor:?}.");
            self.subtype_anchor_free_list.push(_index);
            subtype_anchor.clone()
        } else {
            log::debug!(target: "store", "interring {subtype_anchor:?}.");
            self.subtype_anchor[_index] = Some(subtype_anchor.clone());
            subtype_anchor
        }
    }

    /// Exhume (get) [`SubtypeAnchor`] from the store.
    ///
    pub fn exhume_subtype_anchor(&self, id: &usize) -> Option<Rc<RefCell<SubtypeAnchor>>> {
        match self.subtype_anchor.get(*id) {
            Some(subtype_anchor) => subtype_anchor.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`SubtypeAnchor`] from the store.
    ///
    pub fn exorcise_subtype_anchor(&mut self, id: &usize) -> Option<Rc<RefCell<SubtypeAnchor>>> {
        let result = self.subtype_anchor[*id].take();
        self.subtype_anchor_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeAnchor>`.
    ///
    pub fn iter_subtype_anchor(&self) -> impl Iterator<Item = Rc<RefCell<SubtypeAnchor>>> + '_ {
        let len = self.subtype_anchor.len();
        (0..len)
            .filter(|i| self.subtype_anchor[*i].is_some())
            .map(move |i| {
                self.subtype_anchor[i]
                    .as_ref()
                    .map(|subtype_anchor| subtype_anchor.clone())
                    .unwrap()
            })
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::associative_vec-object-store-persistence"}}}
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
            for acknowledged_event in &self.acknowledged_event {
                if let Some(acknowledged_event) = acknowledged_event {
                    let path = path.join(format!("{}.json", acknowledged_event.borrow().id));
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
            for anchor in &self.anchor {
                if let Some(anchor) = anchor {
                    let path = path.join(format!("{}.json", anchor.borrow().id));
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
            for event in &self.event {
                if let Some(event) = event {
                    let path = path.join(format!("{}.json", event.borrow().id));
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
            for isa_ui in &self.isa_ui {
                if let Some(isa_ui) = isa_ui {
                    let path = path.join(format!("{}.json", isa_ui.borrow().id));
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
            for state in &self.state {
                if let Some(state) = state {
                    let path = path.join(format!("{}.json", state.borrow().id));
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
            for subtype_anchor in &self.subtype_anchor {
                if let Some(subtype_anchor) = subtype_anchor {
                    let path = path.join(format!("{}.json", subtype_anchor.borrow().id));
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
                let acknowledged_event: Rc<RefCell<AcknowledgedEvent>> =
                    serde_json::from_reader(reader)?;
                store.acknowledged_event.insert(
                    acknowledged_event.borrow().id,
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
                let anchor: Rc<RefCell<Anchor>> = serde_json::from_reader(reader)?;
                store
                    .anchor
                    .insert(anchor.borrow().id, Some(anchor.clone()));
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
                let event: Rc<RefCell<Event>> = serde_json::from_reader(reader)?;
                store.event.insert(event.borrow().id, Some(event.clone()));
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
                let isa_ui: Rc<RefCell<IsaUi>> = serde_json::from_reader(reader)?;
                store
                    .isa_ui
                    .insert(isa_ui.borrow().id, Some(isa_ui.clone()));
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
                let state: Rc<RefCell<State>> = serde_json::from_reader(reader)?;
                store.state.insert(state.borrow().id, Some(state.clone()));
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
                let subtype_anchor: Rc<RefCell<SubtypeAnchor>> = serde_json::from_reader(reader)?;
                store
                    .subtype_anchor
                    .insert(subtype_anchor.borrow().id, Some(subtype_anchor.clone()));
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

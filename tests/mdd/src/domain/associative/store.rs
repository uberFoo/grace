//! domain::associative Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::associative-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`AcknowledgedEvent`]
//! * [`Anchor`]
//! * [`Event`]
//! * [`IsaUi`]
//! * [`State`]
//! * [`SubtypeAnchor`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::associative-object-store-definition"}}}
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use fnv::FnvHashMap as HashMap;
use heck::ToUpperCamelCase;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::associative::types::{
    AcknowledgedEvent, Anchor, Event, IsaUi, State, SubtypeAnchor,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    acknowledged_event: HashMap<Uuid, AcknowledgedEvent>,
    anchor: HashMap<Uuid, Anchor>,
    event: HashMap<Uuid, Event>,
    event_by_name: HashMap<String, Event>,
    isa_ui: HashMap<Uuid, IsaUi>,
    state: HashMap<Uuid, State>,
    state_by_name: HashMap<String, State>,
    subtype_anchor: HashMap<Uuid, SubtypeAnchor>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            acknowledged_event: HashMap::default(),
            anchor: HashMap::default(),
            event: HashMap::default(),
            event_by_name: HashMap::default(),
            isa_ui: HashMap::default(),
            state: HashMap::default(),
            state_by_name: HashMap::default(),
            subtype_anchor: HashMap::default(),
        };

        // Initialize Singleton Subtypes

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::associative-object-store-methods"}}}
    /// Inter [`AcknowledgedEvent`] into the store.
    ///
    pub fn inter_acknowledged_event(&mut self, acknowledged_event: AcknowledgedEvent) {
        self.acknowledged_event
            .insert(acknowledged_event.id, acknowledged_event);
    }

    /// Exhume [`AcknowledgedEvent`] from the store.
    ///
    pub fn exhume_acknowledged_event(&self, id: &Uuid) -> Option<&AcknowledgedEvent> {
        self.acknowledged_event.get(id)
    }

    /// Exhume [`AcknowledgedEvent`] from the store — mutably.
    ///
    pub fn exhume_acknowledged_event_mut(&mut self, id: &Uuid) -> Option<&mut AcknowledgedEvent> {
        self.acknowledged_event.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AcknowledgedEvent>`.
    ///
    pub fn iter_acknowledged_event(&self) -> impl Iterator<Item = &AcknowledgedEvent> {
        self.acknowledged_event.values()
    }

    /// Inter [`Anchor`] into the store.
    ///
    pub fn inter_anchor(&mut self, anchor: Anchor) {
        self.anchor.insert(anchor.id, anchor);
    }

    /// Exhume [`Anchor`] from the store.
    ///
    pub fn exhume_anchor(&self, id: &Uuid) -> Option<&Anchor> {
        self.anchor.get(id)
    }

    /// Exhume [`Anchor`] from the store — mutably.
    ///
    pub fn exhume_anchor_mut(&mut self, id: &Uuid) -> Option<&mut Anchor> {
        self.anchor.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Anchor>`.
    ///
    pub fn iter_anchor(&self) -> impl Iterator<Item = &Anchor> {
        self.anchor.values()
    }

    /// Inter [`Event`] into the store.
    ///
    pub fn inter_event(&mut self, event: Event) {
        self.event.insert(event.id, event.clone());
        self.event_by_name
            .insert(event.name.to_upper_camel_case(), event);
    }

    /// Exhume [`Event`] from the store.
    ///
    pub fn exhume_event(&self, id: &Uuid) -> Option<&Event> {
        self.event.get(id)
    }

    /// Exhume [`Event`] from the store — mutably.
    ///
    pub fn exhume_event_mut(&mut self, id: &Uuid) -> Option<&mut Event> {
        self.event.get_mut(id)
    }

    /// Exhume [`Event`] from the store by name.
    ///
    pub fn exhume_event_by_name(&self, name: &str) -> Option<&Event> {
        self.event_by_name.get(name)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Event>`.
    ///
    pub fn iter_event(&self) -> impl Iterator<Item = &Event> {
        self.event.values()
    }

    /// Inter [`IsaUi`] into the store.
    ///
    pub fn inter_isa_ui(&mut self, isa_ui: IsaUi) {
        self.isa_ui.insert(isa_ui.id, isa_ui);
    }

    /// Exhume [`IsaUi`] from the store.
    ///
    pub fn exhume_isa_ui(&self, id: &Uuid) -> Option<&IsaUi> {
        self.isa_ui.get(id)
    }

    /// Exhume [`IsaUi`] from the store — mutably.
    ///
    pub fn exhume_isa_ui_mut(&mut self, id: &Uuid) -> Option<&mut IsaUi> {
        self.isa_ui.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IsaUi>`.
    ///
    pub fn iter_isa_ui(&self) -> impl Iterator<Item = &IsaUi> {
        self.isa_ui.values()
    }

    /// Inter [`State`] into the store.
    ///
    pub fn inter_state(&mut self, state: State) {
        self.state.insert(state.id, state.clone());
        self.state_by_name
            .insert(state.name.to_upper_camel_case(), state);
    }

    /// Exhume [`State`] from the store.
    ///
    pub fn exhume_state(&self, id: &Uuid) -> Option<&State> {
        self.state.get(id)
    }

    /// Exhume [`State`] from the store — mutably.
    ///
    pub fn exhume_state_mut(&mut self, id: &Uuid) -> Option<&mut State> {
        self.state.get_mut(id)
    }

    /// Exhume [`State`] from the store by name.
    ///
    pub fn exhume_state_by_name(&self, name: &str) -> Option<&State> {
        self.state_by_name.get(name)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, State>`.
    ///
    pub fn iter_state(&self) -> impl Iterator<Item = &State> {
        self.state.values()
    }

    /// Inter [`SubtypeAnchor`] into the store.
    ///
    pub fn inter_subtype_anchor(&mut self, subtype_anchor: SubtypeAnchor) {
        self.subtype_anchor
            .insert(subtype_anchor.id, subtype_anchor);
    }

    /// Exhume [`SubtypeAnchor`] from the store.
    ///
    pub fn exhume_subtype_anchor(&self, id: &Uuid) -> Option<&SubtypeAnchor> {
        self.subtype_anchor.get(id)
    }

    /// Exhume [`SubtypeAnchor`] from the store — mutably.
    ///
    pub fn exhume_subtype_anchor_mut(&mut self, id: &Uuid) -> Option<&mut SubtypeAnchor> {
        self.subtype_anchor.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeAnchor>`.
    ///
    pub fn iter_subtype_anchor(&self) -> impl Iterator<Item = &SubtypeAnchor> {
        self.subtype_anchor.values()
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::associative-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        fs::create_dir_all(&path)?;

        let bin_path = path.clone().join("associative.bin");
        let mut bin_file = fs::File::create(bin_path)?;
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        bin_file.write_all(&encoded)?;

        let path = path.join("associative.json");
        fs::create_dir_all(&path)?;

        // Persist Acknowledged Event.
        {
            let path = path.join("acknowledged_event");
            fs::create_dir_all(&path)?;
            for acknowledged_event in self.acknowledged_event.values() {
                let path = path.join(format!("{}.json", acknowledged_event.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &acknowledged_event)?;
            }
        }

        // Persist Anchor.
        {
            let path = path.join("anchor");
            fs::create_dir_all(&path)?;
            for anchor in self.anchor.values() {
                let path = path.join(format!("{}.json", anchor.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &anchor)?;
            }
        }

        // Persist Event.
        {
            let path = path.join("event");
            fs::create_dir_all(&path)?;
            for event in self.event.values() {
                let path = path.join(format!("{}.json", event.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &event)?;
            }
        }

        // Persist IsaUI.
        {
            let path = path.join("isa_ui");
            fs::create_dir_all(&path)?;
            for isa_ui in self.isa_ui.values() {
                let path = path.join(format!("{}.json", isa_ui.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &isa_ui)?;
            }
        }

        // Persist State.
        {
            let path = path.join("state");
            fs::create_dir_all(&path)?;
            for state in self.state.values() {
                let path = path.join(format!("{}.json", state.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &state)?;
            }
        }

        // Persist Subtype Anchor.
        {
            let path = path.join("subtype_anchor");
            fs::create_dir_all(&path)?;
            for subtype_anchor in self.subtype_anchor.values() {
                let path = path.join(format!("{}.json", subtype_anchor.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &subtype_anchor)?;
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
        let path = path.join("associative.json");

        let mut store = Self::new();

        // Load Acknowledged Event.
        {
            let path = path.join("acknowledged_event");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let acknowledged_event: AcknowledgedEvent = serde_json::from_reader(reader)?;
                store
                    .acknowledged_event
                    .insert(acknowledged_event.id, acknowledged_event);
            }
        }

        // Load Anchor.
        {
            let path = path.join("anchor");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let anchor: Anchor = serde_json::from_reader(reader)?;
                store.anchor.insert(anchor.id, anchor);
            }
        }

        // Load Event.
        {
            let path = path.join("event");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let event: Event = serde_json::from_reader(reader)?;
                store
                    .event_by_name
                    .insert(event.name.to_upper_camel_case(), event.clone());
                store.event.insert(event.id, event);
            }
        }

        // Load IsaUI.
        {
            let path = path.join("isa_ui");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let isa_ui: IsaUi = serde_json::from_reader(reader)?;
                store.isa_ui.insert(isa_ui.id, isa_ui);
            }
        }

        // Load State.
        {
            let path = path.join("state");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let state: State = serde_json::from_reader(reader)?;
                store
                    .state_by_name
                    .insert(state.name.to_upper_camel_case(), state.clone());
                store.state.insert(state.id, state);
            }
        }

        // Load Subtype Anchor.
        {
            let path = path.join("subtype_anchor");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let subtype_anchor: SubtypeAnchor = serde_json::from_reader(reader)?;
                store
                    .subtype_anchor
                    .insert(subtype_anchor.id, subtype_anchor);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

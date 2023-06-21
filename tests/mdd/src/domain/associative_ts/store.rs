//! domain::associative_ts Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::associative_ts-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`AcknowledgedEvent`]
//! * [`Anchor`]
//! * [`Event`]
//! * [`IsaUi`]
//! * [`State`]
//! * [`SubtypeAnchor`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::associative_ts-object-store-definition"}}}
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
    time::SystemTime,
};

use fnv::FnvHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::associative_ts::types::{
    AcknowledgedEvent, Anchor, Event, IsaUi, State, SubtypeAnchor,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    acknowledged_event: HashMap<Uuid, (AcknowledgedEvent, SystemTime)>,
    anchor: HashMap<Uuid, (Anchor, SystemTime)>,
    event: HashMap<Uuid, (Event, SystemTime)>,
    isa_ui: HashMap<Uuid, (IsaUi, SystemTime)>,
    state: HashMap<Uuid, (State, SystemTime)>,
    subtype_anchor: HashMap<Uuid, (SubtypeAnchor, SystemTime)>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            acknowledged_event: HashMap::default(),
            anchor: HashMap::default(),
            event: HashMap::default(),
            isa_ui: HashMap::default(),
            state: HashMap::default(),
            subtype_anchor: HashMap::default(),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::associative_ts-object-store-methods"}}}
    /// Inter (insert) [`AcknowledgedEvent`] into the store.
    ///
    pub fn inter_acknowledged_event(&mut self, acknowledged_event: AcknowledgedEvent) {
        self.acknowledged_event.insert(
            acknowledged_event.id,
            (acknowledged_event, SystemTime::now()),
        );
    }

    /// Exhume (get) [`AcknowledgedEvent`] from the store.
    ///
    pub fn exhume_acknowledged_event(&self, id: &Uuid) -> Option<&AcknowledgedEvent> {
        self.acknowledged_event
            .get(id)
            .map(|acknowledged_event| &acknowledged_event.0)
    }

    /// Exorcise (remove) [`AcknowledgedEvent`] from the store.
    ///
    pub fn exorcise_acknowledged_event(&mut self, id: &Uuid) -> Option<AcknowledgedEvent> {
        self.acknowledged_event
            .remove(id)
            .map(|acknowledged_event| acknowledged_event.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AcknowledgedEvent>`.
    ///
    pub fn iter_acknowledged_event(&self) -> impl Iterator<Item = &AcknowledgedEvent> {
        self.acknowledged_event
            .values()
            .map(|acknowledged_event| &acknowledged_event.0)
    }

    /// Get the timestamp for AcknowledgedEvent.
    ///
    pub fn acknowledged_event_timestamp(
        &self,
        acknowledged_event: &AcknowledgedEvent,
    ) -> SystemTime {
        self.acknowledged_event
            .get(&acknowledged_event.id)
            .map(|acknowledged_event| acknowledged_event.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Anchor`] into the store.
    ///
    pub fn inter_anchor(&mut self, anchor: Anchor) {
        self.anchor.insert(anchor.id, (anchor, SystemTime::now()));
    }

    /// Exhume (get) [`Anchor`] from the store.
    ///
    pub fn exhume_anchor(&self, id: &Uuid) -> Option<&Anchor> {
        self.anchor.get(id).map(|anchor| &anchor.0)
    }

    /// Exorcise (remove) [`Anchor`] from the store.
    ///
    pub fn exorcise_anchor(&mut self, id: &Uuid) -> Option<Anchor> {
        self.anchor.remove(id).map(|anchor| anchor.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Anchor>`.
    ///
    pub fn iter_anchor(&self) -> impl Iterator<Item = &Anchor> {
        self.anchor.values().map(|anchor| &anchor.0)
    }

    /// Get the timestamp for Anchor.
    ///
    pub fn anchor_timestamp(&self, anchor: &Anchor) -> SystemTime {
        self.anchor
            .get(&anchor.id)
            .map(|anchor| anchor.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`Event`] into the store.
    ///
    pub fn inter_event(&mut self, event: Event) {
        self.event.insert(event.id, (event, SystemTime::now()));
    }

    /// Exhume (get) [`Event`] from the store.
    ///
    pub fn exhume_event(&self, id: &Uuid) -> Option<&Event> {
        self.event.get(id).map(|event| &event.0)
    }

    /// Exorcise (remove) [`Event`] from the store.
    ///
    pub fn exorcise_event(&mut self, id: &Uuid) -> Option<Event> {
        self.event.remove(id).map(|event| event.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Event>`.
    ///
    pub fn iter_event(&self) -> impl Iterator<Item = &Event> {
        self.event.values().map(|event| &event.0)
    }

    /// Get the timestamp for Event.
    ///
    pub fn event_timestamp(&self, event: &Event) -> SystemTime {
        self.event
            .get(&event.id)
            .map(|event| event.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`IsaUi`] into the store.
    ///
    pub fn inter_isa_ui(&mut self, isa_ui: IsaUi) {
        self.isa_ui.insert(isa_ui.id, (isa_ui, SystemTime::now()));
    }

    /// Exhume (get) [`IsaUi`] from the store.
    ///
    pub fn exhume_isa_ui(&self, id: &Uuid) -> Option<&IsaUi> {
        self.isa_ui.get(id).map(|isa_ui| &isa_ui.0)
    }

    /// Exorcise (remove) [`IsaUi`] from the store.
    ///
    pub fn exorcise_isa_ui(&mut self, id: &Uuid) -> Option<IsaUi> {
        self.isa_ui.remove(id).map(|isa_ui| isa_ui.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IsaUi>`.
    ///
    pub fn iter_isa_ui(&self) -> impl Iterator<Item = &IsaUi> {
        self.isa_ui.values().map(|isa_ui| &isa_ui.0)
    }

    /// Get the timestamp for IsaUi.
    ///
    pub fn isa_ui_timestamp(&self, isa_ui: &IsaUi) -> SystemTime {
        self.isa_ui
            .get(&isa_ui.id)
            .map(|isa_ui| isa_ui.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`State`] into the store.
    ///
    pub fn inter_state(&mut self, state: State) {
        self.state.insert(state.id, (state, SystemTime::now()));
    }

    /// Exhume (get) [`State`] from the store.
    ///
    pub fn exhume_state(&self, id: &Uuid) -> Option<&State> {
        self.state.get(id).map(|state| &state.0)
    }

    /// Exorcise (remove) [`State`] from the store.
    ///
    pub fn exorcise_state(&mut self, id: &Uuid) -> Option<State> {
        self.state.remove(id).map(|state| state.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, State>`.
    ///
    pub fn iter_state(&self) -> impl Iterator<Item = &State> {
        self.state.values().map(|state| &state.0)
    }

    /// Get the timestamp for State.
    ///
    pub fn state_timestamp(&self, state: &State) -> SystemTime {
        self.state
            .get(&state.id)
            .map(|state| state.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter (insert) [`SubtypeAnchor`] into the store.
    ///
    pub fn inter_subtype_anchor(&mut self, subtype_anchor: SubtypeAnchor) {
        self.subtype_anchor
            .insert(subtype_anchor.id, (subtype_anchor, SystemTime::now()));
    }

    /// Exhume (get) [`SubtypeAnchor`] from the store.
    ///
    pub fn exhume_subtype_anchor(&self, id: &Uuid) -> Option<&SubtypeAnchor> {
        self.subtype_anchor
            .get(id)
            .map(|subtype_anchor| &subtype_anchor.0)
    }

    /// Exorcise (remove) [`SubtypeAnchor`] from the store.
    ///
    pub fn exorcise_subtype_anchor(&mut self, id: &Uuid) -> Option<SubtypeAnchor> {
        self.subtype_anchor
            .remove(id)
            .map(|subtype_anchor| subtype_anchor.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeAnchor>`.
    ///
    pub fn iter_subtype_anchor(&self) -> impl Iterator<Item = &SubtypeAnchor> {
        self.subtype_anchor
            .values()
            .map(|subtype_anchor| &subtype_anchor.0)
    }

    /// Get the timestamp for SubtypeAnchor.
    ///
    pub fn subtype_anchor_timestamp(&self, subtype_anchor: &SubtypeAnchor) -> SystemTime {
        self.subtype_anchor
            .get(&subtype_anchor.id)
            .map(|subtype_anchor| subtype_anchor.1)
            .unwrap_or(SystemTime::now())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::associative_ts-object-store-persistence"}}}
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
            for acknowledged_event_tuple in self.acknowledged_event.values() {
                let path = path.join(format!("{}.json", acknowledged_event_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (AcknowledgedEvent, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != acknowledged_event_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &acknowledged_event_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &acknowledged_event_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.acknowledged_event.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Anchor.
        {
            let path = path.join("anchor");
            fs::create_dir_all(&path)?;
            for anchor_tuple in self.anchor.values() {
                let path = path.join(format!("{}.json", anchor_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Anchor, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != anchor_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &anchor_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &anchor_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.anchor.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Event.
        {
            let path = path.join("event");
            fs::create_dir_all(&path)?;
            for event_tuple in self.event.values() {
                let path = path.join(format!("{}.json", event_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Event, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != event_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &event_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &event_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.event.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist IsaUI.
        {
            let path = path.join("isa_ui");
            fs::create_dir_all(&path)?;
            for isa_ui_tuple in self.isa_ui.values() {
                let path = path.join(format!("{}.json", isa_ui_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (IsaUi, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != isa_ui_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &isa_ui_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &isa_ui_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.isa_ui.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist State.
        {
            let path = path.join("state");
            fs::create_dir_all(&path)?;
            for state_tuple in self.state.values() {
                let path = path.join(format!("{}.json", state_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (State, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != state_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &state_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &state_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.state.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Subtype Anchor.
        {
            let path = path.join("subtype_anchor");
            fs::create_dir_all(&path)?;
            for subtype_anchor_tuple in self.subtype_anchor.values() {
                let path = path.join(format!("{}.json", subtype_anchor_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (SubtypeAnchor, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != subtype_anchor_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &subtype_anchor_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &subtype_anchor_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.subtype_anchor.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
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
                let acknowledged_event: (AcknowledgedEvent, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .acknowledged_event
                    .insert(acknowledged_event.0.id, acknowledged_event);
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
                let anchor: (Anchor, SystemTime) = serde_json::from_reader(reader)?;
                store.anchor.insert(anchor.0.id, anchor);
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
                let event: (Event, SystemTime) = serde_json::from_reader(reader)?;
                store.event.insert(event.0.id, event);
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
                let isa_ui: (IsaUi, SystemTime) = serde_json::from_reader(reader)?;
                store.isa_ui.insert(isa_ui.0.id, isa_ui);
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
                let state: (State, SystemTime) = serde_json::from_reader(reader)?;
                store.state.insert(state.0.id, state);
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
                let subtype_anchor: (SubtypeAnchor, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .subtype_anchor
                    .insert(subtype_anchor.0.id, subtype_anchor);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

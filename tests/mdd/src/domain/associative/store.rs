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
use std::collections::HashMap;
use std::{fs, io, path::Path};

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
    isa_ui: HashMap<Uuid, IsaUi>,
    state: HashMap<Uuid, State>,
    subtype_anchor: HashMap<Uuid, SubtypeAnchor>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            acknowledged_event: HashMap::new(),
            anchor: HashMap::new(),
            event: HashMap::new(),
            isa_ui: HashMap::new(),
            state: HashMap::new(),
            subtype_anchor: HashMap::new(),
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
        self.event.insert(event.id, event);
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
        self.state.insert(state.id, state);
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
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let path = path.join("associative.json");
        fs::create_dir_all(&path)?;

        // Persist Acknowledged Event.
        {
            let path = path.join("acknowledged_event.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self
                    .acknowledged_event
                    .values()
                    .map(|x| x)
                    .collect::<Vec<_>>(),
            )?;
        }
        // Persist Anchor.
        {
            let path = path.join("anchor.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.anchor.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Event.
        {
            let path = path.join("event.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.event.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist IsaUI.
        {
            let path = path.join("isa_ui.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.isa_ui.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist State.
        {
            let path = path.join("state.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.state.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Subtype Anchor.
        {
            let path = path.join("subtype_anchor.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.subtype_anchor.values().map(|x| x).collect::<Vec<_>>(),
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
        let path = path.join("associative.json");

        let mut store = Self::new();

        // Load Acknowledged Event.
        {
            let path = path.join("acknowledged_event.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let acknowledged_event: Vec<AcknowledgedEvent> = serde_json::from_reader(reader)?;
            store.acknowledged_event = acknowledged_event
                .into_iter()
                .map(|道| (道.id, 道))
                .collect();
        }
        // Load Anchor.
        {
            let path = path.join("anchor.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let anchor: Vec<Anchor> = serde_json::from_reader(reader)?;
            store.anchor = anchor.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Event.
        {
            let path = path.join("event.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let event: Vec<Event> = serde_json::from_reader(reader)?;
            store.event = event.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load IsaUI.
        {
            let path = path.join("isa_ui.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let isa_ui: Vec<IsaUi> = serde_json::from_reader(reader)?;
            store.isa_ui = isa_ui.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load State.
        {
            let path = path.join("state.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let state: Vec<State> = serde_json::from_reader(reader)?;
            store.state = state.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Subtype Anchor.
        {
            let path = path.join("subtype_anchor.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let subtype_anchor: Vec<SubtypeAnchor> = serde_json::from_reader(reader)?;
            store.subtype_anchor = subtype_anchor.into_iter().map(|道| (道.id, 道)).collect();
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

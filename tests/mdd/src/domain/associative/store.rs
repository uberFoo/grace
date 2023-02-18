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
        Self {
            acknowledged_event: HashMap::new(),
            anchor: HashMap::new(),
            event: HashMap::new(),
            isa_ui: HashMap::new(),
            state: HashMap::new(),
            subtype_anchor: HashMap::new(),
        }
    }

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
    /// Get an iterator over the internal `HashMap<&Uuid, AcknowledgedEvent>`.
    //
    pub fn iter_acknowledged_event(&self) -> impl Iterator<Item = (&Uuid, &AcknowledgedEvent)> {
        self.acknowledged_event.iter()
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
    /// Get an iterator over the internal `HashMap<&Uuid, Anchor>`.
    //
    pub fn iter_anchor(&self) -> impl Iterator<Item = (&Uuid, &Anchor)> {
        self.anchor.iter()
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
    /// Get an iterator over the internal `HashMap<&Uuid, Event>`.
    //
    pub fn iter_event(&self) -> impl Iterator<Item = (&Uuid, &Event)> {
        self.event.iter()
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
    /// Get an iterator over the internal `HashMap<&Uuid, IsaUi>`.
    //
    pub fn iter_isa_ui(&self) -> impl Iterator<Item = (&Uuid, &IsaUi)> {
        self.isa_ui.iter()
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
    /// Get an iterator over the internal `HashMap<&Uuid, State>`.
    //
    pub fn iter_state(&self) -> impl Iterator<Item = (&Uuid, &State)> {
        self.state.iter()
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
    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeAnchor>`.
    //
    pub fn iter_subtype_anchor(&self) -> impl Iterator<Item = (&Uuid, &SubtypeAnchor)> {
        self.subtype_anchor.iter()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

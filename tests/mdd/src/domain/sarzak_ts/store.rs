//! domain::sarzak_ts Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::sarzak_ts-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`AcknowledgedEvent`]
//! * [`AnAssociativeReferent`]
//! * [`Associative`]
//! * [`AssociativeReferent`]
//! * [`AssociativeReferrer`]
//! * [`Attribute`]
//! * [`Binary`]
//! * [`Cardinality`]
//! * [`Conditionality`]
//! * [`Event`]
//! * [`External`]
//! * [`Isa`]
//! * [`Object`]
//! * [`Referent`]
//! * [`Referrer`]
//! * [`Relationship`]
//! * [`State`]
//! * [`Subtype`]
//! * [`Supertype`]
//! * [`Ty`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::sarzak_ts-object-store-definition"}}}
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
    time::SystemTime,
};

use fnv::FnvHashMap as HashMap;
use heck::ToUpperCamelCase;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::sarzak_ts::types::{
    AcknowledgedEvent, AnAssociativeReferent, Associative, AssociativeReferent,
    AssociativeReferrer, Attribute, Binary, Cardinality, Conditionality, Event, External, Isa,
    Object, Referent, Referrer, Relationship, State, Subtype, Supertype, Ty, BOOLEAN, CONDITIONAL,
    FLOAT, INTEGER, MANY, ONE, STRING, UNCONDITIONAL, UUID,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    acknowledged_event: HashMap<Uuid, (AcknowledgedEvent, SystemTime)>,
    an_associative_referent: HashMap<Uuid, (AnAssociativeReferent, SystemTime)>,
    associative: HashMap<Uuid, (Associative, SystemTime)>,
    associative_referent: HashMap<Uuid, (AssociativeReferent, SystemTime)>,
    associative_referrer: HashMap<Uuid, (AssociativeReferrer, SystemTime)>,
    attribute: HashMap<Uuid, (Attribute, SystemTime)>,
    attribute_by_name: HashMap<String, (Attribute, SystemTime)>,
    binary: HashMap<Uuid, (Binary, SystemTime)>,
    cardinality: HashMap<Uuid, (Cardinality, SystemTime)>,
    conditionality: HashMap<Uuid, (Conditionality, SystemTime)>,
    event: HashMap<Uuid, (Event, SystemTime)>,
    event_by_name: HashMap<String, (Event, SystemTime)>,
    external: HashMap<Uuid, (External, SystemTime)>,
    external_by_name: HashMap<String, (External, SystemTime)>,
    isa: HashMap<Uuid, (Isa, SystemTime)>,
    object: HashMap<Uuid, (Object, SystemTime)>,
    object_by_name: HashMap<String, (Object, SystemTime)>,
    referent: HashMap<Uuid, (Referent, SystemTime)>,
    referrer: HashMap<Uuid, (Referrer, SystemTime)>,
    relationship: HashMap<Uuid, (Relationship, SystemTime)>,
    state: HashMap<Uuid, (State, SystemTime)>,
    state_by_name: HashMap<String, (State, SystemTime)>,
    subtype: HashMap<Uuid, (Subtype, SystemTime)>,
    supertype: HashMap<Uuid, (Supertype, SystemTime)>,
    ty: HashMap<Uuid, (Ty, SystemTime)>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            acknowledged_event: HashMap::default(),
            an_associative_referent: HashMap::default(),
            associative: HashMap::default(),
            associative_referent: HashMap::default(),
            associative_referrer: HashMap::default(),
            attribute: HashMap::default(),
            attribute_by_name: HashMap::default(),
            binary: HashMap::default(),
            cardinality: HashMap::default(),
            conditionality: HashMap::default(),
            event: HashMap::default(),
            event_by_name: HashMap::default(),
            external: HashMap::default(),
            external_by_name: HashMap::default(),
            isa: HashMap::default(),
            object: HashMap::default(),
            object_by_name: HashMap::default(),
            referent: HashMap::default(),
            referrer: HashMap::default(),
            relationship: HashMap::default(),
            state: HashMap::default(),
            state_by_name: HashMap::default(),
            subtype: HashMap::default(),
            supertype: HashMap::default(),
            ty: HashMap::default(),
        };

        // Initialize Singleton Subtypes
        store.inter_cardinality(Cardinality::Many(MANY));
        store.inter_cardinality(Cardinality::One(ONE));
        store.inter_conditionality(Conditionality::Conditional(CONDITIONAL));
        store.inter_conditionality(Conditionality::Unconditional(UNCONDITIONAL));
        store.inter_ty(Ty::Boolean(BOOLEAN));
        store.inter_ty(Ty::Float(FLOAT));
        store.inter_ty(Ty::Integer(INTEGER));
        store.inter_ty(Ty::String(STRING));
        store.inter_ty(Ty::Uuid(UUID));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::sarzak_ts-object-store-methods"}}}
    /// Inter [`AcknowledgedEvent`] into the store.
    ///
    pub fn inter_acknowledged_event(&mut self, acknowledged_event: AcknowledgedEvent) {
        self.acknowledged_event.insert(
            acknowledged_event.id,
            (acknowledged_event, SystemTime::now()),
        );
    }

    /// Exhume [`AcknowledgedEvent`] from the store.
    ///
    pub fn exhume_acknowledged_event(&self, id: &Uuid) -> Option<&AcknowledgedEvent> {
        self.acknowledged_event
            .get(id)
            .map(|acknowledged_event| &acknowledged_event.0)
    }

    /// Exhume [`AcknowledgedEvent`] from the store — mutably.
    ///
    pub fn exhume_acknowledged_event_mut(&mut self, id: &Uuid) -> Option<&mut AcknowledgedEvent> {
        self.acknowledged_event
            .get_mut(id)
            .map(|acknowledged_event| &mut acknowledged_event.0)
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

    /// Inter [`AnAssociativeReferent`] into the store.
    ///
    pub fn inter_an_associative_referent(
        &mut self,
        an_associative_referent: AnAssociativeReferent,
    ) {
        self.an_associative_referent.insert(
            an_associative_referent.id,
            (an_associative_referent, SystemTime::now()),
        );
    }

    /// Exhume [`AnAssociativeReferent`] from the store.
    ///
    pub fn exhume_an_associative_referent(&self, id: &Uuid) -> Option<&AnAssociativeReferent> {
        self.an_associative_referent
            .get(id)
            .map(|an_associative_referent| &an_associative_referent.0)
    }

    /// Exhume [`AnAssociativeReferent`] from the store — mutably.
    ///
    pub fn exhume_an_associative_referent_mut(
        &mut self,
        id: &Uuid,
    ) -> Option<&mut AnAssociativeReferent> {
        self.an_associative_referent
            .get_mut(id)
            .map(|an_associative_referent| &mut an_associative_referent.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AnAssociativeReferent>`.
    ///
    pub fn iter_an_associative_referent(&self) -> impl Iterator<Item = &AnAssociativeReferent> {
        self.an_associative_referent
            .values()
            .map(|an_associative_referent| &an_associative_referent.0)
    }

    /// Get the timestamp for AnAssociativeReferent.
    ///
    pub fn an_associative_referent_timestamp(
        &self,
        an_associative_referent: &AnAssociativeReferent,
    ) -> SystemTime {
        self.an_associative_referent
            .get(&an_associative_referent.id)
            .map(|an_associative_referent| an_associative_referent.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Associative`] into the store.
    ///
    pub fn inter_associative(&mut self, associative: Associative) {
        self.associative
            .insert(associative.id, (associative, SystemTime::now()));
    }

    /// Exhume [`Associative`] from the store.
    ///
    pub fn exhume_associative(&self, id: &Uuid) -> Option<&Associative> {
        self.associative.get(id).map(|associative| &associative.0)
    }

    /// Exhume [`Associative`] from the store — mutably.
    ///
    pub fn exhume_associative_mut(&mut self, id: &Uuid) -> Option<&mut Associative> {
        self.associative
            .get_mut(id)
            .map(|associative| &mut associative.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Associative>`.
    ///
    pub fn iter_associative(&self) -> impl Iterator<Item = &Associative> {
        self.associative.values().map(|associative| &associative.0)
    }

    /// Get the timestamp for Associative.
    ///
    pub fn associative_timestamp(&self, associative: &Associative) -> SystemTime {
        self.associative
            .get(&associative.id)
            .map(|associative| associative.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`AssociativeReferent`] into the store.
    ///
    pub fn inter_associative_referent(&mut self, associative_referent: AssociativeReferent) {
        self.associative_referent.insert(
            associative_referent.id,
            (associative_referent, SystemTime::now()),
        );
    }

    /// Exhume [`AssociativeReferent`] from the store.
    ///
    pub fn exhume_associative_referent(&self, id: &Uuid) -> Option<&AssociativeReferent> {
        self.associative_referent
            .get(id)
            .map(|associative_referent| &associative_referent.0)
    }

    /// Exhume [`AssociativeReferent`] from the store — mutably.
    ///
    pub fn exhume_associative_referent_mut(
        &mut self,
        id: &Uuid,
    ) -> Option<&mut AssociativeReferent> {
        self.associative_referent
            .get_mut(id)
            .map(|associative_referent| &mut associative_referent.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AssociativeReferent>`.
    ///
    pub fn iter_associative_referent(&self) -> impl Iterator<Item = &AssociativeReferent> {
        self.associative_referent
            .values()
            .map(|associative_referent| &associative_referent.0)
    }

    /// Get the timestamp for AssociativeReferent.
    ///
    pub fn associative_referent_timestamp(
        &self,
        associative_referent: &AssociativeReferent,
    ) -> SystemTime {
        self.associative_referent
            .get(&associative_referent.id)
            .map(|associative_referent| associative_referent.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`AssociativeReferrer`] into the store.
    ///
    pub fn inter_associative_referrer(&mut self, associative_referrer: AssociativeReferrer) {
        self.associative_referrer.insert(
            associative_referrer.id,
            (associative_referrer, SystemTime::now()),
        );
    }

    /// Exhume [`AssociativeReferrer`] from the store.
    ///
    pub fn exhume_associative_referrer(&self, id: &Uuid) -> Option<&AssociativeReferrer> {
        self.associative_referrer
            .get(id)
            .map(|associative_referrer| &associative_referrer.0)
    }

    /// Exhume [`AssociativeReferrer`] from the store — mutably.
    ///
    pub fn exhume_associative_referrer_mut(
        &mut self,
        id: &Uuid,
    ) -> Option<&mut AssociativeReferrer> {
        self.associative_referrer
            .get_mut(id)
            .map(|associative_referrer| &mut associative_referrer.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AssociativeReferrer>`.
    ///
    pub fn iter_associative_referrer(&self) -> impl Iterator<Item = &AssociativeReferrer> {
        self.associative_referrer
            .values()
            .map(|associative_referrer| &associative_referrer.0)
    }

    /// Get the timestamp for AssociativeReferrer.
    ///
    pub fn associative_referrer_timestamp(
        &self,
        associative_referrer: &AssociativeReferrer,
    ) -> SystemTime {
        self.associative_referrer
            .get(&associative_referrer.id)
            .map(|associative_referrer| associative_referrer.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Attribute`] into the store.
    ///
    pub fn inter_attribute(&mut self, attribute: Attribute) {
        let value = (attribute, SystemTime::now());
        self.attribute.insert(value.0.id, value.clone());
        self.attribute_by_name
            .insert(value.0.name.to_upper_camel_case(), value);
    }

    /// Exhume [`Attribute`] from the store.
    ///
    pub fn exhume_attribute(&self, id: &Uuid) -> Option<&Attribute> {
        self.attribute.get(id).map(|attribute| &attribute.0)
    }

    /// Exhume [`Attribute`] from the store — mutably.
    ///
    pub fn exhume_attribute_mut(&mut self, id: &Uuid) -> Option<&mut Attribute> {
        self.attribute.get_mut(id).map(|attribute| &mut attribute.0)
    }

    /// Exhume [`Attribute`] from the store by name.
    ///
    pub fn exhume_attribute_by_name(&self, name: &str) -> Option<&Attribute> {
        self.attribute_by_name
            .get(name)
            .map(|attribute| &attribute.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Attribute>`.
    ///
    pub fn iter_attribute(&self) -> impl Iterator<Item = &Attribute> {
        self.attribute.values().map(|attribute| &attribute.0)
    }

    /// Get the timestamp for Attribute.
    ///
    pub fn attribute_timestamp(&self, attribute: &Attribute) -> SystemTime {
        self.attribute
            .get(&attribute.id)
            .map(|attribute| attribute.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Binary`] into the store.
    ///
    pub fn inter_binary(&mut self, binary: Binary) {
        self.binary.insert(binary.id, (binary, SystemTime::now()));
    }

    /// Exhume [`Binary`] from the store.
    ///
    pub fn exhume_binary(&self, id: &Uuid) -> Option<&Binary> {
        self.binary.get(id).map(|binary| &binary.0)
    }

    /// Exhume [`Binary`] from the store — mutably.
    ///
    pub fn exhume_binary_mut(&mut self, id: &Uuid) -> Option<&mut Binary> {
        self.binary.get_mut(id).map(|binary| &mut binary.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Binary>`.
    ///
    pub fn iter_binary(&self) -> impl Iterator<Item = &Binary> {
        self.binary.values().map(|binary| &binary.0)
    }

    /// Get the timestamp for Binary.
    ///
    pub fn binary_timestamp(&self, binary: &Binary) -> SystemTime {
        self.binary
            .get(&binary.id)
            .map(|binary| binary.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Cardinality`] into the store.
    ///
    pub fn inter_cardinality(&mut self, cardinality: Cardinality) {
        self.cardinality
            .insert(cardinality.id(), (cardinality, SystemTime::now()));
    }

    /// Exhume [`Cardinality`] from the store.
    ///
    pub fn exhume_cardinality(&self, id: &Uuid) -> Option<&Cardinality> {
        self.cardinality.get(id).map(|cardinality| &cardinality.0)
    }

    /// Exhume [`Cardinality`] from the store — mutably.
    ///
    pub fn exhume_cardinality_mut(&mut self, id: &Uuid) -> Option<&mut Cardinality> {
        self.cardinality
            .get_mut(id)
            .map(|cardinality| &mut cardinality.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Cardinality>`.
    ///
    pub fn iter_cardinality(&self) -> impl Iterator<Item = &Cardinality> {
        self.cardinality.values().map(|cardinality| &cardinality.0)
    }

    /// Get the timestamp for Cardinality.
    ///
    pub fn cardinality_timestamp(&self, cardinality: &Cardinality) -> SystemTime {
        self.cardinality
            .get(&cardinality.id())
            .map(|cardinality| cardinality.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Conditionality`] into the store.
    ///
    pub fn inter_conditionality(&mut self, conditionality: Conditionality) {
        self.conditionality
            .insert(conditionality.id(), (conditionality, SystemTime::now()));
    }

    /// Exhume [`Conditionality`] from the store.
    ///
    pub fn exhume_conditionality(&self, id: &Uuid) -> Option<&Conditionality> {
        self.conditionality
            .get(id)
            .map(|conditionality| &conditionality.0)
    }

    /// Exhume [`Conditionality`] from the store — mutably.
    ///
    pub fn exhume_conditionality_mut(&mut self, id: &Uuid) -> Option<&mut Conditionality> {
        self.conditionality
            .get_mut(id)
            .map(|conditionality| &mut conditionality.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Conditionality>`.
    ///
    pub fn iter_conditionality(&self) -> impl Iterator<Item = &Conditionality> {
        self.conditionality
            .values()
            .map(|conditionality| &conditionality.0)
    }

    /// Get the timestamp for Conditionality.
    ///
    pub fn conditionality_timestamp(&self, conditionality: &Conditionality) -> SystemTime {
        self.conditionality
            .get(&conditionality.id())
            .map(|conditionality| conditionality.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Event`] into the store.
    ///
    pub fn inter_event(&mut self, event: Event) {
        let value = (event, SystemTime::now());
        self.event.insert(value.0.id, value.clone());
        self.event_by_name
            .insert(value.0.name.to_upper_camel_case(), value);
    }

    /// Exhume [`Event`] from the store.
    ///
    pub fn exhume_event(&self, id: &Uuid) -> Option<&Event> {
        self.event.get(id).map(|event| &event.0)
    }

    /// Exhume [`Event`] from the store — mutably.
    ///
    pub fn exhume_event_mut(&mut self, id: &Uuid) -> Option<&mut Event> {
        self.event.get_mut(id).map(|event| &mut event.0)
    }

    /// Exhume [`Event`] from the store by name.
    ///
    pub fn exhume_event_by_name(&self, name: &str) -> Option<&Event> {
        self.event_by_name.get(name).map(|event| &event.0)
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

    /// Inter [`External`] into the store.
    ///
    pub fn inter_external(&mut self, external: External) {
        let value = (external, SystemTime::now());
        self.external.insert(value.0.id, value.clone());
        self.external_by_name
            .insert(value.0.name.to_upper_camel_case(), value);
    }

    /// Exhume [`External`] from the store.
    ///
    pub fn exhume_external(&self, id: &Uuid) -> Option<&External> {
        self.external.get(id).map(|external| &external.0)
    }

    /// Exhume [`External`] from the store — mutably.
    ///
    pub fn exhume_external_mut(&mut self, id: &Uuid) -> Option<&mut External> {
        self.external.get_mut(id).map(|external| &mut external.0)
    }

    /// Exhume [`External`] from the store by name.
    ///
    pub fn exhume_external_by_name(&self, name: &str) -> Option<&External> {
        self.external_by_name.get(name).map(|external| &external.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, External>`.
    ///
    pub fn iter_external(&self) -> impl Iterator<Item = &External> {
        self.external.values().map(|external| &external.0)
    }

    /// Get the timestamp for External.
    ///
    pub fn external_timestamp(&self, external: &External) -> SystemTime {
        self.external
            .get(&external.id)
            .map(|external| external.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Isa`] into the store.
    ///
    pub fn inter_isa(&mut self, isa: Isa) {
        self.isa.insert(isa.id, (isa, SystemTime::now()));
    }

    /// Exhume [`Isa`] from the store.
    ///
    pub fn exhume_isa(&self, id: &Uuid) -> Option<&Isa> {
        self.isa.get(id).map(|isa| &isa.0)
    }

    /// Exhume [`Isa`] from the store — mutably.
    ///
    pub fn exhume_isa_mut(&mut self, id: &Uuid) -> Option<&mut Isa> {
        self.isa.get_mut(id).map(|isa| &mut isa.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Isa>`.
    ///
    pub fn iter_isa(&self) -> impl Iterator<Item = &Isa> {
        self.isa.values().map(|isa| &isa.0)
    }

    /// Get the timestamp for Isa.
    ///
    pub fn isa_timestamp(&self, isa: &Isa) -> SystemTime {
        self.isa
            .get(&isa.id)
            .map(|isa| isa.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Object`] into the store.
    ///
    pub fn inter_object(&mut self, object: Object) {
        let value = (object, SystemTime::now());
        self.object.insert(value.0.id, value.clone());
        self.object_by_name
            .insert(value.0.name.to_upper_camel_case(), value);
    }

    /// Exhume [`Object`] from the store.
    ///
    pub fn exhume_object(&self, id: &Uuid) -> Option<&Object> {
        self.object.get(id).map(|object| &object.0)
    }

    /// Exhume [`Object`] from the store — mutably.
    ///
    pub fn exhume_object_mut(&mut self, id: &Uuid) -> Option<&mut Object> {
        self.object.get_mut(id).map(|object| &mut object.0)
    }

    /// Exhume [`Object`] from the store by name.
    ///
    pub fn exhume_object_by_name(&self, name: &str) -> Option<&Object> {
        self.object_by_name.get(name).map(|object| &object.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Object>`.
    ///
    pub fn iter_object(&self) -> impl Iterator<Item = &Object> {
        self.object.values().map(|object| &object.0)
    }

    /// Get the timestamp for Object.
    ///
    pub fn object_timestamp(&self, object: &Object) -> SystemTime {
        self.object
            .get(&object.id)
            .map(|object| object.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Referent`] into the store.
    ///
    pub fn inter_referent(&mut self, referent: Referent) {
        self.referent
            .insert(referent.id, (referent, SystemTime::now()));
    }

    /// Exhume [`Referent`] from the store.
    ///
    pub fn exhume_referent(&self, id: &Uuid) -> Option<&Referent> {
        self.referent.get(id).map(|referent| &referent.0)
    }

    /// Exhume [`Referent`] from the store — mutably.
    ///
    pub fn exhume_referent_mut(&mut self, id: &Uuid) -> Option<&mut Referent> {
        self.referent.get_mut(id).map(|referent| &mut referent.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Referent>`.
    ///
    pub fn iter_referent(&self) -> impl Iterator<Item = &Referent> {
        self.referent.values().map(|referent| &referent.0)
    }

    /// Get the timestamp for Referent.
    ///
    pub fn referent_timestamp(&self, referent: &Referent) -> SystemTime {
        self.referent
            .get(&referent.id)
            .map(|referent| referent.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Referrer`] into the store.
    ///
    pub fn inter_referrer(&mut self, referrer: Referrer) {
        self.referrer
            .insert(referrer.id, (referrer, SystemTime::now()));
    }

    /// Exhume [`Referrer`] from the store.
    ///
    pub fn exhume_referrer(&self, id: &Uuid) -> Option<&Referrer> {
        self.referrer.get(id).map(|referrer| &referrer.0)
    }

    /// Exhume [`Referrer`] from the store — mutably.
    ///
    pub fn exhume_referrer_mut(&mut self, id: &Uuid) -> Option<&mut Referrer> {
        self.referrer.get_mut(id).map(|referrer| &mut referrer.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Referrer>`.
    ///
    pub fn iter_referrer(&self) -> impl Iterator<Item = &Referrer> {
        self.referrer.values().map(|referrer| &referrer.0)
    }

    /// Get the timestamp for Referrer.
    ///
    pub fn referrer_timestamp(&self, referrer: &Referrer) -> SystemTime {
        self.referrer
            .get(&referrer.id)
            .map(|referrer| referrer.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Relationship`] into the store.
    ///
    pub fn inter_relationship(&mut self, relationship: Relationship) {
        self.relationship
            .insert(relationship.id(), (relationship, SystemTime::now()));
    }

    /// Exhume [`Relationship`] from the store.
    ///
    pub fn exhume_relationship(&self, id: &Uuid) -> Option<&Relationship> {
        self.relationship
            .get(id)
            .map(|relationship| &relationship.0)
    }

    /// Exhume [`Relationship`] from the store — mutably.
    ///
    pub fn exhume_relationship_mut(&mut self, id: &Uuid) -> Option<&mut Relationship> {
        self.relationship
            .get_mut(id)
            .map(|relationship| &mut relationship.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Relationship>`.
    ///
    pub fn iter_relationship(&self) -> impl Iterator<Item = &Relationship> {
        self.relationship
            .values()
            .map(|relationship| &relationship.0)
    }

    /// Get the timestamp for Relationship.
    ///
    pub fn relationship_timestamp(&self, relationship: &Relationship) -> SystemTime {
        self.relationship
            .get(&relationship.id())
            .map(|relationship| relationship.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`State`] into the store.
    ///
    pub fn inter_state(&mut self, state: State) {
        let value = (state, SystemTime::now());
        self.state.insert(value.0.id, value.clone());
        self.state_by_name
            .insert(value.0.name.to_upper_camel_case(), value);
    }

    /// Exhume [`State`] from the store.
    ///
    pub fn exhume_state(&self, id: &Uuid) -> Option<&State> {
        self.state.get(id).map(|state| &state.0)
    }

    /// Exhume [`State`] from the store — mutably.
    ///
    pub fn exhume_state_mut(&mut self, id: &Uuid) -> Option<&mut State> {
        self.state.get_mut(id).map(|state| &mut state.0)
    }

    /// Exhume [`State`] from the store by name.
    ///
    pub fn exhume_state_by_name(&self, name: &str) -> Option<&State> {
        self.state_by_name.get(name).map(|state| &state.0)
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

    /// Inter [`Subtype`] into the store.
    ///
    pub fn inter_subtype(&mut self, subtype: Subtype) {
        self.subtype
            .insert(subtype.id, (subtype, SystemTime::now()));
    }

    /// Exhume [`Subtype`] from the store.
    ///
    pub fn exhume_subtype(&self, id: &Uuid) -> Option<&Subtype> {
        self.subtype.get(id).map(|subtype| &subtype.0)
    }

    /// Exhume [`Subtype`] from the store — mutably.
    ///
    pub fn exhume_subtype_mut(&mut self, id: &Uuid) -> Option<&mut Subtype> {
        self.subtype.get_mut(id).map(|subtype| &mut subtype.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Subtype>`.
    ///
    pub fn iter_subtype(&self) -> impl Iterator<Item = &Subtype> {
        self.subtype.values().map(|subtype| &subtype.0)
    }

    /// Get the timestamp for Subtype.
    ///
    pub fn subtype_timestamp(&self, subtype: &Subtype) -> SystemTime {
        self.subtype
            .get(&subtype.id)
            .map(|subtype| subtype.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Supertype`] into the store.
    ///
    pub fn inter_supertype(&mut self, supertype: Supertype) {
        self.supertype
            .insert(supertype.id, (supertype, SystemTime::now()));
    }

    /// Exhume [`Supertype`] from the store.
    ///
    pub fn exhume_supertype(&self, id: &Uuid) -> Option<&Supertype> {
        self.supertype.get(id).map(|supertype| &supertype.0)
    }

    /// Exhume [`Supertype`] from the store — mutably.
    ///
    pub fn exhume_supertype_mut(&mut self, id: &Uuid) -> Option<&mut Supertype> {
        self.supertype.get_mut(id).map(|supertype| &mut supertype.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Supertype>`.
    ///
    pub fn iter_supertype(&self) -> impl Iterator<Item = &Supertype> {
        self.supertype.values().map(|supertype| &supertype.0)
    }

    /// Get the timestamp for Supertype.
    ///
    pub fn supertype_timestamp(&self, supertype: &Supertype) -> SystemTime {
        self.supertype
            .get(&supertype.id)
            .map(|supertype| supertype.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Ty`] into the store.
    ///
    pub fn inter_ty(&mut self, ty: Ty) {
        self.ty.insert(ty.id(), (ty, SystemTime::now()));
    }

    /// Exhume [`Ty`] from the store.
    ///
    pub fn exhume_ty(&self, id: &Uuid) -> Option<&Ty> {
        self.ty.get(id).map(|ty| &ty.0)
    }

    /// Exhume [`Ty`] from the store — mutably.
    ///
    pub fn exhume_ty_mut(&mut self, id: &Uuid) -> Option<&mut Ty> {
        self.ty.get_mut(id).map(|ty| &mut ty.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Ty>`.
    ///
    pub fn iter_ty(&self) -> impl Iterator<Item = &Ty> {
        self.ty.values().map(|ty| &ty.0)
    }

    /// Get the timestamp for Ty.
    ///
    pub fn ty_timestamp(&self, ty: &Ty) -> SystemTime {
        self.ty
            .get(&ty.id())
            .map(|ty| ty.1)
            .unwrap_or(SystemTime::now())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::sarzak_ts-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        fs::create_dir_all(&path)?;

        let bin_path = path.clone().join("sarzak.bin");
        let mut bin_file = fs::File::create(bin_path)?;
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        bin_file.write_all(&encoded)?;

        let path = path.join("sarzak.json");
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
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.acknowledged_event.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist An Associative Referent.
        {
            let path = path.join("an_associative_referent");
            fs::create_dir_all(&path)?;
            for an_associative_referent_tuple in self.an_associative_referent.values() {
                let path = path.join(format!("{}.json", an_associative_referent_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (AnAssociativeReferent, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0 != an_associative_referent_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &an_associative_referent_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &an_associative_referent_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.an_associative_referent.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Associative.
        {
            let path = path.join("associative");
            fs::create_dir_all(&path)?;
            for associative_tuple in self.associative.values() {
                let path = path.join(format!("{}.json", associative_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Associative, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != associative_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &associative_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &associative_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.associative.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Associative Referent.
        {
            let path = path.join("associative_referent");
            fs::create_dir_all(&path)?;
            for associative_referent_tuple in self.associative_referent.values() {
                let path = path.join(format!("{}.json", associative_referent_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (AssociativeReferent, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0 != associative_referent_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &associative_referent_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &associative_referent_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.associative_referent.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Associative Referrer.
        {
            let path = path.join("associative_referrer");
            fs::create_dir_all(&path)?;
            for associative_referrer_tuple in self.associative_referrer.values() {
                let path = path.join(format!("{}.json", associative_referrer_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (AssociativeReferrer, SystemTime) =
                        serde_json::from_reader(reader)?;
                    if on_disk.0 != associative_referrer_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &associative_referrer_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &associative_referrer_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.associative_referrer.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Attribute.
        {
            let path = path.join("attribute");
            fs::create_dir_all(&path)?;
            for attribute_tuple in self.attribute.values() {
                let path = path.join(format!("{}.json", attribute_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Attribute, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != attribute_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &attribute_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &attribute_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.attribute.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Binary.
        {
            let path = path.join("binary");
            fs::create_dir_all(&path)?;
            for binary_tuple in self.binary.values() {
                let path = path.join(format!("{}.json", binary_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Binary, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != binary_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &binary_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &binary_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.binary.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Cardinality.
        {
            let path = path.join("cardinality");
            fs::create_dir_all(&path)?;
            for cardinality_tuple in self.cardinality.values() {
                let path = path.join(format!("{}.json", cardinality_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Cardinality, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != cardinality_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &cardinality_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &cardinality_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.cardinality.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Conditionality.
        {
            let path = path.join("conditionality");
            fs::create_dir_all(&path)?;
            for conditionality_tuple in self.conditionality.values() {
                let path = path.join(format!("{}.json", conditionality_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Conditionality, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != conditionality_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &conditionality_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &conditionality_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.conditionality.contains_key(&id) {
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
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.event.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist External.
        {
            let path = path.join("external");
            fs::create_dir_all(&path)?;
            for external_tuple in self.external.values() {
                let path = path.join(format!("{}.json", external_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (External, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != external_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &external_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &external_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.external.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Isa.
        {
            let path = path.join("isa");
            fs::create_dir_all(&path)?;
            for isa_tuple in self.isa.values() {
                let path = path.join(format!("{}.json", isa_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Isa, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != isa_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &isa_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &isa_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.isa.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Object.
        {
            let path = path.join("object");
            fs::create_dir_all(&path)?;
            for object_tuple in self.object.values() {
                let path = path.join(format!("{}.json", object_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Object, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != object_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &object_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &object_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.object.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Referent.
        {
            let path = path.join("referent");
            fs::create_dir_all(&path)?;
            for referent_tuple in self.referent.values() {
                let path = path.join(format!("{}.json", referent_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Referent, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != referent_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &referent_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &referent_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.referent.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Referrer.
        {
            let path = path.join("referrer");
            fs::create_dir_all(&path)?;
            for referrer_tuple in self.referrer.values() {
                let path = path.join(format!("{}.json", referrer_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Referrer, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != referrer_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &referrer_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &referrer_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.referrer.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Relationship.
        {
            let path = path.join("relationship");
            fs::create_dir_all(&path)?;
            for relationship_tuple in self.relationship.values() {
                let path = path.join(format!("{}.json", relationship_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Relationship, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != relationship_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &relationship_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &relationship_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.relationship.contains_key(&id) {
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
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.state.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Subtype.
        {
            let path = path.join("subtype");
            fs::create_dir_all(&path)?;
            for subtype_tuple in self.subtype.values() {
                let path = path.join(format!("{}.json", subtype_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Subtype, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != subtype_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &subtype_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &subtype_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.subtype.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Supertype.
        {
            let path = path.join("supertype");
            fs::create_dir_all(&path)?;
            for supertype_tuple in self.supertype.values() {
                let path = path.join(format!("{}.json", supertype_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Supertype, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != supertype_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &supertype_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &supertype_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.supertype.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Type.
        {
            let path = path.join("ty");
            fs::create_dir_all(&path)?;
            for ty_tuple in self.ty.values() {
                let path = path.join(format!("{}.json", ty_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Ty, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != ty_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &ty_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &ty_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.ty.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
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
        let path = path.join("sarzak.json");

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
                let acknowledged_event: (AcknowledgedEvent, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .acknowledged_event
                    .insert(acknowledged_event.0.id, acknowledged_event);
            }
        }

        // Load An Associative Referent.
        {
            let path = path.join("an_associative_referent");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let an_associative_referent: (AnAssociativeReferent, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .an_associative_referent
                    .insert(an_associative_referent.0.id, an_associative_referent);
            }
        }

        // Load Associative.
        {
            let path = path.join("associative");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let associative: (Associative, SystemTime) = serde_json::from_reader(reader)?;
                store.associative.insert(associative.0.id, associative);
            }
        }

        // Load Associative Referent.
        {
            let path = path.join("associative_referent");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let associative_referent: (AssociativeReferent, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .associative_referent
                    .insert(associative_referent.0.id, associative_referent);
            }
        }

        // Load Associative Referrer.
        {
            let path = path.join("associative_referrer");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let associative_referrer: (AssociativeReferrer, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .associative_referrer
                    .insert(associative_referrer.0.id, associative_referrer);
            }
        }

        // Load Attribute.
        {
            let path = path.join("attribute");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let attribute: (Attribute, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .attribute_by_name
                    .insert(attribute.0.name.to_upper_camel_case(), attribute.clone());
                store.attribute.insert(attribute.0.id, attribute);
            }
        }

        // Load Binary.
        {
            let path = path.join("binary");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let binary: (Binary, SystemTime) = serde_json::from_reader(reader)?;
                store.binary.insert(binary.0.id, binary);
            }
        }

        // Load Cardinality.
        {
            let path = path.join("cardinality");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let cardinality: (Cardinality, SystemTime) = serde_json::from_reader(reader)?;
                store.cardinality.insert(cardinality.0.id(), cardinality);
            }
        }

        // Load Conditionality.
        {
            let path = path.join("conditionality");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let conditionality: (Conditionality, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .conditionality
                    .insert(conditionality.0.id(), conditionality);
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
                let event: (Event, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .event_by_name
                    .insert(event.0.name.to_upper_camel_case(), event.clone());
                store.event.insert(event.0.id, event);
            }
        }

        // Load External.
        {
            let path = path.join("external");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let external: (External, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .external_by_name
                    .insert(external.0.name.to_upper_camel_case(), external.clone());
                store.external.insert(external.0.id, external);
            }
        }

        // Load Isa.
        {
            let path = path.join("isa");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let isa: (Isa, SystemTime) = serde_json::from_reader(reader)?;
                store.isa.insert(isa.0.id, isa);
            }
        }

        // Load Object.
        {
            let path = path.join("object");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let object: (Object, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .object_by_name
                    .insert(object.0.name.to_upper_camel_case(), object.clone());
                store.object.insert(object.0.id, object);
            }
        }

        // Load Referent.
        {
            let path = path.join("referent");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let referent: (Referent, SystemTime) = serde_json::from_reader(reader)?;
                store.referent.insert(referent.0.id, referent);
            }
        }

        // Load Referrer.
        {
            let path = path.join("referrer");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let referrer: (Referrer, SystemTime) = serde_json::from_reader(reader)?;
                store.referrer.insert(referrer.0.id, referrer);
            }
        }

        // Load Relationship.
        {
            let path = path.join("relationship");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let relationship: (Relationship, SystemTime) = serde_json::from_reader(reader)?;
                store.relationship.insert(relationship.0.id(), relationship);
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
                let state: (State, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .state_by_name
                    .insert(state.0.name.to_upper_camel_case(), state.clone());
                store.state.insert(state.0.id, state);
            }
        }

        // Load Subtype.
        {
            let path = path.join("subtype");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let subtype: (Subtype, SystemTime) = serde_json::from_reader(reader)?;
                store.subtype.insert(subtype.0.id, subtype);
            }
        }

        // Load Supertype.
        {
            let path = path.join("supertype");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let supertype: (Supertype, SystemTime) = serde_json::from_reader(reader)?;
                store.supertype.insert(supertype.0.id, supertype);
            }
        }

        // Load Type.
        {
            let path = path.join("ty");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let ty: (Ty, SystemTime) = serde_json::from_reader(reader)?;
                store.ty.insert(ty.0.id(), ty);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

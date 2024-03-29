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

use heck::ToUpperCamelCase;
use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::sarzak_ts::types::{
    AcknowledgedEvent, AnAssociativeReferent, Associative, AssociativeReferent,
    AssociativeReferrer, Attribute, Binary, Cardinality, Conditionality, Event, External, Isa,
    Object, Referent, Referrer, Relationship, State, Subtype, Supertype, Ty, BOOLEAN, CONDITIONAL,
    FLOAT, INTEGER, MANY, ONE, S_STRING, S_UUID, UNCONDITIONAL,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    acknowledged_event: HashMap<Uuid, (AcknowledgedEvent, SystemTime)>,
    an_associative_referent: HashMap<Uuid, (AnAssociativeReferent, SystemTime)>,
    associative: HashMap<Uuid, (Associative, SystemTime)>,
    associative_referent: HashMap<Uuid, (AssociativeReferent, SystemTime)>,
    associative_referrer: HashMap<Uuid, (AssociativeReferrer, SystemTime)>,
    attribute: HashMap<Uuid, (Attribute, SystemTime)>,
    binary: HashMap<Uuid, (Binary, SystemTime)>,
    cardinality: HashMap<Uuid, (Cardinality, SystemTime)>,
    conditionality: HashMap<Uuid, (Conditionality, SystemTime)>,
    event: HashMap<Uuid, (Event, SystemTime)>,
    external: HashMap<Uuid, (External, SystemTime)>,
    isa: HashMap<Uuid, (Isa, SystemTime)>,
    object: HashMap<Uuid, (Object, SystemTime)>,
    object_id_by_name: HashMap<String, (Uuid, SystemTime)>,
    referent: HashMap<Uuid, (Referent, SystemTime)>,
    referrer: HashMap<Uuid, (Referrer, SystemTime)>,
    relationship: HashMap<Uuid, (Relationship, SystemTime)>,
    state: HashMap<Uuid, (State, SystemTime)>,
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
            binary: HashMap::default(),
            cardinality: HashMap::default(),
            conditionality: HashMap::default(),
            event: HashMap::default(),
            external: HashMap::default(),
            isa: HashMap::default(),
            object: HashMap::default(),
            object_id_by_name: HashMap::default(),
            referent: HashMap::default(),
            referrer: HashMap::default(),
            relationship: HashMap::default(),
            state: HashMap::default(),
            subtype: HashMap::default(),
            supertype: HashMap::default(),
            ty: HashMap::default(),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥
        store.inter_cardinality(Cardinality::Many(MANY));
        store.inter_cardinality(Cardinality::One(ONE));
        store.inter_conditionality(Conditionality::Conditional(CONDITIONAL));
        store.inter_conditionality(Conditionality::Unconditional(UNCONDITIONAL));
        store.inter_ty(Ty::Boolean(BOOLEAN));
        store.inter_ty(Ty::Float(FLOAT));
        store.inter_ty(Ty::Integer(INTEGER));
        store.inter_ty(Ty::SString(S_STRING));
        store.inter_ty(Ty::SUuid(S_UUID));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::sarzak_ts-object-store-methods"}}}
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

    /// Inter (insert) [`AnAssociativeReferent`] into the store.
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

    /// Exhume (get) [`AnAssociativeReferent`] from the store.
    ///
    pub fn exhume_an_associative_referent(&self, id: &Uuid) -> Option<&AnAssociativeReferent> {
        self.an_associative_referent
            .get(id)
            .map(|an_associative_referent| &an_associative_referent.0)
    }

    /// Exorcise (remove) [`AnAssociativeReferent`] from the store.
    ///
    pub fn exorcise_an_associative_referent(&mut self, id: &Uuid) -> Option<AnAssociativeReferent> {
        self.an_associative_referent
            .remove(id)
            .map(|an_associative_referent| an_associative_referent.0)
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

    /// Inter (insert) [`Associative`] into the store.
    ///
    pub fn inter_associative(&mut self, associative: Associative) {
        self.associative
            .insert(associative.id, (associative, SystemTime::now()));
    }

    /// Exhume (get) [`Associative`] from the store.
    ///
    pub fn exhume_associative(&self, id: &Uuid) -> Option<&Associative> {
        self.associative.get(id).map(|associative| &associative.0)
    }

    /// Exorcise (remove) [`Associative`] from the store.
    ///
    pub fn exorcise_associative(&mut self, id: &Uuid) -> Option<Associative> {
        self.associative.remove(id).map(|associative| associative.0)
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

    /// Inter (insert) [`AssociativeReferent`] into the store.
    ///
    pub fn inter_associative_referent(&mut self, associative_referent: AssociativeReferent) {
        self.associative_referent.insert(
            associative_referent.id,
            (associative_referent, SystemTime::now()),
        );
    }

    /// Exhume (get) [`AssociativeReferent`] from the store.
    ///
    pub fn exhume_associative_referent(&self, id: &Uuid) -> Option<&AssociativeReferent> {
        self.associative_referent
            .get(id)
            .map(|associative_referent| &associative_referent.0)
    }

    /// Exorcise (remove) [`AssociativeReferent`] from the store.
    ///
    pub fn exorcise_associative_referent(&mut self, id: &Uuid) -> Option<AssociativeReferent> {
        self.associative_referent
            .remove(id)
            .map(|associative_referent| associative_referent.0)
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

    /// Inter (insert) [`AssociativeReferrer`] into the store.
    ///
    pub fn inter_associative_referrer(&mut self, associative_referrer: AssociativeReferrer) {
        self.associative_referrer.insert(
            associative_referrer.id,
            (associative_referrer, SystemTime::now()),
        );
    }

    /// Exhume (get) [`AssociativeReferrer`] from the store.
    ///
    pub fn exhume_associative_referrer(&self, id: &Uuid) -> Option<&AssociativeReferrer> {
        self.associative_referrer
            .get(id)
            .map(|associative_referrer| &associative_referrer.0)
    }

    /// Exorcise (remove) [`AssociativeReferrer`] from the store.
    ///
    pub fn exorcise_associative_referrer(&mut self, id: &Uuid) -> Option<AssociativeReferrer> {
        self.associative_referrer
            .remove(id)
            .map(|associative_referrer| associative_referrer.0)
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

    /// Inter (insert) [`Attribute`] into the store.
    ///
    pub fn inter_attribute(&mut self, attribute: Attribute) {
        self.attribute
            .insert(attribute.id, (attribute, SystemTime::now()));
    }

    /// Exhume (get) [`Attribute`] from the store.
    ///
    pub fn exhume_attribute(&self, id: &Uuid) -> Option<&Attribute> {
        self.attribute.get(id).map(|attribute| &attribute.0)
    }

    /// Exorcise (remove) [`Attribute`] from the store.
    ///
    pub fn exorcise_attribute(&mut self, id: &Uuid) -> Option<Attribute> {
        self.attribute.remove(id).map(|attribute| attribute.0)
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

    /// Inter (insert) [`Binary`] into the store.
    ///
    pub fn inter_binary(&mut self, binary: Binary) {
        self.binary.insert(binary.id, (binary, SystemTime::now()));
    }

    /// Exhume (get) [`Binary`] from the store.
    ///
    pub fn exhume_binary(&self, id: &Uuid) -> Option<&Binary> {
        self.binary.get(id).map(|binary| &binary.0)
    }

    /// Exorcise (remove) [`Binary`] from the store.
    ///
    pub fn exorcise_binary(&mut self, id: &Uuid) -> Option<Binary> {
        self.binary.remove(id).map(|binary| binary.0)
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

    /// Inter (insert) [`Cardinality`] into the store.
    ///
    pub fn inter_cardinality(&mut self, cardinality: Cardinality) {
        self.cardinality
            .insert(cardinality.id(), (cardinality, SystemTime::now()));
    }

    /// Exhume (get) [`Cardinality`] from the store.
    ///
    pub fn exhume_cardinality(&self, id: &Uuid) -> Option<&Cardinality> {
        self.cardinality.get(id).map(|cardinality| &cardinality.0)
    }

    /// Exorcise (remove) [`Cardinality`] from the store.
    ///
    pub fn exorcise_cardinality(&mut self, id: &Uuid) -> Option<Cardinality> {
        self.cardinality.remove(id).map(|cardinality| cardinality.0)
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

    /// Inter (insert) [`Conditionality`] into the store.
    ///
    pub fn inter_conditionality(&mut self, conditionality: Conditionality) {
        self.conditionality
            .insert(conditionality.id(), (conditionality, SystemTime::now()));
    }

    /// Exhume (get) [`Conditionality`] from the store.
    ///
    pub fn exhume_conditionality(&self, id: &Uuid) -> Option<&Conditionality> {
        self.conditionality
            .get(id)
            .map(|conditionality| &conditionality.0)
    }

    /// Exorcise (remove) [`Conditionality`] from the store.
    ///
    pub fn exorcise_conditionality(&mut self, id: &Uuid) -> Option<Conditionality> {
        self.conditionality
            .remove(id)
            .map(|conditionality| conditionality.0)
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

    /// Inter (insert) [`External`] into the store.
    ///
    pub fn inter_external(&mut self, external: External) {
        self.external
            .insert(external.id, (external, SystemTime::now()));
    }

    /// Exhume (get) [`External`] from the store.
    ///
    pub fn exhume_external(&self, id: &Uuid) -> Option<&External> {
        self.external.get(id).map(|external| &external.0)
    }

    /// Exorcise (remove) [`External`] from the store.
    ///
    pub fn exorcise_external(&mut self, id: &Uuid) -> Option<External> {
        self.external.remove(id).map(|external| external.0)
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

    /// Inter (insert) [`Isa`] into the store.
    ///
    pub fn inter_isa(&mut self, isa: Isa) {
        self.isa.insert(isa.id, (isa, SystemTime::now()));
    }

    /// Exhume (get) [`Isa`] from the store.
    ///
    pub fn exhume_isa(&self, id: &Uuid) -> Option<&Isa> {
        self.isa.get(id).map(|isa| &isa.0)
    }

    /// Exorcise (remove) [`Isa`] from the store.
    ///
    pub fn exorcise_isa(&mut self, id: &Uuid) -> Option<Isa> {
        self.isa.remove(id).map(|isa| isa.0)
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

    /// Inter (insert) [`Object`] into the store.
    ///
    pub fn inter_object(&mut self, object: Object) {
        let value = (object, SystemTime::now());
        self.object_id_by_name
            .insert(value.0.name.to_upper_camel_case(), (value.0.id, value.1));
        self.object.insert(value.0.id, value);
    }

    /// Exhume (get) [`Object`] from the store.
    ///
    pub fn exhume_object(&self, id: &Uuid) -> Option<&Object> {
        self.object.get(id).map(|object| &object.0)
    }

    /// Exorcise (remove) [`Object`] from the store.
    ///
    pub fn exorcise_object(&mut self, id: &Uuid) -> Option<Object> {
        self.object.remove(id).map(|object| object.0)
    }

    /// Exhume [`Object`] id from the store by name.
    ///
    pub fn exhume_object_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.object_id_by_name.get(name).map(|object| object.0)
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

    /// Inter (insert) [`Referent`] into the store.
    ///
    pub fn inter_referent(&mut self, referent: Referent) {
        self.referent
            .insert(referent.id, (referent, SystemTime::now()));
    }

    /// Exhume (get) [`Referent`] from the store.
    ///
    pub fn exhume_referent(&self, id: &Uuid) -> Option<&Referent> {
        self.referent.get(id).map(|referent| &referent.0)
    }

    /// Exorcise (remove) [`Referent`] from the store.
    ///
    pub fn exorcise_referent(&mut self, id: &Uuid) -> Option<Referent> {
        self.referent.remove(id).map(|referent| referent.0)
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

    /// Inter (insert) [`Referrer`] into the store.
    ///
    pub fn inter_referrer(&mut self, referrer: Referrer) {
        self.referrer
            .insert(referrer.id, (referrer, SystemTime::now()));
    }

    /// Exhume (get) [`Referrer`] from the store.
    ///
    pub fn exhume_referrer(&self, id: &Uuid) -> Option<&Referrer> {
        self.referrer.get(id).map(|referrer| &referrer.0)
    }

    /// Exorcise (remove) [`Referrer`] from the store.
    ///
    pub fn exorcise_referrer(&mut self, id: &Uuid) -> Option<Referrer> {
        self.referrer.remove(id).map(|referrer| referrer.0)
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

    /// Inter (insert) [`Relationship`] into the store.
    ///
    pub fn inter_relationship(&mut self, relationship: Relationship) {
        self.relationship
            .insert(relationship.id(), (relationship, SystemTime::now()));
    }

    /// Exhume (get) [`Relationship`] from the store.
    ///
    pub fn exhume_relationship(&self, id: &Uuid) -> Option<&Relationship> {
        self.relationship
            .get(id)
            .map(|relationship| &relationship.0)
    }

    /// Exorcise (remove) [`Relationship`] from the store.
    ///
    pub fn exorcise_relationship(&mut self, id: &Uuid) -> Option<Relationship> {
        self.relationship
            .remove(id)
            .map(|relationship| relationship.0)
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

    /// Inter (insert) [`Subtype`] into the store.
    ///
    pub fn inter_subtype(&mut self, subtype: Subtype) {
        self.subtype
            .insert(subtype.id, (subtype, SystemTime::now()));
    }

    /// Exhume (get) [`Subtype`] from the store.
    ///
    pub fn exhume_subtype(&self, id: &Uuid) -> Option<&Subtype> {
        self.subtype.get(id).map(|subtype| &subtype.0)
    }

    /// Exorcise (remove) [`Subtype`] from the store.
    ///
    pub fn exorcise_subtype(&mut self, id: &Uuid) -> Option<Subtype> {
        self.subtype.remove(id).map(|subtype| subtype.0)
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

    /// Inter (insert) [`Supertype`] into the store.
    ///
    pub fn inter_supertype(&mut self, supertype: Supertype) {
        self.supertype
            .insert(supertype.id, (supertype, SystemTime::now()));
    }

    /// Exhume (get) [`Supertype`] from the store.
    ///
    pub fn exhume_supertype(&self, id: &Uuid) -> Option<&Supertype> {
        self.supertype.get(id).map(|supertype| &supertype.0)
    }

    /// Exorcise (remove) [`Supertype`] from the store.
    ///
    pub fn exorcise_supertype(&mut self, id: &Uuid) -> Option<Supertype> {
        self.supertype.remove(id).map(|supertype| supertype.0)
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

    /// Inter (insert) [`Ty`] into the store.
    ///
    pub fn inter_ty(&mut self, ty: Ty) {
        self.ty.insert(ty.id(), (ty, SystemTime::now()));
    }

    /// Exhume (get) [`Ty`] from the store.
    ///
    pub fn exhume_ty(&self, id: &Uuid) -> Option<&Ty> {
        self.ty.get(id).map(|ty| &ty.0)
    }

    /// Exorcise (remove) [`Ty`] from the store.
    ///
    pub fn exorcise_ty(&mut self, id: &Uuid) -> Option<Ty> {
        self.ty.remove(id).map(|ty| ty.0)
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
                let id = file_name.split('.').next().unwrap();
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
        let path = path.join("sarzak.json");

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

        // Load An Associative Referent.
        {
            let path = path.join("an_associative_referent");
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let attribute: (Attribute, SystemTime) = serde_json::from_reader(reader)?;
                store.attribute.insert(attribute.0.id, attribute);
            }
        }

        // Load Binary.
        {
            let path = path.join("binary");
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
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

        // Load External.
        {
            let path = path.join("external");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let external: (External, SystemTime) = serde_json::from_reader(reader)?;
                store.external.insert(external.0.id, external);
            }
        }

        // Load Isa.
        {
            let path = path.join("isa");
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let object: (Object, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .object_id_by_name
                    .insert(object.0.name.to_upper_camel_case(), (object.0.id, object.1));
                store.object.insert(object.0.id, object);
            }
        }

        // Load Referent.
        {
            let path = path.join("referent");
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
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

        // Load Subtype.
        {
            let path = path.join("subtype");
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
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
            let entries = fs::read_dir(path)?;
            for entry in entries {
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

//! domain::sarzak_rwlock Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::sarzak_rwlock-object-store-file"}}}
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::sarzak_rwlock-object-store-definition"}}}
use std::sync::Arc;
use std::sync::RwLock;
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use heck::ToUpperCamelCase;
use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::sarzak_rwlock::types::{
    AcknowledgedEvent, AnAssociativeReferent, Associative, AssociativeReferent,
    AssociativeReferrer, Attribute, Binary, Cardinality, Conditionality, Event, External, Isa,
    Object, Referent, Referrer, Relationship, State, Subtype, Supertype, Ty, BOOLEAN, CONDITIONAL,
    FLOAT, INTEGER, MANY, ONE, S_STRING, S_UUID, UNCONDITIONAL,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    acknowledged_event: Arc<RwLock<HashMap<Uuid, Arc<RwLock<AcknowledgedEvent>>>>>,
    an_associative_referent: Arc<RwLock<HashMap<Uuid, Arc<RwLock<AnAssociativeReferent>>>>>,
    associative: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Associative>>>>>,
    associative_referent: Arc<RwLock<HashMap<Uuid, Arc<RwLock<AssociativeReferent>>>>>,
    associative_referrer: Arc<RwLock<HashMap<Uuid, Arc<RwLock<AssociativeReferrer>>>>>,
    attribute: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Attribute>>>>>,
    binary: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Binary>>>>>,
    cardinality: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Cardinality>>>>>,
    conditionality: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Conditionality>>>>>,
    event: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Event>>>>>,
    external: Arc<RwLock<HashMap<Uuid, Arc<RwLock<External>>>>>,
    isa: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Isa>>>>>,
    object: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Object>>>>>,
    object_id_by_name: Arc<RwLock<HashMap<String, Uuid>>>,
    referent: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Referent>>>>>,
    referrer: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Referrer>>>>>,
    relationship: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Relationship>>>>>,
    state: Arc<RwLock<HashMap<Uuid, Arc<RwLock<State>>>>>,
    subtype: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Subtype>>>>>,
    supertype: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Supertype>>>>>,
    ty: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Ty>>>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            acknowledged_event: Arc::new(RwLock::new(HashMap::default())),
            an_associative_referent: Arc::new(RwLock::new(HashMap::default())),
            associative: Arc::new(RwLock::new(HashMap::default())),
            associative_referent: Arc::new(RwLock::new(HashMap::default())),
            associative_referrer: Arc::new(RwLock::new(HashMap::default())),
            attribute: Arc::new(RwLock::new(HashMap::default())),
            binary: Arc::new(RwLock::new(HashMap::default())),
            cardinality: Arc::new(RwLock::new(HashMap::default())),
            conditionality: Arc::new(RwLock::new(HashMap::default())),
            event: Arc::new(RwLock::new(HashMap::default())),
            external: Arc::new(RwLock::new(HashMap::default())),
            isa: Arc::new(RwLock::new(HashMap::default())),
            object: Arc::new(RwLock::new(HashMap::default())),
            object_id_by_name: Arc::new(RwLock::new(HashMap::default())),
            referent: Arc::new(RwLock::new(HashMap::default())),
            referrer: Arc::new(RwLock::new(HashMap::default())),
            relationship: Arc::new(RwLock::new(HashMap::default())),
            state: Arc::new(RwLock::new(HashMap::default())),
            subtype: Arc::new(RwLock::new(HashMap::default())),
            supertype: Arc::new(RwLock::new(HashMap::default())),
            ty: Arc::new(RwLock::new(HashMap::default())),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥
        store.inter_cardinality(Arc::new(RwLock::new(Cardinality::Many(MANY))));
        store.inter_cardinality(Arc::new(RwLock::new(Cardinality::One(ONE))));
        store.inter_conditionality(Arc::new(RwLock::new(Conditionality::Conditional(
            CONDITIONAL,
        ))));
        store.inter_conditionality(Arc::new(RwLock::new(Conditionality::Unconditional(
            UNCONDITIONAL,
        ))));
        store.inter_ty(Arc::new(RwLock::new(Ty::Boolean(BOOLEAN))));
        store.inter_ty(Arc::new(RwLock::new(Ty::Float(FLOAT))));
        store.inter_ty(Arc::new(RwLock::new(Ty::Integer(INTEGER))));
        store.inter_ty(Arc::new(RwLock::new(Ty::SString(S_STRING))));
        store.inter_ty(Arc::new(RwLock::new(Ty::SUuid(S_UUID))));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::sarzak_rwlock-object-store-methods"}}}
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

    /// Inter (insert) [`AnAssociativeReferent`] into the store.
    ///
    pub fn inter_an_associative_referent(
        &mut self,
        an_associative_referent: Arc<RwLock<AnAssociativeReferent>>,
    ) {
        let read = an_associative_referent.read().unwrap();
        self.an_associative_referent
            .write()
            .unwrap()
            .insert(read.id, an_associative_referent.clone());
    }

    /// Exhume (get) [`AnAssociativeReferent`] from the store.
    ///
    pub fn exhume_an_associative_referent(
        &self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<AnAssociativeReferent>>> {
        self.an_associative_referent
            .read()
            .unwrap()
            .get(id)
            .map(|an_associative_referent| an_associative_referent.clone())
    }

    /// Exorcise (remove) [`AnAssociativeReferent`] from the store.
    ///
    pub fn exorcise_an_associative_referent(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<AnAssociativeReferent>>> {
        self.an_associative_referent
            .write()
            .unwrap()
            .remove(id)
            .map(|an_associative_referent| an_associative_referent.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AnAssociativeReferent>`.
    ///
    pub fn iter_an_associative_referent(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<AnAssociativeReferent>>> + '_ {
        let values: Vec<Arc<RwLock<AnAssociativeReferent>>> = self
            .an_associative_referent
            .read()
            .unwrap()
            .values()
            .map(|an_associative_referent| an_associative_referent.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Associative`] into the store.
    ///
    pub fn inter_associative(&mut self, associative: Arc<RwLock<Associative>>) {
        let read = associative.read().unwrap();
        self.associative
            .write()
            .unwrap()
            .insert(read.id, associative.clone());
    }

    /// Exhume (get) [`Associative`] from the store.
    ///
    pub fn exhume_associative(&self, id: &Uuid) -> Option<Arc<RwLock<Associative>>> {
        self.associative
            .read()
            .unwrap()
            .get(id)
            .map(|associative| associative.clone())
    }

    /// Exorcise (remove) [`Associative`] from the store.
    ///
    pub fn exorcise_associative(&mut self, id: &Uuid) -> Option<Arc<RwLock<Associative>>> {
        self.associative
            .write()
            .unwrap()
            .remove(id)
            .map(|associative| associative.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Associative>`.
    ///
    pub fn iter_associative(&self) -> impl Iterator<Item = Arc<RwLock<Associative>>> + '_ {
        let values: Vec<Arc<RwLock<Associative>>> = self
            .associative
            .read()
            .unwrap()
            .values()
            .map(|associative| associative.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`AssociativeReferent`] into the store.
    ///
    pub fn inter_associative_referent(
        &mut self,
        associative_referent: Arc<RwLock<AssociativeReferent>>,
    ) {
        let read = associative_referent.read().unwrap();
        self.associative_referent
            .write()
            .unwrap()
            .insert(read.id, associative_referent.clone());
    }

    /// Exhume (get) [`AssociativeReferent`] from the store.
    ///
    pub fn exhume_associative_referent(
        &self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<AssociativeReferent>>> {
        self.associative_referent
            .read()
            .unwrap()
            .get(id)
            .map(|associative_referent| associative_referent.clone())
    }

    /// Exorcise (remove) [`AssociativeReferent`] from the store.
    ///
    pub fn exorcise_associative_referent(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<AssociativeReferent>>> {
        self.associative_referent
            .write()
            .unwrap()
            .remove(id)
            .map(|associative_referent| associative_referent.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AssociativeReferent>`.
    ///
    pub fn iter_associative_referent(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<AssociativeReferent>>> + '_ {
        let values: Vec<Arc<RwLock<AssociativeReferent>>> = self
            .associative_referent
            .read()
            .unwrap()
            .values()
            .map(|associative_referent| associative_referent.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`AssociativeReferrer`] into the store.
    ///
    pub fn inter_associative_referrer(
        &mut self,
        associative_referrer: Arc<RwLock<AssociativeReferrer>>,
    ) {
        let read = associative_referrer.read().unwrap();
        self.associative_referrer
            .write()
            .unwrap()
            .insert(read.id, associative_referrer.clone());
    }

    /// Exhume (get) [`AssociativeReferrer`] from the store.
    ///
    pub fn exhume_associative_referrer(
        &self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<AssociativeReferrer>>> {
        self.associative_referrer
            .read()
            .unwrap()
            .get(id)
            .map(|associative_referrer| associative_referrer.clone())
    }

    /// Exorcise (remove) [`AssociativeReferrer`] from the store.
    ///
    pub fn exorcise_associative_referrer(
        &mut self,
        id: &Uuid,
    ) -> Option<Arc<RwLock<AssociativeReferrer>>> {
        self.associative_referrer
            .write()
            .unwrap()
            .remove(id)
            .map(|associative_referrer| associative_referrer.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AssociativeReferrer>`.
    ///
    pub fn iter_associative_referrer(
        &self,
    ) -> impl Iterator<Item = Arc<RwLock<AssociativeReferrer>>> + '_ {
        let values: Vec<Arc<RwLock<AssociativeReferrer>>> = self
            .associative_referrer
            .read()
            .unwrap()
            .values()
            .map(|associative_referrer| associative_referrer.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Attribute`] into the store.
    ///
    pub fn inter_attribute(&mut self, attribute: Arc<RwLock<Attribute>>) {
        let read = attribute.read().unwrap();
        self.attribute
            .write()
            .unwrap()
            .insert(read.id, attribute.clone());
    }

    /// Exhume (get) [`Attribute`] from the store.
    ///
    pub fn exhume_attribute(&self, id: &Uuid) -> Option<Arc<RwLock<Attribute>>> {
        self.attribute
            .read()
            .unwrap()
            .get(id)
            .map(|attribute| attribute.clone())
    }

    /// Exorcise (remove) [`Attribute`] from the store.
    ///
    pub fn exorcise_attribute(&mut self, id: &Uuid) -> Option<Arc<RwLock<Attribute>>> {
        self.attribute
            .write()
            .unwrap()
            .remove(id)
            .map(|attribute| attribute.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Attribute>`.
    ///
    pub fn iter_attribute(&self) -> impl Iterator<Item = Arc<RwLock<Attribute>>> + '_ {
        let values: Vec<Arc<RwLock<Attribute>>> = self
            .attribute
            .read()
            .unwrap()
            .values()
            .map(|attribute| attribute.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Binary`] into the store.
    ///
    pub fn inter_binary(&mut self, binary: Arc<RwLock<Binary>>) {
        let read = binary.read().unwrap();
        self.binary.write().unwrap().insert(read.id, binary.clone());
    }

    /// Exhume (get) [`Binary`] from the store.
    ///
    pub fn exhume_binary(&self, id: &Uuid) -> Option<Arc<RwLock<Binary>>> {
        self.binary
            .read()
            .unwrap()
            .get(id)
            .map(|binary| binary.clone())
    }

    /// Exorcise (remove) [`Binary`] from the store.
    ///
    pub fn exorcise_binary(&mut self, id: &Uuid) -> Option<Arc<RwLock<Binary>>> {
        self.binary
            .write()
            .unwrap()
            .remove(id)
            .map(|binary| binary.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Binary>`.
    ///
    pub fn iter_binary(&self) -> impl Iterator<Item = Arc<RwLock<Binary>>> + '_ {
        let values: Vec<Arc<RwLock<Binary>>> = self
            .binary
            .read()
            .unwrap()
            .values()
            .map(|binary| binary.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Cardinality`] into the store.
    ///
    pub fn inter_cardinality(&mut self, cardinality: Arc<RwLock<Cardinality>>) {
        let read = cardinality.read().unwrap();
        self.cardinality
            .write()
            .unwrap()
            .insert(read.id(), cardinality.clone());
    }

    /// Exhume (get) [`Cardinality`] from the store.
    ///
    pub fn exhume_cardinality(&self, id: &Uuid) -> Option<Arc<RwLock<Cardinality>>> {
        self.cardinality
            .read()
            .unwrap()
            .get(id)
            .map(|cardinality| cardinality.clone())
    }

    /// Exorcise (remove) [`Cardinality`] from the store.
    ///
    pub fn exorcise_cardinality(&mut self, id: &Uuid) -> Option<Arc<RwLock<Cardinality>>> {
        self.cardinality
            .write()
            .unwrap()
            .remove(id)
            .map(|cardinality| cardinality.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Cardinality>`.
    ///
    pub fn iter_cardinality(&self) -> impl Iterator<Item = Arc<RwLock<Cardinality>>> + '_ {
        let values: Vec<Arc<RwLock<Cardinality>>> = self
            .cardinality
            .read()
            .unwrap()
            .values()
            .map(|cardinality| cardinality.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Conditionality`] into the store.
    ///
    pub fn inter_conditionality(&mut self, conditionality: Arc<RwLock<Conditionality>>) {
        let read = conditionality.read().unwrap();
        self.conditionality
            .write()
            .unwrap()
            .insert(read.id(), conditionality.clone());
    }

    /// Exhume (get) [`Conditionality`] from the store.
    ///
    pub fn exhume_conditionality(&self, id: &Uuid) -> Option<Arc<RwLock<Conditionality>>> {
        self.conditionality
            .read()
            .unwrap()
            .get(id)
            .map(|conditionality| conditionality.clone())
    }

    /// Exorcise (remove) [`Conditionality`] from the store.
    ///
    pub fn exorcise_conditionality(&mut self, id: &Uuid) -> Option<Arc<RwLock<Conditionality>>> {
        self.conditionality
            .write()
            .unwrap()
            .remove(id)
            .map(|conditionality| conditionality.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Conditionality>`.
    ///
    pub fn iter_conditionality(&self) -> impl Iterator<Item = Arc<RwLock<Conditionality>>> + '_ {
        let values: Vec<Arc<RwLock<Conditionality>>> = self
            .conditionality
            .read()
            .unwrap()
            .values()
            .map(|conditionality| conditionality.clone())
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

    /// Inter (insert) [`External`] into the store.
    ///
    pub fn inter_external(&mut self, external: Arc<RwLock<External>>) {
        let read = external.read().unwrap();
        self.external
            .write()
            .unwrap()
            .insert(read.id, external.clone());
    }

    /// Exhume (get) [`External`] from the store.
    ///
    pub fn exhume_external(&self, id: &Uuid) -> Option<Arc<RwLock<External>>> {
        self.external
            .read()
            .unwrap()
            .get(id)
            .map(|external| external.clone())
    }

    /// Exorcise (remove) [`External`] from the store.
    ///
    pub fn exorcise_external(&mut self, id: &Uuid) -> Option<Arc<RwLock<External>>> {
        self.external
            .write()
            .unwrap()
            .remove(id)
            .map(|external| external.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, External>`.
    ///
    pub fn iter_external(&self) -> impl Iterator<Item = Arc<RwLock<External>>> + '_ {
        let values: Vec<Arc<RwLock<External>>> = self
            .external
            .read()
            .unwrap()
            .values()
            .map(|external| external.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Isa`] into the store.
    ///
    pub fn inter_isa(&mut self, isa: Arc<RwLock<Isa>>) {
        let read = isa.read().unwrap();
        self.isa.write().unwrap().insert(read.id, isa.clone());
    }

    /// Exhume (get) [`Isa`] from the store.
    ///
    pub fn exhume_isa(&self, id: &Uuid) -> Option<Arc<RwLock<Isa>>> {
        self.isa.read().unwrap().get(id).map(|isa| isa.clone())
    }

    /// Exorcise (remove) [`Isa`] from the store.
    ///
    pub fn exorcise_isa(&mut self, id: &Uuid) -> Option<Arc<RwLock<Isa>>> {
        self.isa.write().unwrap().remove(id).map(|isa| isa.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Isa>`.
    ///
    pub fn iter_isa(&self) -> impl Iterator<Item = Arc<RwLock<Isa>>> + '_ {
        let values: Vec<Arc<RwLock<Isa>>> = self
            .isa
            .read()
            .unwrap()
            .values()
            .map(|isa| isa.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Object`] into the store.
    ///
    pub fn inter_object(&mut self, object: Arc<RwLock<Object>>) {
        let read = object.read().unwrap();
        self.object_id_by_name
            .write()
            .unwrap()
            .insert(read.name.to_upper_camel_case(), read.id);
        self.object.write().unwrap().insert(read.id, object.clone());
    }

    /// Exhume (get) [`Object`] from the store.
    ///
    pub fn exhume_object(&self, id: &Uuid) -> Option<Arc<RwLock<Object>>> {
        self.object
            .read()
            .unwrap()
            .get(id)
            .map(|object| object.clone())
    }

    /// Exorcise (remove) [`Object`] from the store.
    ///
    pub fn exorcise_object(&mut self, id: &Uuid) -> Option<Arc<RwLock<Object>>> {
        self.object
            .write()
            .unwrap()
            .remove(id)
            .map(|object| object.clone())
    }

    /// Exhume [`Object`] id from the store by name.
    ///
    pub fn exhume_object_id_by_name(&self, name: &str) -> Option<Uuid> {
        self.object_id_by_name
            .read()
            .unwrap()
            .get(name)
            .map(|object| *object)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Object>`.
    ///
    pub fn iter_object(&self) -> impl Iterator<Item = Arc<RwLock<Object>>> + '_ {
        let values: Vec<Arc<RwLock<Object>>> = self
            .object
            .read()
            .unwrap()
            .values()
            .map(|object| object.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Referent`] into the store.
    ///
    pub fn inter_referent(&mut self, referent: Arc<RwLock<Referent>>) {
        let read = referent.read().unwrap();
        self.referent
            .write()
            .unwrap()
            .insert(read.id, referent.clone());
    }

    /// Exhume (get) [`Referent`] from the store.
    ///
    pub fn exhume_referent(&self, id: &Uuid) -> Option<Arc<RwLock<Referent>>> {
        self.referent
            .read()
            .unwrap()
            .get(id)
            .map(|referent| referent.clone())
    }

    /// Exorcise (remove) [`Referent`] from the store.
    ///
    pub fn exorcise_referent(&mut self, id: &Uuid) -> Option<Arc<RwLock<Referent>>> {
        self.referent
            .write()
            .unwrap()
            .remove(id)
            .map(|referent| referent.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Referent>`.
    ///
    pub fn iter_referent(&self) -> impl Iterator<Item = Arc<RwLock<Referent>>> + '_ {
        let values: Vec<Arc<RwLock<Referent>>> = self
            .referent
            .read()
            .unwrap()
            .values()
            .map(|referent| referent.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Referrer`] into the store.
    ///
    pub fn inter_referrer(&mut self, referrer: Arc<RwLock<Referrer>>) {
        let read = referrer.read().unwrap();
        self.referrer
            .write()
            .unwrap()
            .insert(read.id, referrer.clone());
    }

    /// Exhume (get) [`Referrer`] from the store.
    ///
    pub fn exhume_referrer(&self, id: &Uuid) -> Option<Arc<RwLock<Referrer>>> {
        self.referrer
            .read()
            .unwrap()
            .get(id)
            .map(|referrer| referrer.clone())
    }

    /// Exorcise (remove) [`Referrer`] from the store.
    ///
    pub fn exorcise_referrer(&mut self, id: &Uuid) -> Option<Arc<RwLock<Referrer>>> {
        self.referrer
            .write()
            .unwrap()
            .remove(id)
            .map(|referrer| referrer.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Referrer>`.
    ///
    pub fn iter_referrer(&self) -> impl Iterator<Item = Arc<RwLock<Referrer>>> + '_ {
        let values: Vec<Arc<RwLock<Referrer>>> = self
            .referrer
            .read()
            .unwrap()
            .values()
            .map(|referrer| referrer.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Relationship`] into the store.
    ///
    pub fn inter_relationship(&mut self, relationship: Arc<RwLock<Relationship>>) {
        let read = relationship.read().unwrap();
        self.relationship
            .write()
            .unwrap()
            .insert(read.id(), relationship.clone());
    }

    /// Exhume (get) [`Relationship`] from the store.
    ///
    pub fn exhume_relationship(&self, id: &Uuid) -> Option<Arc<RwLock<Relationship>>> {
        self.relationship
            .read()
            .unwrap()
            .get(id)
            .map(|relationship| relationship.clone())
    }

    /// Exorcise (remove) [`Relationship`] from the store.
    ///
    pub fn exorcise_relationship(&mut self, id: &Uuid) -> Option<Arc<RwLock<Relationship>>> {
        self.relationship
            .write()
            .unwrap()
            .remove(id)
            .map(|relationship| relationship.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Relationship>`.
    ///
    pub fn iter_relationship(&self) -> impl Iterator<Item = Arc<RwLock<Relationship>>> + '_ {
        let values: Vec<Arc<RwLock<Relationship>>> = self
            .relationship
            .read()
            .unwrap()
            .values()
            .map(|relationship| relationship.clone())
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

    /// Inter (insert) [`Subtype`] into the store.
    ///
    pub fn inter_subtype(&mut self, subtype: Arc<RwLock<Subtype>>) {
        let read = subtype.read().unwrap();
        self.subtype
            .write()
            .unwrap()
            .insert(read.id, subtype.clone());
    }

    /// Exhume (get) [`Subtype`] from the store.
    ///
    pub fn exhume_subtype(&self, id: &Uuid) -> Option<Arc<RwLock<Subtype>>> {
        self.subtype
            .read()
            .unwrap()
            .get(id)
            .map(|subtype| subtype.clone())
    }

    /// Exorcise (remove) [`Subtype`] from the store.
    ///
    pub fn exorcise_subtype(&mut self, id: &Uuid) -> Option<Arc<RwLock<Subtype>>> {
        self.subtype
            .write()
            .unwrap()
            .remove(id)
            .map(|subtype| subtype.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Subtype>`.
    ///
    pub fn iter_subtype(&self) -> impl Iterator<Item = Arc<RwLock<Subtype>>> + '_ {
        let values: Vec<Arc<RwLock<Subtype>>> = self
            .subtype
            .read()
            .unwrap()
            .values()
            .map(|subtype| subtype.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Supertype`] into the store.
    ///
    pub fn inter_supertype(&mut self, supertype: Arc<RwLock<Supertype>>) {
        let read = supertype.read().unwrap();
        self.supertype
            .write()
            .unwrap()
            .insert(read.id, supertype.clone());
    }

    /// Exhume (get) [`Supertype`] from the store.
    ///
    pub fn exhume_supertype(&self, id: &Uuid) -> Option<Arc<RwLock<Supertype>>> {
        self.supertype
            .read()
            .unwrap()
            .get(id)
            .map(|supertype| supertype.clone())
    }

    /// Exorcise (remove) [`Supertype`] from the store.
    ///
    pub fn exorcise_supertype(&mut self, id: &Uuid) -> Option<Arc<RwLock<Supertype>>> {
        self.supertype
            .write()
            .unwrap()
            .remove(id)
            .map(|supertype| supertype.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Supertype>`.
    ///
    pub fn iter_supertype(&self) -> impl Iterator<Item = Arc<RwLock<Supertype>>> + '_ {
        let values: Vec<Arc<RwLock<Supertype>>> = self
            .supertype
            .read()
            .unwrap()
            .values()
            .map(|supertype| supertype.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Ty`] into the store.
    ///
    pub fn inter_ty(&mut self, ty: Arc<RwLock<Ty>>) {
        let read = ty.read().unwrap();
        self.ty.write().unwrap().insert(read.id(), ty.clone());
    }

    /// Exhume (get) [`Ty`] from the store.
    ///
    pub fn exhume_ty(&self, id: &Uuid) -> Option<Arc<RwLock<Ty>>> {
        self.ty.read().unwrap().get(id).map(|ty| ty.clone())
    }

    /// Exorcise (remove) [`Ty`] from the store.
    ///
    pub fn exorcise_ty(&mut self, id: &Uuid) -> Option<Arc<RwLock<Ty>>> {
        self.ty.write().unwrap().remove(id).map(|ty| ty.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Ty>`.
    ///
    pub fn iter_ty(&self) -> impl Iterator<Item = Arc<RwLock<Ty>>> + '_ {
        let values: Vec<Arc<RwLock<Ty>>> = self
            .ty
            .read()
            .unwrap()
            .values()
            .map(|ty| ty.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::sarzak_rwlock-object-store-persistence"}}}
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
            for acknowledged_event in self.acknowledged_event.read().unwrap().values() {
                let path = path.join(format!("{}.json", acknowledged_event.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &acknowledged_event)?;
            }
        }

        // Persist An Associative Referent.
        {
            let path = path.join("an_associative_referent");
            fs::create_dir_all(&path)?;
            for an_associative_referent in self.an_associative_referent.read().unwrap().values() {
                let path = path.join(format!(
                    "{}.json",
                    an_associative_referent.read().unwrap().id
                ));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &an_associative_referent)?;
            }
        }

        // Persist Associative.
        {
            let path = path.join("associative");
            fs::create_dir_all(&path)?;
            for associative in self.associative.read().unwrap().values() {
                let path = path.join(format!("{}.json", associative.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &associative)?;
            }
        }

        // Persist Associative Referent.
        {
            let path = path.join("associative_referent");
            fs::create_dir_all(&path)?;
            for associative_referent in self.associative_referent.read().unwrap().values() {
                let path = path.join(format!("{}.json", associative_referent.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &associative_referent)?;
            }
        }

        // Persist Associative Referrer.
        {
            let path = path.join("associative_referrer");
            fs::create_dir_all(&path)?;
            for associative_referrer in self.associative_referrer.read().unwrap().values() {
                let path = path.join(format!("{}.json", associative_referrer.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &associative_referrer)?;
            }
        }

        // Persist Attribute.
        {
            let path = path.join("attribute");
            fs::create_dir_all(&path)?;
            for attribute in self.attribute.read().unwrap().values() {
                let path = path.join(format!("{}.json", attribute.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &attribute)?;
            }
        }

        // Persist Binary.
        {
            let path = path.join("binary");
            fs::create_dir_all(&path)?;
            for binary in self.binary.read().unwrap().values() {
                let path = path.join(format!("{}.json", binary.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &binary)?;
            }
        }

        // Persist Cardinality.
        {
            let path = path.join("cardinality");
            fs::create_dir_all(&path)?;
            for cardinality in self.cardinality.read().unwrap().values() {
                let path = path.join(format!("{}.json", cardinality.read().unwrap().id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &cardinality)?;
            }
        }

        // Persist Conditionality.
        {
            let path = path.join("conditionality");
            fs::create_dir_all(&path)?;
            for conditionality in self.conditionality.read().unwrap().values() {
                let path = path.join(format!("{}.json", conditionality.read().unwrap().id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &conditionality)?;
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

        // Persist External.
        {
            let path = path.join("external");
            fs::create_dir_all(&path)?;
            for external in self.external.read().unwrap().values() {
                let path = path.join(format!("{}.json", external.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &external)?;
            }
        }

        // Persist Isa.
        {
            let path = path.join("isa");
            fs::create_dir_all(&path)?;
            for isa in self.isa.read().unwrap().values() {
                let path = path.join(format!("{}.json", isa.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &isa)?;
            }
        }

        // Persist Object.
        {
            let path = path.join("object");
            fs::create_dir_all(&path)?;
            for object in self.object.read().unwrap().values() {
                let path = path.join(format!("{}.json", object.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &object)?;
            }
        }

        // Persist Referent.
        {
            let path = path.join("referent");
            fs::create_dir_all(&path)?;
            for referent in self.referent.read().unwrap().values() {
                let path = path.join(format!("{}.json", referent.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &referent)?;
            }
        }

        // Persist Referrer.
        {
            let path = path.join("referrer");
            fs::create_dir_all(&path)?;
            for referrer in self.referrer.read().unwrap().values() {
                let path = path.join(format!("{}.json", referrer.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &referrer)?;
            }
        }

        // Persist Relationship.
        {
            let path = path.join("relationship");
            fs::create_dir_all(&path)?;
            for relationship in self.relationship.read().unwrap().values() {
                let path = path.join(format!("{}.json", relationship.read().unwrap().id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &relationship)?;
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

        // Persist Subtype.
        {
            let path = path.join("subtype");
            fs::create_dir_all(&path)?;
            for subtype in self.subtype.read().unwrap().values() {
                let path = path.join(format!("{}.json", subtype.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &subtype)?;
            }
        }

        // Persist Supertype.
        {
            let path = path.join("supertype");
            fs::create_dir_all(&path)?;
            for supertype in self.supertype.read().unwrap().values() {
                let path = path.join(format!("{}.json", supertype.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &supertype)?;
            }
        }

        // Persist Type.
        {
            let path = path.join("ty");
            fs::create_dir_all(&path)?;
            for ty in self.ty.read().unwrap().values() {
                let path = path.join(format!("{}.json", ty.read().unwrap().id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &ty)?;
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

        // Load An Associative Referent.
        {
            let path = path.join("an_associative_referent");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let an_associative_referent: Arc<RwLock<AnAssociativeReferent>> =
                    serde_json::from_reader(reader)?;
                store.an_associative_referent.write().unwrap().insert(
                    an_associative_referent.read().unwrap().id,
                    an_associative_referent.clone(),
                );
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
                let associative: Arc<RwLock<Associative>> = serde_json::from_reader(reader)?;
                store
                    .associative
                    .write()
                    .unwrap()
                    .insert(associative.read().unwrap().id, associative.clone());
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
                let associative_referent: Arc<RwLock<AssociativeReferent>> =
                    serde_json::from_reader(reader)?;
                store.associative_referent.write().unwrap().insert(
                    associative_referent.read().unwrap().id,
                    associative_referent.clone(),
                );
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
                let associative_referrer: Arc<RwLock<AssociativeReferrer>> =
                    serde_json::from_reader(reader)?;
                store.associative_referrer.write().unwrap().insert(
                    associative_referrer.read().unwrap().id,
                    associative_referrer.clone(),
                );
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
                let attribute: Arc<RwLock<Attribute>> = serde_json::from_reader(reader)?;
                store
                    .attribute
                    .write()
                    .unwrap()
                    .insert(attribute.read().unwrap().id, attribute.clone());
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
                let binary: Arc<RwLock<Binary>> = serde_json::from_reader(reader)?;
                store
                    .binary
                    .write()
                    .unwrap()
                    .insert(binary.read().unwrap().id, binary.clone());
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
                let cardinality: Arc<RwLock<Cardinality>> = serde_json::from_reader(reader)?;
                store
                    .cardinality
                    .write()
                    .unwrap()
                    .insert(cardinality.read().unwrap().id(), cardinality.clone());
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
                let conditionality: Arc<RwLock<Conditionality>> = serde_json::from_reader(reader)?;
                store
                    .conditionality
                    .write()
                    .unwrap()
                    .insert(conditionality.read().unwrap().id(), conditionality.clone());
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

        // Load External.
        {
            let path = path.join("external");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let external: Arc<RwLock<External>> = serde_json::from_reader(reader)?;
                store
                    .external
                    .write()
                    .unwrap()
                    .insert(external.read().unwrap().id, external.clone());
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
                let isa: Arc<RwLock<Isa>> = serde_json::from_reader(reader)?;
                store
                    .isa
                    .write()
                    .unwrap()
                    .insert(isa.read().unwrap().id, isa.clone());
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
                let object: Arc<RwLock<Object>> = serde_json::from_reader(reader)?;
                store.object_id_by_name.write().unwrap().insert(
                    object.read().unwrap().name.to_upper_camel_case(),
                    object.read().unwrap().id,
                );
                store
                    .object
                    .write()
                    .unwrap()
                    .insert(object.read().unwrap().id, object.clone());
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
                let referent: Arc<RwLock<Referent>> = serde_json::from_reader(reader)?;
                store
                    .referent
                    .write()
                    .unwrap()
                    .insert(referent.read().unwrap().id, referent.clone());
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
                let referrer: Arc<RwLock<Referrer>> = serde_json::from_reader(reader)?;
                store
                    .referrer
                    .write()
                    .unwrap()
                    .insert(referrer.read().unwrap().id, referrer.clone());
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
                let relationship: Arc<RwLock<Relationship>> = serde_json::from_reader(reader)?;
                store
                    .relationship
                    .write()
                    .unwrap()
                    .insert(relationship.read().unwrap().id(), relationship.clone());
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

        // Load Subtype.
        {
            let path = path.join("subtype");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let subtype: Arc<RwLock<Subtype>> = serde_json::from_reader(reader)?;
                store
                    .subtype
                    .write()
                    .unwrap()
                    .insert(subtype.read().unwrap().id, subtype.clone());
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
                let supertype: Arc<RwLock<Supertype>> = serde_json::from_reader(reader)?;
                store
                    .supertype
                    .write()
                    .unwrap()
                    .insert(supertype.read().unwrap().id, supertype.clone());
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
                let ty: Arc<RwLock<Ty>> = serde_json::from_reader(reader)?;
                store
                    .ty
                    .write()
                    .unwrap()
                    .insert(ty.read().unwrap().id(), ty.clone());
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

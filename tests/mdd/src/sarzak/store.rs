//! sarzak Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"sarzak-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`AcknowledgedEvent`]
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"sarzak-object-store-definition"}}}
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::sarzak::types::{
    AcknowledgedEvent, Associative, AssociativeReferent, AssociativeReferrer, Attribute, Binary,
    Cardinality, Conditionality, Event, External, Isa, Object, Referent, Referrer, Relationship,
    State, Subtype, Supertype, Ty,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    acknowledged_event: HashMap<Uuid, AcknowledgedEvent>,
    associative: HashMap<Uuid, Associative>,
    associative_referent: HashMap<Uuid, AssociativeReferent>,
    associative_referrer: HashMap<Uuid, AssociativeReferrer>,
    attribute: HashMap<Uuid, Attribute>,
    binary: HashMap<Uuid, Binary>,
    cardinality: HashMap<Uuid, Cardinality>,
    conditionality: HashMap<Uuid, Conditionality>,
    event: HashMap<Uuid, Event>,
    external: HashMap<Uuid, External>,
    isa: HashMap<Uuid, Isa>,
    object: HashMap<Uuid, Object>,
    referent: HashMap<Uuid, Referent>,
    referrer: HashMap<Uuid, Referrer>,
    relationship: HashMap<Uuid, Relationship>,
    state: HashMap<Uuid, State>,
    subtype: HashMap<Uuid, Subtype>,
    supertype: HashMap<Uuid, Supertype>,
    ty: HashMap<Uuid, Ty>,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            acknowledged_event: HashMap::new(),
            associative: HashMap::new(),
            associative_referent: HashMap::new(),
            associative_referrer: HashMap::new(),
            attribute: HashMap::new(),
            binary: HashMap::new(),
            cardinality: HashMap::new(),
            conditionality: HashMap::new(),
            event: HashMap::new(),
            external: HashMap::new(),
            isa: HashMap::new(),
            object: HashMap::new(),
            referent: HashMap::new(),
            referrer: HashMap::new(),
            relationship: HashMap::new(),
            state: HashMap::new(),
            subtype: HashMap::new(),
            supertype: HashMap::new(),
            ty: HashMap::new(),
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
    /// Inter [`Associative`] into the store.
    ///
    pub fn inter_associative(&mut self, associative: Associative) {
        self.associative.insert(associative.id, associative);
    }

    /// Exhume [`Associative`] from the store.
    ///
    pub fn exhume_associative(&self, id: &Uuid) -> Option<&Associative> {
        self.associative.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Associative>`.
    //
    pub fn iter_associative(&self) -> impl Iterator<Item = (&Uuid, &Associative)> {
        self.associative.iter()
    }
    /// Inter [`AssociativeReferent`] into the store.
    ///
    pub fn inter_associative_referent(&mut self, associative_referent: AssociativeReferent) {
        self.associative_referent
            .insert(associative_referent.id, associative_referent);
    }

    /// Exhume [`AssociativeReferent`] from the store.
    ///
    pub fn exhume_associative_referent(&self, id: &Uuid) -> Option<&AssociativeReferent> {
        self.associative_referent.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, AssociativeReferent>`.
    //
    pub fn iter_associative_referent(&self) -> impl Iterator<Item = (&Uuid, &AssociativeReferent)> {
        self.associative_referent.iter()
    }
    /// Inter [`AssociativeReferrer`] into the store.
    ///
    pub fn inter_associative_referrer(&mut self, associative_referrer: AssociativeReferrer) {
        self.associative_referrer
            .insert(associative_referrer.id, associative_referrer);
    }

    /// Exhume [`AssociativeReferrer`] from the store.
    ///
    pub fn exhume_associative_referrer(&self, id: &Uuid) -> Option<&AssociativeReferrer> {
        self.associative_referrer.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, AssociativeReferrer>`.
    //
    pub fn iter_associative_referrer(&self) -> impl Iterator<Item = (&Uuid, &AssociativeReferrer)> {
        self.associative_referrer.iter()
    }
    /// Inter [`Attribute`] into the store.
    ///
    pub fn inter_attribute(&mut self, attribute: Attribute) {
        self.attribute.insert(attribute.id, attribute);
    }

    /// Exhume [`Attribute`] from the store.
    ///
    pub fn exhume_attribute(&self, id: &Uuid) -> Option<&Attribute> {
        self.attribute.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Attribute>`.
    //
    pub fn iter_attribute(&self) -> impl Iterator<Item = (&Uuid, &Attribute)> {
        self.attribute.iter()
    }
    /// Inter [`Binary`] into the store.
    ///
    pub fn inter_binary(&mut self, binary: Binary) {
        self.binary.insert(binary.id, binary);
    }

    /// Exhume [`Binary`] from the store.
    ///
    pub fn exhume_binary(&self, id: &Uuid) -> Option<&Binary> {
        self.binary.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Binary>`.
    //
    pub fn iter_binary(&self) -> impl Iterator<Item = (&Uuid, &Binary)> {
        self.binary.iter()
    }
    /// Inter [`Cardinality`] into the store.
    ///
    pub fn inter_cardinality(&mut self, cardinality: Cardinality) {
        self.cardinality.insert(cardinality.id(), cardinality);
    }

    /// Exhume [`Cardinality`] from the store.
    ///
    pub fn exhume_cardinality(&self, id: &Uuid) -> Option<&Cardinality> {
        self.cardinality.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Cardinality>`.
    //
    pub fn iter_cardinality(&self) -> impl Iterator<Item = (&Uuid, &Cardinality)> {
        self.cardinality.iter()
    }
    /// Inter [`Conditionality`] into the store.
    ///
    pub fn inter_conditionality(&mut self, conditionality: Conditionality) {
        self.conditionality
            .insert(conditionality.id(), conditionality);
    }

    /// Exhume [`Conditionality`] from the store.
    ///
    pub fn exhume_conditionality(&self, id: &Uuid) -> Option<&Conditionality> {
        self.conditionality.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Conditionality>`.
    //
    pub fn iter_conditionality(&self) -> impl Iterator<Item = (&Uuid, &Conditionality)> {
        self.conditionality.iter()
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
    /// Inter [`External`] into the store.
    ///
    pub fn inter_external(&mut self, external: External) {
        self.external.insert(external.id, external);
    }

    /// Exhume [`External`] from the store.
    ///
    pub fn exhume_external(&self, id: &Uuid) -> Option<&External> {
        self.external.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, External>`.
    //
    pub fn iter_external(&self) -> impl Iterator<Item = (&Uuid, &External)> {
        self.external.iter()
    }
    /// Inter [`Isa`] into the store.
    ///
    pub fn inter_isa(&mut self, isa: Isa) {
        self.isa.insert(isa.id, isa);
    }

    /// Exhume [`Isa`] from the store.
    ///
    pub fn exhume_isa(&self, id: &Uuid) -> Option<&Isa> {
        self.isa.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Isa>`.
    //
    pub fn iter_isa(&self) -> impl Iterator<Item = (&Uuid, &Isa)> {
        self.isa.iter()
    }
    /// Inter [`Object`] into the store.
    ///
    pub fn inter_object(&mut self, object: Object) {
        self.object.insert(object.id, object);
    }

    /// Exhume [`Object`] from the store.
    ///
    pub fn exhume_object(&self, id: &Uuid) -> Option<&Object> {
        self.object.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Object>`.
    //
    pub fn iter_object(&self) -> impl Iterator<Item = (&Uuid, &Object)> {
        self.object.iter()
    }
    /// Inter [`Referent`] into the store.
    ///
    pub fn inter_referent(&mut self, referent: Referent) {
        self.referent.insert(referent.id, referent);
    }

    /// Exhume [`Referent`] from the store.
    ///
    pub fn exhume_referent(&self, id: &Uuid) -> Option<&Referent> {
        self.referent.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Referent>`.
    //
    pub fn iter_referent(&self) -> impl Iterator<Item = (&Uuid, &Referent)> {
        self.referent.iter()
    }
    /// Inter [`Referrer`] into the store.
    ///
    pub fn inter_referrer(&mut self, referrer: Referrer) {
        self.referrer.insert(referrer.id, referrer);
    }

    /// Exhume [`Referrer`] from the store.
    ///
    pub fn exhume_referrer(&self, id: &Uuid) -> Option<&Referrer> {
        self.referrer.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Referrer>`.
    //
    pub fn iter_referrer(&self) -> impl Iterator<Item = (&Uuid, &Referrer)> {
        self.referrer.iter()
    }
    /// Inter [`Relationship`] into the store.
    ///
    pub fn inter_relationship(&mut self, relationship: Relationship) {
        self.relationship.insert(relationship.id(), relationship);
    }

    /// Exhume [`Relationship`] from the store.
    ///
    pub fn exhume_relationship(&self, id: &Uuid) -> Option<&Relationship> {
        self.relationship.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Relationship>`.
    //
    pub fn iter_relationship(&self) -> impl Iterator<Item = (&Uuid, &Relationship)> {
        self.relationship.iter()
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
    /// Inter [`Subtype`] into the store.
    ///
    pub fn inter_subtype(&mut self, subtype: Subtype) {
        self.subtype.insert(subtype.id, subtype);
    }

    /// Exhume [`Subtype`] from the store.
    ///
    pub fn exhume_subtype(&self, id: &Uuid) -> Option<&Subtype> {
        self.subtype.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Subtype>`.
    //
    pub fn iter_subtype(&self) -> impl Iterator<Item = (&Uuid, &Subtype)> {
        self.subtype.iter()
    }
    /// Inter [`Supertype`] into the store.
    ///
    pub fn inter_supertype(&mut self, supertype: Supertype) {
        self.supertype.insert(supertype.id, supertype);
    }

    /// Exhume [`Supertype`] from the store.
    ///
    pub fn exhume_supertype(&self, id: &Uuid) -> Option<&Supertype> {
        self.supertype.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Supertype>`.
    //
    pub fn iter_supertype(&self) -> impl Iterator<Item = (&Uuid, &Supertype)> {
        self.supertype.iter()
    }
    /// Inter [`Ty`] into the store.
    ///
    pub fn inter_ty(&mut self, ty: Ty) {
        self.ty.insert(ty.id(), ty);
    }

    /// Exhume [`Ty`] from the store.
    ///
    pub fn exhume_ty(&self, id: &Uuid) -> Option<&Ty> {
        self.ty.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Ty>`.
    //
    pub fn iter_ty(&self) -> impl Iterator<Item = (&Uuid, &Ty)> {
        self.ty.iter()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

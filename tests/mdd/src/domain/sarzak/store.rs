//! domain::sarzak Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::sarzak-object-store-file"}}}
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::sarzak-object-store-definition"}}}
use std::collections::HashMap;
use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::sarzak::types::{
    AcknowledgedEvent, Associative, AssociativeReferent, AssociativeReferrer, Attribute, Binary,
    Cardinality, Conditionality, Event, External, Isa, Object, Referent, Referrer, Relationship,
    State, Subtype, Supertype, Ty, BOOLEAN, CONDITIONAL, FLOAT, INTEGER, MANY, ONE, STRING,
    UNCONDITIONAL, UUID,
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
        let mut store = Self {
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

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::sarzak-object-store-methods"}}}
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
    /// Exhume [`Associative`] from the store — mutably.
    ///
    pub fn exhume_associative_mut(&mut self, id: &Uuid) -> Option<&mut Associative> {
        self.associative.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Associative>`.
    ///
    pub fn iter_associative(&self) -> impl Iterator<Item = &Associative> {
        self.associative.values()
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
    /// Exhume [`AssociativeReferent`] from the store — mutably.
    ///
    pub fn exhume_associative_referent_mut(
        &mut self,
        id: &Uuid,
    ) -> Option<&mut AssociativeReferent> {
        self.associative_referent.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, AssociativeReferent>`.
    ///
    pub fn iter_associative_referent(&self) -> impl Iterator<Item = &AssociativeReferent> {
        self.associative_referent.values()
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
    /// Exhume [`AssociativeReferrer`] from the store — mutably.
    ///
    pub fn exhume_associative_referrer_mut(
        &mut self,
        id: &Uuid,
    ) -> Option<&mut AssociativeReferrer> {
        self.associative_referrer.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, AssociativeReferrer>`.
    ///
    pub fn iter_associative_referrer(&self) -> impl Iterator<Item = &AssociativeReferrer> {
        self.associative_referrer.values()
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
    /// Exhume [`Attribute`] from the store — mutably.
    ///
    pub fn exhume_attribute_mut(&mut self, id: &Uuid) -> Option<&mut Attribute> {
        self.attribute.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Attribute>`.
    ///
    pub fn iter_attribute(&self) -> impl Iterator<Item = &Attribute> {
        self.attribute.values()
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
    /// Exhume [`Binary`] from the store — mutably.
    ///
    pub fn exhume_binary_mut(&mut self, id: &Uuid) -> Option<&mut Binary> {
        self.binary.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Binary>`.
    ///
    pub fn iter_binary(&self) -> impl Iterator<Item = &Binary> {
        self.binary.values()
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
    /// Exhume [`Cardinality`] from the store — mutably.
    ///
    pub fn exhume_cardinality_mut(&mut self, id: &Uuid) -> Option<&mut Cardinality> {
        self.cardinality.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Cardinality>`.
    ///
    pub fn iter_cardinality(&self) -> impl Iterator<Item = &Cardinality> {
        self.cardinality.values()
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
    /// Exhume [`Conditionality`] from the store — mutably.
    ///
    pub fn exhume_conditionality_mut(&mut self, id: &Uuid) -> Option<&mut Conditionality> {
        self.conditionality.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Conditionality>`.
    ///
    pub fn iter_conditionality(&self) -> impl Iterator<Item = &Conditionality> {
        self.conditionality.values()
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
    /// Exhume [`External`] from the store — mutably.
    ///
    pub fn exhume_external_mut(&mut self, id: &Uuid) -> Option<&mut External> {
        self.external.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, External>`.
    ///
    pub fn iter_external(&self) -> impl Iterator<Item = &External> {
        self.external.values()
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
    /// Exhume [`Isa`] from the store — mutably.
    ///
    pub fn exhume_isa_mut(&mut self, id: &Uuid) -> Option<&mut Isa> {
        self.isa.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Isa>`.
    ///
    pub fn iter_isa(&self) -> impl Iterator<Item = &Isa> {
        self.isa.values()
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
    /// Exhume [`Object`] from the store — mutably.
    ///
    pub fn exhume_object_mut(&mut self, id: &Uuid) -> Option<&mut Object> {
        self.object.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Object>`.
    ///
    pub fn iter_object(&self) -> impl Iterator<Item = &Object> {
        self.object.values()
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
    /// Exhume [`Referent`] from the store — mutably.
    ///
    pub fn exhume_referent_mut(&mut self, id: &Uuid) -> Option<&mut Referent> {
        self.referent.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Referent>`.
    ///
    pub fn iter_referent(&self) -> impl Iterator<Item = &Referent> {
        self.referent.values()
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
    /// Exhume [`Referrer`] from the store — mutably.
    ///
    pub fn exhume_referrer_mut(&mut self, id: &Uuid) -> Option<&mut Referrer> {
        self.referrer.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Referrer>`.
    ///
    pub fn iter_referrer(&self) -> impl Iterator<Item = &Referrer> {
        self.referrer.values()
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
    /// Exhume [`Relationship`] from the store — mutably.
    ///
    pub fn exhume_relationship_mut(&mut self, id: &Uuid) -> Option<&mut Relationship> {
        self.relationship.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Relationship>`.
    ///
    pub fn iter_relationship(&self) -> impl Iterator<Item = &Relationship> {
        self.relationship.values()
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
    /// Exhume [`Subtype`] from the store — mutably.
    ///
    pub fn exhume_subtype_mut(&mut self, id: &Uuid) -> Option<&mut Subtype> {
        self.subtype.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Subtype>`.
    ///
    pub fn iter_subtype(&self) -> impl Iterator<Item = &Subtype> {
        self.subtype.values()
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
    /// Exhume [`Supertype`] from the store — mutably.
    ///
    pub fn exhume_supertype_mut(&mut self, id: &Uuid) -> Option<&mut Supertype> {
        self.supertype.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Supertype>`.
    ///
    pub fn iter_supertype(&self) -> impl Iterator<Item = &Supertype> {
        self.supertype.values()
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
    /// Exhume [`Ty`] from the store — mutably.
    ///
    pub fn exhume_ty_mut(&mut self, id: &Uuid) -> Option<&mut Ty> {
        self.ty.get_mut(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Ty>`.
    ///
    pub fn iter_ty(&self) -> impl Iterator<Item = &Ty> {
        self.ty.values()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::sarzak-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let path = path.join("sarzak.json");
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
        // Persist Associative.
        {
            let path = path.join("associative.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.associative.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Associative Referent.
        {
            let path = path.join("associative_referent.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self
                    .associative_referent
                    .values()
                    .map(|x| x)
                    .collect::<Vec<_>>(),
            )?;
        }
        // Persist Associative Referrer.
        {
            let path = path.join("associative_referrer.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self
                    .associative_referrer
                    .values()
                    .map(|x| x)
                    .collect::<Vec<_>>(),
            )?;
        }
        // Persist Attribute.
        {
            let path = path.join("attribute.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.attribute.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Binary.
        {
            let path = path.join("binary.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.binary.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Cardinality.
        {
            let path = path.join("cardinality.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.cardinality.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Conditionality.
        {
            let path = path.join("conditionality.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.conditionality.values().map(|x| x).collect::<Vec<_>>(),
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
        // Persist External.
        {
            let path = path.join("external.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.external.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Isa.
        {
            let path = path.join("isa.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.isa.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Object.
        {
            let path = path.join("object.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.object.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Referent.
        {
            let path = path.join("referent.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.referent.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Referrer.
        {
            let path = path.join("referrer.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.referrer.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Relationship.
        {
            let path = path.join("relationship.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.relationship.values().map(|x| x).collect::<Vec<_>>(),
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
        // Persist Subtype.
        {
            let path = path.join("subtype.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.subtype.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Supertype.
        {
            let path = path.join("supertype.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.supertype.values().map(|x| x).collect::<Vec<_>>(),
            )?;
        }
        // Persist Type.
        {
            let path = path.join("ty.json");
            let file = fs::File::create(path)?;
            let mut writer = io::BufWriter::new(file);
            serde_json::to_writer_pretty(
                &mut writer,
                &self.ty.values().map(|x| x).collect::<Vec<_>>(),
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
        let path = path.join("sarzak.json");

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
        // Load Associative.
        {
            let path = path.join("associative.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let associative: Vec<Associative> = serde_json::from_reader(reader)?;
            store.associative = associative.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Associative Referent.
        {
            let path = path.join("associative_referent.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let associative_referent: Vec<AssociativeReferent> = serde_json::from_reader(reader)?;
            store.associative_referent = associative_referent
                .into_iter()
                .map(|道| (道.id, 道))
                .collect();
        }
        // Load Associative Referrer.
        {
            let path = path.join("associative_referrer.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let associative_referrer: Vec<AssociativeReferrer> = serde_json::from_reader(reader)?;
            store.associative_referrer = associative_referrer
                .into_iter()
                .map(|道| (道.id, 道))
                .collect();
        }
        // Load Attribute.
        {
            let path = path.join("attribute.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let attribute: Vec<Attribute> = serde_json::from_reader(reader)?;
            store.attribute = attribute.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Binary.
        {
            let path = path.join("binary.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let binary: Vec<Binary> = serde_json::from_reader(reader)?;
            store.binary = binary.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Cardinality.
        {
            let path = path.join("cardinality.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let cardinality: Vec<Cardinality> = serde_json::from_reader(reader)?;
            store.cardinality = cardinality.into_iter().map(|道| (道.id(), 道)).collect();
        }
        // Load Conditionality.
        {
            let path = path.join("conditionality.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let conditionality: Vec<Conditionality> = serde_json::from_reader(reader)?;
            store.conditionality = conditionality.into_iter().map(|道| (道.id(), 道)).collect();
        }
        // Load Event.
        {
            let path = path.join("event.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let event: Vec<Event> = serde_json::from_reader(reader)?;
            store.event = event.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load External.
        {
            let path = path.join("external.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let external: Vec<External> = serde_json::from_reader(reader)?;
            store.external = external.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Isa.
        {
            let path = path.join("isa.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let isa: Vec<Isa> = serde_json::from_reader(reader)?;
            store.isa = isa.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Object.
        {
            let path = path.join("object.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let object: Vec<Object> = serde_json::from_reader(reader)?;
            store.object = object.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Referent.
        {
            let path = path.join("referent.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let referent: Vec<Referent> = serde_json::from_reader(reader)?;
            store.referent = referent.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Referrer.
        {
            let path = path.join("referrer.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let referrer: Vec<Referrer> = serde_json::from_reader(reader)?;
            store.referrer = referrer.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Relationship.
        {
            let path = path.join("relationship.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let relationship: Vec<Relationship> = serde_json::from_reader(reader)?;
            store.relationship = relationship.into_iter().map(|道| (道.id(), 道)).collect();
        }
        // Load State.
        {
            let path = path.join("state.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let state: Vec<State> = serde_json::from_reader(reader)?;
            store.state = state.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Subtype.
        {
            let path = path.join("subtype.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let subtype: Vec<Subtype> = serde_json::from_reader(reader)?;
            store.subtype = subtype.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Supertype.
        {
            let path = path.join("supertype.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let supertype: Vec<Supertype> = serde_json::from_reader(reader)?;
            store.supertype = supertype.into_iter().map(|道| (道.id, 道)).collect();
        }
        // Load Type.
        {
            let path = path.join("ty.json");
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            let ty: Vec<Ty> = serde_json::from_reader(reader)?;
            store.ty = ty.into_iter().map(|道| (道.id(), 道)).collect();
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

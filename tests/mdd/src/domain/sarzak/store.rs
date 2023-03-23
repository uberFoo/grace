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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::sarzak-object-store-definition"}}}
use fnv::FnvHashMap as HashMap;
use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::sarzak::types::{
    AcknowledgedEvent, AnAssociativeReferent, Associative, AssociativeReferent,
    AssociativeReferrer, Attribute, Binary, Cardinality, Conditionality, Event, External, Isa,
    Object, Referent, Referrer, Relationship, State, Subtype, Supertype, Ty, BOOLEAN, CONDITIONAL,
    FLOAT, INTEGER, MANY, ONE, STRING, UNCONDITIONAL, UUID,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    acknowledged_event: HashMap<Uuid, AcknowledgedEvent>,
    an_associative_referent: HashMap<Uuid, AnAssociativeReferent>,
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
            referent: HashMap::default(),
            referrer: HashMap::default(),
            relationship: HashMap::default(),
            state: HashMap::default(),
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

    /// Inter [`AnAssociativeReferent`] into the store.
    ///
    pub fn inter_an_associative_referent(
        &mut self,
        an_associative_referent: AnAssociativeReferent,
    ) {
        self.an_associative_referent
            .insert(an_associative_referent.id, an_associative_referent);
    }

    /// Exhume [`AnAssociativeReferent`] from the store.
    ///
    pub fn exhume_an_associative_referent(&self, id: &Uuid) -> Option<&AnAssociativeReferent> {
        self.an_associative_referent.get(id)
    }

    /// Exhume [`AnAssociativeReferent`] from the store — mutably.
    ///
    pub fn exhume_an_associative_referent_mut(
        &mut self,
        id: &Uuid,
    ) -> Option<&mut AnAssociativeReferent> {
        self.an_associative_referent.get_mut(id)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AnAssociativeReferent>`.
    ///
    pub fn iter_an_associative_referent(&self) -> impl Iterator<Item = &AnAssociativeReferent> {
        self.an_associative_referent.values()
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
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        let path = path.join("sarzak.json");
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

        // Persist An Associative Referent.
        {
            let path = path.join("an_associative_referent");
            fs::create_dir_all(&path)?;
            for an_associative_referent in self.an_associative_referent.values() {
                let path = path.join(format!("{}.json", an_associative_referent.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &an_associative_referent)?;
            }
        }

        // Persist Associative.
        {
            let path = path.join("associative");
            fs::create_dir_all(&path)?;
            for associative in self.associative.values() {
                let path = path.join(format!("{}.json", associative.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &associative)?;
            }
        }

        // Persist Associative Referent.
        {
            let path = path.join("associative_referent");
            fs::create_dir_all(&path)?;
            for associative_referent in self.associative_referent.values() {
                let path = path.join(format!("{}.json", associative_referent.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &associative_referent)?;
            }
        }

        // Persist Associative Referrer.
        {
            let path = path.join("associative_referrer");
            fs::create_dir_all(&path)?;
            for associative_referrer in self.associative_referrer.values() {
                let path = path.join(format!("{}.json", associative_referrer.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &associative_referrer)?;
            }
        }

        // Persist Attribute.
        {
            let path = path.join("attribute");
            fs::create_dir_all(&path)?;
            for attribute in self.attribute.values() {
                let path = path.join(format!("{}.json", attribute.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &attribute)?;
            }
        }

        // Persist Binary.
        {
            let path = path.join("binary");
            fs::create_dir_all(&path)?;
            for binary in self.binary.values() {
                let path = path.join(format!("{}.json", binary.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &binary)?;
            }
        }

        // Persist Cardinality.
        {
            let path = path.join("cardinality");
            fs::create_dir_all(&path)?;
            for cardinality in self.cardinality.values() {
                let path = path.join(format!("{}.json", cardinality.id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &cardinality)?;
            }
        }

        // Persist Conditionality.
        {
            let path = path.join("conditionality");
            fs::create_dir_all(&path)?;
            for conditionality in self.conditionality.values() {
                let path = path.join(format!("{}.json", conditionality.id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &conditionality)?;
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

        // Persist External.
        {
            let path = path.join("external");
            fs::create_dir_all(&path)?;
            for external in self.external.values() {
                let path = path.join(format!("{}.json", external.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &external)?;
            }
        }

        // Persist Isa.
        {
            let path = path.join("isa");
            fs::create_dir_all(&path)?;
            for isa in self.isa.values() {
                let path = path.join(format!("{}.json", isa.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &isa)?;
            }
        }

        // Persist Object.
        {
            let path = path.join("object");
            fs::create_dir_all(&path)?;
            for object in self.object.values() {
                let path = path.join(format!("{}.json", object.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &object)?;
            }
        }

        // Persist Referent.
        {
            let path = path.join("referent");
            fs::create_dir_all(&path)?;
            for referent in self.referent.values() {
                let path = path.join(format!("{}.json", referent.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &referent)?;
            }
        }

        // Persist Referrer.
        {
            let path = path.join("referrer");
            fs::create_dir_all(&path)?;
            for referrer in self.referrer.values() {
                let path = path.join(format!("{}.json", referrer.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &referrer)?;
            }
        }

        // Persist Relationship.
        {
            let path = path.join("relationship");
            fs::create_dir_all(&path)?;
            for relationship in self.relationship.values() {
                let path = path.join(format!("{}.json", relationship.id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &relationship)?;
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

        // Persist Subtype.
        {
            let path = path.join("subtype");
            fs::create_dir_all(&path)?;
            for subtype in self.subtype.values() {
                let path = path.join(format!("{}.json", subtype.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &subtype)?;
            }
        }

        // Persist Supertype.
        {
            let path = path.join("supertype");
            fs::create_dir_all(&path)?;
            for supertype in self.supertype.values() {
                let path = path.join(format!("{}.json", supertype.id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &supertype)?;
            }
        }

        // Persist Type.
        {
            let path = path.join("ty");
            fs::create_dir_all(&path)?;
            for ty in self.ty.values() {
                let path = path.join(format!("{}.json", ty.id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &ty)?;
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
                let acknowledged_event: AcknowledgedEvent = serde_json::from_reader(reader)?;
                store
                    .acknowledged_event
                    .insert(acknowledged_event.id, acknowledged_event);
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
                let an_associative_referent: AnAssociativeReferent =
                    serde_json::from_reader(reader)?;
                store
                    .an_associative_referent
                    .insert(an_associative_referent.id, an_associative_referent);
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
                let associative: Associative = serde_json::from_reader(reader)?;
                store.associative.insert(associative.id, associative);
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
                let associative_referent: AssociativeReferent = serde_json::from_reader(reader)?;
                store
                    .associative_referent
                    .insert(associative_referent.id, associative_referent);
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
                let associative_referrer: AssociativeReferrer = serde_json::from_reader(reader)?;
                store
                    .associative_referrer
                    .insert(associative_referrer.id, associative_referrer);
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
                let attribute: Attribute = serde_json::from_reader(reader)?;
                store.attribute.insert(attribute.id, attribute);
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
                let binary: Binary = serde_json::from_reader(reader)?;
                store.binary.insert(binary.id, binary);
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
                let cardinality: Cardinality = serde_json::from_reader(reader)?;
                store.cardinality.insert(cardinality.id(), cardinality);
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
                let conditionality: Conditionality = serde_json::from_reader(reader)?;
                store
                    .conditionality
                    .insert(conditionality.id(), conditionality);
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
                store.event.insert(event.id, event);
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
                let external: External = serde_json::from_reader(reader)?;
                store.external.insert(external.id, external);
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
                let isa: Isa = serde_json::from_reader(reader)?;
                store.isa.insert(isa.id, isa);
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
                let object: Object = serde_json::from_reader(reader)?;
                store.object.insert(object.id, object);
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
                let referent: Referent = serde_json::from_reader(reader)?;
                store.referent.insert(referent.id, referent);
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
                let referrer: Referrer = serde_json::from_reader(reader)?;
                store.referrer.insert(referrer.id, referrer);
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
                let relationship: Relationship = serde_json::from_reader(reader)?;
                store.relationship.insert(relationship.id(), relationship);
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
                store.state.insert(state.id, state);
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
                let subtype: Subtype = serde_json::from_reader(reader)?;
                store.subtype.insert(subtype.id, subtype);
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
                let supertype: Supertype = serde_json::from_reader(reader)?;
                store.supertype.insert(supertype.id, supertype);
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
                let ty: Ty = serde_json::from_reader(reader)?;
                store.ty.insert(ty.id(), ty);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

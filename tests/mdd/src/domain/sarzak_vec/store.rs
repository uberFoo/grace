//! domain::sarzak_vec Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::sarzak_vec-object-store-file"}}}
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::sarzak_vec-object-store-definition"}}}
use std::cell::RefCell;
use std::rc::Rc;
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use fnv::FnvHashMap as HashMap;
use heck::ToUpperCamelCase;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::sarzak_vec::types::{
    AcknowledgedEvent, AnAssociativeReferent, Associative, AssociativeReferent,
    AssociativeReferrer, Attribute, Binary, Cardinality, Conditionality, Event, External, Isa,
    Object, Referent, Referrer, Relationship, State, Subtype, Supertype, Ty, BOOLEAN, CONDITIONAL,
    FLOAT, INTEGER, MANY, ONE, S_STRING, S_UUID, UNCONDITIONAL,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    acknowledged_event_free_list: std::sync::Mutex<Vec<usize>>,
    acknowledged_event: Vec<Option<Rc<RefCell<AcknowledgedEvent>>>>,
    an_associative_referent_free_list: std::sync::Mutex<Vec<usize>>,
    an_associative_referent: Vec<Option<Rc<RefCell<AnAssociativeReferent>>>>,
    associative_free_list: std::sync::Mutex<Vec<usize>>,
    associative: Vec<Option<Rc<RefCell<Associative>>>>,
    associative_referent_free_list: std::sync::Mutex<Vec<usize>>,
    associative_referent: Vec<Option<Rc<RefCell<AssociativeReferent>>>>,
    associative_referrer_free_list: std::sync::Mutex<Vec<usize>>,
    associative_referrer: Vec<Option<Rc<RefCell<AssociativeReferrer>>>>,
    attribute_free_list: std::sync::Mutex<Vec<usize>>,
    attribute: Vec<Option<Rc<RefCell<Attribute>>>>,
    binary_free_list: std::sync::Mutex<Vec<usize>>,
    binary: Vec<Option<Rc<RefCell<Binary>>>>,
    cardinality_free_list: std::sync::Mutex<Vec<usize>>,
    cardinality: Vec<Option<Rc<RefCell<Cardinality>>>>,
    conditionality_free_list: std::sync::Mutex<Vec<usize>>,
    conditionality: Vec<Option<Rc<RefCell<Conditionality>>>>,
    event_free_list: std::sync::Mutex<Vec<usize>>,
    event: Vec<Option<Rc<RefCell<Event>>>>,
    external_free_list: std::sync::Mutex<Vec<usize>>,
    external: Vec<Option<Rc<RefCell<External>>>>,
    isa_free_list: std::sync::Mutex<Vec<usize>>,
    isa: Vec<Option<Rc<RefCell<Isa>>>>,
    object_free_list: std::sync::Mutex<Vec<usize>>,
    object: Vec<Option<Rc<RefCell<Object>>>>,
    object_id_by_name: HashMap<String, usize>,
    referent_free_list: std::sync::Mutex<Vec<usize>>,
    referent: Vec<Option<Rc<RefCell<Referent>>>>,
    referrer_free_list: std::sync::Mutex<Vec<usize>>,
    referrer: Vec<Option<Rc<RefCell<Referrer>>>>,
    relationship_free_list: std::sync::Mutex<Vec<usize>>,
    relationship: Vec<Option<Rc<RefCell<Relationship>>>>,
    state_free_list: std::sync::Mutex<Vec<usize>>,
    state: Vec<Option<Rc<RefCell<State>>>>,
    subtype_free_list: std::sync::Mutex<Vec<usize>>,
    subtype: Vec<Option<Rc<RefCell<Subtype>>>>,
    supertype_free_list: std::sync::Mutex<Vec<usize>>,
    supertype: Vec<Option<Rc<RefCell<Supertype>>>>,
    ty_free_list: std::sync::Mutex<Vec<usize>>,
    ty: Vec<Option<Rc<RefCell<Ty>>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            acknowledged_event_free_list: std::sync::Mutex::new(Vec::new()),
            acknowledged_event: Vec::new(),
            an_associative_referent_free_list: std::sync::Mutex::new(Vec::new()),
            an_associative_referent: Vec::new(),
            associative_free_list: std::sync::Mutex::new(Vec::new()),
            associative: Vec::new(),
            associative_referent_free_list: std::sync::Mutex::new(Vec::new()),
            associative_referent: Vec::new(),
            associative_referrer_free_list: std::sync::Mutex::new(Vec::new()),
            associative_referrer: Vec::new(),
            attribute_free_list: std::sync::Mutex::new(Vec::new()),
            attribute: Vec::new(),
            binary_free_list: std::sync::Mutex::new(Vec::new()),
            binary: Vec::new(),
            cardinality_free_list: std::sync::Mutex::new(Vec::new()),
            cardinality: Vec::new(),
            conditionality_free_list: std::sync::Mutex::new(Vec::new()),
            conditionality: Vec::new(),
            event_free_list: std::sync::Mutex::new(Vec::new()),
            event: Vec::new(),
            external_free_list: std::sync::Mutex::new(Vec::new()),
            external: Vec::new(),
            isa_free_list: std::sync::Mutex::new(Vec::new()),
            isa: Vec::new(),
            object_free_list: std::sync::Mutex::new(Vec::new()),
            object: Vec::new(),
            object_id_by_name: HashMap::default(),
            referent_free_list: std::sync::Mutex::new(Vec::new()),
            referent: Vec::new(),
            referrer_free_list: std::sync::Mutex::new(Vec::new()),
            referrer: Vec::new(),
            relationship_free_list: std::sync::Mutex::new(Vec::new()),
            relationship: Vec::new(),
            state_free_list: std::sync::Mutex::new(Vec::new()),
            state: Vec::new(),
            subtype_free_list: std::sync::Mutex::new(Vec::new()),
            subtype: Vec::new(),
            supertype_free_list: std::sync::Mutex::new(Vec::new()),
            supertype: Vec::new(),
            ty_free_list: std::sync::Mutex::new(Vec::new()),
            ty: Vec::new(),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥
        store.inter_cardinality(|id| {
            Rc::new(RefCell::new(Cardinality {
                subtype: super::CardinalityEnum::Many(MANY),
                id,
            }))
        });
        store.inter_cardinality(|id| {
            Rc::new(RefCell::new(Cardinality {
                subtype: super::CardinalityEnum::One(ONE),
                id,
            }))
        });
        store.inter_conditionality(|id| {
            Rc::new(RefCell::new(Conditionality {
                subtype: super::ConditionalityEnum::Conditional(CONDITIONAL),
                id,
            }))
        });
        store.inter_conditionality(|id| {
            Rc::new(RefCell::new(Conditionality {
                subtype: super::ConditionalityEnum::Unconditional(UNCONDITIONAL),
                id,
            }))
        });
        store.inter_ty(|id| {
            Rc::new(RefCell::new(Ty {
                subtype: super::TyEnum::Boolean(BOOLEAN),
                id,
            }))
        });
        store.inter_ty(|id| {
            Rc::new(RefCell::new(Ty {
                subtype: super::TyEnum::Float(FLOAT),
                id,
            }))
        });
        store.inter_ty(|id| {
            Rc::new(RefCell::new(Ty {
                subtype: super::TyEnum::Integer(INTEGER),
                id,
            }))
        });
        store.inter_ty(|id| {
            Rc::new(RefCell::new(Ty {
                subtype: super::TyEnum::SString(S_STRING),
                id,
            }))
        });
        store.inter_ty(|id| {
            Rc::new(RefCell::new(Ty {
                subtype: super::TyEnum::SUuid(S_UUID),
                id,
            }))
        });

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::sarzak_vec-object-store-methods"}}}
    /// Inter (insert) [`AcknowledgedEvent`] into the store.
    ///
    pub fn inter_acknowledged_event<F>(
        &mut self,
        acknowledged_event: F,
    ) -> Rc<RefCell<AcknowledgedEvent>>
    where
        F: Fn(usize) -> Rc<RefCell<AcknowledgedEvent>>,
    {
        if let Some(_index) = self.acknowledged_event_free_list.lock().unwrap().pop() {
            let acknowledged_event = acknowledged_event(_index);
            self.acknowledged_event[_index] = Some(acknowledged_event.clone());
            acknowledged_event
        } else {
            let _index = self.acknowledged_event.len();
            let acknowledged_event = acknowledged_event(_index);
            self.acknowledged_event
                .push(Some(acknowledged_event.clone()));
            acknowledged_event
        }
    }

    /// Exhume (get) [`AcknowledgedEvent`] from the store.
    ///
    pub fn exhume_acknowledged_event(&self, id: &usize) -> Option<Rc<RefCell<AcknowledgedEvent>>> {
        match self.acknowledged_event.get(*id) {
            Some(acknowledged_event) => acknowledged_event.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`AcknowledgedEvent`] from the store.
    ///
    pub fn exorcise_acknowledged_event(
        &mut self,
        id: &usize,
    ) -> Option<Rc<RefCell<AcknowledgedEvent>>> {
        let result = self.acknowledged_event[*id].take();
        self.acknowledged_event_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AcknowledgedEvent>`.
    ///
    pub fn iter_acknowledged_event(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<AcknowledgedEvent>>> + '_ {
        let len = self.acknowledged_event.len();
        (0..len).map(move |i| {
            self.acknowledged_event[i]
                .as_ref()
                .map(|acknowledged_event| acknowledged_event.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`AnAssociativeReferent`] into the store.
    ///
    pub fn inter_an_associative_referent<F>(
        &mut self,
        an_associative_referent: F,
    ) -> Rc<RefCell<AnAssociativeReferent>>
    where
        F: Fn(usize) -> Rc<RefCell<AnAssociativeReferent>>,
    {
        if let Some(_index) = self.an_associative_referent_free_list.lock().unwrap().pop() {
            let an_associative_referent = an_associative_referent(_index);
            self.an_associative_referent[_index] = Some(an_associative_referent.clone());
            an_associative_referent
        } else {
            let _index = self.an_associative_referent.len();
            let an_associative_referent = an_associative_referent(_index);
            self.an_associative_referent
                .push(Some(an_associative_referent.clone()));
            an_associative_referent
        }
    }

    /// Exhume (get) [`AnAssociativeReferent`] from the store.
    ///
    pub fn exhume_an_associative_referent(
        &self,
        id: &usize,
    ) -> Option<Rc<RefCell<AnAssociativeReferent>>> {
        match self.an_associative_referent.get(*id) {
            Some(an_associative_referent) => an_associative_referent.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`AnAssociativeReferent`] from the store.
    ///
    pub fn exorcise_an_associative_referent(
        &mut self,
        id: &usize,
    ) -> Option<Rc<RefCell<AnAssociativeReferent>>> {
        let result = self.an_associative_referent[*id].take();
        self.an_associative_referent_free_list
            .lock()
            .unwrap()
            .push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AnAssociativeReferent>`.
    ///
    pub fn iter_an_associative_referent(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<AnAssociativeReferent>>> + '_ {
        let len = self.an_associative_referent.len();
        (0..len).map(move |i| {
            self.an_associative_referent[i]
                .as_ref()
                .map(|an_associative_referent| an_associative_referent.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Associative`] into the store.
    ///
    pub fn inter_associative<F>(&mut self, associative: F) -> Rc<RefCell<Associative>>
    where
        F: Fn(usize) -> Rc<RefCell<Associative>>,
    {
        if let Some(_index) = self.associative_free_list.lock().unwrap().pop() {
            let associative = associative(_index);
            self.associative[_index] = Some(associative.clone());
            associative
        } else {
            let _index = self.associative.len();
            let associative = associative(_index);
            self.associative.push(Some(associative.clone()));
            associative
        }
    }

    /// Exhume (get) [`Associative`] from the store.
    ///
    pub fn exhume_associative(&self, id: &usize) -> Option<Rc<RefCell<Associative>>> {
        match self.associative.get(*id) {
            Some(associative) => associative.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Associative`] from the store.
    ///
    pub fn exorcise_associative(&mut self, id: &usize) -> Option<Rc<RefCell<Associative>>> {
        let result = self.associative[*id].take();
        self.associative_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Associative>`.
    ///
    pub fn iter_associative(&self) -> impl Iterator<Item = Rc<RefCell<Associative>>> + '_ {
        let len = self.associative.len();
        (0..len).map(move |i| {
            self.associative[i]
                .as_ref()
                .map(|associative| associative.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`AssociativeReferent`] into the store.
    ///
    pub fn inter_associative_referent<F>(
        &mut self,
        associative_referent: F,
    ) -> Rc<RefCell<AssociativeReferent>>
    where
        F: Fn(usize) -> Rc<RefCell<AssociativeReferent>>,
    {
        if let Some(_index) = self.associative_referent_free_list.lock().unwrap().pop() {
            let associative_referent = associative_referent(_index);
            self.associative_referent[_index] = Some(associative_referent.clone());
            associative_referent
        } else {
            let _index = self.associative_referent.len();
            let associative_referent = associative_referent(_index);
            self.associative_referent
                .push(Some(associative_referent.clone()));
            associative_referent
        }
    }

    /// Exhume (get) [`AssociativeReferent`] from the store.
    ///
    pub fn exhume_associative_referent(
        &self,
        id: &usize,
    ) -> Option<Rc<RefCell<AssociativeReferent>>> {
        match self.associative_referent.get(*id) {
            Some(associative_referent) => associative_referent.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`AssociativeReferent`] from the store.
    ///
    pub fn exorcise_associative_referent(
        &mut self,
        id: &usize,
    ) -> Option<Rc<RefCell<AssociativeReferent>>> {
        let result = self.associative_referent[*id].take();
        self.associative_referent_free_list
            .lock()
            .unwrap()
            .push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AssociativeReferent>`.
    ///
    pub fn iter_associative_referent(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<AssociativeReferent>>> + '_ {
        let len = self.associative_referent.len();
        (0..len).map(move |i| {
            self.associative_referent[i]
                .as_ref()
                .map(|associative_referent| associative_referent.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`AssociativeReferrer`] into the store.
    ///
    pub fn inter_associative_referrer<F>(
        &mut self,
        associative_referrer: F,
    ) -> Rc<RefCell<AssociativeReferrer>>
    where
        F: Fn(usize) -> Rc<RefCell<AssociativeReferrer>>,
    {
        if let Some(_index) = self.associative_referrer_free_list.lock().unwrap().pop() {
            let associative_referrer = associative_referrer(_index);
            self.associative_referrer[_index] = Some(associative_referrer.clone());
            associative_referrer
        } else {
            let _index = self.associative_referrer.len();
            let associative_referrer = associative_referrer(_index);
            self.associative_referrer
                .push(Some(associative_referrer.clone()));
            associative_referrer
        }
    }

    /// Exhume (get) [`AssociativeReferrer`] from the store.
    ///
    pub fn exhume_associative_referrer(
        &self,
        id: &usize,
    ) -> Option<Rc<RefCell<AssociativeReferrer>>> {
        match self.associative_referrer.get(*id) {
            Some(associative_referrer) => associative_referrer.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`AssociativeReferrer`] from the store.
    ///
    pub fn exorcise_associative_referrer(
        &mut self,
        id: &usize,
    ) -> Option<Rc<RefCell<AssociativeReferrer>>> {
        let result = self.associative_referrer[*id].take();
        self.associative_referrer_free_list
            .lock()
            .unwrap()
            .push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AssociativeReferrer>`.
    ///
    pub fn iter_associative_referrer(
        &self,
    ) -> impl Iterator<Item = Rc<RefCell<AssociativeReferrer>>> + '_ {
        let len = self.associative_referrer.len();
        (0..len).map(move |i| {
            self.associative_referrer[i]
                .as_ref()
                .map(|associative_referrer| associative_referrer.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Attribute`] into the store.
    ///
    pub fn inter_attribute<F>(&mut self, attribute: F) -> Rc<RefCell<Attribute>>
    where
        F: Fn(usize) -> Rc<RefCell<Attribute>>,
    {
        if let Some(_index) = self.attribute_free_list.lock().unwrap().pop() {
            let attribute = attribute(_index);
            self.attribute[_index] = Some(attribute.clone());
            attribute
        } else {
            let _index = self.attribute.len();
            let attribute = attribute(_index);
            self.attribute.push(Some(attribute.clone()));
            attribute
        }
    }

    /// Exhume (get) [`Attribute`] from the store.
    ///
    pub fn exhume_attribute(&self, id: &usize) -> Option<Rc<RefCell<Attribute>>> {
        match self.attribute.get(*id) {
            Some(attribute) => attribute.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Attribute`] from the store.
    ///
    pub fn exorcise_attribute(&mut self, id: &usize) -> Option<Rc<RefCell<Attribute>>> {
        let result = self.attribute[*id].take();
        self.attribute_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Attribute>`.
    ///
    pub fn iter_attribute(&self) -> impl Iterator<Item = Rc<RefCell<Attribute>>> + '_ {
        let len = self.attribute.len();
        (0..len).map(move |i| {
            self.attribute[i]
                .as_ref()
                .map(|attribute| attribute.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Binary`] into the store.
    ///
    pub fn inter_binary<F>(&mut self, binary: F) -> Rc<RefCell<Binary>>
    where
        F: Fn(usize) -> Rc<RefCell<Binary>>,
    {
        if let Some(_index) = self.binary_free_list.lock().unwrap().pop() {
            let binary = binary(_index);
            self.binary[_index] = Some(binary.clone());
            binary
        } else {
            let _index = self.binary.len();
            let binary = binary(_index);
            self.binary.push(Some(binary.clone()));
            binary
        }
    }

    /// Exhume (get) [`Binary`] from the store.
    ///
    pub fn exhume_binary(&self, id: &usize) -> Option<Rc<RefCell<Binary>>> {
        match self.binary.get(*id) {
            Some(binary) => binary.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Binary`] from the store.
    ///
    pub fn exorcise_binary(&mut self, id: &usize) -> Option<Rc<RefCell<Binary>>> {
        let result = self.binary[*id].take();
        self.binary_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Binary>`.
    ///
    pub fn iter_binary(&self) -> impl Iterator<Item = Rc<RefCell<Binary>>> + '_ {
        let len = self.binary.len();
        (0..len).map(move |i| {
            self.binary[i]
                .as_ref()
                .map(|binary| binary.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Cardinality`] into the store.
    ///
    pub fn inter_cardinality<F>(&mut self, cardinality: F) -> Rc<RefCell<Cardinality>>
    where
        F: Fn(usize) -> Rc<RefCell<Cardinality>>,
    {
        if let Some(_index) = self.cardinality_free_list.lock().unwrap().pop() {
            let cardinality = cardinality(_index);
            self.cardinality[_index] = Some(cardinality.clone());
            cardinality
        } else {
            let _index = self.cardinality.len();
            let cardinality = cardinality(_index);
            self.cardinality.push(Some(cardinality.clone()));
            cardinality
        }
    }

    /// Exhume (get) [`Cardinality`] from the store.
    ///
    pub fn exhume_cardinality(&self, id: &usize) -> Option<Rc<RefCell<Cardinality>>> {
        match self.cardinality.get(*id) {
            Some(cardinality) => cardinality.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Cardinality`] from the store.
    ///
    pub fn exorcise_cardinality(&mut self, id: &usize) -> Option<Rc<RefCell<Cardinality>>> {
        let result = self.cardinality[*id].take();
        self.cardinality_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Cardinality>`.
    ///
    pub fn iter_cardinality(&self) -> impl Iterator<Item = Rc<RefCell<Cardinality>>> + '_ {
        let len = self.cardinality.len();
        (0..len).map(move |i| {
            self.cardinality[i]
                .as_ref()
                .map(|cardinality| cardinality.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Conditionality`] into the store.
    ///
    pub fn inter_conditionality<F>(&mut self, conditionality: F) -> Rc<RefCell<Conditionality>>
    where
        F: Fn(usize) -> Rc<RefCell<Conditionality>>,
    {
        if let Some(_index) = self.conditionality_free_list.lock().unwrap().pop() {
            let conditionality = conditionality(_index);
            self.conditionality[_index] = Some(conditionality.clone());
            conditionality
        } else {
            let _index = self.conditionality.len();
            let conditionality = conditionality(_index);
            self.conditionality.push(Some(conditionality.clone()));
            conditionality
        }
    }

    /// Exhume (get) [`Conditionality`] from the store.
    ///
    pub fn exhume_conditionality(&self, id: &usize) -> Option<Rc<RefCell<Conditionality>>> {
        match self.conditionality.get(*id) {
            Some(conditionality) => conditionality.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Conditionality`] from the store.
    ///
    pub fn exorcise_conditionality(&mut self, id: &usize) -> Option<Rc<RefCell<Conditionality>>> {
        let result = self.conditionality[*id].take();
        self.conditionality_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Conditionality>`.
    ///
    pub fn iter_conditionality(&self) -> impl Iterator<Item = Rc<RefCell<Conditionality>>> + '_ {
        let len = self.conditionality.len();
        (0..len).map(move |i| {
            self.conditionality[i]
                .as_ref()
                .map(|conditionality| conditionality.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Event`] into the store.
    ///
    pub fn inter_event<F>(&mut self, event: F) -> Rc<RefCell<Event>>
    where
        F: Fn(usize) -> Rc<RefCell<Event>>,
    {
        if let Some(_index) = self.event_free_list.lock().unwrap().pop() {
            let event = event(_index);
            self.event[_index] = Some(event.clone());
            event
        } else {
            let _index = self.event.len();
            let event = event(_index);
            self.event.push(Some(event.clone()));
            event
        }
    }

    /// Exhume (get) [`Event`] from the store.
    ///
    pub fn exhume_event(&self, id: &usize) -> Option<Rc<RefCell<Event>>> {
        match self.event.get(*id) {
            Some(event) => event.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Event`] from the store.
    ///
    pub fn exorcise_event(&mut self, id: &usize) -> Option<Rc<RefCell<Event>>> {
        let result = self.event[*id].take();
        self.event_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Event>`.
    ///
    pub fn iter_event(&self) -> impl Iterator<Item = Rc<RefCell<Event>>> + '_ {
        let len = self.event.len();
        (0..len).map(move |i| self.event[i].as_ref().map(|event| event.clone()).unwrap())
    }

    /// Inter (insert) [`External`] into the store.
    ///
    pub fn inter_external<F>(&mut self, external: F) -> Rc<RefCell<External>>
    where
        F: Fn(usize) -> Rc<RefCell<External>>,
    {
        if let Some(_index) = self.external_free_list.lock().unwrap().pop() {
            let external = external(_index);
            self.external[_index] = Some(external.clone());
            external
        } else {
            let _index = self.external.len();
            let external = external(_index);
            self.external.push(Some(external.clone()));
            external
        }
    }

    /// Exhume (get) [`External`] from the store.
    ///
    pub fn exhume_external(&self, id: &usize) -> Option<Rc<RefCell<External>>> {
        match self.external.get(*id) {
            Some(external) => external.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`External`] from the store.
    ///
    pub fn exorcise_external(&mut self, id: &usize) -> Option<Rc<RefCell<External>>> {
        let result = self.external[*id].take();
        self.external_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, External>`.
    ///
    pub fn iter_external(&self) -> impl Iterator<Item = Rc<RefCell<External>>> + '_ {
        let len = self.external.len();
        (0..len).map(move |i| {
            self.external[i]
                .as_ref()
                .map(|external| external.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Isa`] into the store.
    ///
    pub fn inter_isa<F>(&mut self, isa: F) -> Rc<RefCell<Isa>>
    where
        F: Fn(usize) -> Rc<RefCell<Isa>>,
    {
        if let Some(_index) = self.isa_free_list.lock().unwrap().pop() {
            let isa = isa(_index);
            self.isa[_index] = Some(isa.clone());
            isa
        } else {
            let _index = self.isa.len();
            let isa = isa(_index);
            self.isa.push(Some(isa.clone()));
            isa
        }
    }

    /// Exhume (get) [`Isa`] from the store.
    ///
    pub fn exhume_isa(&self, id: &usize) -> Option<Rc<RefCell<Isa>>> {
        match self.isa.get(*id) {
            Some(isa) => isa.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Isa`] from the store.
    ///
    pub fn exorcise_isa(&mut self, id: &usize) -> Option<Rc<RefCell<Isa>>> {
        let result = self.isa[*id].take();
        self.isa_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Isa>`.
    ///
    pub fn iter_isa(&self) -> impl Iterator<Item = Rc<RefCell<Isa>>> + '_ {
        let len = self.isa.len();
        (0..len).map(move |i| self.isa[i].as_ref().map(|isa| isa.clone()).unwrap())
    }

    /// Inter (insert) [`Object`] into the store.
    ///
    pub fn inter_object<F>(&mut self, object: F) -> Rc<RefCell<Object>>
    where
        F: Fn(usize) -> Rc<RefCell<Object>>,
    {
        let object = if let Some(_index) = self.object_free_list.lock().unwrap().pop() {
            let object = object(_index);
            self.object[_index] = Some(object.clone());
            object
        } else {
            let _index = self.object.len();
            let object = object(_index);
            self.object.push(Some(object.clone()));
            object
        };
        self.object_id_by_name.insert(
            object.borrow().name.to_upper_camel_case(),
            object.borrow().id,
        );
        object
    }

    /// Exhume (get) [`Object`] from the store.
    ///
    pub fn exhume_object(&self, id: &usize) -> Option<Rc<RefCell<Object>>> {
        match self.object.get(*id) {
            Some(object) => object.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Object`] from the store.
    ///
    pub fn exorcise_object(&mut self, id: &usize) -> Option<Rc<RefCell<Object>>> {
        let result = self.object[*id].take();
        self.object_free_list.lock().unwrap().push(*id);
        result
    }

    /// Exorcise [`Object`] id from the store by name.
    ///
    pub fn exhume_object_id_by_name(&self, name: &str) -> Option<usize> {
        self.object_id_by_name.get(name).map(|object| *object)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Object>`.
    ///
    pub fn iter_object(&self) -> impl Iterator<Item = Rc<RefCell<Object>>> + '_ {
        let len = self.object.len();
        (0..len).map(move |i| {
            self.object[i]
                .as_ref()
                .map(|object| object.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Referent`] into the store.
    ///
    pub fn inter_referent<F>(&mut self, referent: F) -> Rc<RefCell<Referent>>
    where
        F: Fn(usize) -> Rc<RefCell<Referent>>,
    {
        if let Some(_index) = self.referent_free_list.lock().unwrap().pop() {
            let referent = referent(_index);
            self.referent[_index] = Some(referent.clone());
            referent
        } else {
            let _index = self.referent.len();
            let referent = referent(_index);
            self.referent.push(Some(referent.clone()));
            referent
        }
    }

    /// Exhume (get) [`Referent`] from the store.
    ///
    pub fn exhume_referent(&self, id: &usize) -> Option<Rc<RefCell<Referent>>> {
        match self.referent.get(*id) {
            Some(referent) => referent.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Referent`] from the store.
    ///
    pub fn exorcise_referent(&mut self, id: &usize) -> Option<Rc<RefCell<Referent>>> {
        let result = self.referent[*id].take();
        self.referent_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Referent>`.
    ///
    pub fn iter_referent(&self) -> impl Iterator<Item = Rc<RefCell<Referent>>> + '_ {
        let len = self.referent.len();
        (0..len).map(move |i| {
            self.referent[i]
                .as_ref()
                .map(|referent| referent.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Referrer`] into the store.
    ///
    pub fn inter_referrer<F>(&mut self, referrer: F) -> Rc<RefCell<Referrer>>
    where
        F: Fn(usize) -> Rc<RefCell<Referrer>>,
    {
        if let Some(_index) = self.referrer_free_list.lock().unwrap().pop() {
            let referrer = referrer(_index);
            self.referrer[_index] = Some(referrer.clone());
            referrer
        } else {
            let _index = self.referrer.len();
            let referrer = referrer(_index);
            self.referrer.push(Some(referrer.clone()));
            referrer
        }
    }

    /// Exhume (get) [`Referrer`] from the store.
    ///
    pub fn exhume_referrer(&self, id: &usize) -> Option<Rc<RefCell<Referrer>>> {
        match self.referrer.get(*id) {
            Some(referrer) => referrer.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Referrer`] from the store.
    ///
    pub fn exorcise_referrer(&mut self, id: &usize) -> Option<Rc<RefCell<Referrer>>> {
        let result = self.referrer[*id].take();
        self.referrer_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Referrer>`.
    ///
    pub fn iter_referrer(&self) -> impl Iterator<Item = Rc<RefCell<Referrer>>> + '_ {
        let len = self.referrer.len();
        (0..len).map(move |i| {
            self.referrer[i]
                .as_ref()
                .map(|referrer| referrer.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Relationship`] into the store.
    ///
    pub fn inter_relationship<F>(&mut self, relationship: F) -> Rc<RefCell<Relationship>>
    where
        F: Fn(usize) -> Rc<RefCell<Relationship>>,
    {
        if let Some(_index) = self.relationship_free_list.lock().unwrap().pop() {
            let relationship = relationship(_index);
            self.relationship[_index] = Some(relationship.clone());
            relationship
        } else {
            let _index = self.relationship.len();
            let relationship = relationship(_index);
            self.relationship.push(Some(relationship.clone()));
            relationship
        }
    }

    /// Exhume (get) [`Relationship`] from the store.
    ///
    pub fn exhume_relationship(&self, id: &usize) -> Option<Rc<RefCell<Relationship>>> {
        match self.relationship.get(*id) {
            Some(relationship) => relationship.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Relationship`] from the store.
    ///
    pub fn exorcise_relationship(&mut self, id: &usize) -> Option<Rc<RefCell<Relationship>>> {
        let result = self.relationship[*id].take();
        self.relationship_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Relationship>`.
    ///
    pub fn iter_relationship(&self) -> impl Iterator<Item = Rc<RefCell<Relationship>>> + '_ {
        let len = self.relationship.len();
        (0..len).map(move |i| {
            self.relationship[i]
                .as_ref()
                .map(|relationship| relationship.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`State`] into the store.
    ///
    pub fn inter_state<F>(&mut self, state: F) -> Rc<RefCell<State>>
    where
        F: Fn(usize) -> Rc<RefCell<State>>,
    {
        if let Some(_index) = self.state_free_list.lock().unwrap().pop() {
            let state = state(_index);
            self.state[_index] = Some(state.clone());
            state
        } else {
            let _index = self.state.len();
            let state = state(_index);
            self.state.push(Some(state.clone()));
            state
        }
    }

    /// Exhume (get) [`State`] from the store.
    ///
    pub fn exhume_state(&self, id: &usize) -> Option<Rc<RefCell<State>>> {
        match self.state.get(*id) {
            Some(state) => state.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`State`] from the store.
    ///
    pub fn exorcise_state(&mut self, id: &usize) -> Option<Rc<RefCell<State>>> {
        let result = self.state[*id].take();
        self.state_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, State>`.
    ///
    pub fn iter_state(&self) -> impl Iterator<Item = Rc<RefCell<State>>> + '_ {
        let len = self.state.len();
        (0..len).map(move |i| self.state[i].as_ref().map(|state| state.clone()).unwrap())
    }

    /// Inter (insert) [`Subtype`] into the store.
    ///
    pub fn inter_subtype<F>(&mut self, subtype: F) -> Rc<RefCell<Subtype>>
    where
        F: Fn(usize) -> Rc<RefCell<Subtype>>,
    {
        if let Some(_index) = self.subtype_free_list.lock().unwrap().pop() {
            let subtype = subtype(_index);
            self.subtype[_index] = Some(subtype.clone());
            subtype
        } else {
            let _index = self.subtype.len();
            let subtype = subtype(_index);
            self.subtype.push(Some(subtype.clone()));
            subtype
        }
    }

    /// Exhume (get) [`Subtype`] from the store.
    ///
    pub fn exhume_subtype(&self, id: &usize) -> Option<Rc<RefCell<Subtype>>> {
        match self.subtype.get(*id) {
            Some(subtype) => subtype.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Subtype`] from the store.
    ///
    pub fn exorcise_subtype(&mut self, id: &usize) -> Option<Rc<RefCell<Subtype>>> {
        let result = self.subtype[*id].take();
        self.subtype_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Subtype>`.
    ///
    pub fn iter_subtype(&self) -> impl Iterator<Item = Rc<RefCell<Subtype>>> + '_ {
        let len = self.subtype.len();
        (0..len).map(move |i| {
            self.subtype[i]
                .as_ref()
                .map(|subtype| subtype.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Supertype`] into the store.
    ///
    pub fn inter_supertype<F>(&mut self, supertype: F) -> Rc<RefCell<Supertype>>
    where
        F: Fn(usize) -> Rc<RefCell<Supertype>>,
    {
        if let Some(_index) = self.supertype_free_list.lock().unwrap().pop() {
            let supertype = supertype(_index);
            self.supertype[_index] = Some(supertype.clone());
            supertype
        } else {
            let _index = self.supertype.len();
            let supertype = supertype(_index);
            self.supertype.push(Some(supertype.clone()));
            supertype
        }
    }

    /// Exhume (get) [`Supertype`] from the store.
    ///
    pub fn exhume_supertype(&self, id: &usize) -> Option<Rc<RefCell<Supertype>>> {
        match self.supertype.get(*id) {
            Some(supertype) => supertype.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Supertype`] from the store.
    ///
    pub fn exorcise_supertype(&mut self, id: &usize) -> Option<Rc<RefCell<Supertype>>> {
        let result = self.supertype[*id].take();
        self.supertype_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Supertype>`.
    ///
    pub fn iter_supertype(&self) -> impl Iterator<Item = Rc<RefCell<Supertype>>> + '_ {
        let len = self.supertype.len();
        (0..len).map(move |i| {
            self.supertype[i]
                .as_ref()
                .map(|supertype| supertype.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Ty`] into the store.
    ///
    pub fn inter_ty<F>(&mut self, ty: F) -> Rc<RefCell<Ty>>
    where
        F: Fn(usize) -> Rc<RefCell<Ty>>,
    {
        if let Some(_index) = self.ty_free_list.lock().unwrap().pop() {
            let ty = ty(_index);
            self.ty[_index] = Some(ty.clone());
            ty
        } else {
            let _index = self.ty.len();
            let ty = ty(_index);
            self.ty.push(Some(ty.clone()));
            ty
        }
    }

    /// Exhume (get) [`Ty`] from the store.
    ///
    pub fn exhume_ty(&self, id: &usize) -> Option<Rc<RefCell<Ty>>> {
        match self.ty.get(*id) {
            Some(ty) => ty.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Ty`] from the store.
    ///
    pub fn exorcise_ty(&mut self, id: &usize) -> Option<Rc<RefCell<Ty>>> {
        let result = self.ty[*id].take();
        self.ty_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Ty>`.
    ///
    pub fn iter_ty(&self) -> impl Iterator<Item = Rc<RefCell<Ty>>> + '_ {
        let len = self.ty.len();
        (0..len).map(move |i| self.ty[i].as_ref().map(|ty| ty.clone()).unwrap())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::sarzak_vec-object-store-persistence"}}}
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
            for acknowledged_event in &self.acknowledged_event {
                if let Some(acknowledged_event) = acknowledged_event {
                    let path = path.join(format!("{}.json", acknowledged_event.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &acknowledged_event)?;
                }
            }
        }

        // Persist An Associative Referent.
        {
            let path = path.join("an_associative_referent");
            fs::create_dir_all(&path)?;
            for an_associative_referent in &self.an_associative_referent {
                if let Some(an_associative_referent) = an_associative_referent {
                    let path = path.join(format!("{}.json", an_associative_referent.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &an_associative_referent)?;
                }
            }
        }

        // Persist Associative.
        {
            let path = path.join("associative");
            fs::create_dir_all(&path)?;
            for associative in &self.associative {
                if let Some(associative) = associative {
                    let path = path.join(format!("{}.json", associative.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &associative)?;
                }
            }
        }

        // Persist Associative Referent.
        {
            let path = path.join("associative_referent");
            fs::create_dir_all(&path)?;
            for associative_referent in &self.associative_referent {
                if let Some(associative_referent) = associative_referent {
                    let path = path.join(format!("{}.json", associative_referent.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &associative_referent)?;
                }
            }
        }

        // Persist Associative Referrer.
        {
            let path = path.join("associative_referrer");
            fs::create_dir_all(&path)?;
            for associative_referrer in &self.associative_referrer {
                if let Some(associative_referrer) = associative_referrer {
                    let path = path.join(format!("{}.json", associative_referrer.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &associative_referrer)?;
                }
            }
        }

        // Persist Attribute.
        {
            let path = path.join("attribute");
            fs::create_dir_all(&path)?;
            for attribute in &self.attribute {
                if let Some(attribute) = attribute {
                    let path = path.join(format!("{}.json", attribute.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &attribute)?;
                }
            }
        }

        // Persist Binary.
        {
            let path = path.join("binary");
            fs::create_dir_all(&path)?;
            for binary in &self.binary {
                if let Some(binary) = binary {
                    let path = path.join(format!("{}.json", binary.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &binary)?;
                }
            }
        }

        // Persist Cardinality.
        {
            let path = path.join("cardinality");
            fs::create_dir_all(&path)?;
            for cardinality in &self.cardinality {
                if let Some(cardinality) = cardinality {
                    let path = path.join(format!("{}.json", cardinality.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &cardinality)?;
                }
            }
        }

        // Persist Conditionality.
        {
            let path = path.join("conditionality");
            fs::create_dir_all(&path)?;
            for conditionality in &self.conditionality {
                if let Some(conditionality) = conditionality {
                    let path = path.join(format!("{}.json", conditionality.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &conditionality)?;
                }
            }
        }

        // Persist Event.
        {
            let path = path.join("event");
            fs::create_dir_all(&path)?;
            for event in &self.event {
                if let Some(event) = event {
                    let path = path.join(format!("{}.json", event.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &event)?;
                }
            }
        }

        // Persist External.
        {
            let path = path.join("external");
            fs::create_dir_all(&path)?;
            for external in &self.external {
                if let Some(external) = external {
                    let path = path.join(format!("{}.json", external.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &external)?;
                }
            }
        }

        // Persist Isa.
        {
            let path = path.join("isa");
            fs::create_dir_all(&path)?;
            for isa in &self.isa {
                if let Some(isa) = isa {
                    let path = path.join(format!("{}.json", isa.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &isa)?;
                }
            }
        }

        // Persist Object.
        {
            let path = path.join("object");
            fs::create_dir_all(&path)?;
            for object in &self.object {
                if let Some(object) = object {
                    let path = path.join(format!("{}.json", object.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &object)?;
                }
            }
        }

        // Persist Referent.
        {
            let path = path.join("referent");
            fs::create_dir_all(&path)?;
            for referent in &self.referent {
                if let Some(referent) = referent {
                    let path = path.join(format!("{}.json", referent.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &referent)?;
                }
            }
        }

        // Persist Referrer.
        {
            let path = path.join("referrer");
            fs::create_dir_all(&path)?;
            for referrer in &self.referrer {
                if let Some(referrer) = referrer {
                    let path = path.join(format!("{}.json", referrer.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &referrer)?;
                }
            }
        }

        // Persist Relationship.
        {
            let path = path.join("relationship");
            fs::create_dir_all(&path)?;
            for relationship in &self.relationship {
                if let Some(relationship) = relationship {
                    let path = path.join(format!("{}.json", relationship.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &relationship)?;
                }
            }
        }

        // Persist State.
        {
            let path = path.join("state");
            fs::create_dir_all(&path)?;
            for state in &self.state {
                if let Some(state) = state {
                    let path = path.join(format!("{}.json", state.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &state)?;
                }
            }
        }

        // Persist Subtype.
        {
            let path = path.join("subtype");
            fs::create_dir_all(&path)?;
            for subtype in &self.subtype {
                if let Some(subtype) = subtype {
                    let path = path.join(format!("{}.json", subtype.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &subtype)?;
                }
            }
        }

        // Persist Supertype.
        {
            let path = path.join("supertype");
            fs::create_dir_all(&path)?;
            for supertype in &self.supertype {
                if let Some(supertype) = supertype {
                    let path = path.join(format!("{}.json", supertype.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &supertype)?;
                }
            }
        }

        // Persist Type.
        {
            let path = path.join("ty");
            fs::create_dir_all(&path)?;
            for ty in &self.ty {
                if let Some(ty) = ty {
                    let path = path.join(format!("{}.json", ty.borrow().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &ty)?;
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
                let acknowledged_event: Rc<RefCell<AcknowledgedEvent>> =
                    serde_json::from_reader(reader)?;
                store.inter_acknowledged_event(|id| {
                    acknowledged_event.borrow_mut().id = id;
                    acknowledged_event.clone()
                });
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
                let an_associative_referent: Rc<RefCell<AnAssociativeReferent>> =
                    serde_json::from_reader(reader)?;
                store.inter_an_associative_referent(|id| {
                    an_associative_referent.borrow_mut().id = id;
                    an_associative_referent.clone()
                });
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
                let associative: Rc<RefCell<Associative>> = serde_json::from_reader(reader)?;
                store.inter_associative(|id| {
                    associative.borrow_mut().id = id;
                    associative.clone()
                });
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
                let associative_referent: Rc<RefCell<AssociativeReferent>> =
                    serde_json::from_reader(reader)?;
                store.inter_associative_referent(|id| {
                    associative_referent.borrow_mut().id = id;
                    associative_referent.clone()
                });
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
                let associative_referrer: Rc<RefCell<AssociativeReferrer>> =
                    serde_json::from_reader(reader)?;
                store.inter_associative_referrer(|id| {
                    associative_referrer.borrow_mut().id = id;
                    associative_referrer.clone()
                });
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
                let attribute: Rc<RefCell<Attribute>> = serde_json::from_reader(reader)?;
                store.inter_attribute(|id| {
                    attribute.borrow_mut().id = id;
                    attribute.clone()
                });
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
                let binary: Rc<RefCell<Binary>> = serde_json::from_reader(reader)?;
                store.inter_binary(|id| {
                    binary.borrow_mut().id = id;
                    binary.clone()
                });
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
                let cardinality: Rc<RefCell<Cardinality>> = serde_json::from_reader(reader)?;
                store.inter_cardinality(|id| {
                    cardinality.borrow_mut().id = id;
                    cardinality.clone()
                });
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
                let conditionality: Rc<RefCell<Conditionality>> = serde_json::from_reader(reader)?;
                store.inter_conditionality(|id| {
                    conditionality.borrow_mut().id = id;
                    conditionality.clone()
                });
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
                let event: Rc<RefCell<Event>> = serde_json::from_reader(reader)?;
                store.inter_event(|id| {
                    event.borrow_mut().id = id;
                    event.clone()
                });
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
                let external: Rc<RefCell<External>> = serde_json::from_reader(reader)?;
                store.inter_external(|id| {
                    external.borrow_mut().id = id;
                    external.clone()
                });
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
                let isa: Rc<RefCell<Isa>> = serde_json::from_reader(reader)?;
                store.inter_isa(|id| {
                    isa.borrow_mut().id = id;
                    isa.clone()
                });
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
                let object: Rc<RefCell<Object>> = serde_json::from_reader(reader)?;
                store.object_id_by_name.insert(
                    object.borrow().name.to_upper_camel_case(),
                    object.borrow().id,
                );
                store.inter_object(|id| {
                    object.borrow_mut().id = id;
                    object.clone()
                });
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
                let referent: Rc<RefCell<Referent>> = serde_json::from_reader(reader)?;
                store.inter_referent(|id| {
                    referent.borrow_mut().id = id;
                    referent.clone()
                });
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
                let referrer: Rc<RefCell<Referrer>> = serde_json::from_reader(reader)?;
                store.inter_referrer(|id| {
                    referrer.borrow_mut().id = id;
                    referrer.clone()
                });
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
                let relationship: Rc<RefCell<Relationship>> = serde_json::from_reader(reader)?;
                store.inter_relationship(|id| {
                    relationship.borrow_mut().id = id;
                    relationship.clone()
                });
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
                let state: Rc<RefCell<State>> = serde_json::from_reader(reader)?;
                store.inter_state(|id| {
                    state.borrow_mut().id = id;
                    state.clone()
                });
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
                let subtype: Rc<RefCell<Subtype>> = serde_json::from_reader(reader)?;
                store.inter_subtype(|id| {
                    subtype.borrow_mut().id = id;
                    subtype.clone()
                });
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
                let supertype: Rc<RefCell<Supertype>> = serde_json::from_reader(reader)?;
                store.inter_supertype(|id| {
                    supertype.borrow_mut().id = id;
                    supertype.clone()
                });
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
                let ty: Rc<RefCell<Ty>> = serde_json::from_reader(reader)?;
                store.inter_ty(|id| {
                    ty.borrow_mut().id = id;
                    ty.clone()
                });
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

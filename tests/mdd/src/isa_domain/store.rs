//! isa_domain Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_domain-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`SimpleSupertype`]
//! * [`SubtypeA`]
//! * [`SubtypeB`]
//! * [`SuperT`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_domain-object-store-definition"}}}
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::isa_domain::types::{SimpleSupertype, SubtypeA, SubtypeB, SuperT};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    simple_supertype: HashMap<Uuid, SimpleSupertype>,
    subtype_a: HashMap<Uuid, SubtypeA>,
    subtype_b: HashMap<Uuid, SubtypeB>,
    super_t: HashMap<Uuid, SuperT>,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            simple_supertype: HashMap::new(),
            subtype_a: HashMap::new(),
            subtype_b: HashMap::new(),
            super_t: HashMap::new(),
        }
    }

    /// Inter [`SimpleSupertype`] into the store.
    ///
    pub fn inter_simple_supertype(&mut self, simple_supertype: SimpleSupertype) {
        self.simple_supertype
            .insert(simple_supertype.id(), simple_supertype);
    }

    /// Exhume [`SimpleSupertype`] from the store.
    ///
    pub fn exhume_simple_supertype(&self, id: &Uuid) -> Option<&SimpleSupertype> {
        self.simple_supertype.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, SimpleSupertype>`.
    //
    pub fn iter_simple_supertype(&self) -> impl Iterator<Item = (&Uuid, &SimpleSupertype)> {
        self.simple_supertype.iter()
    }
    /// Inter [`SubtypeA`] into the store.
    ///
    pub fn inter_subtype_a(&mut self, subtype_a: SubtypeA) {
        self.subtype_a.insert(subtype_a.id, subtype_a);
    }

    /// Exhume [`SubtypeA`] from the store.
    ///
    pub fn exhume_subtype_a(&self, id: &Uuid) -> Option<&SubtypeA> {
        self.subtype_a.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeA>`.
    //
    pub fn iter_subtype_a(&self) -> impl Iterator<Item = (&Uuid, &SubtypeA)> {
        self.subtype_a.iter()
    }
    /// Inter [`SubtypeB`] into the store.
    ///
    pub fn inter_subtype_b(&mut self, subtype_b: SubtypeB) {
        self.subtype_b.insert(subtype_b.id, subtype_b);
    }

    /// Exhume [`SubtypeB`] from the store.
    ///
    pub fn exhume_subtype_b(&self, id: &Uuid) -> Option<&SubtypeB> {
        self.subtype_b.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeB>`.
    //
    pub fn iter_subtype_b(&self) -> impl Iterator<Item = (&Uuid, &SubtypeB)> {
        self.subtype_b.iter()
    }
    /// Inter [`SuperT`] into the store.
    ///
    pub fn inter_super_t(&mut self, super_t: SuperT) {
        self.super_t.insert(super_t.id(), super_t);
    }

    /// Exhume [`SuperT`] from the store.
    ///
    pub fn exhume_super_t(&self, id: &Uuid) -> Option<&SuperT> {
        self.super_t.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, SuperT>`.
    //
    pub fn iter_super_t(&self) -> impl Iterator<Item = (&Uuid, &SuperT)> {
        self.super_t.iter()
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

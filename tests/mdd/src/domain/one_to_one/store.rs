//! domain::one_to_one Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_one-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`A`]
//! * [`B`]
//! * [`C`]
//! * [`Parameter`]
//! * [`Referent`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_one-object-store-definition"}}}
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::one_to_one::types::{Parameter, Referent, A, B, C};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    a: HashMap<Uuid, A>,
    b: HashMap<Uuid, B>,
    c: HashMap<Uuid, C>,
    parameter: HashMap<Uuid, Parameter>,
    referent: HashMap<Uuid, Referent>,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            a: HashMap::new(),
            b: HashMap::new(),
            c: HashMap::new(),
            parameter: HashMap::new(),
            referent: HashMap::new(),
        }
    }

    /// Inter [`A`] into the store.
    ///
    pub fn inter_a(&mut self, a: A) {
        self.a.insert(a.id, a);
    }

    /// Exhume [`A`] from the store.
    ///
    pub fn exhume_a(&self, id: &Uuid) -> Option<&A> {
        self.a.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, A>`.
    //
    pub fn iter_a(&self) -> impl Iterator<Item = (&Uuid, &A)> {
        self.a.iter()
    }
    /// Inter [`B`] into the store.
    ///
    pub fn inter_b(&mut self, b: B) {
        self.b.insert(b.id, b);
    }

    /// Exhume [`B`] from the store.
    ///
    pub fn exhume_b(&self, id: &Uuid) -> Option<&B> {
        self.b.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, B>`.
    //
    pub fn iter_b(&self) -> impl Iterator<Item = (&Uuid, &B)> {
        self.b.iter()
    }
    /// Inter [`C`] into the store.
    ///
    pub fn inter_c(&mut self, c: C) {
        self.c.insert(c.id, c);
    }

    /// Exhume [`C`] from the store.
    ///
    pub fn exhume_c(&self, id: &Uuid) -> Option<&C> {
        self.c.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, C>`.
    //
    pub fn iter_c(&self) -> impl Iterator<Item = (&Uuid, &C)> {
        self.c.iter()
    }
    /// Inter [`Parameter`] into the store.
    ///
    pub fn inter_parameter(&mut self, parameter: Parameter) {
        self.parameter.insert(parameter.id, parameter);
    }

    /// Exhume [`Parameter`] from the store.
    ///
    pub fn exhume_parameter(&self, id: &Uuid) -> Option<&Parameter> {
        self.parameter.get(id)
    }
    /// Get an iterator over the internal `HashMap<&Uuid, Parameter>`.
    //
    pub fn iter_parameter(&self) -> impl Iterator<Item = (&Uuid, &Parameter)> {
        self.parameter.iter()
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
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
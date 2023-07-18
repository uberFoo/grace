//! domain::isa_rwlock Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa_rwlock-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`Alpha`]
//! * [`Baz`]
//! * [`Beta`]
//! * [`Borrowed`]
//! * [`Gamma`]
//! * [`Henry`]
//! * [`NotImportant`]
//! * [`OhBoy`]
//! * [`Ownership`]
//! * [`Reference`]
//! * [`SimpleSubtypeA`]
//! * [`SimpleSupertype`]
//! * [`SubtypeA`]
//! * [`SubtypeB`]
//! * [`SuperBar`]
//! * [`SuperFoo`]
//! * [`SuperT`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa_rwlock-object-store-definition"}}}
use std::sync::Arc;
use std::sync::RwLock;
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::isa_rwlock::types::{
    Alpha, Baz, Beta, Borrowed, Gamma, Henry, NotImportant, OhBoy, Ownership, Reference,
    SimpleSubtypeA, SimpleSupertype, SubtypeA, SubtypeB, SuperBar, SuperFoo, SuperT, MUTABLE,
    OWNED, SHARED,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    alpha: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Alpha>>>>>,
    baz: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Baz>>>>>,
    beta: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Beta>>>>>,
    borrowed: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Borrowed>>>>>,
    gamma: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Gamma>>>>>,
    henry: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Henry>>>>>,
    not_important: Arc<RwLock<HashMap<Uuid, Arc<RwLock<NotImportant>>>>>,
    oh_boy: Arc<RwLock<HashMap<Uuid, Arc<RwLock<OhBoy>>>>>,
    ownership: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Ownership>>>>>,
    reference: Arc<RwLock<HashMap<Uuid, Arc<RwLock<Reference>>>>>,
    simple_subtype_a: Arc<RwLock<HashMap<Uuid, Arc<RwLock<SimpleSubtypeA>>>>>,
    simple_supertype: Arc<RwLock<HashMap<Uuid, Arc<RwLock<SimpleSupertype>>>>>,
    subtype_a: Arc<RwLock<HashMap<Uuid, Arc<RwLock<SubtypeA>>>>>,
    subtype_b: Arc<RwLock<HashMap<Uuid, Arc<RwLock<SubtypeB>>>>>,
    super_bar: Arc<RwLock<HashMap<Uuid, Arc<RwLock<SuperBar>>>>>,
    super_foo: Arc<RwLock<HashMap<Uuid, Arc<RwLock<SuperFoo>>>>>,
    super_t: Arc<RwLock<HashMap<Uuid, Arc<RwLock<SuperT>>>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            alpha: Arc::new(RwLock::new(HashMap::default())),
            baz: Arc::new(RwLock::new(HashMap::default())),
            beta: Arc::new(RwLock::new(HashMap::default())),
            borrowed: Arc::new(RwLock::new(HashMap::default())),
            gamma: Arc::new(RwLock::new(HashMap::default())),
            henry: Arc::new(RwLock::new(HashMap::default())),
            not_important: Arc::new(RwLock::new(HashMap::default())),
            oh_boy: Arc::new(RwLock::new(HashMap::default())),
            ownership: Arc::new(RwLock::new(HashMap::default())),
            reference: Arc::new(RwLock::new(HashMap::default())),
            simple_subtype_a: Arc::new(RwLock::new(HashMap::default())),
            simple_supertype: Arc::new(RwLock::new(HashMap::default())),
            subtype_a: Arc::new(RwLock::new(HashMap::default())),
            subtype_b: Arc::new(RwLock::new(HashMap::default())),
            super_bar: Arc::new(RwLock::new(HashMap::default())),
            super_foo: Arc::new(RwLock::new(HashMap::default())),
            super_t: Arc::new(RwLock::new(HashMap::default())),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥
        store.inter_borrowed(Arc::new(RwLock::new(Borrowed::Mutable(MUTABLE))));
        store.inter_borrowed(Arc::new(RwLock::new(Borrowed::Shared(SHARED))));
        store.inter_ownership(Arc::new(RwLock::new(Ownership::Borrowed(
            Borrowed::Mutable(MUTABLE).id(),
        ))));
        store.inter_ownership(Arc::new(RwLock::new(Ownership::Borrowed(
            Borrowed::Shared(SHARED).id(),
        ))));
        store.inter_ownership(Arc::new(RwLock::new(Ownership::Owned(OWNED))));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa_rwlock-object-store-methods"}}}
    /// Inter (insert) [`Alpha`] into the store.
    ///
    pub fn inter_alpha(&mut self, alpha: Arc<RwLock<Alpha>>) {
        let read = alpha.read().unwrap();
        self.alpha.write().unwrap().insert(read.id, alpha.clone());
    }

    /// Exhume (get) [`Alpha`] from the store.
    ///
    pub fn exhume_alpha(&self, id: &Uuid) -> Option<Arc<RwLock<Alpha>>> {
        self.alpha
            .read()
            .unwrap()
            .get(id)
            .map(|alpha| alpha.clone())
    }

    /// Exorcise (remove) [`Alpha`] from the store.
    ///
    pub fn exorcise_alpha(&mut self, id: &Uuid) -> Option<Arc<RwLock<Alpha>>> {
        self.alpha
            .write()
            .unwrap()
            .remove(id)
            .map(|alpha| alpha.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Alpha>`.
    ///
    pub fn iter_alpha(&self) -> impl Iterator<Item = Arc<RwLock<Alpha>>> + '_ {
        let values: Vec<Arc<RwLock<Alpha>>> = self
            .alpha
            .read()
            .unwrap()
            .values()
            .map(|alpha| alpha.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Baz`] into the store.
    ///
    pub fn inter_baz(&mut self, baz: Arc<RwLock<Baz>>) {
        let read = baz.read().unwrap();
        self.baz.write().unwrap().insert(read.id, baz.clone());
    }

    /// Exhume (get) [`Baz`] from the store.
    ///
    pub fn exhume_baz(&self, id: &Uuid) -> Option<Arc<RwLock<Baz>>> {
        self.baz.read().unwrap().get(id).map(|baz| baz.clone())
    }

    /// Exorcise (remove) [`Baz`] from the store.
    ///
    pub fn exorcise_baz(&mut self, id: &Uuid) -> Option<Arc<RwLock<Baz>>> {
        self.baz.write().unwrap().remove(id).map(|baz| baz.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Baz>`.
    ///
    pub fn iter_baz(&self) -> impl Iterator<Item = Arc<RwLock<Baz>>> + '_ {
        let values: Vec<Arc<RwLock<Baz>>> = self
            .baz
            .read()
            .unwrap()
            .values()
            .map(|baz| baz.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Beta`] into the store.
    ///
    pub fn inter_beta(&mut self, beta: Arc<RwLock<Beta>>) {
        let read = beta.read().unwrap();
        self.beta.write().unwrap().insert(read.id, beta.clone());
    }

    /// Exhume (get) [`Beta`] from the store.
    ///
    pub fn exhume_beta(&self, id: &Uuid) -> Option<Arc<RwLock<Beta>>> {
        self.beta.read().unwrap().get(id).map(|beta| beta.clone())
    }

    /// Exorcise (remove) [`Beta`] from the store.
    ///
    pub fn exorcise_beta(&mut self, id: &Uuid) -> Option<Arc<RwLock<Beta>>> {
        self.beta
            .write()
            .unwrap()
            .remove(id)
            .map(|beta| beta.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Beta>`.
    ///
    pub fn iter_beta(&self) -> impl Iterator<Item = Arc<RwLock<Beta>>> + '_ {
        let values: Vec<Arc<RwLock<Beta>>> = self
            .beta
            .read()
            .unwrap()
            .values()
            .map(|beta| beta.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Borrowed`] into the store.
    ///
    pub fn inter_borrowed(&mut self, borrowed: Arc<RwLock<Borrowed>>) {
        let read = borrowed.read().unwrap();
        self.borrowed
            .write()
            .unwrap()
            .insert(read.id(), borrowed.clone());
    }

    /// Exhume (get) [`Borrowed`] from the store.
    ///
    pub fn exhume_borrowed(&self, id: &Uuid) -> Option<Arc<RwLock<Borrowed>>> {
        self.borrowed
            .read()
            .unwrap()
            .get(id)
            .map(|borrowed| borrowed.clone())
    }

    /// Exorcise (remove) [`Borrowed`] from the store.
    ///
    pub fn exorcise_borrowed(&mut self, id: &Uuid) -> Option<Arc<RwLock<Borrowed>>> {
        self.borrowed
            .write()
            .unwrap()
            .remove(id)
            .map(|borrowed| borrowed.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Borrowed>`.
    ///
    pub fn iter_borrowed(&self) -> impl Iterator<Item = Arc<RwLock<Borrowed>>> + '_ {
        let values: Vec<Arc<RwLock<Borrowed>>> = self
            .borrowed
            .read()
            .unwrap()
            .values()
            .map(|borrowed| borrowed.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Gamma`] into the store.
    ///
    pub fn inter_gamma(&mut self, gamma: Arc<RwLock<Gamma>>) {
        let read = gamma.read().unwrap();
        self.gamma.write().unwrap().insert(read.id, gamma.clone());
    }

    /// Exhume (get) [`Gamma`] from the store.
    ///
    pub fn exhume_gamma(&self, id: &Uuid) -> Option<Arc<RwLock<Gamma>>> {
        self.gamma
            .read()
            .unwrap()
            .get(id)
            .map(|gamma| gamma.clone())
    }

    /// Exorcise (remove) [`Gamma`] from the store.
    ///
    pub fn exorcise_gamma(&mut self, id: &Uuid) -> Option<Arc<RwLock<Gamma>>> {
        self.gamma
            .write()
            .unwrap()
            .remove(id)
            .map(|gamma| gamma.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Gamma>`.
    ///
    pub fn iter_gamma(&self) -> impl Iterator<Item = Arc<RwLock<Gamma>>> + '_ {
        let values: Vec<Arc<RwLock<Gamma>>> = self
            .gamma
            .read()
            .unwrap()
            .values()
            .map(|gamma| gamma.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Henry`] into the store.
    ///
    pub fn inter_henry(&mut self, henry: Arc<RwLock<Henry>>) {
        let read = henry.read().unwrap();
        self.henry.write().unwrap().insert(read.id, henry.clone());
    }

    /// Exhume (get) [`Henry`] from the store.
    ///
    pub fn exhume_henry(&self, id: &Uuid) -> Option<Arc<RwLock<Henry>>> {
        self.henry
            .read()
            .unwrap()
            .get(id)
            .map(|henry| henry.clone())
    }

    /// Exorcise (remove) [`Henry`] from the store.
    ///
    pub fn exorcise_henry(&mut self, id: &Uuid) -> Option<Arc<RwLock<Henry>>> {
        self.henry
            .write()
            .unwrap()
            .remove(id)
            .map(|henry| henry.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Henry>`.
    ///
    pub fn iter_henry(&self) -> impl Iterator<Item = Arc<RwLock<Henry>>> + '_ {
        let values: Vec<Arc<RwLock<Henry>>> = self
            .henry
            .read()
            .unwrap()
            .values()
            .map(|henry| henry.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`NotImportant`] into the store.
    ///
    pub fn inter_not_important(&mut self, not_important: Arc<RwLock<NotImportant>>) {
        let read = not_important.read().unwrap();
        self.not_important
            .write()
            .unwrap()
            .insert(read.id, not_important.clone());
    }

    /// Exhume (get) [`NotImportant`] from the store.
    ///
    pub fn exhume_not_important(&self, id: &Uuid) -> Option<Arc<RwLock<NotImportant>>> {
        self.not_important
            .read()
            .unwrap()
            .get(id)
            .map(|not_important| not_important.clone())
    }

    /// Exorcise (remove) [`NotImportant`] from the store.
    ///
    pub fn exorcise_not_important(&mut self, id: &Uuid) -> Option<Arc<RwLock<NotImportant>>> {
        self.not_important
            .write()
            .unwrap()
            .remove(id)
            .map(|not_important| not_important.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, NotImportant>`.
    ///
    pub fn iter_not_important(&self) -> impl Iterator<Item = Arc<RwLock<NotImportant>>> + '_ {
        let values: Vec<Arc<RwLock<NotImportant>>> = self
            .not_important
            .read()
            .unwrap()
            .values()
            .map(|not_important| not_important.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`OhBoy`] into the store.
    ///
    pub fn inter_oh_boy(&mut self, oh_boy: Arc<RwLock<OhBoy>>) {
        let read = oh_boy.read().unwrap();
        self.oh_boy.write().unwrap().insert(read.id, oh_boy.clone());
    }

    /// Exhume (get) [`OhBoy`] from the store.
    ///
    pub fn exhume_oh_boy(&self, id: &Uuid) -> Option<Arc<RwLock<OhBoy>>> {
        self.oh_boy
            .read()
            .unwrap()
            .get(id)
            .map(|oh_boy| oh_boy.clone())
    }

    /// Exorcise (remove) [`OhBoy`] from the store.
    ///
    pub fn exorcise_oh_boy(&mut self, id: &Uuid) -> Option<Arc<RwLock<OhBoy>>> {
        self.oh_boy
            .write()
            .unwrap()
            .remove(id)
            .map(|oh_boy| oh_boy.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, OhBoy>`.
    ///
    pub fn iter_oh_boy(&self) -> impl Iterator<Item = Arc<RwLock<OhBoy>>> + '_ {
        let values: Vec<Arc<RwLock<OhBoy>>> = self
            .oh_boy
            .read()
            .unwrap()
            .values()
            .map(|oh_boy| oh_boy.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Ownership`] into the store.
    ///
    pub fn inter_ownership(&mut self, ownership: Arc<RwLock<Ownership>>) {
        let read = ownership.read().unwrap();
        self.ownership
            .write()
            .unwrap()
            .insert(read.id(), ownership.clone());
    }

    /// Exhume (get) [`Ownership`] from the store.
    ///
    pub fn exhume_ownership(&self, id: &Uuid) -> Option<Arc<RwLock<Ownership>>> {
        self.ownership
            .read()
            .unwrap()
            .get(id)
            .map(|ownership| ownership.clone())
    }

    /// Exorcise (remove) [`Ownership`] from the store.
    ///
    pub fn exorcise_ownership(&mut self, id: &Uuid) -> Option<Arc<RwLock<Ownership>>> {
        self.ownership
            .write()
            .unwrap()
            .remove(id)
            .map(|ownership| ownership.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Ownership>`.
    ///
    pub fn iter_ownership(&self) -> impl Iterator<Item = Arc<RwLock<Ownership>>> + '_ {
        let values: Vec<Arc<RwLock<Ownership>>> = self
            .ownership
            .read()
            .unwrap()
            .values()
            .map(|ownership| ownership.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`Reference`] into the store.
    ///
    pub fn inter_reference(&mut self, reference: Arc<RwLock<Reference>>) {
        let read = reference.read().unwrap();
        self.reference
            .write()
            .unwrap()
            .insert(read.id, reference.clone());
    }

    /// Exhume (get) [`Reference`] from the store.
    ///
    pub fn exhume_reference(&self, id: &Uuid) -> Option<Arc<RwLock<Reference>>> {
        self.reference
            .read()
            .unwrap()
            .get(id)
            .map(|reference| reference.clone())
    }

    /// Exorcise (remove) [`Reference`] from the store.
    ///
    pub fn exorcise_reference(&mut self, id: &Uuid) -> Option<Arc<RwLock<Reference>>> {
        self.reference
            .write()
            .unwrap()
            .remove(id)
            .map(|reference| reference.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Reference>`.
    ///
    pub fn iter_reference(&self) -> impl Iterator<Item = Arc<RwLock<Reference>>> + '_ {
        let values: Vec<Arc<RwLock<Reference>>> = self
            .reference
            .read()
            .unwrap()
            .values()
            .map(|reference| reference.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`SimpleSubtypeA`] into the store.
    ///
    pub fn inter_simple_subtype_a(&mut self, simple_subtype_a: Arc<RwLock<SimpleSubtypeA>>) {
        let read = simple_subtype_a.read().unwrap();
        self.simple_subtype_a
            .write()
            .unwrap()
            .insert(read.id(), simple_subtype_a.clone());
    }

    /// Exhume (get) [`SimpleSubtypeA`] from the store.
    ///
    pub fn exhume_simple_subtype_a(&self, id: &Uuid) -> Option<Arc<RwLock<SimpleSubtypeA>>> {
        self.simple_subtype_a
            .read()
            .unwrap()
            .get(id)
            .map(|simple_subtype_a| simple_subtype_a.clone())
    }

    /// Exorcise (remove) [`SimpleSubtypeA`] from the store.
    ///
    pub fn exorcise_simple_subtype_a(&mut self, id: &Uuid) -> Option<Arc<RwLock<SimpleSubtypeA>>> {
        self.simple_subtype_a
            .write()
            .unwrap()
            .remove(id)
            .map(|simple_subtype_a| simple_subtype_a.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SimpleSubtypeA>`.
    ///
    pub fn iter_simple_subtype_a(&self) -> impl Iterator<Item = Arc<RwLock<SimpleSubtypeA>>> + '_ {
        let values: Vec<Arc<RwLock<SimpleSubtypeA>>> = self
            .simple_subtype_a
            .read()
            .unwrap()
            .values()
            .map(|simple_subtype_a| simple_subtype_a.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`SimpleSupertype`] into the store.
    ///
    pub fn inter_simple_supertype(&mut self, simple_supertype: Arc<RwLock<SimpleSupertype>>) {
        let read = simple_supertype.read().unwrap();
        self.simple_supertype
            .write()
            .unwrap()
            .insert(read.id, simple_supertype.clone());
    }

    /// Exhume (get) [`SimpleSupertype`] from the store.
    ///
    pub fn exhume_simple_supertype(&self, id: &Uuid) -> Option<Arc<RwLock<SimpleSupertype>>> {
        self.simple_supertype
            .read()
            .unwrap()
            .get(id)
            .map(|simple_supertype| simple_supertype.clone())
    }

    /// Exorcise (remove) [`SimpleSupertype`] from the store.
    ///
    pub fn exorcise_simple_supertype(&mut self, id: &Uuid) -> Option<Arc<RwLock<SimpleSupertype>>> {
        self.simple_supertype
            .write()
            .unwrap()
            .remove(id)
            .map(|simple_supertype| simple_supertype.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SimpleSupertype>`.
    ///
    pub fn iter_simple_supertype(&self) -> impl Iterator<Item = Arc<RwLock<SimpleSupertype>>> + '_ {
        let values: Vec<Arc<RwLock<SimpleSupertype>>> = self
            .simple_supertype
            .read()
            .unwrap()
            .values()
            .map(|simple_supertype| simple_supertype.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`SubtypeA`] into the store.
    ///
    pub fn inter_subtype_a(&mut self, subtype_a: Arc<RwLock<SubtypeA>>) {
        let read = subtype_a.read().unwrap();
        self.subtype_a
            .write()
            .unwrap()
            .insert(read.id, subtype_a.clone());
    }

    /// Exhume (get) [`SubtypeA`] from the store.
    ///
    pub fn exhume_subtype_a(&self, id: &Uuid) -> Option<Arc<RwLock<SubtypeA>>> {
        self.subtype_a
            .read()
            .unwrap()
            .get(id)
            .map(|subtype_a| subtype_a.clone())
    }

    /// Exorcise (remove) [`SubtypeA`] from the store.
    ///
    pub fn exorcise_subtype_a(&mut self, id: &Uuid) -> Option<Arc<RwLock<SubtypeA>>> {
        self.subtype_a
            .write()
            .unwrap()
            .remove(id)
            .map(|subtype_a| subtype_a.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeA>`.
    ///
    pub fn iter_subtype_a(&self) -> impl Iterator<Item = Arc<RwLock<SubtypeA>>> + '_ {
        let values: Vec<Arc<RwLock<SubtypeA>>> = self
            .subtype_a
            .read()
            .unwrap()
            .values()
            .map(|subtype_a| subtype_a.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`SubtypeB`] into the store.
    ///
    pub fn inter_subtype_b(&mut self, subtype_b: Arc<RwLock<SubtypeB>>) {
        let read = subtype_b.read().unwrap();
        self.subtype_b
            .write()
            .unwrap()
            .insert(read.id, subtype_b.clone());
    }

    /// Exhume (get) [`SubtypeB`] from the store.
    ///
    pub fn exhume_subtype_b(&self, id: &Uuid) -> Option<Arc<RwLock<SubtypeB>>> {
        self.subtype_b
            .read()
            .unwrap()
            .get(id)
            .map(|subtype_b| subtype_b.clone())
    }

    /// Exorcise (remove) [`SubtypeB`] from the store.
    ///
    pub fn exorcise_subtype_b(&mut self, id: &Uuid) -> Option<Arc<RwLock<SubtypeB>>> {
        self.subtype_b
            .write()
            .unwrap()
            .remove(id)
            .map(|subtype_b| subtype_b.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeB>`.
    ///
    pub fn iter_subtype_b(&self) -> impl Iterator<Item = Arc<RwLock<SubtypeB>>> + '_ {
        let values: Vec<Arc<RwLock<SubtypeB>>> = self
            .subtype_b
            .read()
            .unwrap()
            .values()
            .map(|subtype_b| subtype_b.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`SuperBar`] into the store.
    ///
    pub fn inter_super_bar(&mut self, super_bar: Arc<RwLock<SuperBar>>) {
        let read = super_bar.read().unwrap();
        self.super_bar
            .write()
            .unwrap()
            .insert(read.id(), super_bar.clone());
    }

    /// Exhume (get) [`SuperBar`] from the store.
    ///
    pub fn exhume_super_bar(&self, id: &Uuid) -> Option<Arc<RwLock<SuperBar>>> {
        self.super_bar
            .read()
            .unwrap()
            .get(id)
            .map(|super_bar| super_bar.clone())
    }

    /// Exorcise (remove) [`SuperBar`] from the store.
    ///
    pub fn exorcise_super_bar(&mut self, id: &Uuid) -> Option<Arc<RwLock<SuperBar>>> {
        self.super_bar
            .write()
            .unwrap()
            .remove(id)
            .map(|super_bar| super_bar.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SuperBar>`.
    ///
    pub fn iter_super_bar(&self) -> impl Iterator<Item = Arc<RwLock<SuperBar>>> + '_ {
        let values: Vec<Arc<RwLock<SuperBar>>> = self
            .super_bar
            .read()
            .unwrap()
            .values()
            .map(|super_bar| super_bar.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`SuperFoo`] into the store.
    ///
    pub fn inter_super_foo(&mut self, super_foo: Arc<RwLock<SuperFoo>>) {
        let read = super_foo.read().unwrap();
        self.super_foo
            .write()
            .unwrap()
            .insert(read.id(), super_foo.clone());
    }

    /// Exhume (get) [`SuperFoo`] from the store.
    ///
    pub fn exhume_super_foo(&self, id: &Uuid) -> Option<Arc<RwLock<SuperFoo>>> {
        self.super_foo
            .read()
            .unwrap()
            .get(id)
            .map(|super_foo| super_foo.clone())
    }

    /// Exorcise (remove) [`SuperFoo`] from the store.
    ///
    pub fn exorcise_super_foo(&mut self, id: &Uuid) -> Option<Arc<RwLock<SuperFoo>>> {
        self.super_foo
            .write()
            .unwrap()
            .remove(id)
            .map(|super_foo| super_foo.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SuperFoo>`.
    ///
    pub fn iter_super_foo(&self) -> impl Iterator<Item = Arc<RwLock<SuperFoo>>> + '_ {
        let values: Vec<Arc<RwLock<SuperFoo>>> = self
            .super_foo
            .read()
            .unwrap()
            .values()
            .map(|super_foo| super_foo.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    /// Inter (insert) [`SuperT`] into the store.
    ///
    pub fn inter_super_t(&mut self, super_t: Arc<RwLock<SuperT>>) {
        let read = super_t.read().unwrap();
        self.super_t
            .write()
            .unwrap()
            .insert(read.id, super_t.clone());
    }

    /// Exhume (get) [`SuperT`] from the store.
    ///
    pub fn exhume_super_t(&self, id: &Uuid) -> Option<Arc<RwLock<SuperT>>> {
        self.super_t
            .read()
            .unwrap()
            .get(id)
            .map(|super_t| super_t.clone())
    }

    /// Exorcise (remove) [`SuperT`] from the store.
    ///
    pub fn exorcise_super_t(&mut self, id: &Uuid) -> Option<Arc<RwLock<SuperT>>> {
        self.super_t
            .write()
            .unwrap()
            .remove(id)
            .map(|super_t| super_t.clone())
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SuperT>`.
    ///
    pub fn iter_super_t(&self) -> impl Iterator<Item = Arc<RwLock<SuperT>>> + '_ {
        let values: Vec<Arc<RwLock<SuperT>>> = self
            .super_t
            .read()
            .unwrap()
            .values()
            .map(|super_t| super_t.clone())
            .collect();
        let len = values.len();
        (0..len).map(move |i| values[i].clone())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa_rwlock-object-store-persistence"}}}
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

        let path = path.join("Isa Relationship.json");
        fs::create_dir_all(&path)?;

        // Persist Alpha.
        {
            let path = path.join("alpha");
            fs::create_dir_all(&path)?;
            for alpha in self.alpha.read().unwrap().values() {
                let path = path.join(format!("{}.json", alpha.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &alpha)?;
            }
        }

        // Persist Baz.
        {
            let path = path.join("baz");
            fs::create_dir_all(&path)?;
            for baz in self.baz.read().unwrap().values() {
                let path = path.join(format!("{}.json", baz.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &baz)?;
            }
        }

        // Persist Beta.
        {
            let path = path.join("beta");
            fs::create_dir_all(&path)?;
            for beta in self.beta.read().unwrap().values() {
                let path = path.join(format!("{}.json", beta.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &beta)?;
            }
        }

        // Persist Borrowed.
        {
            let path = path.join("borrowed");
            fs::create_dir_all(&path)?;
            for borrowed in self.borrowed.read().unwrap().values() {
                let path = path.join(format!("{}.json", borrowed.read().unwrap().id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &borrowed)?;
            }
        }

        // Persist Gamma.
        {
            let path = path.join("gamma");
            fs::create_dir_all(&path)?;
            for gamma in self.gamma.read().unwrap().values() {
                let path = path.join(format!("{}.json", gamma.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &gamma)?;
            }
        }

        // Persist Henry.
        {
            let path = path.join("henry");
            fs::create_dir_all(&path)?;
            for henry in self.henry.read().unwrap().values() {
                let path = path.join(format!("{}.json", henry.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &henry)?;
            }
        }

        // Persist Not Important.
        {
            let path = path.join("not_important");
            fs::create_dir_all(&path)?;
            for not_important in self.not_important.read().unwrap().values() {
                let path = path.join(format!("{}.json", not_important.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &not_important)?;
            }
        }

        // Persist Oh Boy!.
        {
            let path = path.join("oh_boy");
            fs::create_dir_all(&path)?;
            for oh_boy in self.oh_boy.read().unwrap().values() {
                let path = path.join(format!("{}.json", oh_boy.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &oh_boy)?;
            }
        }

        // Persist Ownership.
        {
            let path = path.join("ownership");
            fs::create_dir_all(&path)?;
            for ownership in self.ownership.read().unwrap().values() {
                let path = path.join(format!("{}.json", ownership.read().unwrap().id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &ownership)?;
            }
        }

        // Persist Reference.
        {
            let path = path.join("reference");
            fs::create_dir_all(&path)?;
            for reference in self.reference.read().unwrap().values() {
                let path = path.join(format!("{}.json", reference.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &reference)?;
            }
        }

        // Persist Simple Subtype A.
        {
            let path = path.join("simple_subtype_a");
            fs::create_dir_all(&path)?;
            for simple_subtype_a in self.simple_subtype_a.read().unwrap().values() {
                let path = path.join(format!("{}.json", simple_subtype_a.read().unwrap().id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &simple_subtype_a)?;
            }
        }

        // Persist Simple Supertype.
        {
            let path = path.join("simple_supertype");
            fs::create_dir_all(&path)?;
            for simple_supertype in self.simple_supertype.read().unwrap().values() {
                let path = path.join(format!("{}.json", simple_supertype.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &simple_supertype)?;
            }
        }

        // Persist Subtype A.
        {
            let path = path.join("subtype_a");
            fs::create_dir_all(&path)?;
            for subtype_a in self.subtype_a.read().unwrap().values() {
                let path = path.join(format!("{}.json", subtype_a.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &subtype_a)?;
            }
        }

        // Persist Subtype B.
        {
            let path = path.join("subtype_b");
            fs::create_dir_all(&path)?;
            for subtype_b in self.subtype_b.read().unwrap().values() {
                let path = path.join(format!("{}.json", subtype_b.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &subtype_b)?;
            }
        }

        // Persist Super Bar.
        {
            let path = path.join("super_bar");
            fs::create_dir_all(&path)?;
            for super_bar in self.super_bar.read().unwrap().values() {
                let path = path.join(format!("{}.json", super_bar.read().unwrap().id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &super_bar)?;
            }
        }

        // Persist Super Foo.
        {
            let path = path.join("super_foo");
            fs::create_dir_all(&path)?;
            for super_foo in self.super_foo.read().unwrap().values() {
                let path = path.join(format!("{}.json", super_foo.read().unwrap().id()));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &super_foo)?;
            }
        }

        // Persist Super T.
        {
            let path = path.join("super_t");
            fs::create_dir_all(&path)?;
            for super_t in self.super_t.read().unwrap().values() {
                let path = path.join(format!("{}.json", super_t.read().unwrap().id));
                let file = fs::File::create(path)?;
                let mut writer = io::BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &super_t)?;
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
        let path = path.join("Isa Relationship.json");

        let store = Self::new();

        // Load Alpha.
        {
            let path = path.join("alpha");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let alpha: Arc<RwLock<Alpha>> = serde_json::from_reader(reader)?;
                store
                    .alpha
                    .write()
                    .unwrap()
                    .insert(alpha.read().unwrap().id, alpha.clone());
            }
        }

        // Load Baz.
        {
            let path = path.join("baz");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let baz: Arc<RwLock<Baz>> = serde_json::from_reader(reader)?;
                store
                    .baz
                    .write()
                    .unwrap()
                    .insert(baz.read().unwrap().id, baz.clone());
            }
        }

        // Load Beta.
        {
            let path = path.join("beta");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let beta: Arc<RwLock<Beta>> = serde_json::from_reader(reader)?;
                store
                    .beta
                    .write()
                    .unwrap()
                    .insert(beta.read().unwrap().id, beta.clone());
            }
        }

        // Load Borrowed.
        {
            let path = path.join("borrowed");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let borrowed: Arc<RwLock<Borrowed>> = serde_json::from_reader(reader)?;
                store
                    .borrowed
                    .write()
                    .unwrap()
                    .insert(borrowed.read().unwrap().id(), borrowed.clone());
            }
        }

        // Load Gamma.
        {
            let path = path.join("gamma");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let gamma: Arc<RwLock<Gamma>> = serde_json::from_reader(reader)?;
                store
                    .gamma
                    .write()
                    .unwrap()
                    .insert(gamma.read().unwrap().id, gamma.clone());
            }
        }

        // Load Henry.
        {
            let path = path.join("henry");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let henry: Arc<RwLock<Henry>> = serde_json::from_reader(reader)?;
                store
                    .henry
                    .write()
                    .unwrap()
                    .insert(henry.read().unwrap().id, henry.clone());
            }
        }

        // Load Not Important.
        {
            let path = path.join("not_important");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let not_important: Arc<RwLock<NotImportant>> = serde_json::from_reader(reader)?;
                store
                    .not_important
                    .write()
                    .unwrap()
                    .insert(not_important.read().unwrap().id, not_important.clone());
            }
        }

        // Load Oh Boy!.
        {
            let path = path.join("oh_boy");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let oh_boy: Arc<RwLock<OhBoy>> = serde_json::from_reader(reader)?;
                store
                    .oh_boy
                    .write()
                    .unwrap()
                    .insert(oh_boy.read().unwrap().id, oh_boy.clone());
            }
        }

        // Load Ownership.
        {
            let path = path.join("ownership");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let ownership: Arc<RwLock<Ownership>> = serde_json::from_reader(reader)?;
                store
                    .ownership
                    .write()
                    .unwrap()
                    .insert(ownership.read().unwrap().id(), ownership.clone());
            }
        }

        // Load Reference.
        {
            let path = path.join("reference");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let reference: Arc<RwLock<Reference>> = serde_json::from_reader(reader)?;
                store
                    .reference
                    .write()
                    .unwrap()
                    .insert(reference.read().unwrap().id, reference.clone());
            }
        }

        // Load Simple Subtype A.
        {
            let path = path.join("simple_subtype_a");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let simple_subtype_a: Arc<RwLock<SimpleSubtypeA>> =
                    serde_json::from_reader(reader)?;
                store.simple_subtype_a.write().unwrap().insert(
                    simple_subtype_a.read().unwrap().id(),
                    simple_subtype_a.clone(),
                );
            }
        }

        // Load Simple Supertype.
        {
            let path = path.join("simple_supertype");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let simple_supertype: Arc<RwLock<SimpleSupertype>> =
                    serde_json::from_reader(reader)?;
                store.simple_supertype.write().unwrap().insert(
                    simple_supertype.read().unwrap().id,
                    simple_supertype.clone(),
                );
            }
        }

        // Load Subtype A.
        {
            let path = path.join("subtype_a");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let subtype_a: Arc<RwLock<SubtypeA>> = serde_json::from_reader(reader)?;
                store
                    .subtype_a
                    .write()
                    .unwrap()
                    .insert(subtype_a.read().unwrap().id, subtype_a.clone());
            }
        }

        // Load Subtype B.
        {
            let path = path.join("subtype_b");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let subtype_b: Arc<RwLock<SubtypeB>> = serde_json::from_reader(reader)?;
                store
                    .subtype_b
                    .write()
                    .unwrap()
                    .insert(subtype_b.read().unwrap().id, subtype_b.clone());
            }
        }

        // Load Super Bar.
        {
            let path = path.join("super_bar");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let super_bar: Arc<RwLock<SuperBar>> = serde_json::from_reader(reader)?;
                store
                    .super_bar
                    .write()
                    .unwrap()
                    .insert(super_bar.read().unwrap().id(), super_bar.clone());
            }
        }

        // Load Super Foo.
        {
            let path = path.join("super_foo");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let super_foo: Arc<RwLock<SuperFoo>> = serde_json::from_reader(reader)?;
                store
                    .super_foo
                    .write()
                    .unwrap()
                    .insert(super_foo.read().unwrap().id(), super_foo.clone());
            }
        }

        // Load Super T.
        {
            let path = path.join("super_t");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let super_t: Arc<RwLock<SuperT>> = serde_json::from_reader(reader)?;
                store
                    .super_t
                    .write()
                    .unwrap()
                    .insert(super_t.read().unwrap().id, super_t.clone());
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

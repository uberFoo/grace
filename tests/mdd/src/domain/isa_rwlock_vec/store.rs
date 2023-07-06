//! domain::isa_rwlock_vec Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa_rwlock_vec-object-store-file"}}}
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa_rwlock_vec-object-store-definition"}}}
use std::sync::Arc;
use std::sync::RwLock;
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use fnv::FnvHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::isa_rwlock_vec::types::{
    Alpha, Baz, Beta, Borrowed, Gamma, Henry, NotImportant, OhBoy, Ownership, Reference,
    SimpleSubtypeA, SimpleSupertype, SubtypeA, SubtypeB, SuperBar, SuperFoo, SuperT, MUTABLE,
    OWNED, SHARED, SIMPLE_SUBTYPE_B,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    alpha_free_list: std::sync::Mutex<Vec<usize>>,
    alpha: Arc<RwLock<Vec<Option<Arc<RwLock<Alpha>>>>>>,
    baz_free_list: std::sync::Mutex<Vec<usize>>,
    baz: Arc<RwLock<Vec<Option<Arc<RwLock<Baz>>>>>>,
    beta_free_list: std::sync::Mutex<Vec<usize>>,
    beta: Arc<RwLock<Vec<Option<Arc<RwLock<Beta>>>>>>,
    borrowed_free_list: std::sync::Mutex<Vec<usize>>,
    borrowed: Arc<RwLock<Vec<Option<Arc<RwLock<Borrowed>>>>>>,
    gamma_free_list: std::sync::Mutex<Vec<usize>>,
    gamma: Arc<RwLock<Vec<Option<Arc<RwLock<Gamma>>>>>>,
    henry_free_list: std::sync::Mutex<Vec<usize>>,
    henry: Arc<RwLock<Vec<Option<Arc<RwLock<Henry>>>>>>,
    not_important_free_list: std::sync::Mutex<Vec<usize>>,
    not_important: Arc<RwLock<Vec<Option<Arc<RwLock<NotImportant>>>>>>,
    oh_boy_free_list: std::sync::Mutex<Vec<usize>>,
    oh_boy: Arc<RwLock<Vec<Option<Arc<RwLock<OhBoy>>>>>>,
    ownership_free_list: std::sync::Mutex<Vec<usize>>,
    ownership: Arc<RwLock<Vec<Option<Arc<RwLock<Ownership>>>>>>,
    reference_free_list: std::sync::Mutex<Vec<usize>>,
    reference: Arc<RwLock<Vec<Option<Arc<RwLock<Reference>>>>>>,
    simple_subtype_a_free_list: std::sync::Mutex<Vec<usize>>,
    simple_subtype_a: Arc<RwLock<Vec<Option<Arc<RwLock<SimpleSubtypeA>>>>>>,
    simple_supertype_free_list: std::sync::Mutex<Vec<usize>>,
    simple_supertype: Arc<RwLock<Vec<Option<Arc<RwLock<SimpleSupertype>>>>>>,
    subtype_a_free_list: std::sync::Mutex<Vec<usize>>,
    subtype_a: Arc<RwLock<Vec<Option<Arc<RwLock<SubtypeA>>>>>>,
    subtype_b_free_list: std::sync::Mutex<Vec<usize>>,
    subtype_b: Arc<RwLock<Vec<Option<Arc<RwLock<SubtypeB>>>>>>,
    super_bar_free_list: std::sync::Mutex<Vec<usize>>,
    super_bar: Arc<RwLock<Vec<Option<Arc<RwLock<SuperBar>>>>>>,
    super_foo_free_list: std::sync::Mutex<Vec<usize>>,
    super_foo: Arc<RwLock<Vec<Option<Arc<RwLock<SuperFoo>>>>>>,
    super_t_free_list: std::sync::Mutex<Vec<usize>>,
    super_t: Arc<RwLock<Vec<Option<Arc<RwLock<SuperT>>>>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            alpha_free_list: std::sync::Mutex::new(Vec::new()),
            alpha: Arc::new(RwLock::new(Vec::new())),
            baz_free_list: std::sync::Mutex::new(Vec::new()),
            baz: Arc::new(RwLock::new(Vec::new())),
            beta_free_list: std::sync::Mutex::new(Vec::new()),
            beta: Arc::new(RwLock::new(Vec::new())),
            borrowed_free_list: std::sync::Mutex::new(Vec::new()),
            borrowed: Arc::new(RwLock::new(Vec::new())),
            gamma_free_list: std::sync::Mutex::new(Vec::new()),
            gamma: Arc::new(RwLock::new(Vec::new())),
            henry_free_list: std::sync::Mutex::new(Vec::new()),
            henry: Arc::new(RwLock::new(Vec::new())),
            not_important_free_list: std::sync::Mutex::new(Vec::new()),
            not_important: Arc::new(RwLock::new(Vec::new())),
            oh_boy_free_list: std::sync::Mutex::new(Vec::new()),
            oh_boy: Arc::new(RwLock::new(Vec::new())),
            ownership_free_list: std::sync::Mutex::new(Vec::new()),
            ownership: Arc::new(RwLock::new(Vec::new())),
            reference_free_list: std::sync::Mutex::new(Vec::new()),
            reference: Arc::new(RwLock::new(Vec::new())),
            simple_subtype_a_free_list: std::sync::Mutex::new(Vec::new()),
            simple_subtype_a: Arc::new(RwLock::new(Vec::new())),
            simple_supertype_free_list: std::sync::Mutex::new(Vec::new()),
            simple_supertype: Arc::new(RwLock::new(Vec::new())),
            subtype_a_free_list: std::sync::Mutex::new(Vec::new()),
            subtype_a: Arc::new(RwLock::new(Vec::new())),
            subtype_b_free_list: std::sync::Mutex::new(Vec::new()),
            subtype_b: Arc::new(RwLock::new(Vec::new())),
            super_bar_free_list: std::sync::Mutex::new(Vec::new()),
            super_bar: Arc::new(RwLock::new(Vec::new())),
            super_foo_free_list: std::sync::Mutex::new(Vec::new()),
            super_foo: Arc::new(RwLock::new(Vec::new())),
            super_t_free_list: std::sync::Mutex::new(Vec::new()),
            super_t: Arc::new(RwLock::new(Vec::new())),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥
        store.inter_borrowed(|id| {
            Arc::new(RwLock::new(Borrowed {
                subtype: super::BorrowedEnum::Mutable(MUTABLE),
                id,
            }))
        });
        store.inter_borrowed(|id| {
            Arc::new(RwLock::new(Borrowed {
                subtype: super::BorrowedEnum::Shared(SHARED),
                id,
            }))
        });
        store.inter_ownership(|id| {
            Arc::new(RwLock::new(Ownership {
                subtype: super::OwnershipEnum::Owned(OWNED),
                id,
            }))
        });

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa_rwlock_vec-object-store-methods"}}}
    /// Inter (insert) [`Alpha`] into the store.
    ///
    pub fn inter_alpha<F>(&mut self, alpha: F) -> Arc<RwLock<Alpha>>
    where
        F: Fn(usize) -> Arc<RwLock<Alpha>>,
    {
        if let Some(_index) = self.alpha_free_list.lock().unwrap().pop() {
            let alpha = alpha(_index);
            self.alpha.write().unwrap()[_index] = Some(alpha.clone());
            alpha
        } else {
            let _index = self.alpha.read().unwrap().len();
            let alpha = alpha(_index);
            self.alpha.write().unwrap().push(Some(alpha.clone()));
            alpha
        }
    }

    /// Exhume (get) [`Alpha`] from the store.
    ///
    pub fn exhume_alpha(&self, id: &usize) -> Option<Arc<RwLock<Alpha>>> {
        match self.alpha.read().unwrap().get(*id) {
            Some(alpha) => alpha.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Alpha`] from the store.
    ///
    pub fn exorcise_alpha(&mut self, id: &usize) -> Option<Arc<RwLock<Alpha>>> {
        let result = self.alpha.write().unwrap()[*id].take();
        self.alpha_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Alpha>`.
    ///
    pub fn iter_alpha(&self) -> impl Iterator<Item = Arc<RwLock<Alpha>>> + '_ {
        let len = self.alpha.read().unwrap().len();
        (0..len).map(move |i| {
            self.alpha.read().unwrap()[i]
                .as_ref()
                .map(|alpha| alpha.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Baz`] into the store.
    ///
    pub fn inter_baz<F>(&mut self, baz: F) -> Arc<RwLock<Baz>>
    where
        F: Fn(usize) -> Arc<RwLock<Baz>>,
    {
        if let Some(_index) = self.baz_free_list.lock().unwrap().pop() {
            let baz = baz(_index);
            self.baz.write().unwrap()[_index] = Some(baz.clone());
            baz
        } else {
            let _index = self.baz.read().unwrap().len();
            let baz = baz(_index);
            self.baz.write().unwrap().push(Some(baz.clone()));
            baz
        }
    }

    /// Exhume (get) [`Baz`] from the store.
    ///
    pub fn exhume_baz(&self, id: &usize) -> Option<Arc<RwLock<Baz>>> {
        match self.baz.read().unwrap().get(*id) {
            Some(baz) => baz.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Baz`] from the store.
    ///
    pub fn exorcise_baz(&mut self, id: &usize) -> Option<Arc<RwLock<Baz>>> {
        let result = self.baz.write().unwrap()[*id].take();
        self.baz_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Baz>`.
    ///
    pub fn iter_baz(&self) -> impl Iterator<Item = Arc<RwLock<Baz>>> + '_ {
        let len = self.baz.read().unwrap().len();
        (0..len).map(move |i| {
            self.baz.read().unwrap()[i]
                .as_ref()
                .map(|baz| baz.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Beta`] into the store.
    ///
    pub fn inter_beta<F>(&mut self, beta: F) -> Arc<RwLock<Beta>>
    where
        F: Fn(usize) -> Arc<RwLock<Beta>>,
    {
        if let Some(_index) = self.beta_free_list.lock().unwrap().pop() {
            let beta = beta(_index);
            self.beta.write().unwrap()[_index] = Some(beta.clone());
            beta
        } else {
            let _index = self.beta.read().unwrap().len();
            let beta = beta(_index);
            self.beta.write().unwrap().push(Some(beta.clone()));
            beta
        }
    }

    /// Exhume (get) [`Beta`] from the store.
    ///
    pub fn exhume_beta(&self, id: &usize) -> Option<Arc<RwLock<Beta>>> {
        match self.beta.read().unwrap().get(*id) {
            Some(beta) => beta.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Beta`] from the store.
    ///
    pub fn exorcise_beta(&mut self, id: &usize) -> Option<Arc<RwLock<Beta>>> {
        let result = self.beta.write().unwrap()[*id].take();
        self.beta_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Beta>`.
    ///
    pub fn iter_beta(&self) -> impl Iterator<Item = Arc<RwLock<Beta>>> + '_ {
        let len = self.beta.read().unwrap().len();
        (0..len).map(move |i| {
            self.beta.read().unwrap()[i]
                .as_ref()
                .map(|beta| beta.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Borrowed`] into the store.
    ///
    pub fn inter_borrowed<F>(&mut self, borrowed: F) -> Arc<RwLock<Borrowed>>
    where
        F: Fn(usize) -> Arc<RwLock<Borrowed>>,
    {
        if let Some(_index) = self.borrowed_free_list.lock().unwrap().pop() {
            let borrowed = borrowed(_index);
            self.borrowed.write().unwrap()[_index] = Some(borrowed.clone());
            borrowed
        } else {
            let _index = self.borrowed.read().unwrap().len();
            let borrowed = borrowed(_index);
            self.borrowed.write().unwrap().push(Some(borrowed.clone()));
            borrowed
        }
    }

    /// Exhume (get) [`Borrowed`] from the store.
    ///
    pub fn exhume_borrowed(&self, id: &usize) -> Option<Arc<RwLock<Borrowed>>> {
        match self.borrowed.read().unwrap().get(*id) {
            Some(borrowed) => borrowed.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Borrowed`] from the store.
    ///
    pub fn exorcise_borrowed(&mut self, id: &usize) -> Option<Arc<RwLock<Borrowed>>> {
        let result = self.borrowed.write().unwrap()[*id].take();
        self.borrowed_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Borrowed>`.
    ///
    pub fn iter_borrowed(&self) -> impl Iterator<Item = Arc<RwLock<Borrowed>>> + '_ {
        let len = self.borrowed.read().unwrap().len();
        (0..len).map(move |i| {
            self.borrowed.read().unwrap()[i]
                .as_ref()
                .map(|borrowed| borrowed.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Gamma`] into the store.
    ///
    pub fn inter_gamma<F>(&mut self, gamma: F) -> Arc<RwLock<Gamma>>
    where
        F: Fn(usize) -> Arc<RwLock<Gamma>>,
    {
        if let Some(_index) = self.gamma_free_list.lock().unwrap().pop() {
            let gamma = gamma(_index);
            self.gamma.write().unwrap()[_index] = Some(gamma.clone());
            gamma
        } else {
            let _index = self.gamma.read().unwrap().len();
            let gamma = gamma(_index);
            self.gamma.write().unwrap().push(Some(gamma.clone()));
            gamma
        }
    }

    /// Exhume (get) [`Gamma`] from the store.
    ///
    pub fn exhume_gamma(&self, id: &usize) -> Option<Arc<RwLock<Gamma>>> {
        match self.gamma.read().unwrap().get(*id) {
            Some(gamma) => gamma.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Gamma`] from the store.
    ///
    pub fn exorcise_gamma(&mut self, id: &usize) -> Option<Arc<RwLock<Gamma>>> {
        let result = self.gamma.write().unwrap()[*id].take();
        self.gamma_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Gamma>`.
    ///
    pub fn iter_gamma(&self) -> impl Iterator<Item = Arc<RwLock<Gamma>>> + '_ {
        let len = self.gamma.read().unwrap().len();
        (0..len).map(move |i| {
            self.gamma.read().unwrap()[i]
                .as_ref()
                .map(|gamma| gamma.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Henry`] into the store.
    ///
    pub fn inter_henry<F>(&mut self, henry: F) -> Arc<RwLock<Henry>>
    where
        F: Fn(usize) -> Arc<RwLock<Henry>>,
    {
        if let Some(_index) = self.henry_free_list.lock().unwrap().pop() {
            let henry = henry(_index);
            self.henry.write().unwrap()[_index] = Some(henry.clone());
            henry
        } else {
            let _index = self.henry.read().unwrap().len();
            let henry = henry(_index);
            self.henry.write().unwrap().push(Some(henry.clone()));
            henry
        }
    }

    /// Exhume (get) [`Henry`] from the store.
    ///
    pub fn exhume_henry(&self, id: &usize) -> Option<Arc<RwLock<Henry>>> {
        match self.henry.read().unwrap().get(*id) {
            Some(henry) => henry.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Henry`] from the store.
    ///
    pub fn exorcise_henry(&mut self, id: &usize) -> Option<Arc<RwLock<Henry>>> {
        let result = self.henry.write().unwrap()[*id].take();
        self.henry_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Henry>`.
    ///
    pub fn iter_henry(&self) -> impl Iterator<Item = Arc<RwLock<Henry>>> + '_ {
        let len = self.henry.read().unwrap().len();
        (0..len).map(move |i| {
            self.henry.read().unwrap()[i]
                .as_ref()
                .map(|henry| henry.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`NotImportant`] into the store.
    ///
    pub fn inter_not_important<F>(&mut self, not_important: F) -> Arc<RwLock<NotImportant>>
    where
        F: Fn(usize) -> Arc<RwLock<NotImportant>>,
    {
        if let Some(_index) = self.not_important_free_list.lock().unwrap().pop() {
            let not_important = not_important(_index);
            self.not_important.write().unwrap()[_index] = Some(not_important.clone());
            not_important
        } else {
            let _index = self.not_important.read().unwrap().len();
            let not_important = not_important(_index);
            self.not_important
                .write()
                .unwrap()
                .push(Some(not_important.clone()));
            not_important
        }
    }

    /// Exhume (get) [`NotImportant`] from the store.
    ///
    pub fn exhume_not_important(&self, id: &usize) -> Option<Arc<RwLock<NotImportant>>> {
        match self.not_important.read().unwrap().get(*id) {
            Some(not_important) => not_important.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`NotImportant`] from the store.
    ///
    pub fn exorcise_not_important(&mut self, id: &usize) -> Option<Arc<RwLock<NotImportant>>> {
        let result = self.not_important.write().unwrap()[*id].take();
        self.not_important_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, NotImportant>`.
    ///
    pub fn iter_not_important(&self) -> impl Iterator<Item = Arc<RwLock<NotImportant>>> + '_ {
        let len = self.not_important.read().unwrap().len();
        (0..len).map(move |i| {
            self.not_important.read().unwrap()[i]
                .as_ref()
                .map(|not_important| not_important.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`OhBoy`] into the store.
    ///
    pub fn inter_oh_boy<F>(&mut self, oh_boy: F) -> Arc<RwLock<OhBoy>>
    where
        F: Fn(usize) -> Arc<RwLock<OhBoy>>,
    {
        if let Some(_index) = self.oh_boy_free_list.lock().unwrap().pop() {
            let oh_boy = oh_boy(_index);
            self.oh_boy.write().unwrap()[_index] = Some(oh_boy.clone());
            oh_boy
        } else {
            let _index = self.oh_boy.read().unwrap().len();
            let oh_boy = oh_boy(_index);
            self.oh_boy.write().unwrap().push(Some(oh_boy.clone()));
            oh_boy
        }
    }

    /// Exhume (get) [`OhBoy`] from the store.
    ///
    pub fn exhume_oh_boy(&self, id: &usize) -> Option<Arc<RwLock<OhBoy>>> {
        match self.oh_boy.read().unwrap().get(*id) {
            Some(oh_boy) => oh_boy.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`OhBoy`] from the store.
    ///
    pub fn exorcise_oh_boy(&mut self, id: &usize) -> Option<Arc<RwLock<OhBoy>>> {
        let result = self.oh_boy.write().unwrap()[*id].take();
        self.oh_boy_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, OhBoy>`.
    ///
    pub fn iter_oh_boy(&self) -> impl Iterator<Item = Arc<RwLock<OhBoy>>> + '_ {
        let len = self.oh_boy.read().unwrap().len();
        (0..len).map(move |i| {
            self.oh_boy.read().unwrap()[i]
                .as_ref()
                .map(|oh_boy| oh_boy.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Ownership`] into the store.
    ///
    pub fn inter_ownership<F>(&mut self, ownership: F) -> Arc<RwLock<Ownership>>
    where
        F: Fn(usize) -> Arc<RwLock<Ownership>>,
    {
        if let Some(_index) = self.ownership_free_list.lock().unwrap().pop() {
            let ownership = ownership(_index);
            self.ownership.write().unwrap()[_index] = Some(ownership.clone());
            ownership
        } else {
            let _index = self.ownership.read().unwrap().len();
            let ownership = ownership(_index);
            self.ownership
                .write()
                .unwrap()
                .push(Some(ownership.clone()));
            ownership
        }
    }

    /// Exhume (get) [`Ownership`] from the store.
    ///
    pub fn exhume_ownership(&self, id: &usize) -> Option<Arc<RwLock<Ownership>>> {
        match self.ownership.read().unwrap().get(*id) {
            Some(ownership) => ownership.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Ownership`] from the store.
    ///
    pub fn exorcise_ownership(&mut self, id: &usize) -> Option<Arc<RwLock<Ownership>>> {
        let result = self.ownership.write().unwrap()[*id].take();
        self.ownership_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Ownership>`.
    ///
    pub fn iter_ownership(&self) -> impl Iterator<Item = Arc<RwLock<Ownership>>> + '_ {
        let len = self.ownership.read().unwrap().len();
        (0..len).map(move |i| {
            self.ownership.read().unwrap()[i]
                .as_ref()
                .map(|ownership| ownership.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`Reference`] into the store.
    ///
    pub fn inter_reference<F>(&mut self, reference: F) -> Arc<RwLock<Reference>>
    where
        F: Fn(usize) -> Arc<RwLock<Reference>>,
    {
        if let Some(_index) = self.reference_free_list.lock().unwrap().pop() {
            let reference = reference(_index);
            self.reference.write().unwrap()[_index] = Some(reference.clone());
            reference
        } else {
            let _index = self.reference.read().unwrap().len();
            let reference = reference(_index);
            self.reference
                .write()
                .unwrap()
                .push(Some(reference.clone()));
            reference
        }
    }

    /// Exhume (get) [`Reference`] from the store.
    ///
    pub fn exhume_reference(&self, id: &usize) -> Option<Arc<RwLock<Reference>>> {
        match self.reference.read().unwrap().get(*id) {
            Some(reference) => reference.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Reference`] from the store.
    ///
    pub fn exorcise_reference(&mut self, id: &usize) -> Option<Arc<RwLock<Reference>>> {
        let result = self.reference.write().unwrap()[*id].take();
        self.reference_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Reference>`.
    ///
    pub fn iter_reference(&self) -> impl Iterator<Item = Arc<RwLock<Reference>>> + '_ {
        let len = self.reference.read().unwrap().len();
        (0..len).map(move |i| {
            self.reference.read().unwrap()[i]
                .as_ref()
                .map(|reference| reference.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`SimpleSubtypeA`] into the store.
    ///
    pub fn inter_simple_subtype_a<F>(&mut self, simple_subtype_a: F) -> Arc<RwLock<SimpleSubtypeA>>
    where
        F: Fn(usize) -> Arc<RwLock<SimpleSubtypeA>>,
    {
        if let Some(_index) = self.simple_subtype_a_free_list.lock().unwrap().pop() {
            let simple_subtype_a = simple_subtype_a(_index);
            self.simple_subtype_a.write().unwrap()[_index] = Some(simple_subtype_a.clone());
            simple_subtype_a
        } else {
            let _index = self.simple_subtype_a.read().unwrap().len();
            let simple_subtype_a = simple_subtype_a(_index);
            self.simple_subtype_a
                .write()
                .unwrap()
                .push(Some(simple_subtype_a.clone()));
            simple_subtype_a
        }
    }

    /// Exhume (get) [`SimpleSubtypeA`] from the store.
    ///
    pub fn exhume_simple_subtype_a(&self, id: &usize) -> Option<Arc<RwLock<SimpleSubtypeA>>> {
        match self.simple_subtype_a.read().unwrap().get(*id) {
            Some(simple_subtype_a) => simple_subtype_a.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`SimpleSubtypeA`] from the store.
    ///
    pub fn exorcise_simple_subtype_a(&mut self, id: &usize) -> Option<Arc<RwLock<SimpleSubtypeA>>> {
        let result = self.simple_subtype_a.write().unwrap()[*id].take();
        self.simple_subtype_a_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SimpleSubtypeA>`.
    ///
    pub fn iter_simple_subtype_a(&self) -> impl Iterator<Item = Arc<RwLock<SimpleSubtypeA>>> + '_ {
        let len = self.simple_subtype_a.read().unwrap().len();
        (0..len).map(move |i| {
            self.simple_subtype_a.read().unwrap()[i]
                .as_ref()
                .map(|simple_subtype_a| simple_subtype_a.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`SimpleSupertype`] into the store.
    ///
    pub fn inter_simple_supertype<F>(&mut self, simple_supertype: F) -> Arc<RwLock<SimpleSupertype>>
    where
        F: Fn(usize) -> Arc<RwLock<SimpleSupertype>>,
    {
        if let Some(_index) = self.simple_supertype_free_list.lock().unwrap().pop() {
            let simple_supertype = simple_supertype(_index);
            self.simple_supertype.write().unwrap()[_index] = Some(simple_supertype.clone());
            simple_supertype
        } else {
            let _index = self.simple_supertype.read().unwrap().len();
            let simple_supertype = simple_supertype(_index);
            self.simple_supertype
                .write()
                .unwrap()
                .push(Some(simple_supertype.clone()));
            simple_supertype
        }
    }

    /// Exhume (get) [`SimpleSupertype`] from the store.
    ///
    pub fn exhume_simple_supertype(&self, id: &usize) -> Option<Arc<RwLock<SimpleSupertype>>> {
        match self.simple_supertype.read().unwrap().get(*id) {
            Some(simple_supertype) => simple_supertype.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`SimpleSupertype`] from the store.
    ///
    pub fn exorcise_simple_supertype(
        &mut self,
        id: &usize,
    ) -> Option<Arc<RwLock<SimpleSupertype>>> {
        let result = self.simple_supertype.write().unwrap()[*id].take();
        self.simple_supertype_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SimpleSupertype>`.
    ///
    pub fn iter_simple_supertype(&self) -> impl Iterator<Item = Arc<RwLock<SimpleSupertype>>> + '_ {
        let len = self.simple_supertype.read().unwrap().len();
        (0..len).map(move |i| {
            self.simple_supertype.read().unwrap()[i]
                .as_ref()
                .map(|simple_supertype| simple_supertype.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`SubtypeA`] into the store.
    ///
    pub fn inter_subtype_a<F>(&mut self, subtype_a: F) -> Arc<RwLock<SubtypeA>>
    where
        F: Fn(usize) -> Arc<RwLock<SubtypeA>>,
    {
        if let Some(_index) = self.subtype_a_free_list.lock().unwrap().pop() {
            let subtype_a = subtype_a(_index);
            self.subtype_a.write().unwrap()[_index] = Some(subtype_a.clone());
            subtype_a
        } else {
            let _index = self.subtype_a.read().unwrap().len();
            let subtype_a = subtype_a(_index);
            self.subtype_a
                .write()
                .unwrap()
                .push(Some(subtype_a.clone()));
            subtype_a
        }
    }

    /// Exhume (get) [`SubtypeA`] from the store.
    ///
    pub fn exhume_subtype_a(&self, id: &usize) -> Option<Arc<RwLock<SubtypeA>>> {
        match self.subtype_a.read().unwrap().get(*id) {
            Some(subtype_a) => subtype_a.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`SubtypeA`] from the store.
    ///
    pub fn exorcise_subtype_a(&mut self, id: &usize) -> Option<Arc<RwLock<SubtypeA>>> {
        let result = self.subtype_a.write().unwrap()[*id].take();
        self.subtype_a_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeA>`.
    ///
    pub fn iter_subtype_a(&self) -> impl Iterator<Item = Arc<RwLock<SubtypeA>>> + '_ {
        let len = self.subtype_a.read().unwrap().len();
        (0..len).map(move |i| {
            self.subtype_a.read().unwrap()[i]
                .as_ref()
                .map(|subtype_a| subtype_a.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`SubtypeB`] into the store.
    ///
    pub fn inter_subtype_b<F>(&mut self, subtype_b: F) -> Arc<RwLock<SubtypeB>>
    where
        F: Fn(usize) -> Arc<RwLock<SubtypeB>>,
    {
        if let Some(_index) = self.subtype_b_free_list.lock().unwrap().pop() {
            let subtype_b = subtype_b(_index);
            self.subtype_b.write().unwrap()[_index] = Some(subtype_b.clone());
            subtype_b
        } else {
            let _index = self.subtype_b.read().unwrap().len();
            let subtype_b = subtype_b(_index);
            self.subtype_b
                .write()
                .unwrap()
                .push(Some(subtype_b.clone()));
            subtype_b
        }
    }

    /// Exhume (get) [`SubtypeB`] from the store.
    ///
    pub fn exhume_subtype_b(&self, id: &usize) -> Option<Arc<RwLock<SubtypeB>>> {
        match self.subtype_b.read().unwrap().get(*id) {
            Some(subtype_b) => subtype_b.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`SubtypeB`] from the store.
    ///
    pub fn exorcise_subtype_b(&mut self, id: &usize) -> Option<Arc<RwLock<SubtypeB>>> {
        let result = self.subtype_b.write().unwrap()[*id].take();
        self.subtype_b_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeB>`.
    ///
    pub fn iter_subtype_b(&self) -> impl Iterator<Item = Arc<RwLock<SubtypeB>>> + '_ {
        let len = self.subtype_b.read().unwrap().len();
        (0..len).map(move |i| {
            self.subtype_b.read().unwrap()[i]
                .as_ref()
                .map(|subtype_b| subtype_b.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`SuperBar`] into the store.
    ///
    pub fn inter_super_bar<F>(&mut self, super_bar: F) -> Arc<RwLock<SuperBar>>
    where
        F: Fn(usize) -> Arc<RwLock<SuperBar>>,
    {
        if let Some(_index) = self.super_bar_free_list.lock().unwrap().pop() {
            let super_bar = super_bar(_index);
            self.super_bar.write().unwrap()[_index] = Some(super_bar.clone());
            super_bar
        } else {
            let _index = self.super_bar.read().unwrap().len();
            let super_bar = super_bar(_index);
            self.super_bar
                .write()
                .unwrap()
                .push(Some(super_bar.clone()));
            super_bar
        }
    }

    /// Exhume (get) [`SuperBar`] from the store.
    ///
    pub fn exhume_super_bar(&self, id: &usize) -> Option<Arc<RwLock<SuperBar>>> {
        match self.super_bar.read().unwrap().get(*id) {
            Some(super_bar) => super_bar.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`SuperBar`] from the store.
    ///
    pub fn exorcise_super_bar(&mut self, id: &usize) -> Option<Arc<RwLock<SuperBar>>> {
        let result = self.super_bar.write().unwrap()[*id].take();
        self.super_bar_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SuperBar>`.
    ///
    pub fn iter_super_bar(&self) -> impl Iterator<Item = Arc<RwLock<SuperBar>>> + '_ {
        let len = self.super_bar.read().unwrap().len();
        (0..len).map(move |i| {
            self.super_bar.read().unwrap()[i]
                .as_ref()
                .map(|super_bar| super_bar.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`SuperFoo`] into the store.
    ///
    pub fn inter_super_foo<F>(&mut self, super_foo: F) -> Arc<RwLock<SuperFoo>>
    where
        F: Fn(usize) -> Arc<RwLock<SuperFoo>>,
    {
        if let Some(_index) = self.super_foo_free_list.lock().unwrap().pop() {
            let super_foo = super_foo(_index);
            self.super_foo.write().unwrap()[_index] = Some(super_foo.clone());
            super_foo
        } else {
            let _index = self.super_foo.read().unwrap().len();
            let super_foo = super_foo(_index);
            self.super_foo
                .write()
                .unwrap()
                .push(Some(super_foo.clone()));
            super_foo
        }
    }

    /// Exhume (get) [`SuperFoo`] from the store.
    ///
    pub fn exhume_super_foo(&self, id: &usize) -> Option<Arc<RwLock<SuperFoo>>> {
        match self.super_foo.read().unwrap().get(*id) {
            Some(super_foo) => super_foo.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`SuperFoo`] from the store.
    ///
    pub fn exorcise_super_foo(&mut self, id: &usize) -> Option<Arc<RwLock<SuperFoo>>> {
        let result = self.super_foo.write().unwrap()[*id].take();
        self.super_foo_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SuperFoo>`.
    ///
    pub fn iter_super_foo(&self) -> impl Iterator<Item = Arc<RwLock<SuperFoo>>> + '_ {
        let len = self.super_foo.read().unwrap().len();
        (0..len).map(move |i| {
            self.super_foo.read().unwrap()[i]
                .as_ref()
                .map(|super_foo| super_foo.clone())
                .unwrap()
        })
    }

    /// Inter (insert) [`SuperT`] into the store.
    ///
    pub fn inter_super_t<F>(&mut self, super_t: F) -> Arc<RwLock<SuperT>>
    where
        F: Fn(usize) -> Arc<RwLock<SuperT>>,
    {
        if let Some(_index) = self.super_t_free_list.lock().unwrap().pop() {
            let super_t = super_t(_index);
            self.super_t.write().unwrap()[_index] = Some(super_t.clone());
            super_t
        } else {
            let _index = self.super_t.read().unwrap().len();
            let super_t = super_t(_index);
            self.super_t.write().unwrap().push(Some(super_t.clone()));
            super_t
        }
    }

    /// Exhume (get) [`SuperT`] from the store.
    ///
    pub fn exhume_super_t(&self, id: &usize) -> Option<Arc<RwLock<SuperT>>> {
        match self.super_t.read().unwrap().get(*id) {
            Some(super_t) => super_t.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`SuperT`] from the store.
    ///
    pub fn exorcise_super_t(&mut self, id: &usize) -> Option<Arc<RwLock<SuperT>>> {
        let result = self.super_t.write().unwrap()[*id].take();
        self.super_t_free_list.lock().unwrap().push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SuperT>`.
    ///
    pub fn iter_super_t(&self) -> impl Iterator<Item = Arc<RwLock<SuperT>>> + '_ {
        let len = self.super_t.read().unwrap().len();
        (0..len).map(move |i| {
            self.super_t.read().unwrap()[i]
                .as_ref()
                .map(|super_t| super_t.clone())
                .unwrap()
        })
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa_rwlock_vec-object-store-persistence"}}}
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
            for alpha in &*self.alpha.read().unwrap() {
                if let Some(alpha) = alpha {
                    let path = path.join(format!("{}.json", alpha.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &alpha)?;
                }
            }
        }

        // Persist Baz.
        {
            let path = path.join("baz");
            fs::create_dir_all(&path)?;
            for baz in &*self.baz.read().unwrap() {
                if let Some(baz) = baz {
                    let path = path.join(format!("{}.json", baz.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &baz)?;
                }
            }
        }

        // Persist Beta.
        {
            let path = path.join("beta");
            fs::create_dir_all(&path)?;
            for beta in &*self.beta.read().unwrap() {
                if let Some(beta) = beta {
                    let path = path.join(format!("{}.json", beta.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &beta)?;
                }
            }
        }

        // Persist Borrowed.
        {
            let path = path.join("borrowed");
            fs::create_dir_all(&path)?;
            for borrowed in &*self.borrowed.read().unwrap() {
                if let Some(borrowed) = borrowed {
                    let path = path.join(format!("{}.json", borrowed.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &borrowed)?;
                }
            }
        }

        // Persist Gamma.
        {
            let path = path.join("gamma");
            fs::create_dir_all(&path)?;
            for gamma in &*self.gamma.read().unwrap() {
                if let Some(gamma) = gamma {
                    let path = path.join(format!("{}.json", gamma.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &gamma)?;
                }
            }
        }

        // Persist Henry.
        {
            let path = path.join("henry");
            fs::create_dir_all(&path)?;
            for henry in &*self.henry.read().unwrap() {
                if let Some(henry) = henry {
                    let path = path.join(format!("{}.json", henry.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &henry)?;
                }
            }
        }

        // Persist Not Important.
        {
            let path = path.join("not_important");
            fs::create_dir_all(&path)?;
            for not_important in &*self.not_important.read().unwrap() {
                if let Some(not_important) = not_important {
                    let path = path.join(format!("{}.json", not_important.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &not_important)?;
                }
            }
        }

        // Persist Oh Boy!.
        {
            let path = path.join("oh_boy");
            fs::create_dir_all(&path)?;
            for oh_boy in &*self.oh_boy.read().unwrap() {
                if let Some(oh_boy) = oh_boy {
                    let path = path.join(format!("{}.json", oh_boy.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &oh_boy)?;
                }
            }
        }

        // Persist Ownership.
        {
            let path = path.join("ownership");
            fs::create_dir_all(&path)?;
            for ownership in &*self.ownership.read().unwrap() {
                if let Some(ownership) = ownership {
                    let path = path.join(format!("{}.json", ownership.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &ownership)?;
                }
            }
        }

        // Persist Reference.
        {
            let path = path.join("reference");
            fs::create_dir_all(&path)?;
            for reference in &*self.reference.read().unwrap() {
                if let Some(reference) = reference {
                    let path = path.join(format!("{}.json", reference.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &reference)?;
                }
            }
        }

        // Persist Simple Subtype A.
        {
            let path = path.join("simple_subtype_a");
            fs::create_dir_all(&path)?;
            for simple_subtype_a in &*self.simple_subtype_a.read().unwrap() {
                if let Some(simple_subtype_a) = simple_subtype_a {
                    let path = path.join(format!("{}.json", simple_subtype_a.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &simple_subtype_a)?;
                }
            }
        }

        // Persist Simple Supertype.
        {
            let path = path.join("simple_supertype");
            fs::create_dir_all(&path)?;
            for simple_supertype in &*self.simple_supertype.read().unwrap() {
                if let Some(simple_supertype) = simple_supertype {
                    let path = path.join(format!("{}.json", simple_supertype.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &simple_supertype)?;
                }
            }
        }

        // Persist Subtype A.
        {
            let path = path.join("subtype_a");
            fs::create_dir_all(&path)?;
            for subtype_a in &*self.subtype_a.read().unwrap() {
                if let Some(subtype_a) = subtype_a {
                    let path = path.join(format!("{}.json", subtype_a.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &subtype_a)?;
                }
            }
        }

        // Persist Subtype B.
        {
            let path = path.join("subtype_b");
            fs::create_dir_all(&path)?;
            for subtype_b in &*self.subtype_b.read().unwrap() {
                if let Some(subtype_b) = subtype_b {
                    let path = path.join(format!("{}.json", subtype_b.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &subtype_b)?;
                }
            }
        }

        // Persist Super Bar.
        {
            let path = path.join("super_bar");
            fs::create_dir_all(&path)?;
            for super_bar in &*self.super_bar.read().unwrap() {
                if let Some(super_bar) = super_bar {
                    let path = path.join(format!("{}.json", super_bar.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &super_bar)?;
                }
            }
        }

        // Persist Super Foo.
        {
            let path = path.join("super_foo");
            fs::create_dir_all(&path)?;
            for super_foo in &*self.super_foo.read().unwrap() {
                if let Some(super_foo) = super_foo {
                    let path = path.join(format!("{}.json", super_foo.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &super_foo)?;
                }
            }
        }

        // Persist Super T.
        {
            let path = path.join("super_t");
            fs::create_dir_all(&path)?;
            for super_t in &*self.super_t.read().unwrap() {
                if let Some(super_t) = super_t {
                    let path = path.join(format!("{}.json", super_t.read().unwrap().id));
                    let file = fs::File::create(path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &super_t)?;
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
        let path = path.join("Isa Relationship.json");

        let mut store = Self::new();

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
                    .insert(alpha.read().unwrap().id, Some(alpha.clone()));
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
                    .insert(baz.read().unwrap().id, Some(baz.clone()));
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
                    .insert(beta.read().unwrap().id, Some(beta.clone()));
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
                    .insert(borrowed.read().unwrap().id, Some(borrowed.clone()));
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
                    .insert(gamma.read().unwrap().id, Some(gamma.clone()));
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
                    .insert(henry.read().unwrap().id, Some(henry.clone()));
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
                store.not_important.write().unwrap().insert(
                    not_important.read().unwrap().id,
                    Some(not_important.clone()),
                );
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
                    .insert(oh_boy.read().unwrap().id, Some(oh_boy.clone()));
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
                    .insert(ownership.read().unwrap().id, Some(ownership.clone()));
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
                    .insert(reference.read().unwrap().id, Some(reference.clone()));
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
                    simple_subtype_a.read().unwrap().id,
                    Some(simple_subtype_a.clone()),
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
                    Some(simple_supertype.clone()),
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
                    .insert(subtype_a.read().unwrap().id, Some(subtype_a.clone()));
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
                    .insert(subtype_b.read().unwrap().id, Some(subtype_b.clone()));
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
                    .insert(super_bar.read().unwrap().id, Some(super_bar.clone()));
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
                    .insert(super_foo.read().unwrap().id, Some(super_foo.clone()));
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
                    .insert(super_t.read().unwrap().id, Some(super_t.clone()));
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

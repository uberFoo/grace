//! domain::isa_vec Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa_vec-object-store-file"}}}
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa_vec-object-store-definition"}}}
use std::cell::RefCell;
use std::rc::Rc;
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::isa_vec::types::{
    Alpha, Baz, Beta, Borrowed, Gamma, Henry, NotImportant, OhBoy, Ownership, Reference,
    SimpleSubtypeA, SimpleSupertype, SubtypeA, SubtypeB, SuperBar, SuperFoo, SuperT, MUTABLE,
    OWNED, SHARED, SIMPLE_SUBTYPE_B,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    alpha_free_list: Vec<usize>,
    alpha: Vec<Option<Rc<RefCell<Alpha>>>>,
    baz_free_list: Vec<usize>,
    baz: Vec<Option<Rc<RefCell<Baz>>>>,
    beta_free_list: Vec<usize>,
    beta: Vec<Option<Rc<RefCell<Beta>>>>,
    borrowed_free_list: Vec<usize>,
    borrowed: Vec<Option<Rc<RefCell<Borrowed>>>>,
    gamma_free_list: Vec<usize>,
    gamma: Vec<Option<Rc<RefCell<Gamma>>>>,
    henry_free_list: Vec<usize>,
    henry: Vec<Option<Rc<RefCell<Henry>>>>,
    not_important_free_list: Vec<usize>,
    not_important: Vec<Option<Rc<RefCell<NotImportant>>>>,
    oh_boy_free_list: Vec<usize>,
    oh_boy: Vec<Option<Rc<RefCell<OhBoy>>>>,
    ownership_free_list: Vec<usize>,
    ownership: Vec<Option<Rc<RefCell<Ownership>>>>,
    reference_free_list: Vec<usize>,
    reference: Vec<Option<Rc<RefCell<Reference>>>>,
    simple_subtype_a_free_list: Vec<usize>,
    simple_subtype_a: Vec<Option<Rc<RefCell<SimpleSubtypeA>>>>,
    simple_supertype_free_list: Vec<usize>,
    simple_supertype: Vec<Option<Rc<RefCell<SimpleSupertype>>>>,
    subtype_a_free_list: Vec<usize>,
    subtype_a: Vec<Option<Rc<RefCell<SubtypeA>>>>,
    subtype_b_free_list: Vec<usize>,
    subtype_b: Vec<Option<Rc<RefCell<SubtypeB>>>>,
    super_bar_free_list: Vec<usize>,
    super_bar: Vec<Option<Rc<RefCell<SuperBar>>>>,
    super_foo_free_list: Vec<usize>,
    super_foo: Vec<Option<Rc<RefCell<SuperFoo>>>>,
    super_t_free_list: Vec<usize>,
    super_t: Vec<Option<Rc<RefCell<SuperT>>>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            alpha_free_list: Vec::new(),
            alpha: Vec::new(),
            baz_free_list: Vec::new(),
            baz: Vec::new(),
            beta_free_list: Vec::new(),
            beta: Vec::new(),
            borrowed_free_list: Vec::new(),
            borrowed: Vec::new(),
            gamma_free_list: Vec::new(),
            gamma: Vec::new(),
            henry_free_list: Vec::new(),
            henry: Vec::new(),
            not_important_free_list: Vec::new(),
            not_important: Vec::new(),
            oh_boy_free_list: Vec::new(),
            oh_boy: Vec::new(),
            ownership_free_list: Vec::new(),
            ownership: Vec::new(),
            reference_free_list: Vec::new(),
            reference: Vec::new(),
            simple_subtype_a_free_list: Vec::new(),
            simple_subtype_a: Vec::new(),
            simple_supertype_free_list: Vec::new(),
            simple_supertype: Vec::new(),
            subtype_a_free_list: Vec::new(),
            subtype_a: Vec::new(),
            subtype_b_free_list: Vec::new(),
            subtype_b: Vec::new(),
            super_bar_free_list: Vec::new(),
            super_bar: Vec::new(),
            super_foo_free_list: Vec::new(),
            super_foo: Vec::new(),
            super_t_free_list: Vec::new(),
            super_t: Vec::new(),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥
        store.inter_borrowed(|id| {
            Rc::new(RefCell::new(Borrowed {
                subtype: super::BorrowedEnum::Mutable(MUTABLE),
                id,
            }))
        });
        store.inter_borrowed(|id| {
            Rc::new(RefCell::new(Borrowed {
                subtype: super::BorrowedEnum::Shared(SHARED),
                id,
            }))
        });
        store.inter_ownership(|id| {
            Rc::new(RefCell::new(Ownership {
                subtype: super::OwnershipEnum::Owned(OWNED),
                id,
            }))
        });

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa_vec-object-store-methods"}}}
    /// Inter (insert) [`Alpha`] into the store.
    ///
    pub fn inter_alpha<F>(&mut self, alpha: F) -> Rc<RefCell<Alpha>>
    where
        F: Fn(usize) -> Rc<RefCell<Alpha>>,
    {
        let _index = if let Some(_index) = self.alpha_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.alpha.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.alpha.push(None);
            _index
        };
        let alpha = alpha(_index);
        if let Some(Some(alpha)) = self.alpha.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *alpha.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {alpha:?}.");
            self.alpha_free_list.push(_index);
            alpha.clone()
        } else {
            log::debug!(target: "store", "interring {alpha:?}.");
            self.alpha[_index] = Some(alpha.clone());
            alpha
        }
    }

    /// Exhume (get) [`Alpha`] from the store.
    ///
    pub fn exhume_alpha(&self, id: &usize) -> Option<Rc<RefCell<Alpha>>> {
        match self.alpha.get(*id) {
            Some(alpha) => alpha.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Alpha`] from the store.
    ///
    pub fn exorcise_alpha(&mut self, id: &usize) -> Option<Rc<RefCell<Alpha>>> {
        let result = self.alpha[*id].take();
        self.alpha_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Alpha>`.
    ///
    pub fn iter_alpha(&self) -> impl Iterator<Item = Rc<RefCell<Alpha>>> + '_ {
        let len = self.alpha.len();
        (0..len)
            .filter(|i| self.alpha[*i].is_some())
            .map(move |i| self.alpha[i].as_ref().map(|alpha| alpha.clone()).unwrap())
    }

    /// Inter (insert) [`Baz`] into the store.
    ///
    pub fn inter_baz<F>(&mut self, baz: F) -> Rc<RefCell<Baz>>
    where
        F: Fn(usize) -> Rc<RefCell<Baz>>,
    {
        let _index = if let Some(_index) = self.baz_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.baz.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.baz.push(None);
            _index
        };
        let baz = baz(_index);
        if let Some(Some(baz)) = self.baz.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *baz.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {baz:?}.");
            self.baz_free_list.push(_index);
            baz.clone()
        } else {
            log::debug!(target: "store", "interring {baz:?}.");
            self.baz[_index] = Some(baz.clone());
            baz
        }
    }

    /// Exhume (get) [`Baz`] from the store.
    ///
    pub fn exhume_baz(&self, id: &usize) -> Option<Rc<RefCell<Baz>>> {
        match self.baz.get(*id) {
            Some(baz) => baz.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Baz`] from the store.
    ///
    pub fn exorcise_baz(&mut self, id: &usize) -> Option<Rc<RefCell<Baz>>> {
        let result = self.baz[*id].take();
        self.baz_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Baz>`.
    ///
    pub fn iter_baz(&self) -> impl Iterator<Item = Rc<RefCell<Baz>>> + '_ {
        let len = self.baz.len();
        (0..len)
            .filter(|i| self.baz[*i].is_some())
            .map(move |i| self.baz[i].as_ref().map(|baz| baz.clone()).unwrap())
    }

    /// Inter (insert) [`Beta`] into the store.
    ///
    pub fn inter_beta<F>(&mut self, beta: F) -> Rc<RefCell<Beta>>
    where
        F: Fn(usize) -> Rc<RefCell<Beta>>,
    {
        let _index = if let Some(_index) = self.beta_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.beta.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.beta.push(None);
            _index
        };
        let beta = beta(_index);
        if let Some(Some(beta)) = self.beta.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *beta.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {beta:?}.");
            self.beta_free_list.push(_index);
            beta.clone()
        } else {
            log::debug!(target: "store", "interring {beta:?}.");
            self.beta[_index] = Some(beta.clone());
            beta
        }
    }

    /// Exhume (get) [`Beta`] from the store.
    ///
    pub fn exhume_beta(&self, id: &usize) -> Option<Rc<RefCell<Beta>>> {
        match self.beta.get(*id) {
            Some(beta) => beta.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Beta`] from the store.
    ///
    pub fn exorcise_beta(&mut self, id: &usize) -> Option<Rc<RefCell<Beta>>> {
        let result = self.beta[*id].take();
        self.beta_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Beta>`.
    ///
    pub fn iter_beta(&self) -> impl Iterator<Item = Rc<RefCell<Beta>>> + '_ {
        let len = self.beta.len();
        (0..len)
            .filter(|i| self.beta[*i].is_some())
            .map(move |i| self.beta[i].as_ref().map(|beta| beta.clone()).unwrap())
    }

    /// Inter (insert) [`Borrowed`] into the store.
    ///
    pub fn inter_borrowed<F>(&mut self, borrowed: F) -> Rc<RefCell<Borrowed>>
    where
        F: Fn(usize) -> Rc<RefCell<Borrowed>>,
    {
        let _index = if let Some(_index) = self.borrowed_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.borrowed.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.borrowed.push(None);
            _index
        };
        let borrowed = borrowed(_index);
        if let Some(Some(borrowed)) = self.borrowed.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *borrowed.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {borrowed:?}.");
            self.borrowed_free_list.push(_index);
            borrowed.clone()
        } else {
            log::debug!(target: "store", "interring {borrowed:?}.");
            self.borrowed[_index] = Some(borrowed.clone());
            borrowed
        }
    }

    /// Exhume (get) [`Borrowed`] from the store.
    ///
    pub fn exhume_borrowed(&self, id: &usize) -> Option<Rc<RefCell<Borrowed>>> {
        match self.borrowed.get(*id) {
            Some(borrowed) => borrowed.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Borrowed`] from the store.
    ///
    pub fn exorcise_borrowed(&mut self, id: &usize) -> Option<Rc<RefCell<Borrowed>>> {
        let result = self.borrowed[*id].take();
        self.borrowed_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Borrowed>`.
    ///
    pub fn iter_borrowed(&self) -> impl Iterator<Item = Rc<RefCell<Borrowed>>> + '_ {
        let len = self.borrowed.len();
        (0..len)
            .filter(|i| self.borrowed[*i].is_some())
            .map(move |i| {
                self.borrowed[i]
                    .as_ref()
                    .map(|borrowed| borrowed.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Gamma`] into the store.
    ///
    pub fn inter_gamma<F>(&mut self, gamma: F) -> Rc<RefCell<Gamma>>
    where
        F: Fn(usize) -> Rc<RefCell<Gamma>>,
    {
        let _index = if let Some(_index) = self.gamma_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.gamma.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.gamma.push(None);
            _index
        };
        let gamma = gamma(_index);
        if let Some(Some(gamma)) = self.gamma.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *gamma.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {gamma:?}.");
            self.gamma_free_list.push(_index);
            gamma.clone()
        } else {
            log::debug!(target: "store", "interring {gamma:?}.");
            self.gamma[_index] = Some(gamma.clone());
            gamma
        }
    }

    /// Exhume (get) [`Gamma`] from the store.
    ///
    pub fn exhume_gamma(&self, id: &usize) -> Option<Rc<RefCell<Gamma>>> {
        match self.gamma.get(*id) {
            Some(gamma) => gamma.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Gamma`] from the store.
    ///
    pub fn exorcise_gamma(&mut self, id: &usize) -> Option<Rc<RefCell<Gamma>>> {
        let result = self.gamma[*id].take();
        self.gamma_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Gamma>`.
    ///
    pub fn iter_gamma(&self) -> impl Iterator<Item = Rc<RefCell<Gamma>>> + '_ {
        let len = self.gamma.len();
        (0..len)
            .filter(|i| self.gamma[*i].is_some())
            .map(move |i| self.gamma[i].as_ref().map(|gamma| gamma.clone()).unwrap())
    }

    /// Inter (insert) [`Henry`] into the store.
    ///
    pub fn inter_henry<F>(&mut self, henry: F) -> Rc<RefCell<Henry>>
    where
        F: Fn(usize) -> Rc<RefCell<Henry>>,
    {
        let _index = if let Some(_index) = self.henry_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.henry.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.henry.push(None);
            _index
        };
        let henry = henry(_index);
        if let Some(Some(henry)) = self.henry.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *henry.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {henry:?}.");
            self.henry_free_list.push(_index);
            henry.clone()
        } else {
            log::debug!(target: "store", "interring {henry:?}.");
            self.henry[_index] = Some(henry.clone());
            henry
        }
    }

    /// Exhume (get) [`Henry`] from the store.
    ///
    pub fn exhume_henry(&self, id: &usize) -> Option<Rc<RefCell<Henry>>> {
        match self.henry.get(*id) {
            Some(henry) => henry.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Henry`] from the store.
    ///
    pub fn exorcise_henry(&mut self, id: &usize) -> Option<Rc<RefCell<Henry>>> {
        let result = self.henry[*id].take();
        self.henry_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Henry>`.
    ///
    pub fn iter_henry(&self) -> impl Iterator<Item = Rc<RefCell<Henry>>> + '_ {
        let len = self.henry.len();
        (0..len)
            .filter(|i| self.henry[*i].is_some())
            .map(move |i| self.henry[i].as_ref().map(|henry| henry.clone()).unwrap())
    }

    /// Inter (insert) [`NotImportant`] into the store.
    ///
    pub fn inter_not_important<F>(&mut self, not_important: F) -> Rc<RefCell<NotImportant>>
    where
        F: Fn(usize) -> Rc<RefCell<NotImportant>>,
    {
        let _index = if let Some(_index) = self.not_important_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.not_important.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.not_important.push(None);
            _index
        };
        let not_important = not_important(_index);
        if let Some(Some(not_important)) = self.not_important.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *not_important.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {not_important:?}.");
            self.not_important_free_list.push(_index);
            not_important.clone()
        } else {
            log::debug!(target: "store", "interring {not_important:?}.");
            self.not_important[_index] = Some(not_important.clone());
            not_important
        }
    }

    /// Exhume (get) [`NotImportant`] from the store.
    ///
    pub fn exhume_not_important(&self, id: &usize) -> Option<Rc<RefCell<NotImportant>>> {
        match self.not_important.get(*id) {
            Some(not_important) => not_important.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`NotImportant`] from the store.
    ///
    pub fn exorcise_not_important(&mut self, id: &usize) -> Option<Rc<RefCell<NotImportant>>> {
        let result = self.not_important[*id].take();
        self.not_important_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, NotImportant>`.
    ///
    pub fn iter_not_important(&self) -> impl Iterator<Item = Rc<RefCell<NotImportant>>> + '_ {
        let len = self.not_important.len();
        (0..len)
            .filter(|i| self.not_important[*i].is_some())
            .map(move |i| {
                self.not_important[i]
                    .as_ref()
                    .map(|not_important| not_important.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`OhBoy`] into the store.
    ///
    pub fn inter_oh_boy<F>(&mut self, oh_boy: F) -> Rc<RefCell<OhBoy>>
    where
        F: Fn(usize) -> Rc<RefCell<OhBoy>>,
    {
        let _index = if let Some(_index) = self.oh_boy_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.oh_boy.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.oh_boy.push(None);
            _index
        };
        let oh_boy = oh_boy(_index);
        if let Some(Some(oh_boy)) = self.oh_boy.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *oh_boy.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {oh_boy:?}.");
            self.oh_boy_free_list.push(_index);
            oh_boy.clone()
        } else {
            log::debug!(target: "store", "interring {oh_boy:?}.");
            self.oh_boy[_index] = Some(oh_boy.clone());
            oh_boy
        }
    }

    /// Exhume (get) [`OhBoy`] from the store.
    ///
    pub fn exhume_oh_boy(&self, id: &usize) -> Option<Rc<RefCell<OhBoy>>> {
        match self.oh_boy.get(*id) {
            Some(oh_boy) => oh_boy.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`OhBoy`] from the store.
    ///
    pub fn exorcise_oh_boy(&mut self, id: &usize) -> Option<Rc<RefCell<OhBoy>>> {
        let result = self.oh_boy[*id].take();
        self.oh_boy_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, OhBoy>`.
    ///
    pub fn iter_oh_boy(&self) -> impl Iterator<Item = Rc<RefCell<OhBoy>>> + '_ {
        let len = self.oh_boy.len();
        (0..len)
            .filter(|i| self.oh_boy[*i].is_some())
            .map(move |i| {
                self.oh_boy[i]
                    .as_ref()
                    .map(|oh_boy| oh_boy.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Ownership`] into the store.
    ///
    pub fn inter_ownership<F>(&mut self, ownership: F) -> Rc<RefCell<Ownership>>
    where
        F: Fn(usize) -> Rc<RefCell<Ownership>>,
    {
        let _index = if let Some(_index) = self.ownership_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.ownership.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.ownership.push(None);
            _index
        };
        let ownership = ownership(_index);
        if let Some(Some(ownership)) = self.ownership.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *ownership.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {ownership:?}.");
            self.ownership_free_list.push(_index);
            ownership.clone()
        } else {
            log::debug!(target: "store", "interring {ownership:?}.");
            self.ownership[_index] = Some(ownership.clone());
            ownership
        }
    }

    /// Exhume (get) [`Ownership`] from the store.
    ///
    pub fn exhume_ownership(&self, id: &usize) -> Option<Rc<RefCell<Ownership>>> {
        match self.ownership.get(*id) {
            Some(ownership) => ownership.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Ownership`] from the store.
    ///
    pub fn exorcise_ownership(&mut self, id: &usize) -> Option<Rc<RefCell<Ownership>>> {
        let result = self.ownership[*id].take();
        self.ownership_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Ownership>`.
    ///
    pub fn iter_ownership(&self) -> impl Iterator<Item = Rc<RefCell<Ownership>>> + '_ {
        let len = self.ownership.len();
        (0..len)
            .filter(|i| self.ownership[*i].is_some())
            .map(move |i| {
                self.ownership[i]
                    .as_ref()
                    .map(|ownership| ownership.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`Reference`] into the store.
    ///
    pub fn inter_reference<F>(&mut self, reference: F) -> Rc<RefCell<Reference>>
    where
        F: Fn(usize) -> Rc<RefCell<Reference>>,
    {
        let _index = if let Some(_index) = self.reference_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.reference.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.reference.push(None);
            _index
        };
        let reference = reference(_index);
        if let Some(Some(reference)) = self.reference.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *reference.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {reference:?}.");
            self.reference_free_list.push(_index);
            reference.clone()
        } else {
            log::debug!(target: "store", "interring {reference:?}.");
            self.reference[_index] = Some(reference.clone());
            reference
        }
    }

    /// Exhume (get) [`Reference`] from the store.
    ///
    pub fn exhume_reference(&self, id: &usize) -> Option<Rc<RefCell<Reference>>> {
        match self.reference.get(*id) {
            Some(reference) => reference.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`Reference`] from the store.
    ///
    pub fn exorcise_reference(&mut self, id: &usize) -> Option<Rc<RefCell<Reference>>> {
        let result = self.reference[*id].take();
        self.reference_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Reference>`.
    ///
    pub fn iter_reference(&self) -> impl Iterator<Item = Rc<RefCell<Reference>>> + '_ {
        let len = self.reference.len();
        (0..len)
            .filter(|i| self.reference[*i].is_some())
            .map(move |i| {
                self.reference[i]
                    .as_ref()
                    .map(|reference| reference.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`SimpleSubtypeA`] into the store.
    ///
    pub fn inter_simple_subtype_a<F>(&mut self, simple_subtype_a: F) -> Rc<RefCell<SimpleSubtypeA>>
    where
        F: Fn(usize) -> Rc<RefCell<SimpleSubtypeA>>,
    {
        let _index = if let Some(_index) = self.simple_subtype_a_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.simple_subtype_a.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.simple_subtype_a.push(None);
            _index
        };
        let simple_subtype_a = simple_subtype_a(_index);
        if let Some(Some(simple_subtype_a)) = self.simple_subtype_a.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *simple_subtype_a.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {simple_subtype_a:?}.");
            self.simple_subtype_a_free_list.push(_index);
            simple_subtype_a.clone()
        } else {
            log::debug!(target: "store", "interring {simple_subtype_a:?}.");
            self.simple_subtype_a[_index] = Some(simple_subtype_a.clone());
            simple_subtype_a
        }
    }

    /// Exhume (get) [`SimpleSubtypeA`] from the store.
    ///
    pub fn exhume_simple_subtype_a(&self, id: &usize) -> Option<Rc<RefCell<SimpleSubtypeA>>> {
        match self.simple_subtype_a.get(*id) {
            Some(simple_subtype_a) => simple_subtype_a.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`SimpleSubtypeA`] from the store.
    ///
    pub fn exorcise_simple_subtype_a(&mut self, id: &usize) -> Option<Rc<RefCell<SimpleSubtypeA>>> {
        let result = self.simple_subtype_a[*id].take();
        self.simple_subtype_a_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SimpleSubtypeA>`.
    ///
    pub fn iter_simple_subtype_a(&self) -> impl Iterator<Item = Rc<RefCell<SimpleSubtypeA>>> + '_ {
        let len = self.simple_subtype_a.len();
        (0..len)
            .filter(|i| self.simple_subtype_a[*i].is_some())
            .map(move |i| {
                self.simple_subtype_a[i]
                    .as_ref()
                    .map(|simple_subtype_a| simple_subtype_a.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`SimpleSupertype`] into the store.
    ///
    pub fn inter_simple_supertype<F>(&mut self, simple_supertype: F) -> Rc<RefCell<SimpleSupertype>>
    where
        F: Fn(usize) -> Rc<RefCell<SimpleSupertype>>,
    {
        let _index = if let Some(_index) = self.simple_supertype_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.simple_supertype.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.simple_supertype.push(None);
            _index
        };
        let simple_supertype = simple_supertype(_index);
        if let Some(Some(simple_supertype)) = self.simple_supertype.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *simple_supertype.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {simple_supertype:?}.");
            self.simple_supertype_free_list.push(_index);
            simple_supertype.clone()
        } else {
            log::debug!(target: "store", "interring {simple_supertype:?}.");
            self.simple_supertype[_index] = Some(simple_supertype.clone());
            simple_supertype
        }
    }

    /// Exhume (get) [`SimpleSupertype`] from the store.
    ///
    pub fn exhume_simple_supertype(&self, id: &usize) -> Option<Rc<RefCell<SimpleSupertype>>> {
        match self.simple_supertype.get(*id) {
            Some(simple_supertype) => simple_supertype.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`SimpleSupertype`] from the store.
    ///
    pub fn exorcise_simple_supertype(
        &mut self,
        id: &usize,
    ) -> Option<Rc<RefCell<SimpleSupertype>>> {
        let result = self.simple_supertype[*id].take();
        self.simple_supertype_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SimpleSupertype>`.
    ///
    pub fn iter_simple_supertype(&self) -> impl Iterator<Item = Rc<RefCell<SimpleSupertype>>> + '_ {
        let len = self.simple_supertype.len();
        (0..len)
            .filter(|i| self.simple_supertype[*i].is_some())
            .map(move |i| {
                self.simple_supertype[i]
                    .as_ref()
                    .map(|simple_supertype| simple_supertype.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`SubtypeA`] into the store.
    ///
    pub fn inter_subtype_a<F>(&mut self, subtype_a: F) -> Rc<RefCell<SubtypeA>>
    where
        F: Fn(usize) -> Rc<RefCell<SubtypeA>>,
    {
        let _index = if let Some(_index) = self.subtype_a_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.subtype_a.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.subtype_a.push(None);
            _index
        };
        let subtype_a = subtype_a(_index);
        if let Some(Some(subtype_a)) = self.subtype_a.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *subtype_a.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {subtype_a:?}.");
            self.subtype_a_free_list.push(_index);
            subtype_a.clone()
        } else {
            log::debug!(target: "store", "interring {subtype_a:?}.");
            self.subtype_a[_index] = Some(subtype_a.clone());
            subtype_a
        }
    }

    /// Exhume (get) [`SubtypeA`] from the store.
    ///
    pub fn exhume_subtype_a(&self, id: &usize) -> Option<Rc<RefCell<SubtypeA>>> {
        match self.subtype_a.get(*id) {
            Some(subtype_a) => subtype_a.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`SubtypeA`] from the store.
    ///
    pub fn exorcise_subtype_a(&mut self, id: &usize) -> Option<Rc<RefCell<SubtypeA>>> {
        let result = self.subtype_a[*id].take();
        self.subtype_a_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeA>`.
    ///
    pub fn iter_subtype_a(&self) -> impl Iterator<Item = Rc<RefCell<SubtypeA>>> + '_ {
        let len = self.subtype_a.len();
        (0..len)
            .filter(|i| self.subtype_a[*i].is_some())
            .map(move |i| {
                self.subtype_a[i]
                    .as_ref()
                    .map(|subtype_a| subtype_a.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`SubtypeB`] into the store.
    ///
    pub fn inter_subtype_b<F>(&mut self, subtype_b: F) -> Rc<RefCell<SubtypeB>>
    where
        F: Fn(usize) -> Rc<RefCell<SubtypeB>>,
    {
        let _index = if let Some(_index) = self.subtype_b_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.subtype_b.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.subtype_b.push(None);
            _index
        };
        let subtype_b = subtype_b(_index);
        if let Some(Some(subtype_b)) = self.subtype_b.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *subtype_b.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {subtype_b:?}.");
            self.subtype_b_free_list.push(_index);
            subtype_b.clone()
        } else {
            log::debug!(target: "store", "interring {subtype_b:?}.");
            self.subtype_b[_index] = Some(subtype_b.clone());
            subtype_b
        }
    }

    /// Exhume (get) [`SubtypeB`] from the store.
    ///
    pub fn exhume_subtype_b(&self, id: &usize) -> Option<Rc<RefCell<SubtypeB>>> {
        match self.subtype_b.get(*id) {
            Some(subtype_b) => subtype_b.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`SubtypeB`] from the store.
    ///
    pub fn exorcise_subtype_b(&mut self, id: &usize) -> Option<Rc<RefCell<SubtypeB>>> {
        let result = self.subtype_b[*id].take();
        self.subtype_b_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeB>`.
    ///
    pub fn iter_subtype_b(&self) -> impl Iterator<Item = Rc<RefCell<SubtypeB>>> + '_ {
        let len = self.subtype_b.len();
        (0..len)
            .filter(|i| self.subtype_b[*i].is_some())
            .map(move |i| {
                self.subtype_b[i]
                    .as_ref()
                    .map(|subtype_b| subtype_b.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`SuperBar`] into the store.
    ///
    pub fn inter_super_bar<F>(&mut self, super_bar: F) -> Rc<RefCell<SuperBar>>
    where
        F: Fn(usize) -> Rc<RefCell<SuperBar>>,
    {
        let _index = if let Some(_index) = self.super_bar_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.super_bar.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.super_bar.push(None);
            _index
        };
        let super_bar = super_bar(_index);
        if let Some(Some(super_bar)) = self.super_bar.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *super_bar.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {super_bar:?}.");
            self.super_bar_free_list.push(_index);
            super_bar.clone()
        } else {
            log::debug!(target: "store", "interring {super_bar:?}.");
            self.super_bar[_index] = Some(super_bar.clone());
            super_bar
        }
    }

    /// Exhume (get) [`SuperBar`] from the store.
    ///
    pub fn exhume_super_bar(&self, id: &usize) -> Option<Rc<RefCell<SuperBar>>> {
        match self.super_bar.get(*id) {
            Some(super_bar) => super_bar.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`SuperBar`] from the store.
    ///
    pub fn exorcise_super_bar(&mut self, id: &usize) -> Option<Rc<RefCell<SuperBar>>> {
        let result = self.super_bar[*id].take();
        self.super_bar_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SuperBar>`.
    ///
    pub fn iter_super_bar(&self) -> impl Iterator<Item = Rc<RefCell<SuperBar>>> + '_ {
        let len = self.super_bar.len();
        (0..len)
            .filter(|i| self.super_bar[*i].is_some())
            .map(move |i| {
                self.super_bar[i]
                    .as_ref()
                    .map(|super_bar| super_bar.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`SuperFoo`] into the store.
    ///
    pub fn inter_super_foo<F>(&mut self, super_foo: F) -> Rc<RefCell<SuperFoo>>
    where
        F: Fn(usize) -> Rc<RefCell<SuperFoo>>,
    {
        let _index = if let Some(_index) = self.super_foo_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.super_foo.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.super_foo.push(None);
            _index
        };
        let super_foo = super_foo(_index);
        if let Some(Some(super_foo)) = self.super_foo.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *super_foo.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {super_foo:?}.");
            self.super_foo_free_list.push(_index);
            super_foo.clone()
        } else {
            log::debug!(target: "store", "interring {super_foo:?}.");
            self.super_foo[_index] = Some(super_foo.clone());
            super_foo
        }
    }

    /// Exhume (get) [`SuperFoo`] from the store.
    ///
    pub fn exhume_super_foo(&self, id: &usize) -> Option<Rc<RefCell<SuperFoo>>> {
        match self.super_foo.get(*id) {
            Some(super_foo) => super_foo.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`SuperFoo`] from the store.
    ///
    pub fn exorcise_super_foo(&mut self, id: &usize) -> Option<Rc<RefCell<SuperFoo>>> {
        let result = self.super_foo[*id].take();
        self.super_foo_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SuperFoo>`.
    ///
    pub fn iter_super_foo(&self) -> impl Iterator<Item = Rc<RefCell<SuperFoo>>> + '_ {
        let len = self.super_foo.len();
        (0..len)
            .filter(|i| self.super_foo[*i].is_some())
            .map(move |i| {
                self.super_foo[i]
                    .as_ref()
                    .map(|super_foo| super_foo.clone())
                    .unwrap()
            })
    }

    /// Inter (insert) [`SuperT`] into the store.
    ///
    pub fn inter_super_t<F>(&mut self, super_t: F) -> Rc<RefCell<SuperT>>
    where
        F: Fn(usize) -> Rc<RefCell<SuperT>>,
    {
        let _index = if let Some(_index) = self.super_t_free_list.pop() {
            log::trace!(target: "store", "recycling block {_index}.");
            _index
        } else {
            let _index = self.super_t.len();
            log::trace!(target: "store", "allocating block {_index}.");
            self.super_t.push(None);
            _index
        };
        let super_t = super_t(_index);
        if let Some(Some(super_t)) = self.super_t.iter().find(|stored| {
            if let Some(stored) = stored {
                *stored.borrow() == *super_t.borrow()
            } else {
                false
            }
        }) {
            log::debug!(target: "store", "found duplicate {super_t:?}.");
            self.super_t_free_list.push(_index);
            super_t.clone()
        } else {
            log::debug!(target: "store", "interring {super_t:?}.");
            self.super_t[_index] = Some(super_t.clone());
            super_t
        }
    }

    /// Exhume (get) [`SuperT`] from the store.
    ///
    pub fn exhume_super_t(&self, id: &usize) -> Option<Rc<RefCell<SuperT>>> {
        match self.super_t.get(*id) {
            Some(super_t) => super_t.clone(),
            None => None,
        }
    }

    /// Exorcise (remove) [`SuperT`] from the store.
    ///
    pub fn exorcise_super_t(&mut self, id: &usize) -> Option<Rc<RefCell<SuperT>>> {
        let result = self.super_t[*id].take();
        self.super_t_free_list.push(*id);
        result
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SuperT>`.
    ///
    pub fn iter_super_t(&self) -> impl Iterator<Item = Rc<RefCell<SuperT>>> + '_ {
        let len = self.super_t.len();
        (0..len)
            .filter(|i| self.super_t[*i].is_some())
            .map(move |i| {
                self.super_t[i]
                    .as_ref()
                    .map(|super_t| super_t.clone())
                    .unwrap()
            })
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa_vec-object-store-persistence"}}}
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
            for alpha in &self.alpha {
                if let Some(alpha) = alpha {
                    let path = path.join(format!("{}.json", alpha.borrow().id));
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
            for baz in &self.baz {
                if let Some(baz) = baz {
                    let path = path.join(format!("{}.json", baz.borrow().id));
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
            for beta in &self.beta {
                if let Some(beta) = beta {
                    let path = path.join(format!("{}.json", beta.borrow().id));
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
            for borrowed in &self.borrowed {
                if let Some(borrowed) = borrowed {
                    let path = path.join(format!("{}.json", borrowed.borrow().id));
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
            for gamma in &self.gamma {
                if let Some(gamma) = gamma {
                    let path = path.join(format!("{}.json", gamma.borrow().id));
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
            for henry in &self.henry {
                if let Some(henry) = henry {
                    let path = path.join(format!("{}.json", henry.borrow().id));
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
            for not_important in &self.not_important {
                if let Some(not_important) = not_important {
                    let path = path.join(format!("{}.json", not_important.borrow().id));
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
            for oh_boy in &self.oh_boy {
                if let Some(oh_boy) = oh_boy {
                    let path = path.join(format!("{}.json", oh_boy.borrow().id));
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
            for ownership in &self.ownership {
                if let Some(ownership) = ownership {
                    let path = path.join(format!("{}.json", ownership.borrow().id));
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
            for reference in &self.reference {
                if let Some(reference) = reference {
                    let path = path.join(format!("{}.json", reference.borrow().id));
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
            for simple_subtype_a in &self.simple_subtype_a {
                if let Some(simple_subtype_a) = simple_subtype_a {
                    let path = path.join(format!("{}.json", simple_subtype_a.borrow().id));
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
            for simple_supertype in &self.simple_supertype {
                if let Some(simple_supertype) = simple_supertype {
                    let path = path.join(format!("{}.json", simple_supertype.borrow().id));
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
            for subtype_a in &self.subtype_a {
                if let Some(subtype_a) = subtype_a {
                    let path = path.join(format!("{}.json", subtype_a.borrow().id));
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
            for subtype_b in &self.subtype_b {
                if let Some(subtype_b) = subtype_b {
                    let path = path.join(format!("{}.json", subtype_b.borrow().id));
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
            for super_bar in &self.super_bar {
                if let Some(super_bar) = super_bar {
                    let path = path.join(format!("{}.json", super_bar.borrow().id));
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
            for super_foo in &self.super_foo {
                if let Some(super_foo) = super_foo {
                    let path = path.join(format!("{}.json", super_foo.borrow().id));
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
            for super_t in &self.super_t {
                if let Some(super_t) = super_t {
                    let path = path.join(format!("{}.json", super_t.borrow().id));
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
                let alpha: Rc<RefCell<Alpha>> = serde_json::from_reader(reader)?;
                store.alpha.insert(alpha.borrow().id, Some(alpha.clone()));
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
                let baz: Rc<RefCell<Baz>> = serde_json::from_reader(reader)?;
                store.baz.insert(baz.borrow().id, Some(baz.clone()));
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
                let beta: Rc<RefCell<Beta>> = serde_json::from_reader(reader)?;
                store.beta.insert(beta.borrow().id, Some(beta.clone()));
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
                let borrowed: Rc<RefCell<Borrowed>> = serde_json::from_reader(reader)?;
                store
                    .borrowed
                    .insert(borrowed.borrow().id, Some(borrowed.clone()));
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
                let gamma: Rc<RefCell<Gamma>> = serde_json::from_reader(reader)?;
                store.gamma.insert(gamma.borrow().id, Some(gamma.clone()));
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
                let henry: Rc<RefCell<Henry>> = serde_json::from_reader(reader)?;
                store.henry.insert(henry.borrow().id, Some(henry.clone()));
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
                let not_important: Rc<RefCell<NotImportant>> = serde_json::from_reader(reader)?;
                store
                    .not_important
                    .insert(not_important.borrow().id, Some(not_important.clone()));
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
                let oh_boy: Rc<RefCell<OhBoy>> = serde_json::from_reader(reader)?;
                store
                    .oh_boy
                    .insert(oh_boy.borrow().id, Some(oh_boy.clone()));
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
                let ownership: Rc<RefCell<Ownership>> = serde_json::from_reader(reader)?;
                store
                    .ownership
                    .insert(ownership.borrow().id, Some(ownership.clone()));
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
                let reference: Rc<RefCell<Reference>> = serde_json::from_reader(reader)?;
                store
                    .reference
                    .insert(reference.borrow().id, Some(reference.clone()));
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
                let simple_subtype_a: Rc<RefCell<SimpleSubtypeA>> =
                    serde_json::from_reader(reader)?;
                store
                    .simple_subtype_a
                    .insert(simple_subtype_a.borrow().id, Some(simple_subtype_a.clone()));
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
                let simple_supertype: Rc<RefCell<SimpleSupertype>> =
                    serde_json::from_reader(reader)?;
                store
                    .simple_supertype
                    .insert(simple_supertype.borrow().id, Some(simple_supertype.clone()));
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
                let subtype_a: Rc<RefCell<SubtypeA>> = serde_json::from_reader(reader)?;
                store
                    .subtype_a
                    .insert(subtype_a.borrow().id, Some(subtype_a.clone()));
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
                let subtype_b: Rc<RefCell<SubtypeB>> = serde_json::from_reader(reader)?;
                store
                    .subtype_b
                    .insert(subtype_b.borrow().id, Some(subtype_b.clone()));
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
                let super_bar: Rc<RefCell<SuperBar>> = serde_json::from_reader(reader)?;
                store
                    .super_bar
                    .insert(super_bar.borrow().id, Some(super_bar.clone()));
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
                let super_foo: Rc<RefCell<SuperFoo>> = serde_json::from_reader(reader)?;
                store
                    .super_foo
                    .insert(super_foo.borrow().id, Some(super_foo.clone()));
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
                let super_t: Rc<RefCell<SuperT>> = serde_json::from_reader(reader)?;
                store
                    .super_t
                    .insert(super_t.borrow().id, Some(super_t.clone()));
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

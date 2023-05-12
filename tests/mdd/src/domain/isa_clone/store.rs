//! domain::isa_clone Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa_clone-object-store-file"}}}
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa_clone-object-store-definition"}}}
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
    time::SystemTime,
};

use fnv::FnvHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::isa_clone::types::{
    Alpha, Baz, Beta, Borrowed, Gamma, Henry, NotImportant, OhBoy, Ownership, Reference,
    SimpleSubtypeA, SimpleSupertype, SubtypeA, SubtypeB, SuperBar, SuperFoo, SuperT, MUTABLE,
    OWNED, SHARED,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    alpha: HashMap<Uuid, (Alpha, SystemTime)>,
    baz: HashMap<Uuid, (Baz, SystemTime)>,
    beta: HashMap<Uuid, (Beta, SystemTime)>,
    borrowed: HashMap<Uuid, (Borrowed, SystemTime)>,
    gamma: HashMap<Uuid, (Gamma, SystemTime)>,
    henry: HashMap<Uuid, (Henry, SystemTime)>,
    not_important: HashMap<Uuid, (NotImportant, SystemTime)>,
    oh_boy: HashMap<Uuid, (OhBoy, SystemTime)>,
    ownership: HashMap<Uuid, (Ownership, SystemTime)>,
    reference: HashMap<Uuid, (Reference, SystemTime)>,
    simple_subtype_a: HashMap<Uuid, (SimpleSubtypeA, SystemTime)>,
    simple_supertype: HashMap<Uuid, (SimpleSupertype, SystemTime)>,
    subtype_a: HashMap<Uuid, (SubtypeA, SystemTime)>,
    subtype_b: HashMap<Uuid, (SubtypeB, SystemTime)>,
    super_bar: HashMap<Uuid, (SuperBar, SystemTime)>,
    super_foo: HashMap<Uuid, (SuperFoo, SystemTime)>,
    super_t: HashMap<Uuid, (SuperT, SystemTime)>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            alpha: HashMap::default(),
            baz: HashMap::default(),
            beta: HashMap::default(),
            borrowed: HashMap::default(),
            gamma: HashMap::default(),
            henry: HashMap::default(),
            not_important: HashMap::default(),
            oh_boy: HashMap::default(),
            ownership: HashMap::default(),
            reference: HashMap::default(),
            simple_subtype_a: HashMap::default(),
            simple_supertype: HashMap::default(),
            subtype_a: HashMap::default(),
            subtype_b: HashMap::default(),
            super_bar: HashMap::default(),
            super_foo: HashMap::default(),
            super_t: HashMap::default(),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥
        store.inter_borrowed(Borrowed::Mutable(MUTABLE));
        store.inter_borrowed(Borrowed::Shared(SHARED));
        store.inter_ownership(Ownership::Borrowed(Borrowed::Mutable(MUTABLE).id()));
        store.inter_ownership(Ownership::Borrowed(Borrowed::Shared(SHARED).id()));
        store.inter_ownership(Ownership::Owned(OWNED));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa_clone-object-store-methods"}}}
    /// Inter [`Alpha`] into the store.
    ///
    pub fn inter_alpha(&mut self, alpha: Alpha) {
        self.alpha.insert(alpha.id, (alpha, SystemTime::now()));
    }

    /// Exhume [`Alpha`] from the store.
    ///
    pub fn exhume_alpha(&self, id: &Uuid) -> Option<&Alpha> {
        self.alpha.get(id).map(|alpha| &alpha.0)
    }

    /// Exhume [`Alpha`] from the store — mutably.
    ///
    pub fn exhume_alpha_mut(&mut self, id: &Uuid) -> Option<&mut Alpha> {
        self.alpha.get_mut(id).map(|alpha| &mut alpha.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Alpha>`.
    ///
    pub fn iter_alpha(&self) -> impl Iterator<Item = &Alpha> {
        self.alpha.values().map(|alpha| &alpha.0)
    }

    /// Get the timestamp for Alpha.
    ///
    pub fn alpha_timestamp(&self, alpha: &Alpha) -> SystemTime {
        self.alpha
            .get(&alpha.id)
            .map(|alpha| alpha.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Baz`] into the store.
    ///
    pub fn inter_baz(&mut self, baz: Baz) {
        self.baz.insert(baz.id, (baz, SystemTime::now()));
    }

    /// Exhume [`Baz`] from the store.
    ///
    pub fn exhume_baz(&self, id: &Uuid) -> Option<&Baz> {
        self.baz.get(id).map(|baz| &baz.0)
    }

    /// Exhume [`Baz`] from the store — mutably.
    ///
    pub fn exhume_baz_mut(&mut self, id: &Uuid) -> Option<&mut Baz> {
        self.baz.get_mut(id).map(|baz| &mut baz.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Baz>`.
    ///
    pub fn iter_baz(&self) -> impl Iterator<Item = &Baz> {
        self.baz.values().map(|baz| &baz.0)
    }

    /// Get the timestamp for Baz.
    ///
    pub fn baz_timestamp(&self, baz: &Baz) -> SystemTime {
        self.baz
            .get(&baz.id)
            .map(|baz| baz.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Beta`] into the store.
    ///
    pub fn inter_beta(&mut self, beta: Beta) {
        self.beta.insert(beta.id, (beta, SystemTime::now()));
    }

    /// Exhume [`Beta`] from the store.
    ///
    pub fn exhume_beta(&self, id: &Uuid) -> Option<&Beta> {
        self.beta.get(id).map(|beta| &beta.0)
    }

    /// Exhume [`Beta`] from the store — mutably.
    ///
    pub fn exhume_beta_mut(&mut self, id: &Uuid) -> Option<&mut Beta> {
        self.beta.get_mut(id).map(|beta| &mut beta.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Beta>`.
    ///
    pub fn iter_beta(&self) -> impl Iterator<Item = &Beta> {
        self.beta.values().map(|beta| &beta.0)
    }

    /// Get the timestamp for Beta.
    ///
    pub fn beta_timestamp(&self, beta: &Beta) -> SystemTime {
        self.beta
            .get(&beta.id)
            .map(|beta| beta.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Borrowed`] into the store.
    ///
    pub fn inter_borrowed(&mut self, borrowed: Borrowed) {
        self.borrowed
            .insert(borrowed.id(), (borrowed, SystemTime::now()));
    }

    /// Exhume [`Borrowed`] from the store.
    ///
    pub fn exhume_borrowed(&self, id: &Uuid) -> Option<&Borrowed> {
        self.borrowed.get(id).map(|borrowed| &borrowed.0)
    }

    /// Exhume [`Borrowed`] from the store — mutably.
    ///
    pub fn exhume_borrowed_mut(&mut self, id: &Uuid) -> Option<&mut Borrowed> {
        self.borrowed.get_mut(id).map(|borrowed| &mut borrowed.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Borrowed>`.
    ///
    pub fn iter_borrowed(&self) -> impl Iterator<Item = &Borrowed> {
        self.borrowed.values().map(|borrowed| &borrowed.0)
    }

    /// Get the timestamp for Borrowed.
    ///
    pub fn borrowed_timestamp(&self, borrowed: &Borrowed) -> SystemTime {
        self.borrowed
            .get(&borrowed.id())
            .map(|borrowed| borrowed.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Gamma`] into the store.
    ///
    pub fn inter_gamma(&mut self, gamma: Gamma) {
        self.gamma.insert(gamma.id, (gamma, SystemTime::now()));
    }

    /// Exhume [`Gamma`] from the store.
    ///
    pub fn exhume_gamma(&self, id: &Uuid) -> Option<&Gamma> {
        self.gamma.get(id).map(|gamma| &gamma.0)
    }

    /// Exhume [`Gamma`] from the store — mutably.
    ///
    pub fn exhume_gamma_mut(&mut self, id: &Uuid) -> Option<&mut Gamma> {
        self.gamma.get_mut(id).map(|gamma| &mut gamma.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Gamma>`.
    ///
    pub fn iter_gamma(&self) -> impl Iterator<Item = &Gamma> {
        self.gamma.values().map(|gamma| &gamma.0)
    }

    /// Get the timestamp for Gamma.
    ///
    pub fn gamma_timestamp(&self, gamma: &Gamma) -> SystemTime {
        self.gamma
            .get(&gamma.id)
            .map(|gamma| gamma.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Henry`] into the store.
    ///
    pub fn inter_henry(&mut self, henry: Henry) {
        self.henry.insert(henry.id, (henry, SystemTime::now()));
    }

    /// Exhume [`Henry`] from the store.
    ///
    pub fn exhume_henry(&self, id: &Uuid) -> Option<&Henry> {
        self.henry.get(id).map(|henry| &henry.0)
    }

    /// Exhume [`Henry`] from the store — mutably.
    ///
    pub fn exhume_henry_mut(&mut self, id: &Uuid) -> Option<&mut Henry> {
        self.henry.get_mut(id).map(|henry| &mut henry.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Henry>`.
    ///
    pub fn iter_henry(&self) -> impl Iterator<Item = &Henry> {
        self.henry.values().map(|henry| &henry.0)
    }

    /// Get the timestamp for Henry.
    ///
    pub fn henry_timestamp(&self, henry: &Henry) -> SystemTime {
        self.henry
            .get(&henry.id)
            .map(|henry| henry.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`NotImportant`] into the store.
    ///
    pub fn inter_not_important(&mut self, not_important: NotImportant) {
        self.not_important
            .insert(not_important.id, (not_important, SystemTime::now()));
    }

    /// Exhume [`NotImportant`] from the store.
    ///
    pub fn exhume_not_important(&self, id: &Uuid) -> Option<&NotImportant> {
        self.not_important
            .get(id)
            .map(|not_important| &not_important.0)
    }

    /// Exhume [`NotImportant`] from the store — mutably.
    ///
    pub fn exhume_not_important_mut(&mut self, id: &Uuid) -> Option<&mut NotImportant> {
        self.not_important
            .get_mut(id)
            .map(|not_important| &mut not_important.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, NotImportant>`.
    ///
    pub fn iter_not_important(&self) -> impl Iterator<Item = &NotImportant> {
        self.not_important
            .values()
            .map(|not_important| &not_important.0)
    }

    /// Get the timestamp for NotImportant.
    ///
    pub fn not_important_timestamp(&self, not_important: &NotImportant) -> SystemTime {
        self.not_important
            .get(&not_important.id)
            .map(|not_important| not_important.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`OhBoy`] into the store.
    ///
    pub fn inter_oh_boy(&mut self, oh_boy: OhBoy) {
        self.oh_boy.insert(oh_boy.id, (oh_boy, SystemTime::now()));
    }

    /// Exhume [`OhBoy`] from the store.
    ///
    pub fn exhume_oh_boy(&self, id: &Uuid) -> Option<&OhBoy> {
        self.oh_boy.get(id).map(|oh_boy| &oh_boy.0)
    }

    /// Exhume [`OhBoy`] from the store — mutably.
    ///
    pub fn exhume_oh_boy_mut(&mut self, id: &Uuid) -> Option<&mut OhBoy> {
        self.oh_boy.get_mut(id).map(|oh_boy| &mut oh_boy.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, OhBoy>`.
    ///
    pub fn iter_oh_boy(&self) -> impl Iterator<Item = &OhBoy> {
        self.oh_boy.values().map(|oh_boy| &oh_boy.0)
    }

    /// Get the timestamp for OhBoy.
    ///
    pub fn oh_boy_timestamp(&self, oh_boy: &OhBoy) -> SystemTime {
        self.oh_boy
            .get(&oh_boy.id)
            .map(|oh_boy| oh_boy.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Ownership`] into the store.
    ///
    pub fn inter_ownership(&mut self, ownership: Ownership) {
        self.ownership
            .insert(ownership.id(), (ownership, SystemTime::now()));
    }

    /// Exhume [`Ownership`] from the store.
    ///
    pub fn exhume_ownership(&self, id: &Uuid) -> Option<&Ownership> {
        self.ownership.get(id).map(|ownership| &ownership.0)
    }

    /// Exhume [`Ownership`] from the store — mutably.
    ///
    pub fn exhume_ownership_mut(&mut self, id: &Uuid) -> Option<&mut Ownership> {
        self.ownership.get_mut(id).map(|ownership| &mut ownership.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Ownership>`.
    ///
    pub fn iter_ownership(&self) -> impl Iterator<Item = &Ownership> {
        self.ownership.values().map(|ownership| &ownership.0)
    }

    /// Get the timestamp for Ownership.
    ///
    pub fn ownership_timestamp(&self, ownership: &Ownership) -> SystemTime {
        self.ownership
            .get(&ownership.id())
            .map(|ownership| ownership.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Reference`] into the store.
    ///
    pub fn inter_reference(&mut self, reference: Reference) {
        self.reference
            .insert(reference.id, (reference, SystemTime::now()));
    }

    /// Exhume [`Reference`] from the store.
    ///
    pub fn exhume_reference(&self, id: &Uuid) -> Option<&Reference> {
        self.reference.get(id).map(|reference| &reference.0)
    }

    /// Exhume [`Reference`] from the store — mutably.
    ///
    pub fn exhume_reference_mut(&mut self, id: &Uuid) -> Option<&mut Reference> {
        self.reference.get_mut(id).map(|reference| &mut reference.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Reference>`.
    ///
    pub fn iter_reference(&self) -> impl Iterator<Item = &Reference> {
        self.reference.values().map(|reference| &reference.0)
    }

    /// Get the timestamp for Reference.
    ///
    pub fn reference_timestamp(&self, reference: &Reference) -> SystemTime {
        self.reference
            .get(&reference.id)
            .map(|reference| reference.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`SimpleSubtypeA`] into the store.
    ///
    pub fn inter_simple_subtype_a(&mut self, simple_subtype_a: SimpleSubtypeA) {
        self.simple_subtype_a
            .insert(simple_subtype_a.id(), (simple_subtype_a, SystemTime::now()));
    }

    /// Exhume [`SimpleSubtypeA`] from the store.
    ///
    pub fn exhume_simple_subtype_a(&self, id: &Uuid) -> Option<&SimpleSubtypeA> {
        self.simple_subtype_a
            .get(id)
            .map(|simple_subtype_a| &simple_subtype_a.0)
    }

    /// Exhume [`SimpleSubtypeA`] from the store — mutably.
    ///
    pub fn exhume_simple_subtype_a_mut(&mut self, id: &Uuid) -> Option<&mut SimpleSubtypeA> {
        self.simple_subtype_a
            .get_mut(id)
            .map(|simple_subtype_a| &mut simple_subtype_a.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SimpleSubtypeA>`.
    ///
    pub fn iter_simple_subtype_a(&self) -> impl Iterator<Item = &SimpleSubtypeA> {
        self.simple_subtype_a
            .values()
            .map(|simple_subtype_a| &simple_subtype_a.0)
    }

    /// Get the timestamp for SimpleSubtypeA.
    ///
    pub fn simple_subtype_a_timestamp(&self, simple_subtype_a: &SimpleSubtypeA) -> SystemTime {
        self.simple_subtype_a
            .get(&simple_subtype_a.id())
            .map(|simple_subtype_a| simple_subtype_a.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`SimpleSupertype`] into the store.
    ///
    pub fn inter_simple_supertype(&mut self, simple_supertype: SimpleSupertype) {
        self.simple_supertype
            .insert(simple_supertype.id, (simple_supertype, SystemTime::now()));
    }

    /// Exhume [`SimpleSupertype`] from the store.
    ///
    pub fn exhume_simple_supertype(&self, id: &Uuid) -> Option<&SimpleSupertype> {
        self.simple_supertype
            .get(id)
            .map(|simple_supertype| &simple_supertype.0)
    }

    /// Exhume [`SimpleSupertype`] from the store — mutably.
    ///
    pub fn exhume_simple_supertype_mut(&mut self, id: &Uuid) -> Option<&mut SimpleSupertype> {
        self.simple_supertype
            .get_mut(id)
            .map(|simple_supertype| &mut simple_supertype.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SimpleSupertype>`.
    ///
    pub fn iter_simple_supertype(&self) -> impl Iterator<Item = &SimpleSupertype> {
        self.simple_supertype
            .values()
            .map(|simple_supertype| &simple_supertype.0)
    }

    /// Get the timestamp for SimpleSupertype.
    ///
    pub fn simple_supertype_timestamp(&self, simple_supertype: &SimpleSupertype) -> SystemTime {
        self.simple_supertype
            .get(&simple_supertype.id)
            .map(|simple_supertype| simple_supertype.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`SubtypeA`] into the store.
    ///
    pub fn inter_subtype_a(&mut self, subtype_a: SubtypeA) {
        self.subtype_a
            .insert(subtype_a.id, (subtype_a, SystemTime::now()));
    }

    /// Exhume [`SubtypeA`] from the store.
    ///
    pub fn exhume_subtype_a(&self, id: &Uuid) -> Option<&SubtypeA> {
        self.subtype_a.get(id).map(|subtype_a| &subtype_a.0)
    }

    /// Exhume [`SubtypeA`] from the store — mutably.
    ///
    pub fn exhume_subtype_a_mut(&mut self, id: &Uuid) -> Option<&mut SubtypeA> {
        self.subtype_a.get_mut(id).map(|subtype_a| &mut subtype_a.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeA>`.
    ///
    pub fn iter_subtype_a(&self) -> impl Iterator<Item = &SubtypeA> {
        self.subtype_a.values().map(|subtype_a| &subtype_a.0)
    }

    /// Get the timestamp for SubtypeA.
    ///
    pub fn subtype_a_timestamp(&self, subtype_a: &SubtypeA) -> SystemTime {
        self.subtype_a
            .get(&subtype_a.id)
            .map(|subtype_a| subtype_a.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`SubtypeB`] into the store.
    ///
    pub fn inter_subtype_b(&mut self, subtype_b: SubtypeB) {
        self.subtype_b
            .insert(subtype_b.id, (subtype_b, SystemTime::now()));
    }

    /// Exhume [`SubtypeB`] from the store.
    ///
    pub fn exhume_subtype_b(&self, id: &Uuid) -> Option<&SubtypeB> {
        self.subtype_b.get(id).map(|subtype_b| &subtype_b.0)
    }

    /// Exhume [`SubtypeB`] from the store — mutably.
    ///
    pub fn exhume_subtype_b_mut(&mut self, id: &Uuid) -> Option<&mut SubtypeB> {
        self.subtype_b.get_mut(id).map(|subtype_b| &mut subtype_b.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeB>`.
    ///
    pub fn iter_subtype_b(&self) -> impl Iterator<Item = &SubtypeB> {
        self.subtype_b.values().map(|subtype_b| &subtype_b.0)
    }

    /// Get the timestamp for SubtypeB.
    ///
    pub fn subtype_b_timestamp(&self, subtype_b: &SubtypeB) -> SystemTime {
        self.subtype_b
            .get(&subtype_b.id)
            .map(|subtype_b| subtype_b.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`SuperBar`] into the store.
    ///
    pub fn inter_super_bar(&mut self, super_bar: SuperBar) {
        self.super_bar
            .insert(super_bar.id(), (super_bar, SystemTime::now()));
    }

    /// Exhume [`SuperBar`] from the store.
    ///
    pub fn exhume_super_bar(&self, id: &Uuid) -> Option<&SuperBar> {
        self.super_bar.get(id).map(|super_bar| &super_bar.0)
    }

    /// Exhume [`SuperBar`] from the store — mutably.
    ///
    pub fn exhume_super_bar_mut(&mut self, id: &Uuid) -> Option<&mut SuperBar> {
        self.super_bar.get_mut(id).map(|super_bar| &mut super_bar.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SuperBar>`.
    ///
    pub fn iter_super_bar(&self) -> impl Iterator<Item = &SuperBar> {
        self.super_bar.values().map(|super_bar| &super_bar.0)
    }

    /// Get the timestamp for SuperBar.
    ///
    pub fn super_bar_timestamp(&self, super_bar: &SuperBar) -> SystemTime {
        self.super_bar
            .get(&super_bar.id())
            .map(|super_bar| super_bar.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`SuperFoo`] into the store.
    ///
    pub fn inter_super_foo(&mut self, super_foo: SuperFoo) {
        self.super_foo
            .insert(super_foo.id(), (super_foo, SystemTime::now()));
    }

    /// Exhume [`SuperFoo`] from the store.
    ///
    pub fn exhume_super_foo(&self, id: &Uuid) -> Option<&SuperFoo> {
        self.super_foo.get(id).map(|super_foo| &super_foo.0)
    }

    /// Exhume [`SuperFoo`] from the store — mutably.
    ///
    pub fn exhume_super_foo_mut(&mut self, id: &Uuid) -> Option<&mut SuperFoo> {
        self.super_foo.get_mut(id).map(|super_foo| &mut super_foo.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SuperFoo>`.
    ///
    pub fn iter_super_foo(&self) -> impl Iterator<Item = &SuperFoo> {
        self.super_foo.values().map(|super_foo| &super_foo.0)
    }

    /// Get the timestamp for SuperFoo.
    ///
    pub fn super_foo_timestamp(&self, super_foo: &SuperFoo) -> SystemTime {
        self.super_foo
            .get(&super_foo.id())
            .map(|super_foo| super_foo.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`SuperT`] into the store.
    ///
    pub fn inter_super_t(&mut self, super_t: SuperT) {
        self.super_t
            .insert(super_t.id, (super_t, SystemTime::now()));
    }

    /// Exhume [`SuperT`] from the store.
    ///
    pub fn exhume_super_t(&self, id: &Uuid) -> Option<&SuperT> {
        self.super_t.get(id).map(|super_t| &super_t.0)
    }

    /// Exhume [`SuperT`] from the store — mutably.
    ///
    pub fn exhume_super_t_mut(&mut self, id: &Uuid) -> Option<&mut SuperT> {
        self.super_t.get_mut(id).map(|super_t| &mut super_t.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SuperT>`.
    ///
    pub fn iter_super_t(&self) -> impl Iterator<Item = &SuperT> {
        self.super_t.values().map(|super_t| &super_t.0)
    }

    /// Get the timestamp for SuperT.
    ///
    pub fn super_t_timestamp(&self, super_t: &SuperT) -> SystemTime {
        self.super_t
            .get(&super_t.id)
            .map(|super_t| super_t.1)
            .unwrap_or(SystemTime::now())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa_clone-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        fs::create_dir_all(path)?;

        let bin_path = path.clone().join("Isa Relationship.bin");
        let mut bin_file = fs::File::create(bin_path)?;
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        bin_file.write_all(&encoded)?;

        let path = path.join("Isa Relationship.json");
        fs::create_dir_all(&path)?;

        // Persist Alpha.
        {
            let path = path.join("alpha");
            fs::create_dir_all(&path)?;
            for alpha_tuple in self.alpha.values() {
                let path = path.join(format!("{}.json", alpha_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Alpha, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != alpha_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &alpha_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &alpha_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.alpha.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Baz.
        {
            let path = path.join("baz");
            fs::create_dir_all(&path)?;
            for baz_tuple in self.baz.values() {
                let path = path.join(format!("{}.json", baz_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Baz, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != baz_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &baz_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &baz_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.baz.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Beta.
        {
            let path = path.join("beta");
            fs::create_dir_all(&path)?;
            for beta_tuple in self.beta.values() {
                let path = path.join(format!("{}.json", beta_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Beta, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != beta_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &beta_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &beta_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.beta.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Borrowed.
        {
            let path = path.join("borrowed");
            fs::create_dir_all(&path)?;
            for borrowed_tuple in self.borrowed.values() {
                let path = path.join(format!("{}.json", borrowed_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Borrowed, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != borrowed_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &borrowed_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &borrowed_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.borrowed.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Gamma.
        {
            let path = path.join("gamma");
            fs::create_dir_all(&path)?;
            for gamma_tuple in self.gamma.values() {
                let path = path.join(format!("{}.json", gamma_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Gamma, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != gamma_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &gamma_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &gamma_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.gamma.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Henry.
        {
            let path = path.join("henry");
            fs::create_dir_all(&path)?;
            for henry_tuple in self.henry.values() {
                let path = path.join(format!("{}.json", henry_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Henry, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != henry_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &henry_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &henry_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.henry.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Not Important.
        {
            let path = path.join("not_important");
            fs::create_dir_all(&path)?;
            for not_important_tuple in self.not_important.values() {
                let path = path.join(format!("{}.json", not_important_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (NotImportant, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != not_important_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &not_important_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &not_important_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.not_important.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Oh Boy!.
        {
            let path = path.join("oh_boy");
            fs::create_dir_all(&path)?;
            for oh_boy_tuple in self.oh_boy.values() {
                let path = path.join(format!("{}.json", oh_boy_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (OhBoy, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != oh_boy_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &oh_boy_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &oh_boy_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.oh_boy.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Ownership.
        {
            let path = path.join("ownership");
            fs::create_dir_all(&path)?;
            for ownership_tuple in self.ownership.values() {
                let path = path.join(format!("{}.json", ownership_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Ownership, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != ownership_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &ownership_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &ownership_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.ownership.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Reference.
        {
            let path = path.join("reference");
            fs::create_dir_all(&path)?;
            for reference_tuple in self.reference.values() {
                let path = path.join(format!("{}.json", reference_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Reference, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != reference_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &reference_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &reference_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.reference.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Simple Subtype A.
        {
            let path = path.join("simple_subtype_a");
            fs::create_dir_all(&path)?;
            for simple_subtype_a_tuple in self.simple_subtype_a.values() {
                let path = path.join(format!("{}.json", simple_subtype_a_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (SimpleSubtypeA, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != simple_subtype_a_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &simple_subtype_a_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &simple_subtype_a_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.simple_subtype_a.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Simple Supertype.
        {
            let path = path.join("simple_supertype");
            fs::create_dir_all(&path)?;
            for simple_supertype_tuple in self.simple_supertype.values() {
                let path = path.join(format!("{}.json", simple_supertype_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (SimpleSupertype, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != simple_supertype_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &simple_supertype_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &simple_supertype_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.simple_supertype.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Subtype A.
        {
            let path = path.join("subtype_a");
            fs::create_dir_all(&path)?;
            for subtype_a_tuple in self.subtype_a.values() {
                let path = path.join(format!("{}.json", subtype_a_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (SubtypeA, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != subtype_a_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &subtype_a_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &subtype_a_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.subtype_a.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Subtype B.
        {
            let path = path.join("subtype_b");
            fs::create_dir_all(&path)?;
            for subtype_b_tuple in self.subtype_b.values() {
                let path = path.join(format!("{}.json", subtype_b_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (SubtypeB, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != subtype_b_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &subtype_b_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &subtype_b_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.subtype_b.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Super Bar.
        {
            let path = path.join("super_bar");
            fs::create_dir_all(&path)?;
            for super_bar_tuple in self.super_bar.values() {
                let path = path.join(format!("{}.json", super_bar_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (SuperBar, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != super_bar_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &super_bar_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &super_bar_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.super_bar.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Super Foo.
        {
            let path = path.join("super_foo");
            fs::create_dir_all(&path)?;
            for super_foo_tuple in self.super_foo.values() {
                let path = path.join(format!("{}.json", super_foo_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (SuperFoo, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != super_foo_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &super_foo_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &super_foo_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.super_foo.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Super T.
        {
            let path = path.join("super_t");
            fs::create_dir_all(&path)?;
            for super_t_tuple in self.super_t.values() {
                let path = path.join(format!("{}.json", super_t_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (SuperT, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != super_t_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &super_t_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &super_t_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.super_t.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        Ok(())
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
                let alpha: (Alpha, SystemTime) = serde_json::from_reader(reader)?;
                store.alpha.insert(alpha.0.id, alpha);
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
                let baz: (Baz, SystemTime) = serde_json::from_reader(reader)?;
                store.baz.insert(baz.0.id, baz);
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
                let beta: (Beta, SystemTime) = serde_json::from_reader(reader)?;
                store.beta.insert(beta.0.id, beta);
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
                let borrowed: (Borrowed, SystemTime) = serde_json::from_reader(reader)?;
                store.borrowed.insert(borrowed.0.id(), borrowed);
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
                let gamma: (Gamma, SystemTime) = serde_json::from_reader(reader)?;
                store.gamma.insert(gamma.0.id, gamma);
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
                let henry: (Henry, SystemTime) = serde_json::from_reader(reader)?;
                store.henry.insert(henry.0.id, henry);
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
                let not_important: (NotImportant, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .not_important
                    .insert(not_important.0.id, not_important);
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
                let oh_boy: (OhBoy, SystemTime) = serde_json::from_reader(reader)?;
                store.oh_boy.insert(oh_boy.0.id, oh_boy);
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
                let ownership: (Ownership, SystemTime) = serde_json::from_reader(reader)?;
                store.ownership.insert(ownership.0.id(), ownership);
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
                let reference: (Reference, SystemTime) = serde_json::from_reader(reader)?;
                store.reference.insert(reference.0.id, reference);
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
                let simple_subtype_a: (SimpleSubtypeA, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .simple_subtype_a
                    .insert(simple_subtype_a.0.id(), simple_subtype_a);
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
                let simple_supertype: (SimpleSupertype, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .simple_supertype
                    .insert(simple_supertype.0.id, simple_supertype);
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
                let subtype_a: (SubtypeA, SystemTime) = serde_json::from_reader(reader)?;
                store.subtype_a.insert(subtype_a.0.id, subtype_a);
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
                let subtype_b: (SubtypeB, SystemTime) = serde_json::from_reader(reader)?;
                store.subtype_b.insert(subtype_b.0.id, subtype_b);
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
                let super_bar: (SuperBar, SystemTime) = serde_json::from_reader(reader)?;
                store.super_bar.insert(super_bar.0.id(), super_bar);
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
                let super_foo: (SuperFoo, SystemTime) = serde_json::from_reader(reader)?;
                store.super_foo.insert(super_foo.0.id(), super_foo);
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
                let super_t: (SuperT, SystemTime) = serde_json::from_reader(reader)?;
                store.super_t.insert(super_t.0.id, super_t);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

//! domain::isa_clone Object From Trait Implementations
//!
//! These are [`From`] trait implementations for the domain: _Isa Relationship_. They are
//! generated to be used during the extrusion process. This is the process
//! by which instances of one domain are transformed into instances of another.
//! In this case the source domain is `domain::isa`.
//!
//! It is hoped that the model has not changed enough to render
//! these implementations useless. In any case it's expected that
//! the generated code will need to be manually edited.
// {"magic":"","directive":{"Start":{"directive":"ignore-gen","tag":"domain::isa_clone-from-impl-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-gen","tag":"domain::isa_clone-from-impl-definition"}}}
use crate::domain::isa_clone::types::{NotImportant, SimpleSupertype, SubtypeA, SubtypeB, SuperT};
use crate::domain::isa_clone::ObjectStore;

use crate::domain::isa::types::{
    NotImportant as FromNotImportant, SimpleSupertype as FromSimpleSupertype,
    SubtypeA as FromSubtypeA, SubtypeB as FromSubtypeB, SuperT as FromSuperT,
};
use crate::domain::isa::ObjectStore as IsaStore;

impl From<&IsaStore> for ObjectStore {
    fn from(from: &IsaStore) -> Self {
        let mut to = ObjectStore::new();

        for (_, instance) in from.iter_not_important() {
            let instance = NotImportant::from(instance);
            to.inter_not_important(instance);
        }

        for (_, instance) in from.iter_simple_supertype() {
            let instance = SimpleSupertype::from(instance);
            to.inter_simple_supertype(instance);
        }

        for (_, instance) in from.iter_subtype_a() {
            let instance = SubtypeA::from(instance);
            to.inter_subtype_a(instance);
        }

        for (_, instance) in from.iter_subtype_b() {
            let instance = SubtypeB::from(instance);
            to.inter_subtype_b(instance);
        }

        for (_, instance) in from.iter_super_t() {
            let instance = SuperT::from(instance);
            to.inter_super_t(instance);
        }

        to
    }
}

impl From<&FromNotImportant> for NotImportant {
    fn from(src: &FromNotImportant) -> Self {
        Self {
            id: src.id,
            name: src.name,
            x_ref: src.x_ref,
        }
    }
}

impl From<&FromSimpleSupertype> for SimpleSupertype {
    fn from(src: &FromSimpleSupertype) -> Self {
        match src {
            FromSimpleSupertype::SimpleSubtypeA(src) => {
                SimpleSupertype::SimpleSubtypeA(src.clone())
            }
            FromSimpleSupertype::SimpleSubtypeB(src) => {
                SimpleSupertype::SimpleSubtypeB(src.clone())
            }
        }
    }
}
impl From<&FromSubtypeA> for SubtypeA {
    fn from(src: &FromSubtypeA) -> Self {
        Self {
            id: src.id,
            name: src.name.clone(),
        }
    }
}

impl From<&FromSubtypeB> for SubtypeB {
    fn from(src: &FromSubtypeB) -> Self {
        Self {
            id: src.id,
            number: src.number,
        }
    }
}

impl From<&FromSuperT> for SuperT {
    fn from(src: &FromSuperT) -> Self {
        match src {
            FromSuperT::SubtypeA(src) => SuperT::SubtypeA(src.clone()),
            FromSuperT::SubtypeB(src) => SuperT::SubtypeB(src.clone()),
        }
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-gen"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-gen"}}}

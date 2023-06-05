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
use crate::domain::isa_clone::types::{
    Alpha, Baz, Beta, Borrowed, Gamma, Henry, NotImportant, OhBoy, Ownership, Reference,
    SimpleSubtypeA, SimpleSupertype, SubtypeA, SubtypeB, SuperBar, SuperFoo, SuperT,
};
use crate::domain::isa_clone::ObjectStore;

use crate::domain::isa::types::{
    Alpha as FromAlpha, Baz as FromBaz, Beta as FromBeta, Borrowed as FromBorrowed,
    Gamma as FromGamma, Henry as FromHenry, NotImportant as FromNotImportant, OhBoy as FromOhBoy,
    Ownership as FromOwnership, Reference as FromReference, SimpleSubtypeA as FromSimpleSubtypeA,
    SimpleSupertype as FromSimpleSupertype, SubtypeA as FromSubtypeA, SubtypeB as FromSubtypeB,
    SuperBar as FromSuperBar, SuperFoo as FromSuperFoo, SuperT as FromSuperT,
};
use crate::domain::isa::ObjectStore as IsaStore;

impl From<&IsaStore> for ObjectStore {
    fn from(from: &IsaStore) -> Self {
        let mut to = ObjectStore::new();

        for instance in from.iter_alpha() {
            let instance = Alpha::from(instance);
            to.inter_alpha(instance);
        }

        for instance in from.iter_baz() {
            let instance = Baz::from(instance);
            to.inter_baz(instance);
        }

        for instance in from.iter_beta() {
            let instance = Beta::from(instance);
            to.inter_beta(instance);
        }

        for instance in from.iter_borrowed() {
            let instance = Borrowed::from(instance);
            to.inter_borrowed(instance);
        }

        for instance in from.iter_gamma() {
            let instance = Gamma::from(instance);
            to.inter_gamma(instance);
        }

        for instance in from.iter_henry() {
            let instance = Henry::from(instance);
            to.inter_henry(instance);
        }

        for instance in from.iter_not_important() {
            let instance = NotImportant::from(instance);
            to.inter_not_important(instance);
        }

        for instance in from.iter_oh_boy() {
            let instance = OhBoy::from(instance);
            to.inter_oh_boy(instance);
        }

        for instance in from.iter_ownership() {
            let instance = Ownership::from(instance);
            to.inter_ownership(instance);
        }

        for instance in from.iter_reference() {
            let instance = Reference::from(instance);
            to.inter_reference(instance);
        }

        for instance in from.iter_simple_subtype_a() {
            let instance = SimpleSubtypeA::from(instance);
            to.inter_simple_subtype_a(instance);
        }

        for instance in from.iter_simple_supertype() {
            let instance = SimpleSupertype::from(instance);
            to.inter_simple_supertype(instance);
        }

        for instance in from.iter_subtype_a() {
            let instance = SubtypeA::from(instance);
            to.inter_subtype_a(instance);
        }

        for instance in from.iter_subtype_b() {
            let instance = SubtypeB::from(instance);
            to.inter_subtype_b(instance);
        }

        for instance in from.iter_super_bar() {
            let instance = SuperBar::from(instance);
            to.inter_super_bar(instance);
        }

        for instance in from.iter_super_foo() {
            let instance = SuperFoo::from(instance);
            to.inter_super_foo(instance);
        }

        for instance in from.iter_super_t() {
            let instance = SuperT::from(instance);
            to.inter_super_t(instance);
        }

        to
    }
}

impl From<&FromAlpha> for Alpha {
    fn from(src: &FromAlpha) -> Self {
        match src {
            FromAlpha::Gamma(src) => Alpha::Gamma(GAMMA),
        }
    }
}
impl From<&FromBaz> for Baz {
    fn from(src: &FromBaz) -> Self {
        Self {
            id: src.id,
            insanity: src.insanity,
            fugue: src.fugue,
        }
    }
}

impl From<&FromBeta> for Beta {
    fn from(src: &FromBeta) -> Self {
        match src {
            FromBeta::Gamma(src) => Beta::Gamma(GAMMA),
            FromBeta::SuperBar(src) => Beta::SuperBar(SUPER_BAR),
        }
    }
}

impl From<&FromBorrowed> for Borrowed {
    fn from(src: &FromBorrowed) -> Self {
        match src {
            FromBorrowed::Mutable(src) => Borrowed::Mutable(MUTABLE),
            FromBorrowed::Shared(src) => Borrowed::Shared(SHARED),
        }
    }
}
impl From<&FromGamma> for Gamma {
    fn from(src: &FromGamma) -> Self {
        Self {
            id: src.id,
            value: src.value,
        }
    }
}

impl From<&FromHenry> for Henry {
    fn from(src: &FromHenry) -> Self {
        Self {
            id: src.id,
            last_name: src.last_name.clone(),
            bar: src.bar,
        }
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

impl From<&FromOhBoy> for OhBoy {
    fn from(src: &FromOhBoy) -> Self {
        Self {
            attribution: src.attribution.clone(),
            id: src.id,
        }
    }
}

impl From<&FromOwnership> for Ownership {
    fn from(src: &FromOwnership) -> Self {
        match src {
            FromOwnership::Borrowed(src) => Ownership::Borrowed(BORROWED),
            FromOwnership::Owned(src) => Ownership::Owned(OWNED),
        }
    }
}
impl From<&FromReference> for Reference {
    fn from(src: &FromReference) -> Self {
        Self {
            id: src.id,
            name: src.name.clone(),
        }
    }
}

impl From<&FromSimpleSubtypeA> for SimpleSubtypeA {
    fn from(src: &FromSimpleSubtypeA) -> Self {
        match src {
            FromSimpleSubtypeA::OhBoy(src) => SimpleSubtypeA::OhBoy(OH_BOY),
        }
    }
}

impl From<&FromSimpleSupertype> for SimpleSupertype {
    fn from(src: &FromSimpleSupertype) -> Self {
        match src {
            FromSimpleSupertype::SimpleSubtypeA(src) => {
                SimpleSupertype::SimpleSubtypeA(SIMPLE_SUBTYPE_A)
            }
            FromSimpleSupertype::SimpleSubtypeB(src) => {
                SimpleSupertype::SimpleSubtypeB(SIMPLE_SUBTYPE_B)
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

impl From<&FromSuperBar> for SuperBar {
    fn from(src: &FromSuperBar) -> Self {
        match src {
            FromSuperBar::Gamma(src) => SuperBar::Gamma(GAMMA),
        }
    }
}

impl From<&FromSuperFoo> for SuperFoo {
    fn from(src: &FromSuperFoo) -> Self {
        match src {
            FromSuperFoo::Gamma(src) => SuperFoo::Gamma(GAMMA),
        }
    }
}

impl From<&FromSuperT> for SuperT {
    fn from(src: &FromSuperT) -> Self {
        match src {
            FromSuperT::SubtypeA(src) => SuperT::SubtypeA(SUBTYPE_A),
            FromSuperT::SubtypeB(src) => SuperT::SubtypeB(SUBTYPE_B),
        }
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-gen"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-gen"}}}

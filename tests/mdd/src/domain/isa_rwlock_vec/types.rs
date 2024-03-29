//! Domain to test the supertype/subtype relationship.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::isa_rwlock_vec-module-definition-file"}}}
pub mod alpha;
pub mod baz;
pub mod beta;
pub mod borrowed;
pub mod gamma;
pub mod henry;
pub mod mutable;
pub mod not_important;
pub mod oh_boy;
pub mod owned;
pub mod ownership;
pub mod reference;
pub mod shared;
pub mod simple_subtype_a;
pub mod simple_subtype_b;
pub mod simple_supertype;
pub mod subtype_a;
pub mod subtype_b;
pub mod super_bar;
pub mod super_foo;
pub mod super_t;

pub use crate::domain::isa_rwlock_vec::alpha::Alpha;
pub use crate::domain::isa_rwlock_vec::alpha::AlphaEnum;
pub use crate::domain::isa_rwlock_vec::baz::Baz;
pub use crate::domain::isa_rwlock_vec::beta::Beta;
pub use crate::domain::isa_rwlock_vec::beta::BetaEnum;
pub use crate::domain::isa_rwlock_vec::borrowed::Borrowed;
pub use crate::domain::isa_rwlock_vec::borrowed::BorrowedEnum;
pub use crate::domain::isa_rwlock_vec::gamma::Gamma;
pub use crate::domain::isa_rwlock_vec::henry::Henry;
pub use crate::domain::isa_rwlock_vec::mutable::Mutable;
pub use crate::domain::isa_rwlock_vec::mutable::MUTABLE;
pub use crate::domain::isa_rwlock_vec::not_important::NotImportant;
pub use crate::domain::isa_rwlock_vec::oh_boy::OhBoy;
pub use crate::domain::isa_rwlock_vec::owned::Owned;
pub use crate::domain::isa_rwlock_vec::owned::OWNED;
pub use crate::domain::isa_rwlock_vec::ownership::Ownership;
pub use crate::domain::isa_rwlock_vec::ownership::OwnershipEnum;
pub use crate::domain::isa_rwlock_vec::reference::Reference;
pub use crate::domain::isa_rwlock_vec::shared::Shared;
pub use crate::domain::isa_rwlock_vec::shared::SHARED;
pub use crate::domain::isa_rwlock_vec::simple_subtype_a::SimpleSubtypeA;
pub use crate::domain::isa_rwlock_vec::simple_subtype_a::SimpleSubtypeAEnum;
pub use crate::domain::isa_rwlock_vec::simple_subtype_b::SimpleSubtypeB;
pub use crate::domain::isa_rwlock_vec::simple_subtype_b::SIMPLE_SUBTYPE_B;
pub use crate::domain::isa_rwlock_vec::simple_supertype::SimpleSupertype;
pub use crate::domain::isa_rwlock_vec::simple_supertype::SimpleSupertypeEnum;
pub use crate::domain::isa_rwlock_vec::subtype_a::SubtypeA;
pub use crate::domain::isa_rwlock_vec::subtype_b::SubtypeB;
pub use crate::domain::isa_rwlock_vec::super_bar::SuperBar;
pub use crate::domain::isa_rwlock_vec::super_bar::SuperBarEnum;
pub use crate::domain::isa_rwlock_vec::super_foo::SuperFoo;
pub use crate::domain::isa_rwlock_vec::super_foo::SuperFooEnum;
pub use crate::domain::isa_rwlock_vec::super_t::SuperT;
pub use crate::domain::isa_rwlock_vec::super_t::SuperTEnum;
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

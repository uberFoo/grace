//! Domain to test the supertype/subtype relationship.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::isa_clone-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa_clone-module-definition"}}}
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

pub use crate::domain::isa_clone::alpha::Alpha;
pub use crate::domain::isa_clone::alpha::AlphaEnum;
pub use crate::domain::isa_clone::baz::Baz;
pub use crate::domain::isa_clone::beta::Beta;
pub use crate::domain::isa_clone::borrowed::Borrowed;
pub use crate::domain::isa_clone::gamma::Gamma;
pub use crate::domain::isa_clone::henry::Henry;
pub use crate::domain::isa_clone::mutable::MUTABLE;
pub use crate::domain::isa_clone::not_important::NotImportant;
pub use crate::domain::isa_clone::oh_boy::OhBoy;
pub use crate::domain::isa_clone::owned::OWNED;
pub use crate::domain::isa_clone::ownership::Ownership;
pub use crate::domain::isa_clone::reference::Reference;
pub use crate::domain::isa_clone::shared::SHARED;
pub use crate::domain::isa_clone::simple_subtype_a::SimpleSubtypeA;
pub use crate::domain::isa_clone::simple_subtype_b::SIMPLE_SUBTYPE_B;
pub use crate::domain::isa_clone::simple_supertype::SimpleSupertype;
pub use crate::domain::isa_clone::simple_supertype::SimpleSupertypeEnum;
pub use crate::domain::isa_clone::subtype_a::SubtypeA;
pub use crate::domain::isa_clone::subtype_b::SubtypeB;
pub use crate::domain::isa_clone::super_bar::SuperBar;
pub use crate::domain::isa_clone::super_foo::SuperFoo;
pub use crate::domain::isa_clone::super_t::SuperT;
pub use crate::domain::isa_clone::super_t::SuperTEnum;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

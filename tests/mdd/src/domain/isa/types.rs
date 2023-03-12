//! Domain to test the supertype/subtype relationship.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::isa-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa-module-definition"}}}
pub mod borrowed;
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
pub mod super_t;

pub use crate::domain::isa::borrowed::Borrowed;
pub use crate::domain::isa::henry::Henry;
pub use crate::domain::isa::mutable::MUTABLE;
pub use crate::domain::isa::not_important::NotImportant;
pub use crate::domain::isa::oh_boy::OhBoy;
pub use crate::domain::isa::owned::OWNED;
pub use crate::domain::isa::ownership::Ownership;
pub use crate::domain::isa::reference::Reference;
pub use crate::domain::isa::shared::SHARED;
pub use crate::domain::isa::simple_subtype_a::SimpleSubtypeA;
pub use crate::domain::isa::simple_subtype_b::SIMPLE_SUBTYPE_B;
pub use crate::domain::isa::simple_supertype::SimpleSupertype;
pub use crate::domain::isa::simple_supertype::SimpleSupertypeEnum;
pub use crate::domain::isa::subtype_a::SubtypeA;
pub use crate::domain::isa::subtype_b::SubtypeB;
pub use crate::domain::isa::super_t::SuperT;
pub use crate::domain::isa::super_t::SuperTEnum;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

//! Domain to test the supertype/subtype relationship.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::isa-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::isa-module-definition"}}}
pub mod not_important;
pub mod simple_subtype_a;
pub mod simple_subtype_b;
pub mod simple_supertype;
pub mod subtype_a;
pub mod subtype_b;
pub mod super_t;

pub use crate::domain::isa::not_important::NotImportant;
pub use crate::domain::isa::simple_subtype_a::SIMPLE_SUBTYPE_A;
pub use crate::domain::isa::simple_subtype_b::SIMPLE_SUBTYPE_B;
pub use crate::domain::isa::simple_supertype::SimpleSupertype;
pub use crate::domain::isa::subtype_a::SubtypeA;
pub use crate::domain::isa::subtype_b::SubtypeB;
pub use crate::domain::isa::super_t::SuperT;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
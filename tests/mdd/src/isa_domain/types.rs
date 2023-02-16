//! Domain to test the supertype/subtype relationship.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"isa_domain-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_domain-module-definition"}}}
pub mod simple_subtype_a;
pub mod simple_subtype_b;
pub mod simple_supertype;
pub mod subtype_a;
pub mod subtype_b;
pub mod super_t;

pub use crate::isa_domain::simple_subtype_a::SIMPLE_SUBTYPE_A;
pub use crate::isa_domain::simple_subtype_b::SIMPLE_SUBTYPE_B;
pub use crate::isa_domain::simple_supertype::SimpleSupertype;
pub use crate::isa_domain::subtype_a::SubtypeA;
pub use crate::isa_domain::subtype_b::SubtypeB;
pub use crate::isa_domain::super_t::SuperT;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

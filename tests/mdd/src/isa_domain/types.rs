//! Domain to test the supertype/subtype relationship.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"isa_domain-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"isa_domain-module-definition"}}}
pub mod simple_subtype_a;
pub mod simple_subtype_b;
pub mod simple_supertype;
pub mod subtype_a;
pub mod subtype_b;
pub mod super_t;

pub use simple_subtype_a::SimpleSubtypeA;
pub use simple_subtype_b::SimpleSubtypeB;
pub use simple_supertype::SimpleSupertype;
pub use subtype_a::SubtypeA;
pub use subtype_b::SubtypeB;
pub use super_t::SuperT;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}


//! Domain to test 1-M relationships.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"one_to_many_domain-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"one_to_many_domain-module-definition"}}}
pub mod a;
pub mod b;
pub mod c;
pub mod d;
pub mod referent;

pub use crate::one_to_many_domain::a::A;
pub use crate::one_to_many_domain::b::B;
pub use crate::one_to_many_domain::c::C;
pub use crate::one_to_many_domain::d::D;
pub use crate::one_to_many_domain::referent::Referent;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

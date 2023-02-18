//! Domain to test the many flavors of 1-1 relationships.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::one_to_one-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_one-module-definition"}}}
pub mod a;
pub mod b;
pub mod c;
pub mod parameter;
pub mod referent;

pub use crate::domain::one_to_one::a::A;
pub use crate::domain::one_to_one::b::B;
pub use crate::domain::one_to_one::c::C;
pub use crate::domain::one_to_one::parameter::Parameter;
pub use crate::domain::one_to_one::referent::Referent;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

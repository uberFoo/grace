//! Domain to test the many flavors of 1-1 relationships.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::one_to_one_rwlock-module-definition-file"}}}
pub mod a;
pub mod b;
pub mod c;
pub mod parameter;
pub mod referent;

pub use crate::domain::one_to_one_rwlock::a::A;
pub use crate::domain::one_to_one_rwlock::b::B;
pub use crate::domain::one_to_one_rwlock::c::C;
pub use crate::domain::one_to_one_rwlock::parameter::Parameter;
pub use crate::domain::one_to_one_rwlock::referent::Referent;
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

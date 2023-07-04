//! Domain to test the many flavors of 1-1 relationships.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::one_to_one_rwlock_vec-module-definition-file"}}}
pub mod a;
pub mod b;
pub mod c;
pub mod parameter;
pub mod referent;

pub use crate::domain::one_to_one_rwlock_vec::a::A;
pub use crate::domain::one_to_one_rwlock_vec::b::B;
pub use crate::domain::one_to_one_rwlock_vec::c::C;
pub use crate::domain::one_to_one_rwlock_vec::parameter::Parameter;
pub use crate::domain::one_to_one_rwlock_vec::referent::Referent;
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

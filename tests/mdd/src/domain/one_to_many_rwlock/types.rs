//! Domain to test 1-M relationships.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::one_to_many_rwlock-module-definition-file"}}}
pub mod a;
pub mod b;
pub mod c;
pub mod d;
pub mod referent;

pub use crate::domain::one_to_many_rwlock::a::A;
pub use crate::domain::one_to_many_rwlock::b::B;
pub use crate::domain::one_to_many_rwlock::c::C;
pub use crate::domain::one_to_many_rwlock::d::D;
pub use crate::domain::one_to_many_rwlock::referent::Referent;
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

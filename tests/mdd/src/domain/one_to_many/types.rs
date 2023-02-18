//! Domain to test 1-M relationships.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::one_to_many-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_many-module-definition"}}}
pub mod a;
pub mod b;
pub mod c;
pub mod d;
pub mod referent;

pub use crate::domain::one_to_many::a::A;
pub use crate::domain::one_to_many::b::B;
pub use crate::domain::one_to_many::c::C;
pub use crate::domain::one_to_many::d::D;
pub use crate::domain::one_to_many::referent::Referent;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

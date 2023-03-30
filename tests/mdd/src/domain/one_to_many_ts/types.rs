//! Domain to test 1-M relationships.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::one_to_many_ts-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::one_to_many_ts-module-definition"}}}
pub mod a;
pub mod b;
pub mod c;
pub mod d;
pub mod referent;

pub use crate::domain::one_to_many_ts::a::A;
pub use crate::domain::one_to_many_ts::b::B;
pub use crate::domain::one_to_many_ts::c::C;
pub use crate::domain::one_to_many_ts::d::D;
pub use crate::domain::one_to_many_ts::referent::Referent;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

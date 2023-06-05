//! The _Metamodel_
//!
//! This is the model of the model. From here all is generated...
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::sarzak_ts-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::sarzak_ts-module-definition"}}}
pub mod acknowledged_event;
pub mod an_associative_referent;
pub mod associative;
pub mod associative_referent;
pub mod associative_referrer;
pub mod attribute;
pub mod binary;
pub mod boolean;
pub mod cardinality;
pub mod conditional;
pub mod conditionality;
pub mod event;
pub mod external;
pub mod float;
pub mod integer;
pub mod isa;
pub mod many;
pub mod object;
pub mod one;
pub mod referent;
pub mod referrer;
pub mod relationship;
pub mod s_string;
pub mod s_uuid;
pub mod state;
pub mod subtype;
pub mod supertype;
pub mod ty;
pub mod unconditional;

pub use crate::domain::sarzak_ts::acknowledged_event::AcknowledgedEvent;
pub use crate::domain::sarzak_ts::an_associative_referent::AnAssociativeReferent;
pub use crate::domain::sarzak_ts::associative::Associative;
pub use crate::domain::sarzak_ts::associative_referent::AssociativeReferent;
pub use crate::domain::sarzak_ts::associative_referrer::AssociativeReferrer;
pub use crate::domain::sarzak_ts::attribute::Attribute;
pub use crate::domain::sarzak_ts::binary::Binary;
pub use crate::domain::sarzak_ts::boolean::Boolean;
pub use crate::domain::sarzak_ts::boolean::BOOLEAN;
pub use crate::domain::sarzak_ts::cardinality::Cardinality;
pub use crate::domain::sarzak_ts::conditional::Conditional;
pub use crate::domain::sarzak_ts::conditional::CONDITIONAL;
pub use crate::domain::sarzak_ts::conditionality::Conditionality;
pub use crate::domain::sarzak_ts::event::Event;
pub use crate::domain::sarzak_ts::external::External;
pub use crate::domain::sarzak_ts::float::Float;
pub use crate::domain::sarzak_ts::float::FLOAT;
pub use crate::domain::sarzak_ts::integer::Integer;
pub use crate::domain::sarzak_ts::integer::INTEGER;
pub use crate::domain::sarzak_ts::isa::Isa;
pub use crate::domain::sarzak_ts::many::Many;
pub use crate::domain::sarzak_ts::many::MANY;
pub use crate::domain::sarzak_ts::object::Object;
pub use crate::domain::sarzak_ts::one::One;
pub use crate::domain::sarzak_ts::one::ONE;
pub use crate::domain::sarzak_ts::referent::Referent;
pub use crate::domain::sarzak_ts::referrer::Referrer;
pub use crate::domain::sarzak_ts::relationship::Relationship;
pub use crate::domain::sarzak_ts::s_string::SString;
pub use crate::domain::sarzak_ts::s_string::S_STRING;
pub use crate::domain::sarzak_ts::s_uuid::SUuid;
pub use crate::domain::sarzak_ts::s_uuid::S_UUID;
pub use crate::domain::sarzak_ts::state::State;
pub use crate::domain::sarzak_ts::subtype::Subtype;
pub use crate::domain::sarzak_ts::supertype::Supertype;
pub use crate::domain::sarzak_ts::ty::Ty;
pub use crate::domain::sarzak_ts::unconditional::Unconditional;
pub use crate::domain::sarzak_ts::unconditional::UNCONDITIONAL;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

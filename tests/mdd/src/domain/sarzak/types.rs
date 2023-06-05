//! The _Metamodel_
//!
//! This is the model of the model. From here all is generated...
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::sarzak-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::sarzak-module-definition"}}}
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

pub use crate::domain::sarzak::acknowledged_event::AcknowledgedEvent;
pub use crate::domain::sarzak::an_associative_referent::AnAssociativeReferent;
pub use crate::domain::sarzak::associative::Associative;
pub use crate::domain::sarzak::associative_referent::AssociativeReferent;
pub use crate::domain::sarzak::associative_referrer::AssociativeReferrer;
pub use crate::domain::sarzak::attribute::Attribute;
pub use crate::domain::sarzak::binary::Binary;
pub use crate::domain::sarzak::boolean::Boolean;
pub use crate::domain::sarzak::boolean::BOOLEAN;
pub use crate::domain::sarzak::cardinality::Cardinality;
pub use crate::domain::sarzak::conditional::Conditional;
pub use crate::domain::sarzak::conditional::CONDITIONAL;
pub use crate::domain::sarzak::conditionality::Conditionality;
pub use crate::domain::sarzak::event::Event;
pub use crate::domain::sarzak::external::External;
pub use crate::domain::sarzak::float::Float;
pub use crate::domain::sarzak::float::FLOAT;
pub use crate::domain::sarzak::integer::Integer;
pub use crate::domain::sarzak::integer::INTEGER;
pub use crate::domain::sarzak::isa::Isa;
pub use crate::domain::sarzak::many::Many;
pub use crate::domain::sarzak::many::MANY;
pub use crate::domain::sarzak::object::Object;
pub use crate::domain::sarzak::one::One;
pub use crate::domain::sarzak::one::ONE;
pub use crate::domain::sarzak::referent::Referent;
pub use crate::domain::sarzak::referrer::Referrer;
pub use crate::domain::sarzak::relationship::Relationship;
pub use crate::domain::sarzak::s_string::SString;
pub use crate::domain::sarzak::s_string::S_STRING;
pub use crate::domain::sarzak::s_uuid::SUuid;
pub use crate::domain::sarzak::s_uuid::S_UUID;
pub use crate::domain::sarzak::state::State;
pub use crate::domain::sarzak::subtype::Subtype;
pub use crate::domain::sarzak::supertype::Supertype;
pub use crate::domain::sarzak::ty::Ty;
pub use crate::domain::sarzak::unconditional::Unconditional;
pub use crate::domain::sarzak::unconditional::UNCONDITIONAL;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

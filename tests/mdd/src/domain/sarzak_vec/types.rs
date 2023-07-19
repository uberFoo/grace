//! The _Metamodel_
//!
//! This is the model of the model. From here all is generated...
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::sarzak_vec-module-definition-file"}}}
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

pub use crate::domain::sarzak_vec::acknowledged_event::AcknowledgedEvent;
pub use crate::domain::sarzak_vec::an_associative_referent::AnAssociativeReferent;
pub use crate::domain::sarzak_vec::associative::Associative;
pub use crate::domain::sarzak_vec::associative_referent::AssociativeReferent;
pub use crate::domain::sarzak_vec::associative_referrer::AssociativeReferrer;
pub use crate::domain::sarzak_vec::attribute::Attribute;
pub use crate::domain::sarzak_vec::binary::Binary;
pub use crate::domain::sarzak_vec::boolean::Boolean;
pub use crate::domain::sarzak_vec::boolean::BOOLEAN;
pub use crate::domain::sarzak_vec::cardinality::Cardinality;
pub use crate::domain::sarzak_vec::cardinality::CardinalityEnum;
pub use crate::domain::sarzak_vec::conditional::Conditional;
pub use crate::domain::sarzak_vec::conditional::CONDITIONAL;
pub use crate::domain::sarzak_vec::conditionality::Conditionality;
pub use crate::domain::sarzak_vec::conditionality::ConditionalityEnum;
pub use crate::domain::sarzak_vec::event::Event;
pub use crate::domain::sarzak_vec::external::External;
pub use crate::domain::sarzak_vec::float::Float;
pub use crate::domain::sarzak_vec::float::FLOAT;
pub use crate::domain::sarzak_vec::integer::Integer;
pub use crate::domain::sarzak_vec::integer::INTEGER;
pub use crate::domain::sarzak_vec::isa::Isa;
pub use crate::domain::sarzak_vec::many::Many;
pub use crate::domain::sarzak_vec::many::MANY;
pub use crate::domain::sarzak_vec::object::Object;
pub use crate::domain::sarzak_vec::one::One;
pub use crate::domain::sarzak_vec::one::ONE;
pub use crate::domain::sarzak_vec::referent::Referent;
pub use crate::domain::sarzak_vec::referrer::Referrer;
pub use crate::domain::sarzak_vec::relationship::Relationship;
pub use crate::domain::sarzak_vec::relationship::RelationshipEnum;
pub use crate::domain::sarzak_vec::s_string::SString;
pub use crate::domain::sarzak_vec::s_string::S_STRING;
pub use crate::domain::sarzak_vec::s_uuid::SUuid;
pub use crate::domain::sarzak_vec::s_uuid::S_UUID;
pub use crate::domain::sarzak_vec::state::State;
pub use crate::domain::sarzak_vec::subtype::Subtype;
pub use crate::domain::sarzak_vec::supertype::Supertype;
pub use crate::domain::sarzak_vec::ty::Ty;
pub use crate::domain::sarzak_vec::ty::TyEnum;
pub use crate::domain::sarzak_vec::unconditional::Unconditional;
pub use crate::domain::sarzak_vec::unconditional::UNCONDITIONAL;
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
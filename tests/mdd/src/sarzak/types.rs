//! The _Metamodel_
//!
//! This is the model of the model. From here all is generated...
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"sarzak-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"sarzak-module-definition"}}}
pub mod acknowledged_event;
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
pub mod state;
pub mod string;
pub mod subtype;
pub mod supertype;
pub mod ty;
pub mod unconditional;
pub mod uuid;

pub use crate::sarzak::acknowledged_event::AcknowledgedEvent;
pub use crate::sarzak::associative::Associative;
pub use crate::sarzak::associative_referent::AssociativeReferent;
pub use crate::sarzak::associative_referrer::AssociativeReferrer;
pub use crate::sarzak::attribute::Attribute;
pub use crate::sarzak::binary::Binary;
pub use crate::sarzak::boolean::BOOLEAN;
pub use crate::sarzak::cardinality::Cardinality;
pub use crate::sarzak::conditional::CONDITIONAL;
pub use crate::sarzak::conditionality::Conditionality;
pub use crate::sarzak::event::Event;
pub use crate::sarzak::external::External;
pub use crate::sarzak::float::FLOAT;
pub use crate::sarzak::integer::INTEGER;
pub use crate::sarzak::isa::Isa;
pub use crate::sarzak::many::MANY;
pub use crate::sarzak::object::Object;
pub use crate::sarzak::one::ONE;
pub use crate::sarzak::referent::Referent;
pub use crate::sarzak::referrer::Referrer;
pub use crate::sarzak::relationship::Relationship;
pub use crate::sarzak::state::State;
pub use crate::sarzak::string::STRING;
pub use crate::sarzak::subtype::Subtype;
pub use crate::sarzak::supertype::Supertype;
pub use crate::sarzak::ty::Ty;
pub use crate::sarzak::unconditional::UNCONDITIONAL;
pub use crate::sarzak::uuid::UUID;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

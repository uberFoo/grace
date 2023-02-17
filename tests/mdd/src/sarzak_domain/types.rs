//! The _Metamodel_
//!
//! This is the model of the model. From here all is generated...
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"sarzak_domain-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"sarzak_domain-module-definition"}}}
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

pub use crate::sarzak_domain::acknowledged_event::AcknowledgedEvent;
pub use crate::sarzak_domain::associative::Associative;
pub use crate::sarzak_domain::associative_referent::AssociativeReferent;
pub use crate::sarzak_domain::associative_referrer::AssociativeReferrer;
pub use crate::sarzak_domain::attribute::Attribute;
pub use crate::sarzak_domain::binary::Binary;
pub use crate::sarzak_domain::boolean::BOOLEAN;
pub use crate::sarzak_domain::cardinality::Cardinality;
pub use crate::sarzak_domain::conditional::CONDITIONAL;
pub use crate::sarzak_domain::conditionality::Conditionality;
pub use crate::sarzak_domain::event::Event;
pub use crate::sarzak_domain::external::External;
pub use crate::sarzak_domain::float::FLOAT;
pub use crate::sarzak_domain::integer::INTEGER;
pub use crate::sarzak_domain::isa::Isa;
pub use crate::sarzak_domain::many::MANY;
pub use crate::sarzak_domain::object::Object;
pub use crate::sarzak_domain::one::ONE;
pub use crate::sarzak_domain::referent::Referent;
pub use crate::sarzak_domain::referrer::Referrer;
pub use crate::sarzak_domain::relationship::Relationship;
pub use crate::sarzak_domain::state::State;
pub use crate::sarzak_domain::string::STRING;
pub use crate::sarzak_domain::subtype::Subtype;
pub use crate::sarzak_domain::supertype::Supertype;
pub use crate::sarzak_domain::ty::Ty;
pub use crate::sarzak_domain::unconditional::UNCONDITIONAL;
pub use crate::sarzak_domain::uuid::UUID;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

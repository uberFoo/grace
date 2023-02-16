//! Domain to test Associative Objects/Relationships
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"associative_domain-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"associative_domain-module-definition"}}}
pub mod acknowledged_event;
pub mod anchor;
pub mod event;
pub mod isa_ui;
pub mod state;
pub mod subtype_anchor;

pub use crate::associative_domain::acknowledged_event::AcknowledgedEvent;
pub use crate::associative_domain::anchor::Anchor;
pub use crate::associative_domain::event::Event;
pub use crate::associative_domain::isa_ui::IsaUi;
pub use crate::associative_domain::state::State;
pub use crate::associative_domain::subtype_anchor::SubtypeAnchor;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

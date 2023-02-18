//! Domain to test Associative Objects/Relationships
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::associative-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::associative-module-definition"}}}
pub mod acknowledged_event;
pub mod anchor;
pub mod event;
pub mod isa_ui;
pub mod state;
pub mod subtype_anchor;

pub use crate::domain::associative::acknowledged_event::AcknowledgedEvent;
pub use crate::domain::associative::anchor::Anchor;
pub use crate::domain::associative::event::Event;
pub use crate::domain::associative::isa_ui::IsaUi;
pub use crate::domain::associative::state::State;
pub use crate::domain::associative::subtype_anchor::SubtypeAnchor;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

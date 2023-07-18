//! Domain to test Associative Objects/Relationships
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::associative_ts-module-definition-file"}}}
pub mod acknowledged_event;
pub mod anchor;
pub mod event;
pub mod isa_ui;
pub mod state;
pub mod subtype_anchor;

pub use crate::domain::associative_ts::acknowledged_event::AcknowledgedEvent;
pub use crate::domain::associative_ts::anchor::Anchor;
pub use crate::domain::associative_ts::event::Event;
pub use crate::domain::associative_ts::isa_ui::IsaUi;
pub use crate::domain::associative_ts::state::State;
pub use crate::domain::associative_ts::subtype_anchor::SubtypeAnchor;
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

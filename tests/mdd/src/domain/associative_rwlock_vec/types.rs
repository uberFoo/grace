//! Domain to test Associative Objects/Relationships
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"domain::associative_rwlock_vec-module-definition-file"}}}
pub mod acknowledged_event;
pub mod anchor;
pub mod event;
pub mod isa_ui;
pub mod state;
pub mod subtype_anchor;

pub use crate::domain::associative_rwlock_vec::acknowledged_event::AcknowledgedEvent;
pub use crate::domain::associative_rwlock_vec::anchor::Anchor;
pub use crate::domain::associative_rwlock_vec::event::Event;
pub use crate::domain::associative_rwlock_vec::isa_ui::IsaUi;
pub use crate::domain::associative_rwlock_vec::state::State;
pub use crate::domain::associative_rwlock_vec::subtype_anchor::SubtypeAnchor;
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

//! Associative Domain
//!
//! This file was generated by: `sarzak new "associative"`.
//! The purpose of this domain is to help me develop and test domain code generation.
use uuid::{uuid, Uuid};

pub mod store;
pub mod types;

pub use store::ObjectStore;
pub use types::*;

// associative
pub const UUID_NS: Uuid = uuid!("78411374-4d65-54a9-a68a-cecf90597189");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_r10() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut store = ObjectStore::new();

        let ui0 = IsaUi::new(0, &mut store);
        let a0 = Anchor::new(0, &mut store);
        let a1 = Anchor::new(1, &mut store);
        let a2 = Anchor::new(2, &mut store);
        let _ = Anchor::new(3, &mut store);
        let _ = SubtypeAnchor::new(&a0, &ui0, &mut store);
        let _ = SubtypeAnchor::new(&a1, &ui0, &mut store);

        let subtype_anchors = ui0.borrow().r10_subtype_anchor(&store);
        assert_eq!(subtype_anchors.len(), 2);
        assert!(subtype_anchors
            .iter()
            .find(|&x| x.borrow().anchor_id == a0.borrow().id)
            .is_some());
        assert!(subtype_anchors
            .iter()
            .find(|&x| x.borrow().anchor_id == a1.borrow().id)
            .is_some());
        assert!(subtype_anchors
            .iter()
            .find(|&x| x.borrow().anchor_id == a2.borrow().id)
            .is_none());

        let subtype_anchors = a0.borrow().r10_subtype_anchor(&store);
        assert_eq!(subtype_anchors.len(), 1);
        assert!(subtype_anchors
            .iter()
            .find(|&x| x.borrow().isaui_id == ui0.borrow().id)
            .is_some());

        let subtype_anchors = a2.borrow().r10_subtype_anchor(&store);
        assert_eq!(subtype_anchors.len(), 0);
    }

    #[test]
    fn test_r20() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut store = ObjectStore::new();

        let s0 = State::new("foo".to_owned(), &mut store);
        let s1 = State::new("bar".to_owned(), &mut store);
        let s2 = State::new("baz".to_owned(), &mut store);

        let e0 = Event::new("one".to_owned(), &mut store);
        let e1 = Event::new("two".to_owned(), &mut store);
        let e2 = Event::new("three".to_owned(), &mut store);

        let _ = AcknowledgedEvent::new(&e1, &s0, &mut store);
        let _ = AcknowledgedEvent::new(&e2, &s0, &mut store);

        let _ = AcknowledgedEvent::new(&e2, &s1, &mut store);

        assert_eq!(e0.borrow().r20_acknowledged_event(&store).len(), 0);

        let ae = e1.borrow().r20_acknowledged_event(&store);
        assert_eq!(ae.len(), 1);
        assert_eq!(ae[0].borrow().state_id, s0.borrow().id);

        let ae = e2.borrow().r20_acknowledged_event(&store);
        assert_eq!(ae.len(), 2);
        assert!(ae
            .iter()
            .find(|&x| x.borrow().state_id == s0.borrow().id)
            .is_some());
        assert!(ae
            .iter()
            .find(|&x| x.borrow().state_id == s1.borrow().id)
            .is_some());

        let ae = s0.borrow().r20_acknowledged_event(&store);
        assert_eq!(ae.len(), 2);
        assert!(ae
            .iter()
            .find(|&x| x.borrow().event_id == e1.borrow().id)
            .is_some());
        assert!(ae
            .iter()
            .find(|&x| x.borrow().event_id == e2.borrow().id)
            .is_some());

        let ae = s1.borrow().r20_acknowledged_event(&store);
        assert_eq!(ae.len(), 1);
        assert!(ae
            .iter()
            .find(|&x| x.borrow().event_id == e2.borrow().id)
            .is_some());

        let ae = s2.borrow().r20_acknowledged_event(&store);
        assert_eq!(ae.len(), 0);
    }
}

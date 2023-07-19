// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"nunchuck-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-use-statements"}}}
use uuid::Uuid;

use crate::domain::external_ts::types::timestamp::Timestamp;
use serde::{Deserialize, Serialize};

use crate::domain::external_ts::store::ObjectStore as ExternalTsStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-struct-documentation"}}}
/// Bruce Lee
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-struct-definition"}}}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Nunchuck {
    pub id: Uuid,
    /// R1: [`Nunchuck`] 'needs a' [`Timestamp`]
    pub time: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-implementation"}}}
impl Nunchuck {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-struct-impl-new"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-struct-impl-new_"}}}
    /// Inter a new 'Nunchuck' in the store, and return it's `id`.
    pub fn new(time: &Timestamp, store: &mut ExternalTsStore) -> Nunchuck {
        let id = Uuid::new_v4();
        let new = Nunchuck { id, time: time.id };
        store.inter_nunchuck(new.clone());
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-struct-impl-new"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-struct-impl-new_"}}}
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"nunchuck-struct-impl-nav-forward-to-time"}}}
    /// Navigate to [`Timestamp`] across R1(1-*)
    pub fn r1_timestamp<'a>(&'a self, store: &'a ExternalTsStore) -> Vec<&Timestamp> {
        vec![store.exhume_timestamp(&self.time).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

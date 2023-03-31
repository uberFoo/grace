// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"d-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-use-statements"}}}
use uuid::Uuid;

use crate::domain::one_to_many_ts::types::referent::Referent;
use crate::domain::one_to_many_ts::UUID_NS;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_many_ts::store::ObjectStore as OneToManyTsStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-struct-documentation"}}}
/// Just an unassuming D
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct D {
    pub appellation: String,
    pub id: Uuid,
    /// R4: [`D`] 'points at' [`Referent`]
    pub ptr: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-implementation"}}}
impl D {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-struct-impl-new"}}}
    /// Inter a new 'D' in the store, and return it's `id`.
    pub fn new(appellation: String, ptr: Option<&Referent>, store: &mut OneToManyTsStore) -> D {
        let id = Uuid::new_v5(&UUID_NS, format!("{}:{:?}", appellation, ptr).as_bytes());
        let new = D {
            appellation: appellation,
            id: id,
            ptr: ptr.map(|referent| referent.id),
        };
        store.inter_d(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-struct-impl-nav-forward-cond-to-ptr"}}}
    /// Navigate to [`Referent`] across R4(1-*c)
    pub fn r4_referent<'a>(&'a self, store: &'a OneToManyTsStore) -> Vec<&Referent> {
        match self.ptr {
            Some(ref ptr) => vec![store.exhume_referent(ptr).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
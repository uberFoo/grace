// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"d-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-use-statements"}}}
use crate::domain::one_to_many::UUID_NS;
use uuid::Uuid;

use crate::domain::one_to_many::types::referent::Referent;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_many::store::ObjectStore as OneToManyStore;
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
    pub fn new(appellation: String, ptr: Option<&Referent>, store: &mut OneToManyStore) -> D {
        let id = Uuid::new_v5(&UUID_NS, format!("{}:{:?}", appellation, ptr).as_bytes());
        let new = D {
            appellation: appellation,
            ptr: ptr.map(|referent| referent.id),
            id,
        };
        store.inter_d(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-struct-impl-nav-forward-cond-to-ptr"}}}
    /// Navigate to [`Referent`] across R4(1-*c)
    pub fn r4_referent<'a>(&'a self, store: &'a OneToManyStore) -> Vec<&Referent> {
        match self.ptr {
            Some(ref ptr) => vec![store.exhume_referent(ptr).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

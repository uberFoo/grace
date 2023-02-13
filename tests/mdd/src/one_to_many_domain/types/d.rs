// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"d-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::one_to_many_domain::UUID_NS;

// Referrer imports
use crate::one_to_many_domain::types::referent::Referent;

use crate::one_to_many_domain::store::ObjectStore as OneToManyDomainStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"d-struct-documentation"}}}
/// Just an unassuming D
// {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct D {
    pub appellation: String,
    pub id: Uuid,
    /// R4: [`D`] 'points at' [`Referent`]
    pub ptr: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"d-struct-implementation"}}}
impl D {
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"d-struct-impl-new"}}}
    /// Inter a new D in the store, and return it's `id`.
    pub fn new(appellation: String, ptr: Option<&Referent>, store: &mut OneToManyDomainStore) -> D {
        let id = Uuid::new_v5(&UUID_NS, format!("{}:{:?}", appellation, ptr).as_bytes());
        let new = D {
            appellation: appellation,
            ptr: ptr.map(|referent| referent.id),
            id,
        };
        store.inter_d(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"d-struct-impl-nav-forward-cond-to-ptr"}}}
    /// Navigate to [`Referent`] across R4(1-?c)
    pub fn referent<'a>(&'a self, store: &'a OneToManyDomainStore) -> Vec<&Referent> {
        match self.ptr {
            Some(ref ptr) => vec![store.exhume_referent(ptr).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

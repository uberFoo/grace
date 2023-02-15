// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"b-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::one_to_many_domain::UUID_NS;

// Referrer imports
use crate::one_to_many_domain::types::referent::Referent;

use crate::one_to_many_domain::store::ObjectStore as OneToManyDomainStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-documentation"}}}
/// Connected to TGT via _R2_.
///
/// This is for testing a 1c-M relationship.
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct B {
    pub baz: String,
    pub id: Uuid,
    /// R2: [`B`] 'points at' [`Referent`]
    pub ptr: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-implementation"}}}
impl B {
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"b-struct-impl-new"}}}
    /// Inter a new B in the store, and return it's `id`.
    pub fn new(baz: String, ptr: Option<&Referent>, store: &mut OneToManyDomainStore) -> B {
        let id = Uuid::new_v5(&UUID_NS, format!("{}:{:?}", baz, ptr).as_bytes());
        let new = B {
            baz: baz,
            ptr: ptr.map(|referent| referent.id),
            id,
        };
        store.inter_b(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"b-struct-impl-nav-forward-cond-to-ptr"}}}
    /// Navigate to [`Referent`] across R2(1-?c)
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

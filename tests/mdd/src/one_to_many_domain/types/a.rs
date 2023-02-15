// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"a-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::one_to_many_domain::UUID_NS;

// Referrer imports
use crate::one_to_many_domain::types::referent::Referent;

use crate::one_to_many_domain::store::ObjectStore as OneToManyDomainStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-struct-documentation"}}}
/// This is the [`Referrer`] side of a 1-M relationship
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct A {
    pub id: Uuid,
    pub name: String,
    /// R1: [`A`] 'points at' [`Referent`]
    pub ptr: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-implementation"}}}
impl A {
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"a-struct-impl-new"}}}
    /// Inter a new A in the store, and return it's `id`.
    pub fn new(name: String, ptr: &Referent, store: &mut OneToManyDomainStore) -> A {
        let id = Uuid::new_v5(&UUID_NS, format!("{}:{:?}", name, ptr).as_bytes());
        let new = A {
            name: name,
            ptr: ptr.id,
            id,
        };
        store.inter_a(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"a-struct-impl-nav-forward-to-ptr"}}}
    /// Navigate to [`Referent`] across R1(1-?)
    pub fn referent<'a>(&'a self, store: &'a OneToManyDomainStore) -> Vec<&Referent> {
        vec![store.exhume_referent(&self.ptr).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

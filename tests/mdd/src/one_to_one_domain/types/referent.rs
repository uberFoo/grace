// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"referent-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::one_to_one_domain::UUID_NS;

// Referent imports
use crate::one_to_one_domain::types::a::A;
use crate::one_to_one_domain::types::b::B;
use crate::one_to_one_domain::types::c::C;

use crate::one_to_one_domain::store::ObjectStore as OneToOneDomainStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"referent-struct-documentation"}}}
/// The target of our relationship tests.
///
/// It is conditionally related to [`OneToOneConditional`] across _R2_, and it is unconditionally related to [`OneToOneUnconditional`] across _R1_.
// {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Referent {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-implementation"}}}
impl Referent {
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"referent-struct-impl-new"}}}
    /// Inter a new Referent in the store, and return it's `id`.
    pub fn new(name: String, store: &mut OneToOneDomainStore) -> Referent {
        let id = Uuid::new_v5(&UUID_NS, format!("{}", name).as_bytes());
        let new = Referent { name: name, id };
        store.inter_referent(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"referent-struct-impl-navigate-backwards-to-a"}}}
    /// Navigate to [`A`] across R1(1-1)
    //     pub fn a<'a>(&'a self, store: &'a OneToOneDomainStore) -> &A {
    //         store.iter_a().find(|a| a.1.ptr == self.id).unwrap().1
    //     }
    pub fn a<'a>(&'a self, store: &'a OneToOneDomainStore) -> &A {
        store.iter_a().find(|a| a.1.ptr == self.id).unwrap().1
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"referent-struct-impl-navigate-backwards-to-b"}}}
    /// Navigate to [`B`] across R2(1-1)
    //     pub fn b<'a>(&'a self, store: &'a OneToOneDomainStore) -> &B {
    //         store.iter_b().find(|b| b.1.ptr == self.id).unwrap().1
    //     }
    pub fn b<'a>(&'a self, store: &'a OneToOneDomainStore) -> &B {
        store.iter_b().find(|b| b.1.ptr == self.id).unwrap().1
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"referent-struct-impl-navigate-backwards-to-c"}}}
    /// Navigate to [`C`] across R3(1-1)
    pub fn c<'a>(&'a self, store: &'a OneToOneDomainStore) -> &C {
        store.iter_c().find(|c| c.1.ptr == self.id).unwrap().1
//     }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"referent-struct-impl-navigate-backwards-to-b"}}}
    /// Navigate to [`B`] across R2(1-1)
    //     pub fn b<'a>(&'a self, store: &'a OneToOneDomainStore) -> &B {
    //         store.iter_b().find(|b| b.1.ptr == self.id).unwrap().1
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"referent-struct-impl-navigate-backwards-to-a"}}}
    /// Navigate to [`A`] across R1(1-1)
//     pub fn a<'a>(&'a self, store: &'a OneToOneDomainStore) -> &A {
//         store.iter_a().find(|a| a.1.ptr == self.id).unwrap().1
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}


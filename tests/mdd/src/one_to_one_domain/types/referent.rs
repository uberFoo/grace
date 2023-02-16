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

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-documentation"}}}
/// The target of our relationship tests.
///
/// It is conditionally related to [`OneToOneConditional`] across _R2_, and it is unconditionally
/// related to [`OneToOneUnconditional`] across _R1_.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Referent {
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-implementation"}}}
impl Referent {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-new"}}}
    /// Inter a new Referent in the store, and return it's `id`.
    pub fn new(name: String, store: &mut OneToOneDomainStore) -> Referent {
        let id = Uuid::new_v5(&UUID_NS, format!("{}", name).as_bytes());
        let new = Referent { name: name, id };
        store.inter_referent(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-cond-to-a"}}}
    /// Navigate to [`A`] across R1(1-1c)
    pub fn r1_a<'a>(&'a self, store: &'a OneToOneDomainStore) -> Vec<&A> {
        let a = store.iter_a().find(|a| a.1.ptr == self.id);
        match a {
            Some(ref a) => vec![a.1],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-one-to-b"}}}
    /// Navigate to [`B`] across R2(1-1)
    pub fn r2_b<'a>(&'a self, store: &'a OneToOneDomainStore) -> Vec<&B> {
        vec![store.iter_b().find(|b| b.1.ptr == self.id).unwrap().1]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-one-bi-cond-to-c"}}}
    /// Navigate to [`C`] across R3(1c-1c)
    pub fn r3_c<'a>(&'a self, store: &'a OneToOneDomainStore) -> Vec<&C> {
        let c = store.iter_c().find(|c| c.1.ptr == Some(self.id));
        match c {
            Some(ref c) => vec![c.1],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

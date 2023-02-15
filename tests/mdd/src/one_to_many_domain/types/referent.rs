// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"referent-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::one_to_many_domain::UUID_NS;

// Referent imports
use crate::one_to_many_domain::types::a::A;
use crate::one_to_many_domain::types::b::B;
use crate::one_to_many_domain::types::c::C;
use crate::one_to_many_domain::types::d::D;

use crate::one_to_many_domain::store::ObjectStore as OneToManyDomainStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-documentation"}}}
/// The object of so many relationships
///
/// I’m related to stuff.
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
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"referent-struct-impl-new"}}}
    /// Inter a new Referent in the store, and return it's `id`.
    pub fn new(name: String, store: &mut OneToManyDomainStore) -> Referent {
        let id = Uuid::new_v5(&UUID_NS, format!("{}", name).as_bytes());
        let new = Referent { name: name, id };
        store.inter_referent(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"referent-struct-impl-nav-backward-1_M-to-a"}}}
    /// Navigate to [`A`] across R1(1-M)
    pub fn a<'a>(&'a self, store: &'a OneToManyDomainStore) -> Vec<&A> {
        store
            .iter_a()
            .filter_map(|a| if a.1.ptr == self.id { Some(a.1) } else { None })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"referent-struct-impl-nav-backward-1_Mc-to-b"}}}
    /// Navigate to [`B`] across R2(1-Mc)
    pub fn b<'a>(&'a self, store: &'a OneToManyDomainStore) -> Vec<&B> {
        store
            .iter_b()
            .filter_map(|b| {
                if b.1.ptr == Some(self.id) {
                    Some(b.1)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"referent-struct-impl-nav-backward-1_M-to-c"}}}
    /// Navigate to [`C`] across R3(1-M)
    pub fn c<'a>(&'a self, store: &'a OneToManyDomainStore) -> Vec<&C> {
        store
            .iter_c()
            .filter_map(|c| if c.1.ptr == self.id { Some(c.1) } else { None })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"referent-struct-impl-nav-backward-1_Mc-to-d"}}}
    /// Navigate to [`D`] across R4(1-Mc)
    pub fn d<'a>(&'a self, store: &'a OneToManyDomainStore) -> Vec<&D> {
        store
            .iter_d()
            .filter_map(|d| {
                if d.1.ptr == Some(self.id) {
                    Some(d.1)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

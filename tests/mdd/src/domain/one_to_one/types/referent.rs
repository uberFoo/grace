// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"referent-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-use-statements"}}}
use uuid::Uuid;

use crate::domain::one_to_one::types::a::A;
use crate::domain::one_to_one::types::b::B;
use crate::domain::one_to_one::types::c::C;
use crate::domain::one_to_one::UUID_NS;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_one::store::ObjectStore as OneToOneStore;
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
    /// Inter a new 'Referent' in the store, and return it's `id`.
    pub fn new(name: String, store: &mut OneToOneStore) -> Referent {
        let id = Uuid::new_v4();
        let new = Referent { id: id, name: name };
        store.inter_referent(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-cond-to-a"}}}
    /// Navigate to [`A`] across R1(1-1c)
    pub fn r1c_a<'a>(&'a self, store: &'a OneToOneStore) -> Vec<&A> {
        let a = store.iter_a().find(|a| a.ptr == self.id);
        match a {
            Some(ref a) => vec![a],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-one-to-b"}}}
    /// Navigate to [`B`] across R2(1-1)
    pub fn r2_b<'a>(&'a self, store: &'a OneToOneStore) -> Vec<&B> {
        vec![store.iter_b().find(|b| b.ptr == self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referent-struct-impl-nav-backward-one-bi-cond-to-c"}}}
    /// Navigate to [`C`] across R3(1c-1c)
    pub fn r3c_c<'a>(&'a self, store: &'a OneToOneStore) -> Vec<&C> {
        let c = store.iter_c().find(|c| c.ptr == Some(self.id));
        match c {
            Some(ref c) => vec![c],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

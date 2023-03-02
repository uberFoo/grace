// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"b-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-use-statements"}}}
use uuid::Uuid;

use crate::domain::one_to_one::types::referent::Referent;
use crate::domain::one_to_one::UUID_NS;
use serde::{Deserialize, Serialize};

use crate::domain::one_to_one::store::ObjectStore as OneToOneStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-documentation"}}}
/// B: Referrer Unconditional to Referent
///
/// This is a plain Jayne 😉 1-1 relationship, where this guy is formalizing.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct B {
    pub bit: bool,
    pub id: Uuid,
    /// R2: [`B`] 'points at' [`Referent`]
    pub ptr: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-implementation"}}}
impl B {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-impl-new"}}}
    /// Inter a new 'B' in the store, and return it's `id`.
    pub fn new(bit: bool, ptr: &Referent, store: &mut OneToOneStore) -> B {
        let id = Uuid::new_v5(&UUID_NS, format!("{}:{:?}", bit, ptr).as_bytes());
        let new = B {
            bit: bit,
            ptr: ptr.id,
            id,
        };
        store.inter_b(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"b-struct-impl-nav-forward-to-ptr"}}}
    /// Navigate to [`Referent`] across R2(1-*)
    pub fn r2_referent<'a>(&'a self, store: &'a OneToOneStore) -> Vec<&Referent> {
        vec![store.exhume_referent(&self.ptr).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

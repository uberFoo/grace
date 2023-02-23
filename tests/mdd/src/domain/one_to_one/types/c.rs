// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"c-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::domain::one_to_one::UUID_NS;

// Referrer imports
use crate::domain::one_to_one::types::referent::Referent;

use crate::domain::one_to_one::store::ObjectStore as OneToOneStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-documentation"}}}
/// C: Referrer to [`Referent`] Bi-Conditional
///
/// This will be an interesting one to translate. Hopefully not too gnarly.🤘
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct C {
    pub id: Uuid,
    pub like_water: f64,
    /// R3: [`C`] 'points at' [`Referent`]
    pub ptr: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-implementation"}}}
impl C {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-impl-new"}}}
    /// Inter a new C in the store, and return it's `id`.
    pub fn new(like_water: f64, ptr: Option<&Referent>, store: &mut OneToOneStore) -> C {
        let id = Uuid::new_v5(&UUID_NS, format!("{}:{:?}", like_water, ptr).as_bytes());
        let new = C {
            like_water: like_water,
            ptr: ptr.map(|referent| referent.id),
            id,
        };
        store.inter_c(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-impl-nav-forward-cond-to-ptr"}}}
    /// Navigate to [`Referent`] across R3(1-*c)
    pub fn r3_referent<'a>(&'a self, store: &'a OneToOneStore) -> Vec<&Referent> {
        match self.ptr {
            Some(ref ptr) => vec![store.exhume_referent(ptr).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

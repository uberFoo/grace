// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"c-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::one_to_one_domain::UUID_NS;

// Referrer imports
use crate::one_to_one_domain::types::referent::Referent;

use crate::one_to_one_domain::store::ObjectStore as OneToOneDomainStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"c-struct-documentation"}}}
/// C: Referrer to [`Referent`] Bi-Conditional
///
/// This will be an interesting one to translate. Hopefully not too gnarly.🤘
// {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct C {
    pub id: Uuid,
    pub like_water: f64,
    /// R3: [`C`] 'points at' [`Referent`]
    pub ptr: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-struct-implementation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"c-implementation"}}}
impl C {
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"c-struct-impl-new"}}}
    /// Inter a new C in the store, and return it's `id`.
    //     pub fn new(like_water: f64, ptr: &Referent, store: &mut OneToOneDomainStore) -> C {
    //     pub fn new(like_water: f64) -> C {
    //     pub fn new(like_water: f64, ptr: &Referent, store: &mut OneToOneDomainStore) -> C {
    //         let id = Uuid::new_v5(&UUID_NS, format!("{}:{:?}", like_water, ptr).as_bytes());
    //     pub fn new(like_water: f64, ptr: Option<&Referent>, store: &mut OneToOneDomainStore) -> C {
    //         let id = Uuid::new_v5(&UUID_NS, format!("{}:{}", like_water, ptr).as_bytes());
    //         let id = Uuid::new_v5(&UUID_NS, format!("{}:{:?}", like_water, ptr).as_bytes());
    //     pub fn new(like_water: f64, store: &mut OneToOneDomainStore) -> C {
    //         let id = Uuid::new_v5(&UUID_NS, format!("{}", like_water).as_bytes());
    pub fn new(like_water: f64, ptr: Option<&Referent>, store: &mut OneToOneDomainStore) -> C {
        let id = Uuid::new_v5(&UUID_NS, format!("{}:{:?}", like_water, ptr).as_bytes());
        let new = C {
            like_water: like_water,
            //             ptr: ptr.id,
            //             ptr: ptr,
            //             ptr: ptr.map(|referent| referent.id),
            ptr: ptr.map(|referent| referent.id),
            id,
        };
        store.inter_c(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"c-struct-impl-navigate-to-ptr"}}}
    /// Navigate to [`Referent`] across R3(1-1)
    //     pub fn ptr<'a>(&'a self, store: &'a OneToOneDomainStore) -> &Referent {
    //         store.exhume_referent(&self.ptr).unwrap()
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"c-struct-impl-navigate-backwards-to-referent"}}}
    /// Navigate to [`Referent`] across R3(1-1c)
    //     pub fn referent<'a>(&'a self, store: &'a OneToOneDomainStore) -> Vec<&Referent> {
    //         vec![
    //             store
    //                 .iter_referent()
    //                 .find(|referent| referent.1.ptr == Some(self.id))
    //                 .unwrap()
    //                 .1,
    //         ]
    /// Navigate to [`Referent`] across R3(1c-1)
    //     pub fn ptr<'a>(&'a self, store: &'a OneToOneDomainStore) -> Vec<&Referent> {
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"c-struct-impl-nav-forward-cond-to-ptr"}}}
    /// Navigate to [`Referent`] across R3(1-?c)
    pub fn referent<'a>(&'a self, store: &'a OneToOneDomainStore) -> Vec<&Referent> {
        match self.ptr {
            Some(ref ptr) => vec![store.exhume_referent(ptr).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

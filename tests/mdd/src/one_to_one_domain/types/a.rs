// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"a-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::one_to_one_domain::UUID_NS;

// Referrer imports
use crate::one_to_one_domain::types::referent::Referent;

use crate::one_to_one_domain::store::ObjectStore as OneToOneDomainStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"a-struct-documentation"}}}
/// A: Referrer with Conditional [`Referent`]
///
/// This type is related to the [`Referent`] across a conditional relationship. This is 1-1c, and given that I am the referrer, I have the referential attribute/I am formalizing the relationship. I think I prefer the latter language, but the former is very descriptive...
// {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct A {
    pub id: Uuid,
    pub number: i64,
    /// R1: [`A`] 'points at' [`Referent`]
    pub ptr: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-struct-implementation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"a-implementation"}}}
impl A {
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"a-struct-impl-new"}}}
    /// Inter a new A in the store, and return it's `id`.
    //     pub fn new(number: i64, ptr: &Referent, store: &mut OneToOneDomainStore) -> A {
    //         let id = Uuid::new_v5(&UUID_NS, format!("{}:{:?}", number, ptr).as_bytes());
    //         let new = A {
    //             number: number,
    //             ptr: ptr.id,
    //             id,
    //         };
    //     pub fn new(number: i64, store: &mut OneToOneDomainStore) -> A {
    //         let id = Uuid::new_v5(&UUID_NS, format!("{}", number).as_bytes());
    //         let new = A { number: number, id };
    //     pub fn new(number: i64, ptr: Option<&Referent>, store: &mut OneToOneDomainStore) -> A {
    pub fn new(number: i64, ptr: &Referent, store: &mut OneToOneDomainStore) -> A {
        let id = Uuid::new_v5(&UUID_NS, format!("{}:{:?}", number, ptr).as_bytes());
        let new = A {
            number: number,
            //             ptr: ptr.map(|referent| referent.id),
            ptr: ptr.id,
            id,
        };
        store.inter_a(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"a-struct-impl-navigate-to-ptr"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"a-struct-impl-navigate-backwards-to-referent"}}}
    /// Navigate to [`Referent`] across R1(1-1)
    //     pub fn ptr<'a>(&'a self, store: &'a OneToOneDomainStore) -> &Referent {
    //         store.exhume_referent(&self.ptr).unwrap()
    //     pub fn referent<'a>(&'a self, store: &'a OneToOneDomainStore) -> Vec<&Referent> {
    //         vec![
    //             store
    //                 .iter_referent()
    //                 .find(|referent| referent.1.ptr == self.id)
    //                 .unwrap()
    //                 .1,
    //         ]
    //     pub fn ptr<'a>(&'a self, store: &'a OneToOneDomainStore) -> Vec<&Referent> {
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"a-struct-impl-nav-forward-to-ptr"}}}
    /// Navigate to [`Referent`] across R1(1-?)
    pub fn referent<'a>(&'a self, store: &'a OneToOneDomainStore) -> Vec<&Referent> {
        vec![store.exhume_referent(&self.ptr).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"everything-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::everything_domain::UUID_NS;

// Referrer imports
use crate::everything_domain::types::rando_object::RandoObject;

use crate::everything_domain::store::ObjectStore as EverythingDomainStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"everything-struct-documentation"}}}
/// An object, with everything on it!
// {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Everything {
    pub bool: bool,
    pub float: f64,
    pub id: Uuid,
    pub int: i64,
    pub string: String,
    /// R1: [`Everything`] 'points at' [`RandoObject`]
    pub rando: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-struct-implementation"}}}
impl Everything {
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"everything-struct-impl-new"}}}
    /// Inter a new Everything in the store, and return it's `id`.
    pub fn new(
        bool: bool,
        float: f64,
        int: i64,
        string: String,
        //         rando: &RandoObject,
        rando: &RandoObject,
        store: &mut EverythingDomainStore,
    ) -> Everything {
        let id = Uuid::new_v5(
            &UUID_NS,
            //             format!("{}:{}:{}:{}:{:?}", bool, float, int, string, rando).as_bytes(),
            //             format!("{}:{}:{}:{}", bool, float, int, string).as_bytes(),
            format!("{}:{}:{}:{}:{:?}", bool, float, int, string, rando).as_bytes(),
        );
        let new = Everything {
            bool: bool,
            float: float,
            int: int,
            string: string,
            //             rando: rando.id,
            rando: rando.id,
            id,
        };
        store.inter_everything(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"rando_object-struct-impl-rando"}}}
    /// Navigate R1 → RandoObject
    //     pub fn rando(&self) {}
    //     pub fn rando(&self) -> RandoObject {}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"everything-struct-impl-rando"}}}
    /// Navigate to RandoObject across R1
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"everything-struct-impl-navigate-to-rando"}}}
    /// Navigate to [`RandoObject`] across R1
    //     pub fn rando(&self) -> &RandoObject {}
    //     pub fn rando(&self) -> &RandoObject {
    //         self.rando
    //     pub fn rando(&self, store: &ObjectStore) -> &RandoObject {
    //     pub fn rando(&self, store: &EverythingDomainStore) -> &RandoObject {
    //         store.exhume_rando_object(self.rando).unwrap()
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"everything-struct-impl-navigate-backwards-to-rando_object"}}}
    /// Navigate to [`RandoObject`] across R1(1-1)
    //     pub fn rando<'a>(&'a self, store: &'a EverythingDomainStore) -> &RandoObject {
    //         store.exhume_rando_object(&self.rando).unwrap()
    //     pub fn rando_object<'a>(&'a self, store: &'a EverythingDomainStore) -> Vec<&RandoObject> {
    //         vec![
    //             store
    //                 .iter_rando_object()
    //                 .find(|rando_object| rando_object.1.rando == self.id)
    //                 .unwrap()
    //                 .1,
    //         ]
    //     pub fn rando<'a>(&'a self, store: &'a EverythingDomainStore) -> Vec<&RandoObject> {
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"everything-struct-impl-nav-forward-to-rando"}}}
    /// Navigate to [`RandoObject`] across R1(1-?)
    pub fn rando_object<'a>(&'a self, store: &'a EverythingDomainStore) -> Vec<&RandoObject> {
        vec![store.exhume_rando_object(&self.rando).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

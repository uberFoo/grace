// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"no-obj-here-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"rando_object-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::everything_domain::store::ObjectStore as EverythingDomainStore;
use crate::everything_domain::UUID_NS;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"rando_object-struct-documentation"}}}
/// Just some random object with which we wish to relate
///
/// How tawdry.
// {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct RandoObject {
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-struct-implementation"}}}
impl RandoObject {
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"rando_object-struct-impl-new"}}}
    /// Inter a new RandoObject in the store, and return it's `id`.
    pub fn new(store: &mut EverythingDomainStore) -> RandoObject {
        let id = Uuid::new_v5(&UUID_NS, format!("",).as_bytes());
        let new = RandoObject { id };
        //         store.inter_rando_object(new.clone());
        //         new
        //     }
        // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-struct-implementation"}}}
        // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"rando_object-struct-impl-newish"}}}
        /// Inter a new RandoObject in the store, and return it's `id`.
        //     pub fn new() -> RandoObject {
        //         let id = Uuid::new_v5(&UUID_NS, format!("",).as_bytes());
        //     pub fn new(store: EverythingDomainStore) -> RandoObject {
        //     pub fn new(store: &EverythingDomainStore) -> RandoObject {
        //         let id = Uuid::new_v5(&UUID_NS, format!("{}", store).as_bytes());
        //     pub fn new(store: &mut EverythingDomainStore) -> RandoObject {
        //     pub fn new(store: &uberfoo) -> RandoObject {
        //     pub fn new(store: &mut EverythingDomainStore) -> RandoObject {
        //     pub fn newish(store: &mut EverythingDomainStore) -> RandoObject {
        //         let id = Uuid::new_v5(&UUID_NS, format!("",).as_bytes());
        //         let new = RandoObject { id };
        //         let newish = RandoObject { id };
        store.inter_rando_object(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-struct-implementation"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"rando_object-struct-impl-newish"}}}
    /// Inter a new RandoObject in the store, and return it's `id`.
    pub fn newish(store: &mut EverythingDomainStore) -> RandoObject {
        let id = Uuid::new_v5(&UUID_NS, format!("",).as_bytes());
        let newish = RandoObject { id };
        store.inter_rando_object(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

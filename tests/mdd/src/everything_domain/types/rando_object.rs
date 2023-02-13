// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"rando_object-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::everything_domain::UUID_NS;

// Referent imports
use crate::everything_domain::types::everything::Everything;

use crate::everything_domain::store::ObjectStore as EverythingDomainStore;
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
    //     pub fn new(store: &mut EverythingDomainStore) -> RandoObject {
    //         let id = Uuid::new_v5(&UUID_NS, format!("",).as_bytes());
    //         let new = RandoObject { id };
    //     pub fn new(rando: &Everything, store: &mut EverythingDomainStore) -> RandoObject {
    //         let id = Uuid::new_v5(&UUID_NS, format!("{:?}", rando).as_bytes());
    //         let new = RandoObject {
    //             rando: rando.id,
    //             id,
    //         };
    pub fn new(store: &mut EverythingDomainStore) -> RandoObject {
        let id = Uuid::new_v5(&UUID_NS, format!("",).as_bytes());
        let new = RandoObject { id };
        store.inter_rando_object(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"everything-struct-impl-everything"}}}
    /// Navigate R1 → Everything
    //     pub fn everything(&self) {}
    //     pub fn everything(&self) -> Everything {}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"rando_object-struct-impl-everything"}}}
    /// Navigate to Everything across R1
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"rando_object-struct-impl-navigate-backwards-to-everything"}}}
    /// Navigate to [`Everything`] across R1
    //     pub fn everything(&self) -> &Everything {}
    //     pub fn everything(&self, store: &ObjectStore) -> &Everything {
    //     pub fn everything(&self, store: &EverythingDomainStore) -> &Everything {
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"rando_object-struct-impl-navigate-to-rando"}}}
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"rando_object-struct-impl-nav-backward-one-to-everything"}}}
    /// Navigate to [`Everything`] across R1(1-1)
    //     pub fn everything<'a>(&'a self, store: &'a EverythingDomainStore) -> &Everything {
    //         store.exhume_everything(&self.id).unwrap()
    //         store
    //             .iter_everything()
    //             .filter(|everything| everything.1.rando == self.id)
    //             .find(|everything| everything.1.rando == self.id)
    //             .unwrap()
    //             .1
    //     pub fn rando<'a>(&'a self, store: &'a EverythingDomainStore) -> Vec<&Everything> {
    //         vec![store.exhume_everything(&self.rando).unwrap()]
    pub fn everything<'a>(&'a self, store: &'a EverythingDomainStore) -> Vec<&Everything> {
        vec![
            store
                .iter_everything()
                .find(|everything| everything.1.rando == self.id)
                .unwrap()
                .1,
        ]
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

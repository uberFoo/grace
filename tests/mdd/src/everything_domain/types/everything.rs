// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"everything-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::everything_domain::store::ObjectStore as EverythingDomainStore;
use crate::everything_domain::types::rando_object::RandoObject;
use crate::everything_domain::UUID_NS;
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
        rando: &RandoObject,
        //         store: EverythingDomainStore,
        //         store: &EverythingDomainStore,
        store: &mut EverythingDomainStore,
    ) -> Everything {
        let id = Uuid::new_v5(
            &UUID_NS,
            //             format!("{}:{}:{}:{}:{:?}", bool, float, int, string, rando).as_bytes(),
            //             format!(
            //                 "{}:{}:{}:{}:{:?}:{}",
            //                 bool, float, int, string, rando, store
            //             )
            //             .as_bytes(),
            format!("{}:{}:{}:{}:{:?}", bool, float, int, string, rando).as_bytes(),
        );
        let new = Everything {
            bool: bool,
            float: float,
            int: int,
            string: string,
            rando: rando.id,
            id,
        };
        store.inter_everything(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

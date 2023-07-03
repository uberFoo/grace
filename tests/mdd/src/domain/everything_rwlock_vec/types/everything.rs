// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"everything-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use crate::domain::everything_rwlock_vec::types::rando_object::RandoObject;
use serde::{Deserialize, Serialize};

use crate::domain::everything_rwlock_vec::store::ObjectStore as EverythingRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-struct-documentation"}}}
/// An object, with everything on it!
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Everything {
    pub bool: bool,
    pub float: f64,
    pub id: usize,
    pub int: i64,
    pub s_string: String,
    /// R1: [`Everything`] 'points at' [`RandoObject`]
    pub rando: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-implementation"}}}
impl Everything {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-struct-impl-new"}}}
    /// Inter a new 'Everything' in the store, and return it's `id`.
    pub fn new(
        bool: bool,
        float: f64,
        int: i64,
        s_string: String,
        rando: &Arc<RwLock<RandoObject>>,
        store: &mut EverythingRwlockVecStore,
    ) -> Arc<RwLock<Everything>> {
        store.inter_everything(|id| {
            Arc::new(RwLock::new(Everything {
                bool,
                float,
                id,
                int,
                s_string: s_string.to_owned(),
                rando: rando.read().unwrap().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-struct-impl-nav-forward-to-rando"}}}
    /// Navigate to [`RandoObject`] across R1(1-*)
    pub fn r1_rando_object<'a>(
        &'a self,
        store: &'a EverythingRwlockVecStore,
    ) -> Vec<Arc<RwLock<RandoObject>>> {
        span!("r1_rando_object");
        vec![store.exhume_rando_object(&self.rando).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

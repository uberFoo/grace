// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"alpha-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"alpha-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::isa_rwlock::types::gamma::Gamma;
use serde::{Deserialize, Serialize};

use crate::domain::isa_rwlock::store::ObjectStore as IsaRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"alpha-hybrid-struct-definition"}}}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Alpha {
    pub subtype: AlphaEnum,
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"alpha-hybrid-enum-definition"}}}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum AlphaEnum {
    Gamma(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"alpha-implementation"}}}
impl Alpha {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"alpha-struct-impl-new_gamma"}}}
    /// Inter a new Alpha in the store, and return it's `id`.
    pub fn new_gamma(
        name: String,
        subtype: &Arc<RwLock<Gamma>>,
        store: &mut IsaRwlockStore,
    ) -> Arc<RwLock<Alpha>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Alpha {
            name: name,
            subtype: AlphaEnum::Gamma(subtype.read().unwrap().id), // b
            id,
        }));
        store.inter_alpha(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

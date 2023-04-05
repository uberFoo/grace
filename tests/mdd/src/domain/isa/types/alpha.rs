// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"alpha-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"alpha-use-statements"}}}
use uuid::Uuid;

use crate::domain::isa::types::gamma::Gamma;
use serde::{Deserialize, Serialize};

use crate::domain::isa::store::ObjectStore as IsaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"alpha-hybrid-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Alpha {
    pub subtype: AlphaEnum,
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"alpha-hybrid-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum AlphaEnum {
    Gamma(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"alpha-implementation"}}}
impl Alpha {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"alpha-struct-impl-new"}}}
    /// Inter a new `Gamma' in the store, and return it's `id`.
    pub fn new_gamma(name: String, subtype: &Gamma, store: &mut IsaStore) -> Alpha {
        let id = subtype.id;
        let new = Alpha {
            id: id,
            name: name,
            subtype: AlphaEnum::Gamma(subtype.id),
        };
        store.inter_alpha(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"alpha-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"alpha-use-statements"}}}
use uuid::Uuid;

use crate::domain::isa_clone::types::gamma::Gamma;
use serde::{Deserialize, Serialize};

use crate::domain::isa_clone::store::ObjectStore as IsaCloneStore;
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
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"alpha-struct-impl-new_gamma"}}}
    /// Inter a new Alpha in the store, and return it's `id`.
    pub fn new_gamma(name: String, subtype: &Gamma, store: &mut IsaCloneStore) -> Alpha {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
        let new = Alpha {
            name: name,
            subtype: AlphaEnum::Gamma(subtype.id),
            id,
        };
        store.inter_alpha(new.clone());
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"alpha-struct-impl-new_gamma_"}}}
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

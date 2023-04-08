// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"beta-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-use-statements"}}}
use uuid::Uuid;

use crate::domain::isa::types::gamma::Gamma;
use crate::domain::isa::types::super_bar::SuperBar;
use serde::{Deserialize, Serialize};

use crate::domain::isa::store::ObjectStore as IsaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-enum-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-hybrid-documentation"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-enum-definition"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-hybrid-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Beta {
    pub subtype: BetaEnum,
    pub id: Uuid,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-hybrid-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum BetaEnum {
    Gamma(Uuid),
    SuperBar(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-implementation"}}}
impl Beta {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-new-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-struct-impl-new"}}}
    /// Inter a new Beta in the store, and return it's `id`.
    pub fn new_gamma(name: String, subtype: &Gamma, store: &mut IsaStore) -> Beta {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
        let new = Beta {
            name: name,
            subtype: BetaEnum::Gamma(subtype.id),
            id,
        };
        store.inter_beta(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-get-id-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-struct-impl-new"}}}
    /// Inter a new Beta in the store, and return it's `id`.
    pub fn new_super_bar(name: String, subtype: &SuperBar, store: &mut IsaStore) -> Beta {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id();
        let new = Beta {
            name: name,
            subtype: BetaEnum::SuperBar(subtype.id()),
            id,
        };
        store.inter_beta(new.clone());
        new
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-get-id-impl"}}}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

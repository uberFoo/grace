// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"beta-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-use-statements"}}}
use uuid::Uuid;

use crate::domain::isa::types::gamma::Gamma;
use crate::domain::isa::types::super_bar::SuperBar;
use serde::{Deserialize, Serialize};

use crate::domain::isa::store::ObjectStore as IsaStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-hybrid-documentation"}}}
/// This test is not complete.
///
/// To complete this test add Gamma to R11. The relationship to "Super Bar" is just so that
/// we can create an instance of Beta to soothe the compiler.
///
/// See grace#58.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
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
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-struct-impl-new"}}}
    /// Inter a new `Gamma' in the store, and return it's `id`.
    pub fn new_gamma(name: String, subtype: &Gamma, store: &mut IsaStore) -> Beta {
        let id = subtype.id;
        let new = Beta {
            id: id,
            name: name,
            subtype: BetaEnum::Gamma(subtype.id),
        };
        store.inter_beta(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"beta-struct-impl-new"}}}
    /// Inter a new `Super Bar' in the store, and return it's `id`.
    pub fn new_super_bar(name: String, subtype: &SuperBar, store: &mut IsaStore) -> Beta {
        let id = subtype.id();
        let new = Beta {
            id: id,
            name: name,
            subtype: BetaEnum::SuperBar(subtype.id()),
        };
        store.inter_beta(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

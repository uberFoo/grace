// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"alpha-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"alpha-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::domain::isa_vec::types::gamma::Gamma;
use serde::{Deserialize, Serialize};

use crate::domain::isa_vec::store::ObjectStore as IsaVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"alpha-hybrid-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Alpha {
    pub subtype: AlphaEnum,
    pub id: usize,
    pub name: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"alpha-hybrid-enum-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum AlphaEnum {
    Gamma(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"alpha-implementation"}}}
impl Alpha {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"alpha-struct-impl-new_gamma"}}}
    /// Inter a new Alpha in the store, and return it's `id`.
    pub fn new_gamma(
        name: String,
        subtype: &Rc<RefCell<Gamma>>,
        store: &mut IsaVecStore,
    ) -> Rc<RefCell<Alpha>> {
        store.inter_alpha(|id| {
            Rc::new(RefCell::new(Alpha {
                name: name.to_owned(),
                subtype: AlphaEnum::Gamma(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"alpha-implementation"}}}
impl PartialEq for Alpha {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.name == other.name
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

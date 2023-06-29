// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"super_foo-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-use-statements"}}}
use crate::domain::isa_vec::store::ObjectStore as IsaVecStore;
use crate::domain::isa_vec::types::gamma::Gamma;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-enum-definition"}}}
#[derive(Copy, Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum SuperFoo {
    Gamma(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-implementation"}}}
impl SuperFoo {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-new-impl"}}}
    /// Create a new instance of SuperFoo::Gamma
    pub fn new_gamma(gamma: &Rc<RefCell<Gamma>>, store: &mut IsaVecStore) -> Rc<RefCell<Self>> {
        let id = gamma.borrow().id;
        if let Some(gamma) = store.exhume_super_foo(id) {
            gamma
        } else {
            store.inter_super_foo(|id| Rc::new(RefCell::new(Self::Gamma(id))))
        }
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"super_foo-get-id-impl"}}}
    pub fn id(&self) -> usize {
        match self {
            SuperFoo::Gamma(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

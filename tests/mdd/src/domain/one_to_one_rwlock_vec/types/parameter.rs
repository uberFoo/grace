// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"parameter-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use tracy_client::span;
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::domain::one_to_one_rwlock_vec::store::ObjectStore as OneToOneRwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-documentation"}}}
/// Parameter
///
/// A parameter is an input to a function.
///
/// This is testing a reflexive relationship.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-definition"}}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Parameter {
    pub id: usize,
    pub name: String,
    /// R8: [`Parameter`] 'came before' [`Parameter`]
    pub next: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-implementation"}}}
impl Parameter {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-new"}}}
    /// Inter a new 'Parameter' in the store, and return it's `id`.
    pub fn new(
        name: String,
        next: Option<&Arc<RwLock<Parameter>>>,
        store: &mut OneToOneRwlockVecStore,
    ) -> Arc<RwLock<Parameter>> {
        store.inter_parameter(|id| {
            Arc::new(RwLock::new(Parameter {
                id,
                name: name.to_owned(),
                next: next.map(|parameter| parameter.read().unwrap().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`Parameter`] across R8(1-*c)
    pub fn r8_parameter<'a>(
        &'a self,
        store: &'a OneToOneRwlockVecStore,
    ) -> Vec<Arc<RwLock<Parameter>>> {
        span!("r8_parameter");
        match self.next {
            Some(ref next) => vec![store.exhume_parameter(&next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-backward-one-bi-cond-to-parameter"}}}
    /// Navigate to [`Parameter`] across R8(1c-1c)
    pub fn r8c_parameter<'a>(
        &'a self,
        store: &'a OneToOneRwlockVecStore,
    ) -> Vec<Arc<RwLock<Parameter>>> {
        span!("r8_parameter");
        let parameter = store
            .iter_parameter()
            .find(|parameter| parameter.read().unwrap().next == Some(self.id));
        match parameter {
            Some(ref parameter) => vec![parameter.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-implementation"}}}
impl PartialEq for Parameter {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.next == other.next
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

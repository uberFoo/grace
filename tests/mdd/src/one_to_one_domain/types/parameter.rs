// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"parameter-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::one_to_one_domain::UUID_NS;

use crate::one_to_one_domain::store::ObjectStore as OneToOneDomainStore;
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
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Parameter {
    pub id: Uuid,
    pub name: String,
    /// R8: [`Parameter`] 'came before' [`Parameter`]
    pub next: Option<Uuid>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-implementation"}}}
impl Parameter {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-new"}}}
    /// Inter a new Parameter in the store, and return it's `id`.
    pub fn new(
        name: String,
        next: Option<&Parameter>,
        store: &mut OneToOneDomainStore,
    ) -> Parameter {
        let id = Uuid::new_v5(&UUID_NS, format!("{}:{:?}", name, next).as_bytes());
        let new = Parameter {
            name: name,
            next: next.map(|parameter| parameter.id),
            id,
        };
        store.inter_parameter(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-to-next"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`Parameter`] across R8(1-?c)
    pub fn r8_parameter<'a>(&'a self, store: &'a OneToOneDomainStore) -> Vec<&Parameter> {
        match self.next {
            Some(ref next) => vec![store.exhume_parameter(next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-backward-cond-to-parameter"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-backward-one-bi-cond-to-parameter"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"parameter-struct-impl-nav-backward-one-to-parameter"}}}
    /// Navigate to [`Parameter`] across R8(1c-1c)
    pub fn r8c_parameter<'a>(&'a self, store: &'a OneToOneDomainStore) -> Vec<&Parameter> {
        let parameter = store
            .iter_parameter()
            .find(|parameter| parameter.1.next == Some(self.id));
        match parameter {
            Some(ref parameter) => vec![parameter.1],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

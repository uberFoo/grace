// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"referrer-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referrer-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::sarzak_domain::UUID_NS;

// Referrer imports
use crate::sarzak_domain::types::cardinality::Cardinality;
use crate::sarzak_domain::types::conditionality::Conditionality;
use crate::sarzak_domain::types::object::Object;

// Referent imports
use crate::sarzak_domain::types::binary::Binary;

use crate::sarzak_domain::store::ObjectStore as SarzakDomainStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referrer-struct-documentation"}}}
/// This is the side of a binary relationship that is doing the pointing, thus it contains the
/// referential attribute. It is connected to the “from” side of a binary relationship.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referrer-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Referrer {
    pub description: String,
    pub id: Uuid,
    pub referential_attribute: String,
    /// R9: [`Referrer`] 'has' [`Cardinality`]
    pub cardinality: Uuid,
    /// R11: [`Referrer`] 'has' [`Conditionality`]
    pub conditionality: Uuid,
    /// R17: [`Referrer`] 'is an instance of an' [`Object`]
    pub obj_id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referrer-implementation"}}}
impl Referrer {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referrer-struct-impl-new"}}}
    /// Inter a new Referrer in the store, and return it's `id`.
    pub fn new(
        description: String,
        referential_attribute: String,
        cardinality: &Cardinality,
        conditionality: &Conditionality,
        obj_id: &Object,
        store: &mut SarzakDomainStore,
    ) -> Referrer {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!(
                "{}:{}:{:?}:{:?}:{:?}",
                description, referential_attribute, cardinality, conditionality, obj_id
            )
            .as_bytes(),
        );
        let new = Referrer {
            description: description,
            referential_attribute: referential_attribute,
            cardinality: cardinality.id(),
            conditionality: conditionality.id(),
            obj_id: obj_id.id,
            id,
        };
        store.inter_referrer(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referrer-struct-impl-nav-forward-to-cardinality"}}}
    /// Navigate to [`Cardinality`] across R9(1-?)
    pub fn r9_cardinality<'a>(&'a self, store: &'a SarzakDomainStore) -> Vec<&Cardinality> {
        vec![store.exhume_cardinality(&self.cardinality).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referrer-struct-impl-nav-forward-to-conditionality"}}}
    /// Navigate to [`Conditionality`] across R11(1-?)
    pub fn r11_conditionality<'a>(&'a self, store: &'a SarzakDomainStore) -> Vec<&Conditionality> {
        vec![store.exhume_conditionality(&self.conditionality).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referrer-struct-impl-nav-forward-to-obj_id"}}}
    /// Navigate to [`Object`] across R17(1-?)
    pub fn r17_object<'a>(&'a self, store: &'a SarzakDomainStore) -> Vec<&Object> {
        vec![store.exhume_object(&self.obj_id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"referrer-struct-impl-nav-backward-one-to-binary"}}}
    /// Navigate to [`Binary`] across R6(1-1)
    pub fn r6_binary<'a>(&'a self, store: &'a SarzakDomainStore) -> Vec<&Binary> {
        vec![
            store
                .iter_binary()
                .find(|binary| binary.1.from == self.id)
                .unwrap()
                .1,
        ]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

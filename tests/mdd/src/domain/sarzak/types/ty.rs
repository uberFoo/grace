// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"ty-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
use crate::domain::sarzak::store::ObjectStore as SarzakStore;
use crate::domain::sarzak::types::boolean::BOOLEAN;
use crate::domain::sarzak::types::external::External;
use crate::domain::sarzak::types::float::FLOAT;
use crate::domain::sarzak::types::integer::INTEGER;
use crate::domain::sarzak::types::object::Object;
use crate::domain::sarzak::types::string::STRING;
use crate::domain::sarzak::types::uuid::UUID;

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-enum-documentation"}}}
/// The type of a value
///
/// There are several values available: [Integer], [Boolean], [Float], [String], and [UUID]
///.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-enum-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum Ty {
    Boolean(Uuid),
    External(Uuid),
    Float(Uuid),
    Integer(Uuid),
    Object(Uuid),
    String(Uuid),
    Uuid(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-implementation"}}}
impl Ty {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-new-impl"}}}
    /// Create a new instance of Ty::Boolean
    pub fn new_boolean(_store: &mut SarzakStore) -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Boolean(BOOLEAN)
    }

    /// Create a new instance of Ty::External
    pub fn new_external(external: &External, store: &mut SarzakStore) -> Self {
        let new = Self::External(external.id);
        store.inter_ty(new.clone());
        new
    }

    /// Create a new instance of Ty::Float
    pub fn new_float(_store: &mut SarzakStore) -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Float(FLOAT)
    }

    /// Create a new instance of Ty::Integer
    pub fn new_integer(_store: &mut SarzakStore) -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Integer(INTEGER)
    }

    /// Create a new instance of Ty::Object
    pub fn new_object(object: &Object, store: &mut SarzakStore) -> Self {
        let new = Self::Object(object.id);
        store.inter_ty(new.clone());
        new
    }

    /// Create a new instance of Ty::String
    pub fn new_string(_store: &mut SarzakStore) -> Self {
        // This is already in the store, see associated function `new` above.
        Self::String(STRING)
    }

    /// Create a new instance of Ty::Uuid
    pub fn new_uuid(_store: &mut SarzakStore) -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Uuid(UUID)
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Ty::Boolean(id) => *id,
            Ty::External(id) => *id,
            Ty::Float(id) => *id,
            Ty::Integer(id) => *id,
            Ty::Object(id) => *id,
            Ty::String(id) => *id,
            Ty::Uuid(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

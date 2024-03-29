// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"ty-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-use-statements"}}}
use crate::domain::sarzak::store::ObjectStore as SarzakStore;
use crate::domain::sarzak::types::attribute::Attribute;
use crate::domain::sarzak::types::boolean::BOOLEAN;
use crate::domain::sarzak::types::external::External;
use crate::domain::sarzak::types::float::FLOAT;
use crate::domain::sarzak::types::integer::INTEGER;
use crate::domain::sarzak::types::object::Object;
use crate::domain::sarzak::types::s_string::S_STRING;
use crate::domain::sarzak::types::s_uuid::S_UUID;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-enum-documentation"}}}
/// The type of a value
///
/// There are several values available: [Integer], [Boolean], [Float], [String], and [UUID]
/// .
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-enum-definition"}}}
#[derive(Copy, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Ty {
    Boolean(Uuid),
    External(Uuid),
    Float(Uuid),
    Integer(Uuid),
    Object(Uuid),
    SString(Uuid),
    SUuid(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-implementation"}}}
impl Ty {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-new-impl"}}}
    /// Create a new instance of Ty::Boolean
    pub fn new_boolean() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Boolean(BOOLEAN)
    }

    /// Create a new instance of Ty::External
    pub fn new_external(external: &External, store: &mut SarzakStore) -> Self {
        let new = Self::External(external.id);
        store.inter_ty(new.clone());
        new
    } // wtf?

    /// Create a new instance of Ty::Float
    pub fn new_float() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Float(FLOAT)
    }

    /// Create a new instance of Ty::Integer
    pub fn new_integer() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::Integer(INTEGER)
    }

    /// Create a new instance of Ty::Object
    pub fn new_object(object: &Object, store: &mut SarzakStore) -> Self {
        let new = Self::Object(object.id);
        store.inter_ty(new.clone());
        new
    } // wtf?

    /// Create a new instance of Ty::SString
    pub fn new_s_string() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::SString(S_STRING)
    }

    /// Create a new instance of Ty::SUuid
    pub fn new_s_uuid() -> Self {
        // This is already in the store, see associated function `new` above.
        Self::SUuid(S_UUID)
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-get-id-impl"}}}
    pub fn id(&self) -> Uuid {
        match self {
            Self::Boolean(id) => *id,
            Self::External(id) => *id,
            Self::Float(id) => *id,
            Self::Integer(id) => *id,
            Self::Object(id) => *id,
            Self::SString(id) => *id,
            Self::SUuid(id) => *id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"ty-struct-impl-nav-backward-one-to-attribute"}}}
    /// Navigate to [`Attribute`] across R2(1-1)
    pub fn r2_attribute<'a>(&'a self, store: &'a SarzakStore) -> Vec<&Attribute> {
        vec![store
            .iter_attribute()
            .find(|attribute| attribute.ty == self.id())
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

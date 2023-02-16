// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"external-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external-use-statements"}}}
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::sarzak::UUID_NS;

use crate::sarzak::store::ObjectStore as SarzakStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external-struct-documentation"}}}
/// External Type
///
/// This may literally be anything. It's used during code generation to generate variables names
/// and type names for things that are outside of a modeled domain. For example, a timer would
/// be an external type. The specifics of how it is used is up to the model compiler.
///
/// In grace, the `name` attribute is used during code generation to create variable names by
/// converting it to `snake_case`. When used as a type, it is converted to `UpperCamelCase`
///.
///
/// We use `path` as the path is a `use` statement.
///
/// I'm updating this while trying to use it, so this description is going to be rather incoherent
/// until things settle down.
///
/// The way I'm using this, and hopefully the way that will always accommodate, is as a singleton
/// within a particular function scope. Maybe it's a system-wide singleton? I dunno. But it's
/// a singleton.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external-struct-definition"}}}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct External {
    pub id: Uuid,
    pub name: String,
    pub path: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external-implementation"}}}
impl External {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"external-struct-impl-new"}}}
    /// Inter a new External in the store, and return it's `id`.
    pub fn new(name: String, path: String, store: &mut SarzakStore) -> External {
        let id = Uuid::new_v5(&UUID_NS, format!("{}:{}", name, path).as_bytes());
        let new = External {
            name: name,
            path: path,
            id,
        };
        store.inter_external(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"uuid-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"uuid-use-statements"}}}
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"uuid-const-documentation"}}}
/// The UUID Type
///
/// I feel like there are too many implementation details here.
///
/// This UUID is expected to be version 5. Generally we produce input
/// to the hash function from other UUIDs, coupled with additional
/// information from the creator to ensure a unique UUID.
///
/// The `ns` attribute is the namespace used to generate generate UUIDs
/// given a particular instance of `UUID`.
///
/// ❗️{"singleton_object": true, "translation_name": "SarzakUuid"}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"uuid-const-definition"}}}
pub const UUID: Uuid = uuid!["8d63fc44-28ea-599b-8654-877b133f0640"];
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

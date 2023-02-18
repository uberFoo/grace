// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"one-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"one-use-statements"}}}
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"one-const-documentation"}}}
/// A constant value that indicates a cardinality of _one_.
///
/// ❗️{"singleton_object": true}
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"one-const-definition"}}}
pub const ONE: Uuid = uuid!["84d60cb6-04cf-5c82-9e38-79d3999b5d5c"];
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

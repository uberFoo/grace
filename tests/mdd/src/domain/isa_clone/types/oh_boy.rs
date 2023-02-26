// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"oh_boy-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"oh_boy-use-statements"}}}
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"oh_boy-const-documentation"}}}
/// This should break all sorts of shit.
///
/// The only purpose of this is to make [`SimpleSubtypeA`] a supertype. Then I got crafty with
/// the name. One of these days I'm going to throw an emoji in there...😝
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"oh_boy-const-definition"}}}
pub const OH_BOY: Uuid = uuid!["c009341f-3100-599c-8751-16e317ec87dc"];
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"no-obj-here-struct-definition-file"}}}
use uuid::Uuid;

// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"rando_object-struct-documentation"}}}
/// Just some random object with which we wish to relate
///
/// How tawdry.
// {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-struct-definition"}}}
#[derive(Debug)]
pub struct RandoObject {
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

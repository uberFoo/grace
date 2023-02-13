// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"no-obj-here-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"rando_object-struct-definition-file"}}}
use crate::everything::UUID_NS;
use uuid::Uuid;

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-referrer-use-statements"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"rando_object-struct-documentation"}}}
/// Just some random object with which we wish to relate
///
/// How tawdry.
// {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-struct-definition"}}}
#[derive(Debug, PartialEq)]
pub struct RandoObject {
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"rando_object-struct-implementation"}}}
impl RandoObject {
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"rando_object-struct-impl-new"}}}
    /// Inter a new RandoObject in the store, and return it's `id`.
    pub fn new() -> RandoObject {
        //         let id = Uuid::new_v5(&UUID_NS, format!("",).as_bytes());
        //         let new = RandoObject { id };
        //         let id = Uuid::new_v5(&UUID_NS, format!("{:?}", rando).as_bytes());
        //         let new = RandoObject { rando: rando, id };
        let id = Uuid::new_v5(&UUID_NS, format!("",).as_bytes());
        let new = RandoObject { id };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

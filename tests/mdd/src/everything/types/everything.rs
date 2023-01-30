// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"no-obj-here-struct-definition-file"}}}
use uuid::Uuid;

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-referrer-use-statements"}}}
use crate::everything::types::rando_object::RandoObject;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"everything-struct-documentation"}}}
/// An object, with everything on it!
// {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-struct-definition"}}}
#[derive(Debug)]
pub struct Everything<'a> {
    pub bool: bool,
    pub float: f64,
    pub id: Uuid,
    pub int: i64,
    pub string: String,
    /// R1: points at
    pub rando: &'a RandoObject,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}

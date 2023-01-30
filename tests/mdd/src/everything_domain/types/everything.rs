// {"magic":"","directive":{"Start":{"directive":"ignore-gen","tag":"no-obj-here-struct-definition-file"}}}
use uuid::Uuid;

// {"magic":"","directive":{"Start":{"directive":"ignore-gen","tag":"everything-struct-definition"}}}
#[derive(Debug)]
pub struct Everything {
    pub bool: bool,
    pub float: f64,
    pub id: Uuid,
    pub int: i64,
    pub string: String,
    // {"magic":"","directive":{"Start":{"directive":"ignore-gen","tag":"everything-referrer-use-statements"}}}
    /// R1: points at
    pub rando: Uuid,
    // {"magic":"","directive":{"End":{"directive":"ignore-gen"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-gen"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-gen"}}}

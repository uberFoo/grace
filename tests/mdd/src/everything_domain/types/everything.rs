// {"magic":"","directive":"provenance","tag":"no-obj-here-struct-definition-file"}
use uuid::Uuid;

// {"magic":"","directive":"prefer-new","tag":"everything-referrer-use-statements"}
use crate::everything_domain::types::rando_object::RandoObject;
// {"magic":"","directive":"prefer-new","tag":"everything-referrer-use-statements"}
// {"magic":"","directive":"prefer-new","tag":"everything-struct-definition"}
#[derive(Debug)]
pub struct Everything {
    pub bool: bool,
    pub float: f64,
    pub id: Uuid,
    pub int: i64,
    pub string: String,
    /// R1: points at
    pub rando: Uuid,
}
// {"magic":"","directive":"prefer-new","tag":"everything-struct-definition"}
// {"magic":"","directive":"provenance","tag":"no-obj-here-struct-definition-file"}

// {"magic":"","directive":"provenance","tag":"something better than this"}
use uuid::Uuid;

// {"magic":"","directive":"prefer-new","tag":"everything-referrer-use-statements"}
use crate::everything::types::rando_object::RandoObject;
// {"magic":"","directive":"prefer-new","tag":"everything-referrer-use-statements"}
// {"magic":"","directive":"prefer-new","tag":"everything-struct-definition"}
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
// {"magic":"","directive":"prefer-new","tag":"everything-struct-definition"}
// {"magic":"","directive":"provenance","tag":"something better than this"}

// ✨
use uuid::Uuid;

// ✨
use crate::everything::types::rando_object::RandoObject;
// ✨
// ✨
pub struct Everything<'a> {
pub bool: bool,
pub float: f64,
pub id: Uuid,
pub int: i64,
pub string: String,
/// R1: points at
pub rando: &'a RandoObject
}
// ✨
// ✨

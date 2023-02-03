//! These are things that need to be modeled
//!
//! Not that the whole things shouldn't be, but this stuff is low hanging fruit.
//!

use sarzak::{sarzak::types::Type, woog::types::Parameter};
use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct LValue {
    /// The variable name
    ///
    /// It's assumed that it's already been sanitized.
    pub name: String,
    pub ty: Uuid,
}

impl LValue {
    pub(crate) fn new<S: AsRef<str>>(name: S, ty: &Type) -> Self {
        Self {
            name: name.as_ref().to_string(),
            ty: ty.get_id(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct RValue {
    pub name: String,
    pub ty: Uuid,
}

impl RValue {
    pub(crate) fn new(name: String, ty: &Type) -> Self {
        Self {
            name,
            ty: ty.get_id(),
        }
    }
}

impl From<Parameter> for RValue {
    fn from(value: Parameter) -> Self {
        Self {
            name: value.name,
            ty: value.ty,
        }
    }
}

impl From<&Parameter> for RValue {
    fn from(value: &Parameter) -> Self {
        Self {
            name: value.name.clone(),
            ty: value.ty,
        }
    }
}

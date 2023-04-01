//! These are things that need to be modeled
//!
//! Not that the whole things shouldn't be, but this stuff is low hanging fruit.
//!

use sarzak::sarzak::types::{External as SarzakExternal, Ty};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum GType {
    Integer,
    Boolean,
    Float,
    String,
    Uuid,
    Object(Uuid),
    Reference(Uuid),
    Option(Box<GType>),
    External(External),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct External {
    pub name: String,
    pub path: String,
    pub lvalue: Option<Box<LValue>>,
}

impl From<&SarzakExternal> for External {
    fn from(value: &SarzakExternal) -> Self {
        Self {
            name: value.name.clone(),
            path: value.path.clone(),
            lvalue: None,
        }
    }
}

// impl From<External> for SarzakExternal {
//     fn from(value: External) -> Self {
//         Self {
//             id: Uuid::new_v4(),
//             name: value.name,
//             path: value.path,
//         }
//     }
// }

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LValue {
    /// The variable name
    ///
    /// It's assumed that it's already been sanitized. Really?
    pub name: String,
    pub ty: GType,
    pub hack: Option<GType>,
}

impl LValue {
    pub(crate) fn new<S: AsRef<str>>(name: S, ty: GType, hack: Option<GType>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            ty,
            hack,
        }
    }
}

#[derive(Debug)]
pub(crate) struct RValue {
    pub name: String,
    pub ty: GType,
}

impl RValue {
    pub(crate) fn new<S: AsRef<str>>(name: S, ty: GType) -> Self {
        Self {
            name: name.as_ref().to_string(),
            ty,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Parameter<'a> {
    pub mutability: Uuid,
    pub next: Option<&'a Parameter<'a>>,
    pub ty: GType,
    pub _visibility: Uuid,
    pub name: String,
}

impl<'a> Parameter<'a> {
    pub(crate) fn new<S: AsRef<str>>(
        mutability: Uuid,
        next: Option<&'a Parameter<'a>>,
        ty: GType,
        _visibility: Uuid,
        name: S,
    ) -> Self {
        Self {
            mutability,
            next,
            ty,
            _visibility,
            name: name.as_ref().to_string(),
        }
    }
}

pub(crate) struct ObjectMethod<'a> {
    pub param: Option<&'a Parameter<'a>>,
    pub _object: Uuid,
    pub ty: GType,
    pub _visibility: Uuid,
    pub name: String,
    pub _description: String,
}

impl<'a> ObjectMethod<'a> {
    pub(crate) fn new<S: AsRef<str>>(
        param: Option<&'a Parameter>,
        _object: Uuid,
        ty: GType,
        _visibility: Uuid,
        name: S,
        description: S,
    ) -> Self {
        Self {
            param,
            _object,
            ty,
            _visibility,
            name: name.as_ref().to_string(),
            _description: description.as_ref().to_string(),
        }
    }
}

impl<'a> From<&'a Parameter<'a>> for RValue {
    fn from(value: &'a Parameter) -> Self {
        Self {
            name: value.name.clone(),
            ty: value.ty.clone(),
        }
    }
}

impl From<&Ty> for GType {
    fn from(value: &Ty) -> Self {
        match value {
            Ty::Integer(_) => Self::Integer,
            Ty::Boolean(_) => Self::Boolean,
            Ty::Float(_) => Self::Float,
            Ty::String(_) => Self::String,
            Ty::Uuid(_) => Self::Uuid,
            Ty::Object(uuid) => Self::Object(uuid.clone()),
            _ => unimplemented!(),
        }
    }
}

// pub(crate) struct Statement {
//     pub(crate) lvalue: LValue,
//     pub(crate) rvalue: RValue,
// }

// impl Statement {
//     pub(crate) fn new(lvalue: LValue, rvalue: RValue) -> Self {
//         Self { lvalue, rvalue }
//     }
// }

// pub(crate) struct ExternalInvocation {
//     pub is_initialized: bool,
//     pub init_func: String,
//     pub external: Uuid,
// }

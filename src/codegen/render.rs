use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use sarzak::{
    sarzak::{
        store::ObjectStore as SarzakStore,
        types::{Attribute, Event, External, Object, State, Type},
    },
    woog::types::{Mutability, Parameter},
};

use crate::todo::{External as TodoExternal, GType, ObjectMethod};

macro_rules! render_ident {
    ($($t:ident),+) => {
        $(
            impl RenderIdent for $t {
                fn as_ident(&self) -> String {
                    self.name.to_snake_case()
                }
            }
        )+
    };
}

macro_rules! render_const {
    ($($t:ident),+) => {
        $(
            impl RenderConst for $t {
                fn as_const(&self) -> String {
                    self.name.to_shouty_snake_case()
                }
            }
        )+
    };
}

macro_rules! render_type {
    ($($t:ident),+) => {
        $(
            impl RenderType for $t {
                fn as_type(&self, mutability: &Mutability, store: &SarzakStore) -> String {
                    self.name.as_type(mutability, store)
                }
            }
        )+
    };
}

/// Trait for rendering type as an identifier
///
/// This trait represents the sanitization of an unknown string, into one
/// suitable for being an identifier in Rust. For example, this trait would
/// render  "RenderIdent" as `render_ident`, and "Rando Object" as `rando_object`.
pub(crate) trait RenderIdent {
    fn as_ident(&self) -> String;
}

render_ident!(Attribute, Event, Object, State, Parameter);

impl RenderIdent for ObjectMethod<'_> {
    fn as_ident(&self) -> String {
        self.name.to_snake_case()
    }
}

impl RenderIdent for String {
    fn as_ident(&self) -> String {
        self.to_snake_case()
    }
}

impl RenderIdent for &str {
    fn as_ident(&self) -> String {
        self.to_snake_case()
    }
}

/// Trait for rendering type as a Type
///
/// This trait represents the sanitization of an unknown string, into one
/// suitable for being a type name in Rust. For example, this trait would
/// render  "Rando Object" as `RandoObject`.
///
/// It takes a reference to the store so that Type (see below) works. I've got
/// [a possible workaround](https://git.uberfoo.com/sarzak/sarzak/-/issues/8).
pub(crate) trait RenderType {
    fn as_type(&self, mutability: &Mutability, store: &SarzakStore) -> String;
}

render_type!(Attribute, Event, Object, State, External, TodoExternal);

impl RenderType for String {
    fn as_type(&self, mutability: &Mutability, _store: &SarzakStore) -> String {
        match mutability {
            Mutability::Mutable(_) => format!("mut {}", self.to_upper_camel_case()),
            _ => self.to_upper_camel_case(),
        }
    }
}

impl RenderType for &str {
    fn as_type(&self, mutability: &Mutability, _store: &SarzakStore) -> String {
        match mutability {
            Mutability::Mutable(_) => format!("mut {}", self.to_upper_camel_case()),
            _ => self.to_upper_camel_case(),
        }
    }
}

/// RenderType implementation for Type
///
/// How recursive...
///
/// Eventually we'll need to expand the model to include size options for
/// sized types. Probably need more types too. we'll just have to see.
///
/// One thing that worries me is what happens when we get to references?
impl RenderType for Type {
    fn as_type(&self, mutability: &Mutability, store: &SarzakStore) -> String {
        match self {
            Type::Boolean(_) => "bool".to_owned(),
            Type::Object(o) => {
                let object = store.exhume_object(&o).unwrap();
                format!("{}", object.as_type(&mutability, &store))
            }
            Type::String(_) => "String".to_owned(),
            Type::Uuid(_) => "Uuid".to_owned(),
            Type::External(e) => {
                let ext = store.exhume_external(&e).unwrap();
                format!("&{}", ext.as_type(&mutability, &store))
            }
            Type::Float(_) => "f64".to_owned(),
            Type::Integer(_) => "i64".to_owned(),
        }
    }
}

/// RenderType implementation for GType
///
/// How recursive...
///
/// Eventually we'll need to expand the model to include size options for
/// sized types. Probably need more types too. we'll just have to see.
///
/// One thing that worries me is what happens when we get to references?
impl RenderType for GType {
    fn as_type(&self, mutability: &Mutability, store: &SarzakStore) -> String {
        match self {
            GType::Boolean => "bool".to_owned(),
            GType::Object(o) => {
                let object = store.exhume_object(&o).unwrap();
                format!("{}", object.as_type(&mutability, &store))
            }
            GType::Reference(r) => {
                let object = store.exhume_object(&r).unwrap();
                format!("&{}", object.as_type(&mutability, &store))
            }
            GType::Option(o) => {
                format!("Option<{}>", o.as_type(&mutability, &store))
            }
            GType::External(e) => {
                format!("&{}", e.as_type(&mutability, &store))
            }
            GType::String => "String".to_owned(),
            GType::Uuid => "Uuid".to_owned(),
            GType::Float => "f64".to_owned(),
            GType::Integer => "i64".to_owned(),
        }
    }
}

/// Trait for rendering type as a constant
///
/// This trait represents the sanitization of an unknown string, into one
/// suitable for being a constant identifier in Rust. For example, this trait would
/// render  "RenderIdent" as `RENDER_IDENT`, and "Rando Object" as `RANDO_OBJECT`.

pub(crate) trait RenderConst {
    fn as_const(&self) -> String;
}

render_const!(Object);

impl RenderConst for String {
    fn as_const(&self) -> String {
        self.to_shouty_snake_case()
    }
}

impl RenderConst for &str {
    fn as_const(&self) -> String {
        self.to_shouty_snake_case()
    }
}

use heck::{ToSnakeCase, ToUpperCamelCase};
use sarzak::sarzak::types::{Attribute, Event, Object, State, Type};

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

macro_rules! render_type {
    ($($t:ident),+) => {
        $(
            impl RenderType for $t {
                fn as_type(&self) -> String {
                    self.name.to_upper_camel_case()
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

render_ident!(Attribute, Event, Object, State);

/// Trait for rendering type as a Type
///
/// This trait represents the sanitization of an unknown string, into one
/// suitable for being a type name in Rust. For example, this trait would
/// render  "Rando Object" as `RandoObject`.
pub(crate) trait RenderType {
    fn as_type(&self) -> String;
}

render_type!(Attribute, Event, Object, State);

/// RenderType implementation for Type
///
/// How recursive...
///
/// Eventually we'll need to expand the model to include size options for
/// sized types. Probably need more types too. we'll just have to see.
///
/// One thing that worries me is what happens when we get to references?
impl RenderType for Type {
    fn as_type(&self) -> String {
        match self {
            Type::Boolean(_) => "bool".to_owned(),
            Type::String(_) => "String".to_owned(),
            Type::Uuid(_) => "Uuid".to_owned(),
            Type::Float(_) => "f64".to_owned(),
            Type::Integer(_) => "i64".to_owned(),
        }
    }
}

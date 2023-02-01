use heck::{ToSnakeCase, ToUpperCamelCase};
use sarzak::sarzak::{
    store::ObjectStore as SarzakStore,
    types::{Attribute, Event, Object, State, Type},
};

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
                fn as_type(&self, _store: &SarzakStore) -> String {
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
///
/// It takes a reference to the store so that Type (see below) works. I've got
/// [a possible workaround](https://git.uberfoo.com/sarzak/sarzak/-/issues/8).
pub(crate) trait RenderType {
    fn as_type(&self, store: &SarzakStore) -> String;
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
    fn as_type(&self, store: &SarzakStore) -> String {
        match self {
            Type::Boolean(_) => "bool".to_owned(),
            // I don't have a good feeling about this one...
            Type::Reference(r) => {
                let reference = store.exhume_reference(&r).unwrap();
                let object = store.exhume_object(&reference.object).unwrap();
                format!("&{}", object.as_type(&store))
            }
            Type::String(_) => "String".to_owned(),
            Type::Uuid(_) => "Uuid".to_owned(),
            Type::Float(_) => "f64".to_owned(),
            Type::Integer(_) => "i64".to_owned(),
        }
    }
}

impl RenderIdent for String {
    fn as_ident(&self) -> String {
        self.to_snake_case()
    }
}

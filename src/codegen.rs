//! Things necessary for code generation
//!

use heck::{ToSnakeCase, ToUpperCamelCase};
use sarzak::sarzak::types::Object;

pub(crate) trait RenderIdent {
    fn as_ident(&self) -> String;
}

impl RenderIdent for Object {
    fn as_ident(&self) -> String {
        self.name.to_snake_case()
    }
}

pub(crate) trait RenderType {
    fn as_type(&self) -> String;
}

impl RenderType for Object {
    fn as_type(&self) -> String {
        self.name.to_upper_camel_case()
    }
}

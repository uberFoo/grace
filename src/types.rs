//! Generate Types
//!
//! This is the entry point for all type generation.
use std::fmt::Write;

use log;
use sarzak::{
    mc::{CompilerSnafu, Result},
    sarzak::Object,
};
use snafu::prelude::*;

use crate::{
    buffer::{Buffer, CodeWriter, TypeWriter},
    codegen::RenderType,
};

pub(crate) struct TypeBuilder<'a> {
    object: &'a Object,
    struct_definition: Option<Box<dyn StructDefinition + 'a>>,
}

impl<'a> TypeBuilder<'a> {
    pub fn new(object: &'a Object) -> Self {
        Self {
            object: object,
            struct_definition: None,
        }
    }

    pub fn using_struct_defn(mut self, builder: DefaultStructBuilder<'a>) -> Result<Self> {
        self.struct_definition = Some(builder.object(self.object).build()?);

        Ok(self)
    }

    pub fn build(self) -> Result<Box<Type<'a>>> {
        ensure!(
            self.struct_definition.is_some(),
            CompilerSnafu {
                description: "missing StructDefinition writer"
            }
        );
        Ok(Box::new(Type {
            object: self.object,
            struct_definition: self.struct_definition.unwrap(),
        }))
    }
}

pub(crate) trait TypeDefinition: CodeWriter {}
pub(crate) trait StructDefinition: CodeWriter {}

pub(crate) struct Type<'a> {
    object: &'a Object,
    struct_definition: Box<dyn StructDefinition + 'a>,
}

impl<'a> TypeWriter for Type<'a> {}

impl<'a> CodeWriter for Type<'a> {
    fn write_code(&self, buffer: &mut Buffer) {
        self.struct_definition.write_code(buffer);
    }
}

pub(crate) struct DefaultStructBuilder<'a> {
    object: Option<&'a Object>,
}

impl<'a> DefaultStructBuilder<'a> {
    pub(crate) fn new() -> Self {
        DefaultStructBuilder { object: None }
    }

    pub(crate) fn object(mut self, object: &'a Object) -> Self {
        self.object = Some(object);

        self
    }

    pub(crate) fn build(self) -> Result<Box<DefaultStruct<'a>>> {
        ensure!(
            self.object.is_some(),
            CompilerSnafu {
                description: "missing object"
            }
        );

        Ok(Box::new(DefaultStruct::new(self.object.unwrap())))
    }
}

pub(crate) struct DefaultStruct<'a> {
    object: &'a Object,
}

impl<'a> StructDefinition for DefaultStruct<'a> {}

impl<'a> DefaultStruct<'a> {
    fn new(object: &'a Object) -> Self {
        Self { object: object }
    }
}

impl<'a> CodeWriter for DefaultStruct<'a> {
    fn write_code(&self, buffer: &mut Buffer) {
        log::debug!("writing Struct Definition for {}", self.object.name);
        // We need a builder for this so that we can add privacy modifiers, as
        // well as derives, and attributes
        write!(buffer, "pub struct {} {{", self.object.as_type());
        write!(buffer, "}}");
    }
}

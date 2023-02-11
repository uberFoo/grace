//! Render Traits
//!
//! And implementations. This needs some housecleaning.
//!
use std::collections::HashSet;

use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use names::Generator;
use sarzak::{
    sarzak::{
        store::ObjectStore as SarzakStore,
        types::{Attribute, Event, External, Object, Reference, State, Type},
    },
    woog::{
        macros::woog_maybe_get_one_param_across_r5,
        store::ObjectStore as WoogStore,
        types::{Mutability, ObjectMethod, Parameter, BORROWED},
    },
};

use crate::todo::{External as TodoExternal, GType, LValue, ObjectMethod, RValue, Statement};

macro_rules! render_ident {
    ($($t:ident),+) => {
        $(
            impl RenderIdent for $t {
                fn as_ident(&self) -> String {
                    self.name.sanitize().to_snake_case()
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
                    self.name.sanitize().to_shouty_snake_case()
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
                    self.name.sanitize().as_type(mutability, store)
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
        self.name.sanitize().to_snake_case()
    }
}

impl RenderIdent for String {
    fn as_ident(&self) -> String {
        self.sanitize().to_snake_case()
    }
}

impl RenderIdent for &str {
    fn as_ident(&self) -> String {
        self.sanitize().to_snake_case()
    }
}
pub(crate) trait RenderRval {
    fn as_rval(&self) -> String;
}

impl RenderRval for String {
    fn as_rval(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl RenderRval for i64 {
    fn as_rval(&self) -> String {
        format!("{}", self)
    }
}

impl RenderRval for f64 {
    fn as_rval(&self) -> String {
        format!("{}", self)
    }
}

impl RenderRval for bool {
    fn as_rval(&self) -> String {
        format!("{}", self)
    }
}

pub(crate) trait RenderStatement {
    fn as_statement(
        &self,
        package: &str,
        module: &str,
        woog: &WoogStore,
        sarzak: &SarzakStore,
        uses: &mut HashSet<String>,
    ) -> Vec<Statement>;
}

impl RenderStatement for Type {
    fn as_statement(
        &self,
        package: &str,
        module: &str,
        woog: &WoogStore,
        sarzak: &SarzakStore,
        uses: &mut HashSet<String>,
    ) -> Vec<Statement> {
        match self {
            Type::Boolean(_) => {
                let var = Generator::default().next().unwrap().to_snake_case();
                let stmt = Statement::new(
                    LValue::new(var.clone(), &self),
                    RValue::new("true".to_owned(), &self),
                );
                vec![stmt]
            }
            Type::Object(o) => {
                let object = sarzak.exhume_object(&o).unwrap();
                let mut iter = woog.iter_object_method();
                let method = loop {
                    match iter.next() {
                        Some((_, method)) => {
                            if method.object == object.id && method.name == "new" {
                                break method;
                            }
                        }
                        None => {
                            panic!("Unable to find the new method for {}", object.name);
                        }
                    }
                };

                uses.insert(format!(
                    "use mdd::{}::types::{};",
                    module,
                    object.as_type(&Mutability::Borrowed(BORROWED), sarzak)
                ));

                // Recurse into the method
                method.as_statement(package, module, woog, sarzak, uses)
            }
            Type::Reference(r) => {
                // If the type is a reference, we want to create a new object?
                let reference = sarzak.exhume_reference(&r).unwrap();
                let object = sarzak.exhume_object(&reference.object).unwrap();
                let mut iter = woog.iter_object_method();
                let method = loop {
                    match iter.next() {
                        Some((_, method)) => {
                            if method.object == object.id && method.name == "new" {
                                break method;
                            }
                        }
                        None => {
                            panic!("Unable to find the new method for {}", object.name);
                        }
                    }
                };

                uses.insert(format!(
                    "use mdd::{}::types::{};",
                    module,
                    object.as_type(&Mutability::Borrowed(BORROWED), sarzak)
                ));

                // Recurse into the method
                method.as_statement(package, module, woog, sarzak, uses)
            }
            Type::String(_) => {
                let var = Generator::default().next().unwrap().to_snake_case();
                let stmt = Statement::new(
                    LValue::new(var.clone(), &self),
                    RValue::new(Generator::default().next().unwrap(), &self),
                );

                vec![stmt]
            }
            Type::Uuid(_) => {
                let var = Generator::default().next().unwrap().to_snake_case();
                let stmt = Statement::new(
                    LValue::new(var.clone(), &self),
                    RValue::new(
                        format!(
                            "Uuid::new_v5(&UUID_NS, \"{}\")",
                            Generator::default().next().unwrap().to_snake_case()
                        ),
                        &self,
                    ),
                );
                uses.insert("use uuid::Uuid;".to_owned());
                vec![stmt]
            }
            Type::External(e) => {
                let ext = sarzak.exhume_external(&e).unwrap();
                let var = if let Some(lvalue) = ext.lvalue {
                    lvalue
                } else {
                    let var = Generator::default().next().unwrap().to_snake_case();
                    uses.insert(format!(
                        "use {}::{} as {};",
                        package,
                        ext.path,
                        ext.as_type(&Mutability::Borrowed(BORROWED), &sarzak)
                    ));
                    ext.lvalue = Some(var.clone());
                    var
                };
                let stmt = Statement::new(
                    LValue::new(var, &self),
                    RValue::new(
                        format!(
                            "{}::{};",
                            ext.as_type(&Mutability::Borrowed(BORROWED), &sarzak),
                            ext.initialization,
                        ),
                        &self,
                    ),
                );
                vec![stmt]
            }
            Type::Float(_) => {
                let var = Generator::default().next().unwrap().to_snake_case();
                let stmt = Statement::new(
                    LValue::new(var.clone(), &self),
                    RValue::new("42.0".to_owned(), &self),
                );
                vec![stmt]
            }
            Type::Integer(_) => {
                let var = Generator::default().next().unwrap().to_snake_case();
                let stmt = Statement::new(
                    LValue::new(var.clone(), &self),
                    RValue::new("42".to_owned(), &self),
                );
                vec![stmt]
            }
        }
    }
}

/// Render a Parameter as an Rval
///
/// This function is recursive.
impl RenderStatement for Parameter {
    fn as_statement(
        &self,
        package: &str,
        module: &str,
        woog: &WoogStore,
        sarzak: &SarzakStore,
        uses: &mut HashSet<String>,
    ) -> Vec<Statement> {
        log::trace!("{}:{} as rval, next: {:?}", self.name, self.id, self.next);
        let mut statements = Vec::new();

        // Get an instance of our type
        let ty = sarzak.exhume_ty(&self.ty).unwrap();
        let stmt = ty.as_statement(package, module, woog, sarzak, uses);
        statements.push(stmt);

        match self.next {
            Some(p) => {
                let param = woog.exhume_parameter(&p).unwrap();
                log::trace!("invoking next: {}:{}", param.name, param.id);
                // Recurse
                let stmt = param.as_statement(package, module, woog, sarzak, uses);
                statements.push(stmt);
            }
            _ => {}
        };

        statements.into_iter().flatten().collect()
    }
}

impl RenderStatement for ObjectMethod {
    fn as_statement(
        &self,
        package: &str,
        module: &str,
        woog: &WoogStore,
        sarzak: &SarzakStore,
        uses: &mut HashSet<String>,
    ) -> Vec<Statement> {
        log::trace!("{}:{} as rval", self.name, self.id);
        let mut use_statements = String::new();
        let mut statements = Vec::new();

        let obj = sarzak.exhume_object(&self.object).unwrap();
        let mut param = woog_maybe_get_one_param_across_r5!(self, woog);
        match param {
            Some(p) => {
                // Recurse
                let stmt = p.as_statement(package, module, woog, sarzak, uses);
                statements.push(stmt);
            }
            _ => {}
        }

        // Add the method call
        let var = Generator::default().next().unwrap().to_snake_case();
        // I let copilot write the following code. It's what I had after the
        // for loop, before it was here. I don't love the forced return, or the
        // panic at the end (which I added), but it does work.
        for (id, reference) in sarzak.iter_reference() {
            if reference.object == self.object {
                let ty = Type::Reference(*id);
                statements.push(vec![Statement::new(
                    LValue::new(var.clone(), &ty),
                    RValue::new(
                        format!(
                            "{}::{}()",
                            obj.as_type(&Mutability::Borrowed(BORROWED), sarzak),
                            self.name.as_ident(),
                        ),
                        &ty,
                    ),
                )]);
                return statements.into_iter().flatten().collect();
            }
        }
        panic!("Could not find reference for object method: {}", self.id);
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
            Mutability::Mutable(_) => format!("mut {}", self.sanitize().to_upper_camel_case()),
            _ => self.sanitize().to_upper_camel_case(),
        }
    }
}

impl RenderType for &str {
    fn as_type(&self, mutability: &Mutability, _store: &SarzakStore) -> String {
        match mutability {
            Mutability::Mutable(_) => format!("mut {}", self.sanitize().to_upper_camel_case()),
            _ => self.sanitize().to_upper_camel_case(),
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
        self.sanitize().to_shouty_snake_case()
    }
}

impl RenderConst for &str {
    fn as_const(&self) -> String {
        self.sanitize().to_shouty_snake_case()
    }
}

trait Sanitize {
    fn sanitize(&self) -> String;
}

impl Sanitize for &str {
    fn sanitize(&self) -> String {
        match *self {
            "type" => "ty".to_owned(),
            "Type" => "ty".to_owned(),
            "crate" => "krate".to_owned(),
            "Crate" => "krate".to_owned(),
            "ref" => "x_ref".to_owned(),
            _ => self.to_string(),
        }
    }
}

impl Sanitize for String {
    fn sanitize(&self) -> String {
        match self.as_str() {
            "type" => "ty".to_owned(),
            "Type" => "ty".to_owned(),
            "crate" => "krate".to_owned(),
            "Crate" => "krate".to_owned(),
            "ref" => "x_ref".to_owned(),
            _ => self.to_owned(),
        }
    }
}

/// Render a function call
///
/// Given an [`OBjectMethod`], emit code to call it. This means that we need to
/// create rvals for the parameters, and then call the method. Creating the
/// rvals is undoubtedly involve calling other methods, etc. Therefore, it's
/// expected that this function will be recursive. Unless rust gets in the way,
/// and then we'll do the stack thing.
///
/// We need a way to render rvals that's generic. Something that may ultimately
/// call this function. I'm thinking a trait. I can throw it in render. Actually,
/// this should be in render...
///
/// I'm adding this in render, and looking at the traits in there that might
/// call me, and I'm noticing that they all return `String`. I was expecting
/// that I'd be returning a `Result<()>`, and taking a [`Buffer`]. Hell, maybe
/// that's not the right thing for this. I only need a Buffer if I'm going to
/// wrap my output in guards. Given the recursive nature of this function, I
/// think guards would be overkill. Not only that, I think that if I tried to
/// use a buffer, I'd run into issues with the borrow checker.
///
/// Oooh! We'll need to track use statements as well.
pub(crate) fn render_function_call() {}

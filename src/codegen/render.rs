//! Render Traits
//!
//! And implementations. This needs some housecleaning.
//!
use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
// use names::Generator;
use sarzak::{
    sarzak::types::{Attribute, Event, External as SarzakExternal, Object, State, Type},
    v1::domain::Domain,
    woog::types::Mutability,
};

use crate::todo::{External, GType, ObjectMethod, Parameter};

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
                fn as_type(&self, mutability: &Mutability, store: &Domain) -> String {
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

render_ident!(Attribute, Event, Object, State);

impl<'a> RenderIdent for ObjectMethod<'a> {
    fn as_ident(&self) -> String {
        self.name.sanitize().to_snake_case()
    }
}

impl<'a> RenderIdent for Parameter<'a> {
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

// 🚧 Put this back in once I'm done moving to v2.
/*/
pub(crate) trait RenderStatement {
    fn as_statement(
        &self,
        package: &str,
        module: &str,
        woog: &WoogStore,
        domain: &Domain,
        uses: &mut HashSet<String>,
    ) -> Vec<Statement>;
}

impl RenderStatement for GType {
    fn as_statement(
        &self,
        package: &str,
        module: &str,
        woog: &WoogStore,
        domain: &Domain,
        uses: &mut HashSet<String>,
    ) -> Vec<Statement> {
        match self {
            Self::Option(o) => {
                let var = Generator::default().next().unwrap().to_snake_case();
                let stmt = Statement::new(
                    LValue::new(var.clone(), self.clone()),
                    // 🚧  We should put defaults someplace where they are easy
                    // to get to.
                    RValue::new("None".to_owned(), self.clone()),
                );
                vec![stmt]
            }
            Self::Boolean => {
                let var = Generator::default().next().unwrap().to_snake_case();
                let stmt = Statement::new(
                    LValue::new(var.clone(), self.clone()),
                    // 🚧  We should put defaults someplace where they are easy
                    // to get to.
                    RValue::new("true".to_owned(), self.clone()),
                );
                vec![stmt]
            }
            Self::Object(o) => {
                let object = domain.sarzak().exhume_object(&o).unwrap();
                // 🚧 I don't currently have these. I am going to create what I
                // know that I need, and then later when I'm integrating woog,
                // I'll fix this.
                //
                // Shit. I can't, because I don't know what the parameters are.
                // I guess this whole mess is going to be put on hold untl later.
                //
                // let mut iter = woog.iter_object_method();
                // let method = loop {
                //     match iter.next() {
                //         Some((_, method)) => {
                //             if method.object == object.id && method.name == "new" {
                //                 break method;
                //             }
                //         }
                //         None => {
                //             panic!("Unable to find the new method for {}", object.name);
                //         }
                //     }
                // };

                let store = find_store(module, domain);
                uses.insert(format!(
                    // 🚧  Oh, man, I was getting desperate here. I've solved this
                    // problem in the interim.
                    "use mdd::{}::types::{};",
                    module,
                    object.as_type(&Mutability::Borrowed(BORROWED), domain)
                ));

                // Recurse into the method
                method.as_statement(package, module, woog, domain.sarzak(), uses)
            }
            Self::Reference(r) => {
                // If the type is a reference, we want to create a new object?
                let object = domain.sarzak().exhume_object(&r).unwrap();
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
                    object.as_type(&Mutability::Borrowed(BORROWED), domain)
                ));

                // Recurse into the method
                method.as_statement(package, module, woog, domain.sarzak(), uses)
            }
            Self::String => {
                let var = Generator::default().next().unwrap().to_snake_case();
                let stmt = Statement::new(
                    LValue::new(var.clone(), self.clone()),
                    RValue::new(Generator::default().next().unwrap(), self.clone()),
                );

                vec![stmt]
            }
            Self::Uuid => {
                let var = Generator::default().next().unwrap().to_snake_case();
                let stmt = Statement::new(
                    LValue::new(var.clone(), self.clone()),
                    RValue::new(
                        format!(
                            "Uuid::new_v5(&UUID_NS, \"{}\")",
                            Generator::default().next().unwrap().to_snake_case()
                        ),
                        self.clone(),
                    ),
                );
                uses.insert("use uuid::Uuid;".to_owned());
                vec![stmt]
            }
            Self::External(e) => {
                let var = Generator::default().next().unwrap().to_snake_case();
                let store = find_store(module, domain);
                uses.insert(format!("use {} as {};", store.path, store.name));

                let stmt = Statement::new(
                    LValue::new(var, self.clone()),
                    RValue::new(
                        format!(
                            "{}::{};",
                            e.as_type(&Mutability::Borrowed(BORROWED), domain),
                            // 🚧  Oops. I don't have this any longer, and I'm not putting
                            // it back until I'm on v2. So here's the hack.
                            // ext.initialization,
                            "new()", // 🚧  This is a hack.
                        ),
                        self.clone(),
                    ),
                );
                vec![stmt]
            }
            Self::Float => {
                let var = Generator::default().next().unwrap().to_snake_case();
                let stmt = Statement::new(
                    LValue::new(var.clone(), self.clone()),
                    RValue::new("42.0".to_owned(), self.clone()),
                );
                vec![stmt]
            }
            Self::Integer => {
                let var = Generator::default().next().unwrap().to_snake_case();
                let stmt = Statement::new(
                    LValue::new(var.clone(), self.clone()),
                    RValue::new("42".to_owned(), self.clone()),
                );
                vec![stmt]
            }
        }
    }
}

/// Render a Parameter as an Rval
///
/// This function is recursive.
impl<'a> RenderStatement for Parameter<'a> {
    fn as_statement(
        &self,
        package: &str,
        module: &str,
        woog: &WoogStore,
        domain: &Domain,
        uses: &mut HashSet<String>,
    ) -> Vec<Statement> {
        log::trace!("{} as rval, next: {:?}", self.name, self.next);
        let mut statements = Vec::new();

        // Get an instance of our type
        let stmt = self.ty.as_statement(package, module, woog, domain, uses);
        statements.push(stmt);

        match self.next {
            Some(param) => {
                // let param = woog.exhume_parameter(&p).unwrap();
                log::trace!("invoking next: {}", param.name);
                // Recurse
                let stmt = param.as_statement(package, module, woog, domain, uses);
                statements.push(stmt);
            }
            _ => {}
        };

        statements.into_iter().flatten().collect()
    }
}

impl<'a> RenderStatement for ObjectMethod<'a> {
    fn as_statement(
        &self,
        package: &str,
        module: &str,
        woog: &WoogStore,
        domain: &Domain,
        uses: &mut HashSet<String>,
    ) -> Vec<Statement> {
        log::trace!("{} as rval", self.name);
        let mut use_statements = String::new();
        let mut statements = Vec::new();

        let obj = domain.sarzak().exhume_object(&self.object).unwrap();
        let mut param = self.param;
        match param {
            Some(p) => {
                // Recurse
                let stmt = p.as_statement(package, module, woog, domain, uses);
                statements.push(stmt);
            }
            _ => {}
        }

        // Add the method call
        let var = Generator::default().next().unwrap().to_snake_case();
        // I let copilot write the following code. It's what I had after the
        // for loop, before it was here. I don't love the forced return, or the
        // panic at the end (which I added), but it does work.
        let ty = GType::Reference(obj.id);
        statements.push(vec![Statement::new(
            LValue::new(var.clone(), ty),
            RValue::new(
                format!(
                    "{}::{}()",
                    obj.as_type(&Mutability::Borrowed(BORROWED), domain),
                    self.name.as_ident(),
                ),
                ty,
            ),
        )]);
        statements.into_iter().flatten().collect()
    }
}
*/
/// Trait for rendering type as a Type
///
/// This trait represents the sanitization of an unknown string, into one
/// suitable for being a type name in Rust. For example, this trait would
/// render  "Rando Object" as `RandoObject`.
///
/// It takes a reference to the store so that Type (see below) works. I've got
/// [a possible workaround](https://git.uberfoo.com/sarzak/sarzak/-/issues/8).
pub(crate) trait RenderType {
    fn as_type(&self, mutability: &Mutability, domain: &Domain) -> String;
}

render_type!(Attribute, Event, Object, State, External, SarzakExternal);

impl RenderType for String {
    fn as_type(&self, mutability: &Mutability, _domain: &Domain) -> String {
        match mutability {
            Mutability::Mutable(_) => format!("mut {}", self.sanitize().to_upper_camel_case()),
            _ => self.sanitize().to_upper_camel_case(),
        }
    }
}

impl RenderType for &str {
    fn as_type(&self, mutability: &Mutability, _domain: &Domain) -> String {
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
    fn as_type(&self, mutability: &Mutability, domain: &Domain) -> String {
        match self {
            Type::Boolean(_) => "bool".to_owned(),
            Type::Object(o) => {
                let object = domain.sarzak().exhume_object(&o).unwrap();
                format!("{}", object.as_type(mutability, domain))
            }
            Type::String(_) => "String".to_owned(),
            Type::Uuid(_) => "Uuid".to_owned(),
            Type::External(e) => {
                let ext = domain.sarzak().exhume_external(&e).unwrap();
                format!("&{}", ext.as_type(mutability, domain))
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
    fn as_type(&self, mutability: &Mutability, domain: &Domain) -> String {
        match self {
            GType::Boolean => "bool".to_owned(),
            GType::Object(o) => {
                let object = domain.sarzak().exhume_object(&o).unwrap();
                format!("{}", object.as_type(&mutability, &domain))
            }
            GType::Reference(r) => {
                let object = domain.sarzak().exhume_object(&r).unwrap();
                format!("&{}", object.as_type(&mutability, &domain))
            }
            GType::Option(o) => {
                format!("Option<{}>", o.as_type(&mutability, &domain))
            }
            GType::External(e) => {
                format!("&{}", e.as_type(&mutability, &domain))
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

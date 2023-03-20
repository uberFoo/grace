//! Render Traits
//!
//! And implementations. This needs some housecleaning.
//!
use std::fmt::Write;

use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
// use names::Generator;
use sarzak::{
    mc::{FormatSnafu, Result},
    sarzak::types::{
        Attribute, Conditionality, Event, External as SarzakExternal, Object, State, Ty,
    },
    v2::domain::Domain,
    woog::{store::ObjectStore as WoogStore, Function, GraceType, Ownership, Variable, SHARED},
};
use snafu::prelude::*;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        get_referrers_sorted,
    },
    todo::{External, GType, ObjectMethod, Parameter as todoP},
};

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
                fn as_type(&self, mutability: &Ownership, woog: &WoogStore, store: &Domain) -> String {
                    self.name.sanitize().as_type(mutability, woog, store)
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

render_ident!(Attribute, Event, Object, State, Function, Variable);

impl<'a> RenderIdent for ObjectMethod<'a> {
    fn as_ident(&self) -> String {
        self.name.sanitize().to_snake_case()
    }
}

impl<'a> RenderIdent for todoP<'a> {
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

                let store = find_store(module, woog, domain);
                uses.insert(format!(
                    // 🚧  Oh, man, I was getting desperate here. I've solved this
                    // problem in the interim.
                    "use mdd::{}::types::{};",
                    module,
                    object.as_type(&woog.exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id()).unwrap(), woog, domain)
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
                    object.as_type(&woog.exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id()).unwrap(), woog, domain)
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
                let store = find_store(module, woog, domain);
                uses.insert(format!("use {} as {};", store.path, store.name));

                let stmt = Statement::new(
                    LValue::new(var, self.clone()),
                    RValue::new(
                        format!(
                            "{}::{};",
                            e.as_type(&woog.exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id()).unwrap(), woog, domain),
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
                    obj.as_type(&woog.exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id()).unwrap(), woog, domain),
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
    fn as_type(&self, mutability: &Ownership, woog: &WoogStore, domain: &Domain) -> String;
}

render_type!(Attribute, Event, Object, State, External, SarzakExternal);

impl RenderType for String {
    fn as_type(&self, mutability: &Ownership, _woog: &WoogStore, _domain: &Domain) -> String {
        match mutability {
            Ownership::Borrowed(id) => match id {
                MUTABLE => format!("mut {}", self.sanitize().to_upper_camel_case()),
                BORROWED => self.sanitize().to_upper_camel_case(),
            },
            Ownership::Owned(_) => self.sanitize().to_upper_camel_case(),
        }
    }
}

impl RenderType for &str {
    fn as_type(&self, mutability: &Ownership, _woog: &WoogStore, _domain: &Domain) -> String {
        match mutability {
            Ownership::Borrowed(id) => match id {
                MUTABLE => format!("mut {}", self.sanitize().to_upper_camel_case()),
                BORROWED => self.sanitize().to_upper_camel_case(),
            },
            Ownership::Owned(_) => self.sanitize().to_upper_camel_case(),
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
impl RenderType for Ty {
    fn as_type(&self, mutability: &Ownership, woog: &WoogStore, domain: &Domain) -> String {
        match self {
            Self::Boolean(_) => "bool".to_owned(),
            Self::Object(o) => {
                let object = domain.sarzak().exhume_object(&o).unwrap();
                format!("{}", object.as_type(mutability, woog, domain))
            }
            Self::String(_) => "String".to_owned(),
            Self::Uuid(_) => "Uuid".to_owned(),
            Self::External(e) => {
                let ext = domain.sarzak().exhume_external(&e).unwrap();
                // format!("&{}", ext.as_type(mutability, woog, domain))
                match mutability {
                    Ownership::Borrowed(id) => match id {
                        MUTABLE => format!("&mut {}", ext.name.sanitize().to_upper_camel_case()),
                        BORROWED => format!("&{}", ext.name.sanitize().to_upper_camel_case()),
                    },
                    Ownership::Owned(_) => format!("{}", ext.name.sanitize().to_upper_camel_case()),
                }
            }
            Self::Float(_) => "f64".to_owned(),
            Self::Integer(_) => "i64".to_owned(),
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
    fn as_type(&self, mutability: &Ownership, woog: &WoogStore, domain: &Domain) -> String {
        match self {
            GType::Boolean => "bool".to_owned(),
            GType::Object(o) => {
                let object = domain.sarzak().exhume_object(&o).unwrap();
                format!("{}", object.as_type(mutability, woog, domain))
            }
            GType::Reference(r) => {
                let object = domain.sarzak().exhume_object(&r).unwrap();
                format!("&{}", object.as_type(mutability, woog, domain))
            }
            GType::Option(o) => {
                format!("Option<{}>", o.as_type(mutability, woog, domain))
            }
            GType::External(e) => {
                format!("&{}", e.as_type(mutability, woog, domain))
            }
            GType::String => "String".to_owned(),
            GType::Uuid => "Uuid".to_owned(),
            GType::Float => "f64".to_owned(),
            GType::Integer => "i64".to_owned(),
        }
    }
}

impl RenderType for GraceType {
    fn as_type(&self, mutability: &Ownership, woog: &WoogStore, domain: &Domain) -> String {
        match self {
            Self::Ty(t) => {
                let ty = domain.sarzak().exhume_ty(&t).unwrap();
                ty.as_type(mutability, woog, domain)
            }
            Self::WoogOption(o) => {
                let o = woog.exhume_woog_option(&o).unwrap();
                let inner = o.r20_grace_type(woog)[0];
                format!("Option<{}>", inner.as_type(mutability, woog, domain))
            }
            Self::Reference(r) => {
                let reference = woog.exhume_reference(&r).unwrap();
                let object = reference.r13_object(domain.sarzak())[0];
                format!("&{}", object.as_type(mutability, woog, domain))
            }
            Self::TimeStamp(_) => "SystemTime".to_owned(),
            Self::Function(_) => unimplemented!(), // Now this is going to be neat.
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
            "macro" => "x_macro".to_owned(),
            "Macro" => "x_macro".to_owned(),
            "let" => "x_let".to_owned(),
            "Let" => "x_let".to_owned(),
            "option" => "woog_option".to_owned(),
            "Option" => "woog_option".to_owned(),
            "enum" => "woog_enum".to_owned(),
            "Enum" => "woog_enum".to_owned(),
            "struct" => "woog_struct".to_owned(),
            "Struct" => "woog_struct".to_owned(),
            "const" => "woog_const".to_owned(),
            "Const" => "woog_const".to_owned(),
            "true" => "true_literal".to_owned(),
            "True" => "true_literal".to_owned(),
            "false" => "false_literal".to_owned(),
            "False" => "false_literal".to_owned(),
            "uuid" => "woog_uuid".to_owned(),
            "Uuid" => "woog_uuid".to_owned(),
            "box" => "x_box".to_owned(),
            "Box" => "x_box".to_owned(),
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
            "macro" => "x_macro".to_owned(),
            "Macro" => "x_macro".to_owned(),
            "let" => "x_let".to_owned(),
            "Let" => "x_let".to_owned(),
            "option" => "woog_option".to_owned(),
            "Option" => "woog_option".to_owned(),
            "enum" => "woog_enum".to_owned(),
            "Enum" => "woog_enum".to_owned(),
            "struct" => "woog_struct".to_owned(),
            "Struct" => "woog_struct".to_owned(),
            "const" => "woog_const".to_owned(),
            "Const" => "woog_const".to_owned(),
            "true" => "true_literal".to_owned(),
            "True" => "true_literal".to_owned(),
            "false" => "false_literal".to_owned(),
            "False" => "false_literal".to_owned(),
            "uuid" => "woog_uuid".to_owned(),
            "Uuid" => "woog_uuid".to_owned(),
            "box" => "x_box".to_owned(),
            "Box" => "x_box".to_owned(),
            _ => self.to_owned(),
        }
    }
}

pub(crate) fn render_attributes(
    buffer: &mut Buffer,
    obj: &Object,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    let mut attrs = obj.r1_attribute(domain.sarzak());
    attrs.sort_by(|a, b| a.name.cmp(&b.name));
    for attr in attrs {
        log::trace!(
            "Rendering attribute: {}, for object: {}.",
            attr.name,
            obj.name
        );
        let ty = attr.r2_ty(domain.sarzak())[0];
        emit!(
            buffer,
            "pub {}: {},",
            attr.as_ident(),
            ty.as_type(
                &woog
                    .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                    .unwrap(),
                woog,
                domain
            )
        );
    }

    Ok(())
}

pub(crate) fn render_referential_attributes(
    buffer: &mut Buffer,
    obj: &Object,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    for referrer in get_referrers_sorted!(obj, domain.sarzak()) {
        let binary = referrer.r6_binary(domain.sarzak())[0];
        let referent = binary.r5_referent(domain.sarzak())[0];
        let r_obj = referent.r16_object(domain.sarzak())[0];

        // Conditionality is confusing for me to think about for some reason,
        // so I'm going to put it down here. I should probably remove this and
        // put it in the book, once I'm done.
        //
        // These aren't really all that tricky, they just get jumbled about
        // in my head.
        //
        // # 1-1
        // This is the easy case. Just output a field for the referential attribute.
        //
        // It's also easy creating the navigation functions. The formalizing side
        // just does a lookup on the store. The other side has to iterate over
        // the instances of the formalizing side (from the store) and find the
        // one that matches it's id. It'll be there. Easy peasy.
        //
        // # 1-1c
        // This is when my brain starts to hurt. For one, the referential
        // attribute should always be on the side that is unconditional.
        // Therefore, there is no need for an Option when we output the
        // field that contains the id of the referent.
        //
        // Navigation is slightly trickier. Going from referrer to referent
        // is the same as 1-1. Going from referent to referrer is a bit
        // trickier. We have to iterate over the instances of the referrer,
        // looking for an id that matches the referent. However, we can't
        // assume that there will always be one.
        //
        // # 1c-1c
        // Here is where we start getting into Options. The referrer side
        // still has a pointer to the referent, but there may not be a
        // referent on the other side. So we need to store it in an Option.
        //
        // Navigation is different going from the referrer to the referent
        // because the referential attribute is inside of an Option. Otherwise
        // the store lookup is the same.
        //
        // Going from referent to referrer is the same as 1-1c.
        //

        // So, what that means, practically, is that I need to check the
        // conditionality of the referent side here.
        //
        // Fuck me. I just came to the opposite conclusion! 😱💩 Maybe
        // I was thinking of where the 'c' is drawn?
        //
        // We should only wrap our pointer in an option when we are conditional.
        // That means that we need to check the conditionality of the referrer.
        //
        let cond = referrer.r11_conditionality(domain.sarzak())[0];

        emit!(
            buffer,
            "/// R{}: [`{}`] '{}' [`{}`]",
            binary.number,
            obj.as_type(
                &woog
                    .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                    .unwrap(),
                woog,
                domain
            ),
            referrer.description,
            r_obj.as_type(
                &woog
                    .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                    .unwrap(),
                woog,
                domain
            )
        );
        match cond {
            Conditionality::Conditional(_) => emit!(
                buffer,
                "pub {}: Option<Uuid>,",
                referrer.referential_attribute.as_ident(),
            ),
            Conditionality::Unconditional(_) => emit!(
                buffer,
                "pub {}: Uuid,",
                referrer.referential_attribute.as_ident(),
            ),
        }
    }

    Ok(())
}

pub(crate) fn render_associative_attributes(
    buffer: &mut Buffer,
    obj: &Object,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    for assoc_referrer in obj.r26_associative_referrer(domain.sarzak()) {
        let assoc = assoc_referrer.r21_associative(domain.sarzak())[0];

        // ⛔️ It looks like r22 and r23 are aliased in the store.
        // Sometimes code comes out with one before other, and
        // sometimes other before one. See grace#39.
        let one = assoc.r23_associative_referent(domain.sarzak())[0];
        let one_obj = one.r25_object(domain.sarzak())[0];

        let other = assoc.r22_associative_referent(domain.sarzak())[0];
        let other_obj = other.r25_object(domain.sarzak())[0];

        emit!(
            buffer,
            "/// R{}: [`{}`] '{}' [`{}`]",
            assoc.number,
            one_obj.as_type(
                &woog
                    .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                    .unwrap(),
                woog,
                domain
            ),
            // one_obj.description,
            "🚧 Out of order — see sarzak#14.".to_owned(),
            one_obj.as_type(
                &woog
                    .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                    .unwrap(),
                woog,
                domain
            )
        );
        emit!(
            buffer,
            "pub {}: Uuid,",
            assoc_referrer.one_referential_attribute.as_ident(),
        );

        emit!(
            buffer,
            "/// R{}: [`{}`] '{}' [`{}`]",
            assoc.number,
            other_obj.as_type(
                &woog
                    .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                    .unwrap(),
                woog,
                domain
            ),
            // other_obj.description,
            "🚧 Out of order — see sarzak#14.".to_owned(),
            other_obj.as_type(
                &woog
                    .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                    .unwrap(),
                woog,
                domain
            )
        );
        emit!(
            buffer,
            "pub {}: Uuid,",
            assoc_referrer.other_referential_attribute.as_ident(),
        );
    }

    Ok(())
}

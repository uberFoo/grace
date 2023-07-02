//! Render Traits
//!
//! And implementations. This needs some housecleaning.
//!
use std::fmt::Write;

use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
// use names::Generator;
use log::debug;
use sarzak::{
    mc::{FormatSnafu, Result},
    sarzak::types::{
        Attribute, Conditionality, Event, External as SarzakExternal, Object, State, Ty,
    },
    v2::domain::Domain,
    woog::{
        store::ObjectStore as WoogStore,
        types::{Field, Function, GraceType, Ownership, Variable},
    },
};
use snafu::prelude::*;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        get_assoc_referent_from_referrer_sorted, get_binary_referrers_sorted,
    },
    options::{GraceConfig, UberStoreOptions},
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

render_ident!(Attribute, Event, Object, State, Function, Field, Variable);

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
pub(crate) trait ForStore {
    fn for_store(
        &self,
        mutability: &Ownership,
        config: &GraceConfig,
        woog: &WoogStore,
        domain: &Domain,
    ) -> String;
}

impl ForStore for GraceType {
    fn for_store(
        &self,
        mutability: &Ownership,
        config: &GraceConfig,
        woog: &WoogStore,
        domain: &Domain,
    ) -> String {
        let is_uber = config.is_uber_store();

        match self {
            Self::Ty(t) => {
                let ty = domain.sarzak().exhume_ty(t).unwrap();
                ty.as_type(mutability, woog, domain)
            }
            Self::WoogOption(o) => {
                let o = woog.exhume_woog_option(o).unwrap();
                let inner = o.r20_grace_type(woog)[0];

                let (inner, _imported) = if let GraceType::Reference(ref id) = inner {
                    let reference = woog.exhume_reference(id).unwrap();
                    let object = reference.r13_object(domain.sarzak())[0];

                    // Here we swizzle the type from a reference to just an object, as
                    // we don't want it to output as a reference.
                    let ty = domain.sarzak().exhume_ty(&object.id).unwrap();
                    let ty = woog.exhume_grace_type(&ty.id()).unwrap();

                    (ty, config.is_imported(&object.id))
                } else {
                    (inner, false)
                };

                if is_uber {
                    use UberStoreOptions::*;
                    match config.get_uber_store().unwrap() {
                        Disabled => unreachable!(),
                        Single => format!(
                            "Option<&Rc<RefCell<{}>>>",
                            inner.as_type(&Ownership::new_borrowed(), woog, domain)
                        ),
                        StdRwLock | ParkingLotRwLock | AsyncRwLock | NDRwLock => format!(
                            "Option<&Arc<RwLock<{}>>>",
                            inner.as_type(&Ownership::new_borrowed(), woog, domain)
                        ),
                        StdMutex | ParkingLotMutex => format!(
                            "Option<&Arc<Mutex<{}>>>",
                            inner.as_type(&Ownership::new_borrowed(), woog, domain)
                        ),
                    }
                } else {
                    format!("Option<{}>", inner.as_type(mutability, woog, domain))
                }
            }
            Self::Reference(r) => {
                let reference = woog.exhume_reference(r).unwrap();
                let object = reference.r13_object(domain.sarzak())[0];
                let imported = config.is_imported(&object.id);

                if is_uber && !imported {
                    use UberStoreOptions::*;
                    match config.get_uber_store().unwrap() {
                        Disabled => unreachable!(),
                        Single => format!(
                            "&Rc<RefCell<{}>>",
                            object.as_type(&Ownership::new_borrowed(), woog, domain)
                        ),
                        StdRwLock | ParkingLotRwLock | AsyncRwLock | NDRwLock => format!(
                            "&Arc<RwLock<{}>>",
                            object.as_type(&Ownership::new_borrowed(), woog, domain)
                        ),
                        StdMutex | ParkingLotMutex => format!(
                            "&Arc<Mutex<{}>>",
                            object.as_type(&Ownership::new_borrowed(), woog, domain)
                        ),
                    }
                } else {
                    format!("&{}", object.as_type(mutability, woog, domain))
                }
            }
            Self::TimeStamp(_) => "SystemTime".to_owned(),
            Self::Usize(_) => "usize".to_owned(),
        }
    }
}

impl ForStore for GType {
    fn for_store(
        &self,
        mutability: &Ownership,
        config: &GraceConfig,
        woog: &WoogStore,
        domain: &Domain,
    ) -> String {
        let is_uber = config.is_uber_store();
        debug_assert!(is_uber);

        match self {
            GType::Boolean => "bool".to_owned(),
            GType::Object(o) => {
                let object = domain.sarzak().exhume_object(o).unwrap();
                object.as_type(mutability, woog, domain)
            }
            GType::Reference(r) => {
                let object = domain.sarzak().exhume_object(r).unwrap();

                if is_uber {
                    use UberStoreOptions::*;
                    match config.get_uber_store().unwrap() {
                        Disabled => unreachable!(),
                        Single => format!(
                            "&Rc<RefCell<{}>>",
                            object.as_type(&Ownership::new_borrowed(), woog, domain)
                        ),
                        StdRwLock | ParkingLotRwLock | AsyncRwLock | NDRwLock => format!(
                            "&Arc<RwLock<{}>>",
                            object.as_type(&Ownership::new_borrowed(), woog, domain)
                        ),
                        StdMutex | ParkingLotMutex => format!(
                            "&Arc<Mutex<{}>>",
                            object.as_type(&Ownership::new_borrowed(), woog, domain)
                        ),
                    }
                } else {
                    format!("&{}", object.as_type(mutability, woog, domain))
                }
            }
            GType::Option(o) => {
                let (o, imported) = if let GType::Reference(id) = **o {
                    let object = domain.sarzak().exhume_object(&id).unwrap();

                    // Here we swizzle the type from a reference to just an object, as
                    // we don't want it to output as a reference.
                    let _ty = domain.sarzak().exhume_ty(&object.id).unwrap();

                    (Box::new(GType::Object(id)), config.is_imported(&object.id))
                } else {
                    (o.clone(), false)
                };

                if is_uber && !imported {
                    use UberStoreOptions::*;
                    match config.get_uber_store().unwrap() {
                        Disabled => unreachable!(),
                        Single => format!(
                            "Option<&Rc<RefCell<{}>>>",
                            o.as_type(&Ownership::new_borrowed(), woog, domain)
                        ),
                        StdRwLock | ParkingLotRwLock | AsyncRwLock | NDRwLock => format!(
                            "Option<&Arc<RwLock<{}>>>",
                            o.as_type(&Ownership::new_borrowed(), woog, domain)
                        ),
                        StdMutex | ParkingLotMutex => format!(
                            "Option<&Arc<Mutex<{}>>>",
                            o.as_type(&Ownership::new_borrowed(), woog, domain)
                        ),
                    }
                } else {
                    format!("Option<{}>", o.as_type(mutability, woog, domain))
                }
            }
            GType::External(e) => {
                format!("&{}", e.as_type(mutability, woog, domain))
            }
            GType::String => "String".to_owned(),
            GType::Uuid => "Uuid".to_owned(),
            GType::Usize => "usize".to_owned(),
            GType::Float => "f64".to_owned(),
            GType::Integer => "i64".to_owned(),
        }
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
    fn as_type(&self, mutability: &Ownership, woog: &WoogStore, domain: &Domain) -> String;
}

render_type!(Attribute, Event, Object, State, External, SarzakExternal);

impl RenderType for String {
    fn as_type(&self, mutability: &Ownership, _woog: &WoogStore, _domain: &Domain) -> String {
        match mutability {
            Ownership::Mutable(_) => format!("mut {}", self.sanitize().to_upper_camel_case()),
            _ => self.sanitize().to_upper_camel_case(),
        }
    }
}

impl RenderType for &str {
    fn as_type(&self, mutability: &Ownership, _woog: &WoogStore, _domain: &Domain) -> String {
        match mutability {
            Ownership::Mutable(_) => format!("mut {}", self.sanitize().to_upper_camel_case()),
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
impl RenderType for Ty {
    fn as_type(&self, mutability: &Ownership, woog: &WoogStore, domain: &Domain) -> String {
        match self {
            Self::Boolean(_) => "bool".to_owned(),
            Self::Object(o) => {
                let object = domain.sarzak().exhume_object(o).unwrap();
                object.as_type(mutability, woog, domain)
            }
            Self::SString(_) => "String".to_owned(),
            Self::SUuid(_) => "Uuid".to_owned(),
            Self::External(e) => {
                let ext = domain.sarzak().exhume_external(e).unwrap();
                // format!("&{}", ext.as_type(mutability, woog, domain))
                match mutability {
                    Ownership::Owned(_) => ext.name.sanitize().to_upper_camel_case(),
                    Ownership::Borrowed(_) => {
                        format!("&{}", ext.name.sanitize().to_upper_camel_case())
                    }
                    Ownership::Mutable(_) => {
                        // Shit, this doesn't belong here. Is it a reference or owned? Let's pretend
                        // Mutable means mutable reference, what do you say?
                        format!("&mut {}", ext.name.sanitize().to_upper_camel_case())
                    }
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
                let object = domain.sarzak().exhume_object(o).unwrap();
                object.as_type(mutability, woog, domain)
            }
            GType::Reference(r) => {
                let object = domain.sarzak().exhume_object(r).unwrap();
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
            GType::Usize => "usize".to_owned(),
            GType::Float => "f64".to_owned(),
            GType::Integer => "i64".to_owned(),
        }
    }
}

impl RenderType for GraceType {
    fn as_type(&self, mutability: &Ownership, woog: &WoogStore, domain: &Domain) -> String {
        match self {
            Self::Ty(t) => {
                let ty = domain.sarzak().exhume_ty(t).unwrap();
                ty.as_type(mutability, woog, domain)
            }
            Self::WoogOption(o) => {
                let o = woog.exhume_woog_option(o).unwrap();
                let inner = o.r20_grace_type(woog)[0];
                format!("Option<{}>", inner.as_type(mutability, woog, domain))
            }
            Self::Reference(r) => {
                let reference = woog.exhume_reference(r).unwrap();
                let object = reference.r13_object(domain.sarzak())[0];
                format!("&{}", object.as_type(mutability, woog, domain))
            }
            Self::TimeStamp(_) => "SystemTime".to_owned(),
            Self::Usize(_) => "usize".to_owned(),
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
        let result = match *self {
            "box" => "x_box".to_owned(),
            "Box" => "x_box".to_owned(),
            "break" => "x_break".to_owned(),
            "crate" => "krate".to_owned(),
            "Crate" => "krate".to_owned(),
            "const" => "woog_const".to_owned(),
            "Const" => "woog_const".to_owned(),
            "enum" => "woog_enum".to_owned(),
            "Enum" => "woog_enum".to_owned(),
            "false" => "false_literal".to_owned(),
            "False" => "false_literal".to_owned(),
            "if" => "x_if".to_owned(),
            "If" => "x_if".to_owned(),
            // "index" => "x_index".to_owned(),
            // "Index" => "x_index".to_owned(),
            "let" => "x_let".to_owned(),
            "Let" => "x_let".to_owned(),
            "macro" => "x_macro".to_owned(),
            "Macro" => "x_macro".to_owned(),
            "model" => "x_model".to_owned(),
            "Model" => "x_model".to_owned(),
            "None" => "z_none".to_owned(),
            "Object Store" => "z_object_store".to_owned(),
            "option" => "woog_option".to_owned(),
            "Option" => "woog_option".to_owned(),
            "ref" => "x_ref".to_owned(),
            "return" => "x_return".to_owned(),
            "Return" => "x_return".to_owned(),
            "Some" => "z_some".to_owned(),
            "string" => "z_string".to_owned(),
            "String" => "z_string".to_owned(),
            "struct" => "woog_struct".to_owned(),
            "Struct" => "woog_struct".to_owned(),
            "super" => "z_super".to_owned(),
            "Super" => "z_super".to_owned(),
            "true" => "true_literal".to_owned(),
            "True" => "true_literal".to_owned(),
            "type" => "ty".to_owned(),
            "Type" => "ty".to_owned(),
            "uuid" => "z_uuid".to_owned(),
            "Uuid" => "z_uuid".to_owned(),
            "UUID" => "z_uuid".to_owned(),
            "value" => "x_value".to_owned(),
            "Value" => "x_value".to_owned(),
            _ => self.to_string(),
        };

        if self != &result {
            debug!("sanitized: {} -> {}", self, result);
        }
        result
    }
}

impl Sanitize for String {
    fn sanitize(&self) -> String {
        let result = match self.as_str() {
            "box" => "x_box".to_owned(),
            "Box" => "x_box".to_owned(),
            "break" => "x_break".to_owned(),
            "crate" => "krate".to_owned(),
            "Crate" => "krate".to_owned(),
            "const" => "woog_const".to_owned(),
            "Const" => "woog_const".to_owned(),
            "enum" => "woog_enum".to_owned(),
            "Enum" => "woog_enum".to_owned(),
            "false" => "false_literal".to_owned(),
            "False" => "false_literal".to_owned(),
            "if" => "x_if".to_owned(),
            "If" => "x_if".to_owned(),
            // "index" => "x_index".to_owned(),
            // "Index" => "x_index".to_owned(),
            "let" => "x_let".to_owned(),
            "Let" => "x_let".to_owned(),
            "macro" => "x_macro".to_owned(),
            "Macro" => "x_macro".to_owned(),
            "model" => "x_model".to_owned(),
            "Model" => "x_model".to_owned(),
            "None" => "z_none".to_owned(),
            "Object Store" => "z_object_store".to_owned(),
            "option" => "woog_option".to_owned(),
            "Option" => "woog_option".to_owned(),
            "ref" => "x_ref".to_owned(),
            "return" => "x_return".to_owned(),
            "Return" => "x_return".to_owned(),
            "Some" => "z_some".to_owned(),
            "String" => "s_string".to_owned(),
            "string" => "s_string".to_owned(),
            "struct" => "woog_struct".to_owned(),
            "Struct" => "woog_struct".to_owned(),
            "super" => "x_super".to_owned(),
            "Super" => "x_super".to_owned(),
            "true" => "true_literal".to_owned(),
            "True" => "true_literal".to_owned(),
            "type" => "ty".to_owned(),
            "Type" => "ty".to_owned(),
            "uuid" => "s_uuid".to_owned(),
            "Uuid" => "s_uuid".to_owned(),
            "UUID" => "s_uuid".to_owned(),
            "value" => "x_value".to_owned(),
            "Value" => "x_value".to_owned(),
            _ => self.to_owned(),
        };

        if self != &result {
            debug!("sanitized: {} -> {}", self, result);
        }
        result
    }
}

pub(crate) fn render_attributes(
    buffer: &mut Buffer,
    obj: &Object,
    config: &GraceConfig,
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
        // This is a really weird thing and I'm not really sure how to address it.
        // I suspect that my troubles stem from the fact that I'm generating rust
        // directly from a sarzak model. Maybe in dwarf this would be a triviality.
        // I think I'd have to add a usize to lu-dog. Maybe an index type? Right
        // now I'm just testing that it's an int and casting it to usize. And that
        // might work, depending on how the emit stuff is implemented, I think.
        // Not sure.
        // Anyway, I'm doing the really ugly thing here.
        if attr.name == "id" && config.get_optimization_level() == &crate::OptimizationLevel::Vec {
            emit!(buffer, "pub {}: usize,", attr.as_ident());
        } else if attr.name != "hack" {
            // Ugly thing times two. The "hack" thing is added when we first process
            // the domain. If it's a Vec store, we want to promote any enums to hybrids.
            // This is the fast button.
            let ty = attr.r2_ty(domain.sarzak())[0];
            emit!(
                buffer,
                "pub {}: {},",
                attr.as_ident(),
                ty.as_type(&Ownership::new_borrowed(), woog, domain)
            );
        }
    }

    Ok(())
}

pub(crate) fn render_referential_attributes(
    buffer: &mut Buffer,
    obj: &Object,
    config: &GraceConfig,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    for referrer in get_binary_referrers_sorted!(obj, domain.sarzak()) {
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
        let ty = if config.is_uber_store() && !config.is_imported(&r_obj.id) {
            "usize"
        } else {
            "Uuid"
        };

        emit!(
            buffer,
            "/// R{}: [`{}`] '{}' [`{}`]",
            binary.number,
            obj.as_type(&Ownership::new_borrowed(), woog, domain),
            referrer.description,
            r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
        );
        match cond {
            Conditionality::Conditional(_) => emit!(
                buffer,
                "pub {}: Option<{ty}>,",
                referrer.referential_attribute.as_ident(),
            ),
            Conditionality::Unconditional(_) => emit!(
                buffer,
                "pub {}: {ty},",
                referrer.referential_attribute.as_ident(),
            ),
        }
    }

    Ok(())
}

pub(crate) fn render_associative_attributes(
    buffer: &mut Buffer,
    obj: &Object,
    config: &GraceConfig,
    woog: &WoogStore,
    domain: &Domain,
) -> Result<()> {
    for assoc_referrer in obj.r26_associative_referrer(domain.sarzak()) {
        let assoc = assoc_referrer.r21_associative(domain.sarzak())[0];
        let referents = get_assoc_referent_from_referrer_sorted!(assoc_referrer, domain.sarzak());
        let ty = if config.is_uber_store() && !config.is_imported(&obj.id) {
            "usize"
        } else {
            "Uuid"
        };

        for referent in referents {
            let an_ass = referent.r22_an_associative_referent(domain.sarzak())[0];
            let assoc_obj = referent.r25_object(domain.sarzak())[0];

            emit!(
                buffer,
                "/// R{}: [`{}`] '{}' [`{}`]",
                assoc.number,
                assoc_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                // one_obj.description,
                "🚧 Out of order — see sarzak#14.".to_owned(),
                assoc_obj.as_type(&Ownership::new_borrowed(), woog, domain)
            );
            emit!(
                buffer,
                "pub {}: {ty},",
                an_ass.referential_attribute.as_ident(),
            );
        }
    }

    Ok(())
}

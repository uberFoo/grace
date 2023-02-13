use std::fmt::Write;

use log;
use sarzak::{
    domain::Domain,
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::{
        macros::{
            sarzak_get_many_as_across_r1, sarzak_get_one_card_across_r9,
            sarzak_get_one_cond_across_r11, sarzak_get_one_cond_across_r12,
            sarzak_get_one_obj_across_r16, sarzak_get_one_obj_across_r17,
            sarzak_get_one_r_bin_across_r5, sarzak_get_one_r_bin_across_r6,
            sarzak_get_one_r_from_across_r6, sarzak_get_one_r_to_across_r5,
            sarzak_get_one_t_across_r2, sarzak_maybe_get_many_r_froms_across_r17,
            sarzak_maybe_get_many_r_tos_across_r16,
        },
        types::{
            Attribute, Binary, Cardinality, Conditionality, External as SarzakExternal, Object,
            Referent, Referrer, Type,
        },
    },
    woog::{store::ObjectStore as WoogStore, Mutability, BORROWED, MUTABLE, PUBLIC},
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        generator::CodeWriter,
        render::{RenderIdent, RenderType},
        render_make_uuid, render_method_definition, render_new_instance,
    },
    options::GraceCompilerOptions,
    todo::{External, GType, LValue, ObjectMethod, Parameter, RValue},
    types::{MethodImplementation, StructDefinition, StructImplementation},
};

macro_rules! get_referrers {
    ($obj:expr, $store:expr) => {{
        let mut referrers = sarzak_maybe_get_many_r_froms_across_r17!($obj, $store);
        referrers.sort_by(|a, b| {
            let binary = sarzak_get_one_r_bin_across_r6!(&a, $store);
            let referent = sarzak_get_one_r_to_across_r5!(binary, $store);
            let obj_a = sarzak_get_one_obj_across_r16!(referent, $store);

            let binary = sarzak_get_one_r_bin_across_r6!(&b, $store);
            let referent = sarzak_get_one_r_to_across_r5!(binary, $store);
            let obj_b = sarzak_get_one_obj_across_r16!(referent, $store);

            obj_a.name.cmp(&obj_b.name)
        });
        referrers
    }};
}

macro_rules! get_referents {
    ($obj:expr, $store:expr) => {{
        let mut referents = sarzak_maybe_get_many_r_tos_across_r16!($obj, $store);
        referents.sort_by(|a, b| {
            let binary = sarzak_get_one_r_bin_across_r5!(&a, $store);
            let referrer = sarzak_get_one_r_from_across_r6!(binary, $store);
            let obj_a = sarzak_get_one_obj_across_r17!(referrer, $store);

            let binary = sarzak_get_one_r_bin_across_r5!(&b, $store);
            let referrer = sarzak_get_one_r_from_across_r6!(binary, $store);
            let obj_b = sarzak_get_one_obj_across_r17!(referrer, $store);

            obj_a.name.cmp(&obj_b.name)
        });
        referents
    }};
}

/// Domain Struct Generator / CodeWriter
///
/// We need a builder for this so that we can add privacy modifiers, as
/// well as derives.
pub(crate) struct DomainStruct;

impl DomainStruct {
    pub(crate) fn new() -> Box<dyn StructDefinition> {
        Box::new(Self)
    }
}

impl StructDefinition for DomainStruct {}

impl CodeWriter for DomainStruct {
    fn write_code(
        &self,
        options: &GraceCompilerOptions,
        domain: &Domain,
        _woog: &mut WoogStore,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by DomainStruct"
            }
        );
        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();

        // These need to be sorted, as they are output as attributes and we require
        // stable output.
        let referrers = get_referrers!(obj, domain.sarzak());
        let referents = get_referents!(obj, domain.sarzak());

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-use-statements", obj.as_ident()),
            |buffer| {
                // Everything has an `id`, everything needs this.
                emit!(buffer, "use uuid::Uuid;");
                emit!(buffer, "");

                // Add the use statements from the options.
                if let Some(use_paths) = &options.use_paths {
                    for path in use_paths {
                        emit!(buffer, "use {};", path);
                    }
                    emit!(buffer, "");
                }

                // We need this to create id's.
                emit!(buffer, "use crate::{}::UUID_NS;", module);

                // Add use statements for all the referrers.
                if referrers.len() > 0 {
                    emit!(buffer, "");
                    emit!(buffer, "// Referrer imports");
                }
                for referrer in &referrers {
                    let binary = sarzak_get_one_r_bin_across_r6!(referrer, domain.sarzak());
                    let referent = sarzak_get_one_r_to_across_r5!(binary, domain.sarzak());
                    let r_obj = sarzak_get_one_obj_across_r16!(referent, domain.sarzak());

                    emit!(
                        buffer,
                        "use crate::{}::types::{}::{};",
                        module,
                        r_obj.as_ident(),
                        r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                    );
                }

                // Add use statements for all the referents.
                if referents.len() > 0 {
                    emit!(buffer, "");
                    emit!(buffer, "// Referent imports");
                }
                for referent in &referents {
                    let binary = sarzak_get_one_r_bin_across_r5!(referent, domain.sarzak());
                    let referrer = sarzak_get_one_r_from_across_r6!(binary, domain.sarzak());
                    let r_obj = sarzak_get_one_obj_across_r17!(referrer, domain.sarzak());

                    emit!(
                        buffer,
                        "use crate::{}::types::{}::{};",
                        module,
                        r_obj.as_ident(),
                        r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                    );
                }

                // Add the ObjectStore
                emit!(buffer, "");
                let mut iter = domain.sarzak().iter_ty();
                let name = format!(
                    "{}Store",
                    module.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                );
                let store = loop {
                    let ty = iter.next();
                    match ty {
                        Some((_, ty)) => match ty {
                            Type::External(e) => {
                                let ext = domain.sarzak().exhume_external(&e).unwrap();
                                if ext.name == name {
                                    break ext;
                                }
                            }
                            _ => continue,
                        },
                        None => panic!("Could not find store type for {}", module),
                    }
                };
                emit!(buffer, "use {} as {};", store.path, store.name);

                Ok(())
            },
        )?;
        emit!(buffer, "");

        log::debug!("writing Struct Definition for {}", obj.name);

        buffer.block(
            DirectiveKind::CommentOrig,
            format!("{}-struct-documentation", obj.as_ident()),
            |buffer| {
                for line in obj.description.split_terminator('\n') {
                    emit!(buffer, "/// {}", line);
                }
                Ok(())
            },
        )?;

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-struct-definition", obj.as_ident()),
            |buffer| {
                if let Some(derive) = &options.derive {
                    write!(buffer, "#[derive(").context(FormatSnafu)?;
                    for d in derive {
                        write!(buffer, "{},", d).context(FormatSnafu)?;
                    }
                    emit!(buffer, ")]");
                }

                emit!(
                    buffer,
                    "pub struct {} {{",
                    obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                );

                let mut attrs = sarzak_get_many_as_across_r1!(obj, domain.sarzak());
                attrs.sort_by(|a, b| a.name.cmp(&b.name));
                for attr in attrs {
                    let ty = sarzak_get_one_t_across_r2!(attr, domain.sarzak());
                    emit!(
                        buffer,
                        "pub {}: {},",
                        attr.as_ident(),
                        ty.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                    );
                }

                for referrer in &referrers {
                    let binary = sarzak_get_one_r_bin_across_r6!(referrer, domain.sarzak());
                    let referent = sarzak_get_one_r_to_across_r5!(binary, domain.sarzak());
                    let r_obj = sarzak_get_one_obj_across_r16!(referent, domain.sarzak());
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
                    let cond = sarzak_get_one_cond_across_r11!(referrer, domain.sarzak());

                    emit!(
                        buffer,
                        "/// R{}: [`{}`] '{}' [`{}`]",
                        binary.number,
                        obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak()),
                        referrer.description,
                        r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
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

                emit!(buffer, "}}");
                Ok(())
            },
        )?;

        Ok(())
    }
}

pub(crate) struct DomainImplBuilder {
    methods: Vec<Box<dyn MethodImplementation>>,
}

impl DomainImplBuilder {
    pub(crate) fn new() -> DomainImplBuilder {
        Self {
            methods: Vec::new(),
        }
    }

    pub(crate) fn method(mut self, method: Box<dyn MethodImplementation>) -> Self {
        self.methods.push(method);

        self
    }

    pub(crate) fn build(self) -> Box<dyn StructImplementation> {
        Box::new(DomainImplementation {
            methods: self.methods,
        })
    }
}

pub(crate) struct DomainImplementation {
    methods: Vec<Box<dyn MethodImplementation>>,
}

impl StructImplementation for DomainImplementation {}

impl CodeWriter for DomainImplementation {
    fn write_code(
        &self,
        options: &GraceCompilerOptions,
        domain: &Domain,
        woog: &mut WoogStore,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by DomainImplementation"
            }
        );
        let obj_id = obj_id.unwrap();
        let object = domain.sarzak().exhume_object(&obj_id).unwrap();

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-struct-implementation", object.as_ident()),
            |buffer| {
                let obj = domain.sarzak().exhume_object(&obj_id).unwrap();

                emit!(
                    buffer,
                    "impl {} {{",
                    obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                );

                for method in &self.methods {
                    method.write_code(options, domain, woog, module, Some(obj_id), buffer)?;
                }

                emit!(buffer, "}}");

                Ok(())
            },
        )
    }
}

/// Domain New Implementation
///
/// This generates a new implementation for the object. The new implementation
/// calculates the object's `id` based on the string representation of it's
/// attributes.
///
/// __NB__ --- this implies that the lexicographical sum of it's attributes,
/// across all instances, must be unique.
pub(crate) struct DomainNewImpl;

impl DomainNewImpl {
    pub(crate) fn new() -> Box<dyn MethodImplementation> {
        Box::new(Self)
    }
}

impl MethodImplementation for DomainNewImpl {}

impl CodeWriter for DomainNewImpl {
    fn write_code(
        &self,
        _options: &GraceCompilerOptions,
        domain: &Domain,
        woog: &mut WoogStore,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by DomainNewImpl"
            }
        );
        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();

        // These are more attributes on our object, and they should be sorted.
        let referrers = get_referrers!(obj, domain.sarzak());

        // Collect the attributes
        let mut params: Vec<Parameter> = Vec::new();
        // This is used in the new_instance call. These fields are meant to be
        // matched up with the input arguments, and type checked. Since I'm
        // generating both, I'm beginning to wonder what the point is.
        //
        // So just now the type system reminded me that I need to turn a referince
        // into a UUID. So maybe it's worth keeping.
        let mut fields: Vec<LValue> = Vec::new();
        let mut attrs = sarzak_get_many_as_across_r1!(obj, domain.sarzak());
        attrs.sort_by(|a, b| a.name.cmp(&b.name));
        for attr in attrs {
            // We are going to generate the id, so don't include it in the
            // list of parameters.
            if attr.name != "id" {
                let ty = sarzak_get_one_t_across_r2!(attr, domain.sarzak());
                fields.push(LValue::new(attr.name.as_ident(), ty.into()));
                params.push(Parameter::new(
                    BORROWED,
                    None,
                    ty.into(),
                    PUBLIC,
                    attr.as_ident(),
                ));
            }
        }

        // And the referential attributes
        for referrer in &referrers {
            let binary = sarzak_get_one_r_bin_across_r6!(referrer, domain.sarzak());
            let referent = sarzak_get_one_r_to_across_r5!(binary, domain.sarzak());
            let r_obj = sarzak_get_one_obj_across_r16!(referent, domain.sarzak());
            let cond = sarzak_get_one_cond_across_r11!(referrer, domain.sarzak());

            // If the relationship is conditional, then we need to make the
            // parameter an Option, and make the field match.
            match cond {
                Conditionality::Conditional(_) => {
                    fields.push(LValue::new(
                        referrer.referential_attribute.as_ident(),
                        GType::Option(Box::new(GType::Uuid)),
                    ));
                    params.push(Parameter::new(
                        BORROWED,
                        None,
                        GType::Option(Box::new(GType::Reference(r_obj.id))),
                        PUBLIC,
                        referrer.referential_attribute.as_ident(),
                    ));
                }
                Conditionality::Unconditional(_) => {
                    fields.push(LValue::new(
                        referrer.referential_attribute.as_ident(),
                        GType::Uuid,
                    ));
                    params.push(Parameter::new(
                        BORROWED,
                        None,
                        GType::Reference(r_obj.id),
                        PUBLIC,
                        referrer.referential_attribute.as_ident(),
                    ));
                }
            }
        }

        // Add the store to the end of the  input parameters
        let mut iter = domain.sarzak().iter_ty();
        let name = format!(
            "{}Store",
            module.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
        );
        let store_type = loop {
            let ty = iter.next();
            match ty {
                Some((_, ty)) => match ty {
                    Type::External(e) => {
                        let ext = domain.sarzak().exhume_external(&e).unwrap();
                        if ext.name == name {
                            break GType::External(External::new(
                                ext.name.clone(),
                                ext.path.clone(),
                                None,
                            ));
                        }
                    }
                    _ => continue,
                },
                None => panic!("Could not find store type for {}", module),
            }
        };
        params.push(Parameter::new(
            MUTABLE,
            None,
            store_type,
            PUBLIC,
            "store".to_owned(),
        ));

        // Collect rvals for rendering the method.
        let rvals = params.clone();
        let mut rvals: Vec<RValue> = rvals.iter().map(|p| p.into()).collect();
        // Remove the store.
        rvals.pop();

        // Link the params. The result is the head of the list.
        let param = if params.len() > 0 {
            let mut iter = params.iter_mut().rev();
            let mut last = iter.next().unwrap();
            loop {
                match iter.next() {
                    Some(param) => {
                        param.next = Some(last);
                        last = param;
                    }
                    None => break,
                }
            }
            log::trace!("param: {:?}", last);
            Some(last.clone())
        } else {
            None
        };

        // Create an ObjectMethod
        // The uniqueness of this instance depends on the inputs to it's
        // new method. Param can be None, and two methods on the same
        // object will have the same obj. So it comes down to a unique
        // name for each object. So just "new" should suffice for name,
        // because it's scoped by obj already.
        let method = ObjectMethod::new(
            param.as_ref(),
            obj.id,
            GType::Object(obj.id),
            PUBLIC,
            "new".to_owned(),
            "Create a new instance".to_owned(),
        );

        buffer.block(
            DirectiveKind::CommentOrig,
            format!("{}-struct-impl-new", obj.as_ident()),
            |buffer| {
                // Output a docstring
                emit!(
                    buffer,
                    "/// Inter a new {} in the store, and return it's `id`.",
                    obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                );

                // Output the top of the function definition
                render_method_definition(buffer, &method, woog, domain.sarzak())?;

                // Output the code to create the `id`.
                let id = LValue::new("id", GType::Uuid);
                render_make_uuid(buffer, &id, &rvals, domain.sarzak())?;

                // Output code to create the instance
                let new = LValue::new("new", GType::Reference(obj.id));
                render_new_instance(buffer, obj, Some(&new), &fields, &rvals, domain.sarzak())?;

                emit!(buffer, "store.inter_{}(new.clone());", obj.as_ident());
                emit!(buffer, "new");
                emit!(buffer, "}}");

                Ok(())
            },
        )
    }
}

/// Domain Relationship Navigation Implementation
///
/// This generates relationship navigation methods for a type. A method will be
/// generated for each relationship in which this object participates. This
/// applies to both formalizing and non-formalizing relationships.
pub(crate) struct DomainRelNavImpl;

impl DomainRelNavImpl {
    pub(crate) fn new() -> Box<dyn MethodImplementation> {
        Box::new(Self)
    }

    fn forward(
        buffer: &mut Buffer,
        obj: &Object,
        referrer: &Referrer,
        binary: &Binary,
        store: &SarzakExternal,
        r_obj: &Object,
        domain: &Domain,
    ) -> Result<()> {
        buffer.block(
            DirectiveKind::CommentOrig,
            format!(
                "{}-struct-impl-nav-forward-to-{}",
                obj.as_ident(),
                referrer.referential_attribute.as_ident()
            ),
            |buffer| {
                emit!(
                    buffer,
                    "/// Navigate to [`{}`] across R{}(1-?)",
                    r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak()),
                    binary.number,
                );
                emit!(
                    buffer,
                    "pub fn {}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                );
                emit!(
                    buffer,
                    "vec![store.exhume_{}(&self.{}).unwrap()]",
                    r_obj.as_ident(),
                    referrer.referential_attribute.as_ident()
                );
                emit!(buffer, "}}");

                Ok(())
            },
        )
    }

    fn forward_conditional(
        buffer: &mut Buffer,
        obj: &Object,
        referrer: &Referrer,
        binary: &Binary,
        store: &SarzakExternal,
        r_obj: &Object,
        domain: &Domain,
    ) -> Result<()> {
        buffer.block(
            DirectiveKind::CommentOrig,
            format!(
                "{}-struct-impl-nav-forward-cond-to-{}",
                obj.as_ident(),
                referrer.referential_attribute.as_ident()
            ),
            |buffer| {
                emit!(
                    buffer,
                    "/// Navigate to [`{}`] across R{}(1-?c)",
                    r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak()),
                    binary.number,
                );
                emit!(
                    buffer,
                    "pub fn {}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                );
                emit!(
                    buffer,
                    "match self.{} {{",
                    referrer.referential_attribute.as_ident()
                );
                emit!(
                    buffer,
                    "Some(ref {}) => vec![store.exhume_{}({}).unwrap()],",
                    referrer.referential_attribute.as_ident(),
                    r_obj.as_ident(),
                    referrer.referential_attribute.as_ident()
                );
                emit!(buffer, "None => Vec::new(),");
                emit!(buffer, "}}");
                emit!(buffer, "}}");

                Ok(())
            },
        )
    }

    fn backward_one(
        buffer: &mut Buffer,
        obj: &Object,
        r_obj: &Object,
        binary: &Binary,
        store: &SarzakExternal,
        referrer: &Referrer,
        domain: &Domain,
    ) -> Result<()> {
        buffer.block(
            DirectiveKind::CommentOrig,
            format!(
                "{}-struct-impl-nav-backward-one-to-{}",
                obj.as_ident(),
                r_obj.as_ident()
            ),
            |buffer| {
                emit!(
                    buffer,
                    "/// Navigate to [`{}`] across R{}(1-1)",
                    r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak()),
                    binary.number
                );
                emit!(
                    buffer,
                    "pub fn {}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                );
                emit!(buffer, "vec![store.iter_{}()", r_obj.as_ident());
                emit!(
                    buffer,
                    ".find(|{}| {}.1.{} == self.id).unwrap().1]",
                    r_obj.as_ident(),
                    r_obj.as_ident(),
                    referrer.referential_attribute.as_ident()
                );
                emit!(buffer, "}}");

                Ok(())
            },
        )
    }

    fn backward_one_conditional(
        buffer: &mut Buffer,
        obj: &Object,
        r_obj: &Object,
        binary: &Binary,
        store: &SarzakExternal,
        referrer: &Referrer,
        domain: &Domain,
    ) -> Result<()> {
        buffer.block(
            DirectiveKind::CommentOrig,
            format!(
                "{}-struct-impl-nav-backward-cond-to-{}",
                obj.as_ident(),
                r_obj.as_ident()
            ),
            |buffer| {
                emit!(
                    buffer,
                    "/// Navigate to [`{}`] across R{}(1-1c)",
                    r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak()),
                    binary.number
                );
                emit!(
                    buffer,
                    "pub fn {}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                );
                emit!(
                    buffer,
                    "let {} = store.iter_{}()",
                    r_obj.as_ident(),
                    r_obj.as_ident()
                );
                emit!(
                    buffer,
                    ".find(|{}| {}.1.{} == self.id);",
                    r_obj.as_ident(),
                    r_obj.as_ident(),
                    referrer.referential_attribute.as_ident()
                );
                emit!(buffer, "match {} {{", r_obj.as_ident());
                emit!(
                    buffer,
                    "Some(ref {}) => vec![{}.1],",
                    r_obj.as_ident(),
                    r_obj.as_ident()
                );
                emit!(buffer, "None => Vec::new(),");
                emit!(buffer, "}}");
                emit!(buffer, "}}");

                Ok(())
            },
        )
    }

    fn backward_one_biconditional(
        buffer: &mut Buffer,
        obj: &Object,
        r_obj: &Object,
        binary: &Binary,
        store: &SarzakExternal,
        referrer: &Referrer,
        domain: &Domain,
    ) -> Result<()> {
        buffer.block(
            DirectiveKind::CommentOrig,
            format!(
                "{}-struct-impl-nav-backward-one-bi-cond-to-{}",
                obj.as_ident(),
                r_obj.as_ident()
            ),
            |buffer| {
                emit!(
                    buffer,
                    "/// Navigate to [`{}`] across R{}(1c-1c)",
                    r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak()),
                    binary.number
                );
                emit!(
                    buffer,
                    "pub fn {}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                );
                emit!(
                    buffer,
                    "let {} = store.iter_{}()",
                    r_obj.as_ident(),
                    r_obj.as_ident()
                );
                emit!(
                    buffer,
                    ".find(|{}| {}.1.{} == Some(self.id));",
                    r_obj.as_ident(),
                    r_obj.as_ident(),
                    referrer.referential_attribute.as_ident()
                );
                emit!(buffer, "match {} {{", r_obj.as_ident());
                emit!(
                    buffer,
                    "Some(ref {}) => vec![{}.1],",
                    r_obj.as_ident(),
                    r_obj.as_ident()
                );
                emit!(buffer, "None => Vec::new(),");
                emit!(buffer, "}}");
                emit!(buffer, "}}");

                Ok(())
            },
        )
    }

    fn backward_1_m(
        buffer: &mut Buffer,
        obj: &Object,
        r_obj: &Object,
        binary: &Binary,
        store: &SarzakExternal,
        referrer: &Referrer,
        domain: &Domain,
    ) -> Result<()> {
        buffer.block(
            DirectiveKind::CommentOrig,
            format!(
                "{}-struct-impl-nav-backward-1_M-to-{}",
                obj.as_ident(),
                r_obj.as_ident()
            ),
            |buffer| {
                emit!(
                    buffer,
                    "/// Navigate to [`{}`] across R{}(1-M)",
                    r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak()),
                    binary.number
                );
                emit!(
                    buffer,
                    "pub fn {}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                );
                emit!(buffer, "store.iter_{}()", r_obj.as_ident());
                emit!(
                    buffer,
                    ".filter_map(|{}| if {}.1.{} == self.id {{ Some({}.1) }} else {{ None }})",
                    r_obj.as_ident(),
                    r_obj.as_ident(),
                    referrer.referential_attribute.as_ident(),
                    r_obj.as_ident(),
                );
                emit!(buffer, ".collect()");
                emit!(buffer, "}}");

                Ok(())
            },
        )
    }

    fn backward_1_mc(
        buffer: &mut Buffer,
        obj: &Object,
        r_obj: &Object,
        binary: &Binary,
        store: &SarzakExternal,
        referrer: &Referrer,
        domain: &Domain,
    ) -> Result<()> {
        buffer.block(
            DirectiveKind::CommentOrig,
            format!(
                "{}-struct-impl-nav-backward-1_Mc-to-{}",
                obj.as_ident(),
                r_obj.as_ident()
            ),
            |buffer| {
                emit!(
                    buffer,
                    "/// Navigate to [`{}`] across R{}(1-Mc)",
                    r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak()),
                    binary.number
                );
                emit!(
                    buffer,
                    "pub fn {}<'a>(&'a self, store: &'a {}) -> Vec<&{}> {{",
                    r_obj.as_ident(),
                    store.name,
                    r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                );
                emit!(buffer, "store.iter_{}()", r_obj.as_ident());
                emit!(
                    buffer,
                    ".filter_map(|{}| if {}.1.{} == Some(self.id) {{ Some({}.1) }} else {{ None }})",
                    r_obj.as_ident(),
                    r_obj.as_ident(),
                    referrer.referential_attribute.as_ident(),
                    r_obj.as_ident(),
                );
                emit!(buffer, ".collect()");
                emit!(buffer, "}}");

                Ok(())
            },
        )
    }
}

impl MethodImplementation for DomainRelNavImpl {}

impl CodeWriter for DomainRelNavImpl {
    fn write_code(
        &self,
        _options: &GraceCompilerOptions,
        domain: &Domain,
        woog: &mut WoogStore,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by DomainRelNavImpl"
            }
        );
        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();

        // Grab a reference to the store so that we can use it to exhume
        // things.
        let mut iter = domain.sarzak().iter_ty();
        let name = format!(
            "{}Store",
            module.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
        );
        let store = loop {
            let ty = iter.next();
            match ty {
                Some((_, ty)) => match ty {
                    Type::External(e) => {
                        let ext = domain.sarzak().exhume_external(&e).unwrap();
                        if ext.name == name {
                            break ext;
                        }
                    }
                    _ => continue,
                },
                None => panic!("Could not find store type for {}", module),
            }
        };

        // These are relationships that we formalize
        let referrers = get_referrers!(obj, domain.sarzak());
        // These are relationships of which we are the target
        let referents = get_referents!(obj, domain.sarzak());

        for referrer in &referrers {
            let binary = sarzak_get_one_r_bin_across_r6!(referrer, domain.sarzak());
            let referent = sarzak_get_one_r_to_across_r5!(binary, domain.sarzak());
            let r_obj = sarzak_get_one_obj_across_r16!(referent, domain.sarzak());
            let cond = sarzak_get_one_cond_across_r12!(referrer, domain.sarzak());

            // Cardinality does not matter from the referrer, because it's always
            // one. This is because of the normalized, table-nature of the store,
            // and more importantly the method.
            match cond {
                Conditionality::Unconditional(_) => {
                    DomainRelNavImpl::forward(buffer, obj, referrer, binary, store, r_obj, &domain)?
                }
                Conditionality::Conditional(_) => DomainRelNavImpl::forward_conditional(
                    buffer, obj, referrer, binary, store, r_obj, &domain,
                )?,
            }
        }

        for referent in &referents {
            let binary = sarzak_get_one_r_bin_across_r5!(referent, domain.sarzak());
            let referrer = sarzak_get_one_r_from_across_r6!(binary, domain.sarzak());
            let r_obj = sarzak_get_one_obj_across_r17!(referrer, domain.sarzak());
            let my_cond = sarzak_get_one_cond_across_r11!(referent, domain.sarzak());
            let other_cond = sarzak_get_one_cond_across_r12!(referrer, domain.sarzak());
            // The non-formalizing side will only ever be one, unless it's in an associative
            // relationship. We do however need to check the cardinality of the formalizing side.
            let card = sarzak_get_one_card_across_r9!(referrer, domain.sarzak());

            match card {
                Cardinality::One(_) => match my_cond {
                    Conditionality::Unconditional(_) => DomainRelNavImpl::backward_one(
                        buffer, obj, r_obj, binary, store, referrer, &domain,
                    )?,
                    Conditionality::Conditional(_) => match other_cond {
                        Conditionality::Unconditional(_) => {
                            DomainRelNavImpl::backward_one_conditional(
                                buffer, obj, r_obj, binary, store, referrer, &domain,
                            )?
                        }
                        Conditionality::Conditional(_) => {
                            DomainRelNavImpl::backward_one_biconditional(
                                buffer, obj, r_obj, binary, store, referrer, &domain,
                            )?
                        }
                    },
                },
                // It's interesting that there are only really two possibilities, and
                // that neither of them depend on the conditionality of the this side.
                Cardinality::Many(_) => match other_cond {
                    Conditionality::Unconditional(_) => DomainRelNavImpl::backward_1_m(
                        buffer, obj, r_obj, binary, store, referrer, &domain,
                    )?,
                    Conditionality::Conditional(_) => DomainRelNavImpl::backward_1_mc(
                        buffer, obj, r_obj, binary, store, referrer, &domain,
                    )?,
                },
            }
        }

        Ok(())
    }
}

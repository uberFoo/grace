use std::fmt::Write;

use log;
use sarzak::{
    domain::Domain,
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::{
        macros::{
            sarzak_get_many_as_across_r1, sarzak_get_one_obj_across_r16,
            sarzak_get_one_obj_across_r17, sarzak_get_one_r_bin_across_r5,
            sarzak_get_one_r_bin_across_r6, sarzak_get_one_r_from_across_r6,
            sarzak_get_one_r_to_across_r5, sarzak_get_one_t_across_r2,
            sarzak_maybe_get_many_r_froms_across_r17, sarzak_maybe_get_many_r_tos_across_r16,
            sarzak_maybe_get_one_t_ref_across_r27,
        },
        types::{Attribute, Referent, Referrer, Type, UUID},
    },
    woog::{
        store::ObjectStore as WoogStore, Mutability, ObjectMethod, Parameter, Visibility, BORROWED,
        MUTABLE, PUBLIC,
    },
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
    todo::{LValue, RValue},
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
        let mut referrers = sarzak_maybe_get_many_r_tos_across_r16!($obj, $store);
        referrers.sort_by(|a, b| {
            let binary = sarzak_get_one_r_bin_across_r5!(&a, $store);
            let referent = sarzak_get_one_r_from_across_r6!(binary, $store);
            let obj_a = sarzak_get_one_obj_across_r17!(referent, $store);

            let binary = sarzak_get_one_r_bin_across_r5!(&b, $store);
            let referent = sarzak_get_one_r_from_across_r6!(binary, $store);
            let obj_b = sarzak_get_one_obj_across_r17!(referent, $store);

            obj_a.name.cmp(&obj_b.name)
        });
        referrers
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

                    emit!(
                        buffer,
                        "/// R{}: [`{}`] '{}' [`{}`]",
                        binary.number,
                        obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak()),
                        referrer.description,
                        r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                    );
                    emit!(
                        buffer,
                        "pub {}: Uuid,",
                        referrer.referential_attribute.as_ident(),
                    );
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
        let mut fields: Vec<LValue> = Vec::new();
        let mut attrs = sarzak_get_many_as_across_r1!(obj, domain.sarzak());
        attrs.sort_by(|a, b| a.name.cmp(&b.name));
        for attr in attrs {
            // We are going to generate the id, so don't include it in the
            // list of parameters.
            if attr.name != "id" {
                let ty = sarzak_get_one_t_across_r2!(attr, domain.sarzak());
                fields.push(LValue::new(attr.name.as_ident(), &ty));
                params.push(Parameter::new(
                    woog,
                    &Mutability::Borrowed(BORROWED),
                    None,
                    &ty,
                    &Visibility::Public(PUBLIC),
                    attr.as_ident(),
                ));
            }
        }

        // And the referential attributes
        for referrer in &referrers {
            let binary = sarzak_get_one_r_bin_across_r6!(referrer, domain.sarzak());
            let referent = sarzak_get_one_r_to_across_r5!(binary, domain.sarzak());
            let r_obj = sarzak_get_one_obj_across_r16!(referent, domain.sarzak());
            let reference = sarzak_maybe_get_one_t_ref_across_r27!(r_obj, domain.sarzak()).unwrap();

            // This determines how a reference is stored in the struct. In this
            // case a UUID.
            fields.push(LValue::new(
                referrer.referential_attribute.as_ident(),
                &Type::Uuid(UUID),
            ));
            params.push(Parameter::new(
                woog,
                &Mutability::Borrowed(BORROWED),
                None,
                &Type::Reference(reference.id),
                &Visibility::Public(PUBLIC),
                referrer.referential_attribute.as_ident(),
            ));
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
                            break ty;
                        }
                    }
                    _ => continue,
                },
                None => panic!("Could not find store type for {}", module),
            }
        };
        params.push(Parameter::new(
            woog,
            &Mutability::Mutable(MUTABLE),
            None,
            &store_type,
            &Visibility::Public(PUBLIC),
            "store".to_owned(),
        ));

        // Link the params
        let mut iter = params.iter_mut().peekable();
        loop {
            if let Some(param) = iter.next() {
                if let Some(next) = iter.peek() {
                    param.next = Some(next.id);
                    woog.inter_parameter(param.clone());
                }
            } else {
                break;
            }
        }

        // Create an ObjectMethod
        // The uniqueness of this instance depends on the inputs to it's
        // new method. Param can be None, and two methods on the same
        // object will have the same obj. So it comes down to a unique
        // name for each object. So just "new" should suffice for name,
        // because it's scoped by obj already.
        let param = match params.len() {
            0 => None,
            _ => Some(&params[0]),
        };
        // We need to find the type that corresponds to this object
        let mut iter = domain.sarzak().iter_ty();
        let ty = loop {
            if let Some((id, ty)) = iter.next() {
                if id == &obj.id {
                    break Some(ty);
                }
            } else {
                break None;
            }
        };
        let method = ObjectMethod::new(
            woog,
            param,
            obj,
            ty.unwrap(),
            &Visibility::Public(PUBLIC),
            "new".to_owned(),
            "Create a new instance".to_owned(),
        );

        let mut rvals: Vec<RValue> = params.iter().map(|p| p.into()).collect();
        // Remove the store.
        rvals.pop();

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
                let id = LValue::new("id", &Type::Uuid(UUID));
                render_make_uuid(buffer, &id, &rvals, domain.sarzak())?;

                // Output code to create the instance
                let new = LValue::new("new", &Type::Reference(obj.id));
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
            buffer.block(
                DirectiveKind::CommentOrig,
                format!(
                    "{}-struct-impl-navigate-to-{}",
                    obj.as_ident(),
                    referrer.referential_attribute.as_ident()
                ),
                |buffer| {
                    emit!(
                        buffer,
                        "/// Navigate to [`{}`] across R{}(1-1)",
                        r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak()),
                        binary.number,
                    );
                    emit!(
                        buffer,
                        "pub fn {}<'a>(&'a self, store: &'a {}) -> &{} {{",
                        referrer.referential_attribute.as_ident(),
                        store.name,
                        r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                    );
                    emit!(
                        buffer,
                        "store.exhume_{}(&self.{}).unwrap()",
                        r_obj.as_ident(),
                        referrer.referential_attribute.as_ident()
                    );
                    emit!(buffer, "}}");

                    Ok(())
                },
            )?;
        }

        for referent in &referents {
            let binary = sarzak_get_one_r_bin_across_r5!(referent, domain.sarzak());
            let referrer = sarzak_get_one_r_from_across_r6!(binary, domain.sarzak());
            let r_obj = sarzak_get_one_obj_across_r17!(referrer, domain.sarzak());
            buffer.block(
                DirectiveKind::CommentOrig,
                format!(
                    "{}-struct-impl-navigate-backwards-to-{}",
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
                        "pub fn {}<'a>(&'a self, store: &'a {}) -> &{} {{",
                        r_obj.as_ident(),
                        store.name,
                        r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                    );
                    emit!(buffer, "store.iter_{}()", r_obj.as_ident());
                    emit!(
                        buffer,
                        ".find(|{}| {}.1.{} == self.id).unwrap().1",
                        r_obj.as_ident(),
                        r_obj.as_ident(),
                        referrer.referential_attribute.as_ident()
                    );
                    emit!(buffer, "}}");

                    Ok(())
                },
            )?;
        }

        Ok(())
    }
}

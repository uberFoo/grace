//! Domain Struct Handling
//!
//! This is for generating structs that are used as part of a Domain.
use std::fmt::Write;

use log;
use sarzak::{
    domain::Domain,
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::{
        macros::{
            sarzak_get_many_as_across_r1, sarzak_get_one_obj_across_r16,
            sarzak_get_one_r_bin_across_r6, sarzak_get_one_r_to_across_r5,
            sarzak_get_one_t_across_r2, sarzak_maybe_get_many_r_froms_across_r17,
            sarzak_maybe_get_one_t_ref_across_r27,
        },
        types::{Attribute, Referrer, Type, UUID},
    },
    woog::{store::ObjectStore as WoogStore, ObjectMethod, Parameter},
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
    todo::LValue,
    types::{StructDefinition, StructImplementation},
};

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
                description: "obj_id is required by DefaultStructGenerator"
            }
        );
        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();

        // These need to be sorted, as they are output as attributes and we require
        // stable output.
        let mut referrers = sarzak_maybe_get_many_r_froms_across_r17!(obj, domain.sarzak());
        referrers.sort_by(|a, b| {
            let obj_a = domain.sarzak().exhume_object(&a.obj_id).unwrap();
            let obj_b = domain.sarzak().exhume_object(&b.obj_id).unwrap();
            obj_a.name.cmp(&obj_b.name)
        });

        // Everything has an `id`, everything needs these.
        emit!(buffer, "use uuid::Uuid;");
        emit!(buffer, "use crate::{}::UUID_NS;", module);
        emit!(buffer, "");

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-referrer-use-statements", obj.as_ident()),
            |buffer| {
                for referrer in &referrers {
                    let binary = sarzak_get_one_r_bin_across_r6!(referrer, domain.sarzak());
                    let referent = sarzak_get_one_r_to_across_r5!(binary, domain.sarzak());
                    let r_obj = sarzak_get_one_obj_across_r16!(referent, domain.sarzak());

                    emit!(
                        buffer,
                        "use crate::{}::types::{}::{};",
                        module,
                        r_obj.as_ident(),
                        r_obj.as_type(&domain.sarzak())
                    );
                }

                Ok(())
            },
        )?;

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

                emit!(buffer, "pub struct {} {{", obj.as_type(&domain.sarzak()));

                let mut attrs = sarzak_get_many_as_across_r1!(obj, domain.sarzak());
                attrs.sort_by(|a, b| a.name.cmp(&b.name));
                for attr in attrs {
                    let ty = sarzak_get_one_t_across_r2!(attr, domain.sarzak());
                    emit!(
                        buffer,
                        "pub {}: {},",
                        attr.as_ident(),
                        ty.as_type(&domain.sarzak())
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
                        obj.as_type(&domain.sarzak()),
                        referrer.description,
                        r_obj.as_type(&domain.sarzak())
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
    implementation: Option<Box<dyn StructImplementation>>,
}

impl DomainImplBuilder {
    pub(crate) fn new() -> DomainImplBuilder {
        Self {
            implementation: None,
        }
    }

    pub(crate) fn implementation(mut self, implementation: Box<dyn StructImplementation>) -> Self {
        self.implementation = Some(implementation);

        self
    }

    pub(crate) fn build(self) -> Box<dyn StructImplementation> {
        Box::new(DomainImplementation {
            implementation: self.implementation,
        })
    }
}

pub(crate) struct DomainImplementation {
    implementation: Option<Box<dyn StructImplementation>>,
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
                description: "obj_id is required by DefaultStructGenerator"
            }
        );
        let obj_id = obj_id.unwrap();
        let object = domain.sarzak().exhume_object(&obj_id).unwrap();

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-struct-implementation", object.as_ident()),
            |buffer| {
                let obj = domain.sarzak().exhume_object(&obj_id).unwrap();

                emit!(buffer, "impl {} {{", obj.as_type(&domain.sarzak()));

                if let Some(implementation) = &self.implementation {
                    implementation.write_code(
                        options,
                        domain,
                        woog,
                        module,
                        Some(obj_id),
                        buffer,
                    )?;
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
    pub(crate) fn new() -> Box<dyn StructImplementation> {
        Box::new(Self)
    }
}

impl StructImplementation for DomainNewImpl {}

impl CodeWriter for DomainNewImpl {
    fn write_code(
        &self,
        _options: &GraceCompilerOptions,
        domain: &Domain,
        woog: &mut WoogStore,
        _module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by DefaultStructGenerator"
            }
        );
        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();

        // These are more attributes on our object, and they should be sorted.
        let mut referrers = sarzak_maybe_get_many_r_froms_across_r17!(obj, domain.sarzak());
        referrers.sort_by(|a, b| {
            let obj_a = domain.sarzak().exhume_object(&a.obj_id).unwrap();
            let obj_b = domain.sarzak().exhume_object(&b.obj_id).unwrap();
            obj_a.name.cmp(&obj_b.name)
        });

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
                params.push(Parameter::new(woog, None, &ty, attr.as_ident()));
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
                None,
                &Type::Reference(reference.id),
                referrer.referential_attribute.as_ident(),
            ));
        }

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
                    obj.as_type(&domain.sarzak())
                );

                // Output the top of the function definition
                render_method_definition(buffer, &method, woog, domain.sarzak())?;

                // Output the code to create the `id`.
                let id = LValue::new("id", &Type::Uuid(UUID));
                render_make_uuid(buffer, &id, &params, domain.sarzak())?;

                // Output code to create the instance
                let new = LValue::new("new", &Type::Reference(obj.id));
                let rvals = params.iter().map(|p| p.into()).collect();
                render_new_instance(buffer, obj, Some(&new), &fields, &rvals, domain.sarzak())?;

                emit!(buffer, "new");
                emit!(buffer, "}}");

                Ok(())
            },
        )
    }
}

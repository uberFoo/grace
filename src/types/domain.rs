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
        types::{Attribute, Referrer},
    },
    woog::{ObjectMethod, Parameter},
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        generator::CodeWriter,
        render::{RenderIdent, RenderType},
    },
    options::GraceCompilerOptions,
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
        let referrers = sarzak_maybe_get_many_r_froms_across_r17!(obj, domain.sarzak());

        // Everything has an `id`, everything needs these.
        emit!(buffer, "use uuid::Uuid;");
        emit!(buffer, "use crate::{}::UUID_NS;", module);
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

                // This doesn't need to be in it's own block, and it's probably
                // distracting to leave it so. But this is interesting for
                // testing the diff that I'm about to add.
                buffer.block(
                    DirectiveKind::IgnoreOrig,
                    format!("{}-referrer-use-statements", obj.as_ident()),
                    |buffer| {
                        // This is sort of long, and sticks out. Maybe it goes into a function?
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
                            emit!(buffer, "pub {}: Uuid,", referrer.referential_attribute,);
                        }

                        Ok(())
                    },
                )?;

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

impl DomainImplementation {
    pub(crate) fn new() -> Box<dyn StructImplementation> {
        Box::new(Self {
            implementation: None,
        })
    }

    pub(crate) fn implementation(mut self, implementation: Box<dyn StructImplementation>) -> Self {
        self.implementation = Some(implementation);

        self
    }
}

impl StructImplementation for DomainImplementation {}

impl CodeWriter for DomainImplementation {
    fn write_code(
        &self,
        options: &GraceCompilerOptions,
        domain: &Domain,
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
                    implementation.write_code(options, domain, module, Some(obj_id), buffer)?;
                }

                emit!(buffer, "}}");

                Ok(())
            },
        )
    }
}

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
        options: &GraceCompilerOptions,
        domain: &Domain,
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
        let referrers = sarzak_maybe_get_many_r_froms_across_r17!(obj, domain.sarzak());

        let mut woog = sarzak::woog::store::ObjectStore::new();

        buffer.block(
            DirectiveKind::CommentOrig,
            format!("{}-struct-impl-new", obj.as_ident()),
            |buffer| {
                let mut params: Vec<Parameter> = Vec::new();

                // Collect the attributes
                let mut attrs = sarzak_get_many_as_across_r1!(obj, domain.sarzak());
                attrs.sort_by(|a, b| a.name.cmp(&b.name));
                for attr in attrs {
                    // We are going to generate the id, so don't include it in the
                    // list of parameters.
                    if attr.name != "id" {
                        let ty = sarzak_get_one_t_across_r2!(attr, domain.sarzak());
                        // params.push((attr.as_ident(), ty.as_type()));
                        params.push(Parameter::new(&mut woog, None, ty, attr.as_ident()));
                    }
                }

                // And the referential attributes
                for referrer in &referrers {
                    let binary = sarzak_get_one_r_bin_across_r6!(referrer, domain.sarzak());
                    let referent = sarzak_get_one_r_to_across_r5!(binary, domain.sarzak());
                    let r_obj = sarzak_get_one_obj_across_r16!(referent, domain.sarzak());
                    let reference =
                        sarzak_maybe_get_one_t_ref_across_r27!(r_obj, domain.sarzak()).unwrap();

                    // If don't remember why I don't have a macro for this. Going the other
                    // direction is trivial, but this way is trickier.
                    let mut iter = domain.sarzak().iter_ty();
                    let ty = loop {
                        if let Some((id, ty)) = iter.next() {
                            if ty.get_id() == reference.id {
                                break ty;
                            }
                        }
                    };

                    // params.push((referrer.referential_attribute.clone(), "&Uuid".to_owned()));
                    // let reference = Reference::new(&mut domain.sarzak().borrow_mut(), r_obj);
                    // let ty = Type::Reference(reference.id);
                    params.push(Parameter::new(
                        &mut woog,
                        None,
                        &ty,
                        referrer.referential_attribute.as_ident(),
                    ));
                }

                // Link the params, and build a format string while we're at it.
                let mut format_string = String::new();
                let mut iter = params.iter_mut().peekable();
                loop {
                    if let Some(param) = iter.next() {
                        format_string.extend(["{}:"]);
                        if let Some(next) = iter.peek() {
                            param.next = Some(next.id);
                        }
                    } else {
                        break;
                    }
                }
                format_string.pop();

                let foo: Vec<(String, String)> = params
                    .iter()
                    .map(|p| {
                        let ty = domain.sarzak().exhume_ty(&p.ty).unwrap();
                        (ty.as_type(&domain.sarzak()), p.name.as_ident())
                    })
                    .collect();

                emit!(
                    buffer,
                    "/// Inter a new {} in the store, and return it's `id`.",
                    obj.as_type(&domain.sarzak())
                );

                emit!(
                    buffer,
                    "pub fn new({}) -> {} {{",
                    foo.iter()
                        .map(|p| format!("{}: {}", p.0, p.1))
                        .collect::<Vec<String>>()
                        .join(","),
                    obj.as_type(&domain.sarzak())
                );
                emit!(
                    buffer,
                    "let id = Uuid::new_v5(&UUID_NS, format!(\"{}\", {}).as_bytes());",
                    format_string,
                    foo.iter()
                        .map(|p| format!("{}", p.1))
                        .collect::<Vec<String>>()
                        .join(",")
                );
                emit!(buffer, "let new = Self {{");
                emit!(buffer, "id,");
                emit!(
                    buffer,
                    "{}",
                    foo.iter()
                        .map(|p| format!("{}", p.1))
                        .collect::<Vec<String>>()
                        .join(",")
                );
                emit!(buffer, "}};");
                emit!(buffer, "new");
                emit!(buffer, "}}");

                Ok(())
            },
        )
    }
}

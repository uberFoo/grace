//! Default Struct Handling
//!
//! This is the place to find all the default implementations for generating structs.
//! These are meant to be used in an application domain.
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
        types::{Attribute, Object, Referrer, Type, UUID},
    },
    woog::{store::ObjectStore as WoogStore, ObjectMethod, Parameter},
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        generator::{CodeWriter, FileGenerator},
        render::{RenderIdent, RenderType},
        render_make_uuid, render_method_definition, render_new_instance,
    },
    options::GraceCompilerOptions,
    todo::LValue,
    types::{ModuleDefinition, StructDefinition, StructImplementation},
};

pub(crate) struct DefaultStructBuilder {
    definition: Option<Box<dyn StructDefinition>>,
    implementation: Option<Box<dyn StructImplementation>>,
}

impl DefaultStructBuilder {
    pub(crate) fn new() -> Self {
        DefaultStructBuilder {
            definition: None,
            implementation: None,
        }
    }

    pub(crate) fn definition(mut self, definition: Box<dyn StructDefinition>) -> Self {
        self.definition = Some(definition);

        self
    }

    pub(crate) fn implementation(mut self, implementation: Box<dyn StructImplementation>) -> Self {
        self.implementation = Some(implementation);

        self
    }

    pub(crate) fn build(self) -> Result<Box<DefaultStructGenerator>> {
        ensure!(
            self.definition.is_some(),
            CompilerSnafu {
                description: "missing StructDefinition"
            }
        );

        Ok(Box::new(DefaultStructGenerator {
            definition: self.definition.unwrap(),
            implementation: self.implementation,
        }))
    }
}

/// Generator -- Code Generator Engine
///
/// This is supposed to be general, but it's very much geared towards generating
/// a file that contains a struct definition and implementations. I need to
/// do some refactoring.
///
/// As just hinted at, the idea is that you plug in different code writers that
/// know how to write different parts of some rust code. This one is for
/// structs.
pub(crate) struct DefaultStructGenerator {
    definition: Box<dyn StructDefinition>,
    implementation: Option<Box<dyn StructImplementation>>,
}

impl FileGenerator for DefaultStructGenerator {
    fn generate(
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
            DirectiveKind::AllowEditing,
            format!("{}-struct-definition-file", object.as_ident()),
            |buffer| {
                // It's important that we maintain ordering for code injection and
                // redaction. We begin with the struct definition.
                self.definition
                    .write_code(options, domain, woog, module, Some(obj_id), buffer)?;

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

                Ok(())
            },
        )?;

        Ok(())
    }
}

/// Default Struct Generator / CodeWriter
///
/// We need a builder for this so that we can add privacy modifiers, as
/// well as derives.
pub(crate) struct DefaultStruct;

impl DefaultStruct {
    pub(crate) fn new() -> Box<dyn StructDefinition> {
        Box::new(Self)
    }
}

impl StructDefinition for DefaultStruct {}

impl CodeWriter for DefaultStruct {
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

        let mut referrers = sarzak_maybe_get_many_r_froms_across_r17!(obj, domain.sarzak());
        referrers.sort_by(|a, b| {
            let obj_a = domain.sarzak().exhume_object(&a.obj_id).unwrap();
            let obj_b = domain.sarzak().exhume_object(&b.obj_id).unwrap();
            obj_a.name.cmp(&obj_b.name)
        });

        let has_referential_attrs = referrers.len() > 0;

        // Everything has an `id`, everything needs these.
        emit!(buffer, "use uuid::Uuid;");
        emit!(buffer, "use crate::{}::UUID_NS;", module);
        emit!(buffer, "");

        let mut paste = Buffer::new();
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

                    emit!(
                        paste,
                        "/// R{}: [`{}`] '{}' [`{}`]",
                        binary.number,
                        obj.as_type(&domain.sarzak()),
                        referrer.description,
                        r_obj.as_type(&domain.sarzak())
                    );
                    emit!(
                        paste,
                        "pub {}: &'a {},",
                        referrer.referential_attribute.as_ident(),
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

                if has_referential_attrs {
                    // Lifetime parameters. Really, we should assign one for each attribute. TBD.
                    emit!(
                        buffer,
                        "pub struct {}<'a> {{",
                        obj.as_type(&domain.sarzak())
                    );
                } else {
                    emit!(buffer, "pub struct {} {{", obj.as_type(&domain.sarzak()));
                }

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

                // Paste in the referential attributes, computed above.
                *buffer += paste;

                emit!(buffer, "}}");
                Ok(())
            },
        )?;

        Ok(())
    }
}

pub(crate) struct DefaultImplBuilder {
    implementation: Option<Box<dyn StructImplementation>>,
}

impl DefaultImplBuilder {
    pub(crate) fn new() -> DefaultImplBuilder {
        Self {
            implementation: None,
        }
    }

    pub(crate) fn implementation(mut self, implementation: Box<dyn StructImplementation>) -> Self {
        self.implementation = Some(implementation);

        self
    }

    pub(crate) fn build(self) -> Box<dyn StructImplementation> {
        Box::new(DefaultImplementation {
            implementation: self.implementation,
        })
    }
}

pub(crate) struct DefaultImplementation {
    implementation: Option<Box<dyn StructImplementation>>,
}

impl StructImplementation for DefaultImplementation {}

impl CodeWriter for DefaultImplementation {
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

                let referrers = sarzak_maybe_get_many_r_froms_across_r17!(obj, domain.sarzak());
                let has_referential_attrs = referrers.len() > 0;

                if has_referential_attrs {
                    emit!(buffer, "impl<'a> {}<'a> {{", obj.as_type(&domain.sarzak()));
                } else {
                    emit!(buffer, "impl {} {{", obj.as_type(&domain.sarzak()));
                }

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

/// Default New Implementation
///
/// This generates a new implementation for the object. The new implementation
/// calculates the object's `id` based on the string representation of it's
/// attributes.
///
/// __NB__ --- this implies that the lexicographical sum of it's attributes,
/// across all instances, must be unique.
///
/// I think that I may add optional references to the non-formalizing side of
/// relationships.
pub(crate) struct DefaultNewImpl;

impl DefaultNewImpl {
    pub(crate) fn new() -> Box<dyn StructImplementation> {
        Box::new(Self)
    }
}

impl StructImplementation for DefaultNewImpl {}

impl CodeWriter for DefaultNewImpl {
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
            // case a reference.
            fields.push(LValue::new(
                referrer.referential_attribute.as_ident(),
                &Type::Reference(reference.id),
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

pub(crate) struct DefaultModuleBuilder {
    definition: Option<Box<dyn ModuleDefinition>>,
}

impl DefaultModuleBuilder {
    pub(crate) fn new() -> Self {
        DefaultModuleBuilder { definition: None }
    }

    pub(crate) fn definition(mut self, definition: Box<dyn ModuleDefinition>) -> Self {
        self.definition = Some(definition);

        self
    }

    pub(crate) fn build(self) -> Result<Box<DefaultModuleGenerator>> {
        ensure!(
            self.definition.is_some(),
            CompilerSnafu {
                description: "missing ModuleDefinition"
            }
        );

        Ok(Box::new(DefaultModuleGenerator {
            definition: self.definition.unwrap(),
        }))
    }
}

/// Generator -- Code Generator Engine
///
/// This is supposed to be general, but it's very much geared towards generating
/// a file that contains a struct definition and implementations. I need to
/// do some refactoring.
///
/// As just hinted at, the idea is that you plug in different code writers that
/// know how to write different parts of some rust code. This one is for
/// structs.
pub(crate) struct DefaultModuleGenerator {
    definition: Box<dyn ModuleDefinition>,
}

impl FileGenerator for DefaultModuleGenerator {
    fn generate(
        &self,
        options: &GraceCompilerOptions,
        domain: &Domain,
        woog: &mut WoogStore,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        // Output the domain/module documentation/description
        emit!(buffer, "//! {}", domain.description());

        buffer.block(
            DirectiveKind::AllowEditing,
            format!("{}-module-definition-file", module),
            |buffer| {
                // It's important that we maintain ordering for code injection and
                // redaction. We begin with the struct definition.
                self.definition
                    .write_code(options, domain, woog, module, obj_id, buffer)?;

                Ok(())
            },
        )?;

        Ok(())
    }
}

/// Default Types Module Generator / CodeWriter
///
/// This generates a rust file that imports the generated type implementations.
pub(crate) struct DefaultModule;

impl DefaultModule {
    pub(crate) fn new() -> Box<dyn ModuleDefinition> {
        Box::new(Self)
    }
}

impl ModuleDefinition for DefaultModule {}

impl CodeWriter for DefaultModule {
    fn write_code(
        &self,
        _options: &GraceCompilerOptions,
        domain: &Domain,
        _woog: &mut WoogStore,
        module: &str,
        _obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-module-definition", module),
            |buffer| {
                let mut objects: Vec<(&Uuid, &Object)> = domain.sarzak().iter_object().collect();
                objects.sort_by(|a, b| a.1.name.cmp(&b.1.name));
                for (_, obj) in &objects {
                    emit!(buffer, "pub mod {};", obj.as_ident());
                }

                Ok(())
            },
        )?;

        Ok(())
    }
}

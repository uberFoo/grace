//! Default Struct Handling
//!
//! This is the place to find all the default implementations for generating structs.
//! These are meant to be used in an application domain.
use std::{collections::HashMap, fmt::Write};

use log;
use sarzak::{
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::{
        macros::{
            sarzak_get_many_as_across_r1, sarzak_get_one_obj_across_r16,
            sarzak_get_one_r_bin_across_r6, sarzak_get_one_r_to_across_r5,
            sarzak_get_one_t_across_r2, sarzak_maybe_get_many_r_froms_across_r17,
        },
        types::{Attribute, Object, Referrer},
    },
    v1::domain::Domain,
    woog::{store::ObjectStore as WoogStore, Mutability, BORROWED, PUBLIC},
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        emit_object_comments,
        generator::{CodeWriter, FileGenerator, GenerationAction},
        object_is_singleton, object_is_supertype,
        render::{RenderConst, RenderIdent, RenderType},
        render_make_uuid, render_method_definition, render_new_instance,
    },
    options::GraceConfig,
    todo::{GType, LValue, ObjectMethod, Parameter, RValue},
    types::{ModuleDefinition, TypeDefinition, TypeImplementation},
};

pub(crate) struct DefaultStructBuilder {
    definition: Option<Box<dyn TypeDefinition>>,
    implementations: Vec<Box<dyn TypeImplementation>>,
}

impl DefaultStructBuilder {
    pub(crate) fn new() -> Self {
        DefaultStructBuilder {
            definition: None,
            implementations: Vec::new(),
        }
    }

    pub(crate) fn definition(mut self, definition: Box<dyn TypeDefinition>) -> Self {
        self.definition = Some(definition);

        self
    }

    pub(crate) fn implementation(mut self, implementation: Box<dyn TypeImplementation>) -> Self {
        self.implementations.push(implementation);

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
            implementations: self.implementations,
        }))
    }
}

/// Default Struct Generator
///
/// Called by the [`Generator`] to write code for a struct.
pub(crate) struct DefaultStructGenerator {
    definition: Box<dyn TypeDefinition>,
    implementations: Vec<Box<dyn TypeImplementation>>,
}

impl FileGenerator for DefaultStructGenerator {
    fn generate(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        woog: &Option<&mut WoogStore>,
        imports: &Option<&HashMap<String, Domain>>,
        package: &str,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<GenerationAction> {
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
                self.definition.write_code(
                    config,
                    domain,
                    woog,
                    imports,
                    package,
                    module,
                    Some(obj_id),
                    buffer,
                )?;

                for implementation in &self.implementations {
                    implementation.write_code(
                        config,
                        domain,
                        woog,
                        imports,
                        package,
                        module,
                        Some(obj_id),
                        buffer,
                    )?;
                }

                Ok(())
            },
        )?;

        Ok(GenerationAction::Write)
    }
}

/// Default Struct Generator / CodeWriter
///
/// We need a builder for this so that we can add privacy modifiers, as
/// well as derives.
pub(crate) struct DefaultStruct;

impl DefaultStruct {
    pub(crate) fn new() -> Box<dyn TypeDefinition> {
        Box::new(Self)
    }
}

impl TypeDefinition for DefaultStruct {}

impl CodeWriter for DefaultStruct {
    fn write_code(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        _woog: &Option<&mut WoogStore>,
        _imports: &Option<&HashMap<String, Domain>>,
        package: &str,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by DefaultStruct"
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
                        r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                    );

                    emit!(
                        paste,
                        "/// R{}: [`{}`] '{}' [`{}`]",
                        binary.number,
                        obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak()),
                        referrer.description,
                        r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                    );
                    emit!(
                        paste,
                        "pub {}: &'a {},",
                        referrer.referential_attribute.as_ident(),
                        r_obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                    );
                }

                Ok(())
            },
        )?;

        log::debug!("writing Struct Definition for {}", obj.name);

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-struct-documentation", obj.as_ident()),
            |buffer| emit_object_comments(obj.description.as_str(), "///", buffer),
        )?;

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-struct-definition", obj.as_ident()),
            |buffer| {
                if let Some(derives) = config.get_derives(&obj.id) {
                    write!(buffer, "#[derive(").context(FormatSnafu)?;
                    for d in derives {
                        write!(buffer, "{},", d).context(FormatSnafu)?;
                    }
                    emit!(buffer, ")]");
                }

                if has_referential_attrs {
                    // Lifetime parameters. Really, we should assign one for each attribute. TBD.
                    emit!(
                        buffer,
                        "pub struct {}<'a> {{",
                        obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                    );
                } else {
                    emit!(
                        buffer,
                        "pub struct {} {{",
                        obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                    );
                }

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
    implementation: Option<Box<dyn TypeImplementation>>,
}

impl DefaultImplBuilder {
    pub(crate) fn new() -> DefaultImplBuilder {
        Self {
            implementation: None,
        }
    }

    pub(crate) fn implementation(mut self, implementation: Box<dyn TypeImplementation>) -> Self {
        self.implementation = Some(implementation);

        self
    }

    pub(crate) fn build(self) -> Box<dyn TypeImplementation> {
        Box::new(DefaultImplementation {
            implementation: self.implementation,
        })
    }
}

pub(crate) struct DefaultImplementation {
    implementation: Option<Box<dyn TypeImplementation>>,
}

impl TypeImplementation for DefaultImplementation {}

impl CodeWriter for DefaultImplementation {
    fn write_code(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        woog: &Option<&mut WoogStore>,
        imports: &Option<&HashMap<String, Domain>>,
        package: &str,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by DefaultImplementation"
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
                    emit!(
                        buffer,
                        "impl<'a> {}<'a> {{",
                        obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                    );
                } else {
                    emit!(
                        buffer,
                        "impl {} {{",
                        obj.as_type(&Mutability::Borrowed(BORROWED), &domain.sarzak())
                    );
                }

                if let Some(implementation) = &self.implementation {
                    implementation.write_code(
                        config,
                        domain,
                        woog,
                        imports,
                        package,
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
pub(crate) struct DefaultNewImpl {
    generate_tests: bool,
}

impl DefaultNewImpl {
    pub(crate) fn new(generate_tests: bool) -> Box<dyn StructImplementation> {
        Box::new(Self { generate_tests })
    }
}

impl TypeImplementation for DefaultStructNewImpl {}

impl CodeWriter for DefaultStructNewImpl {
    fn write_code(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        woog: &Option<&mut WoogStore>,
        _imports: &Option<&HashMap<String, Domain>>,
        package: &str,
        _module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by DefaultNewImpl"
            }
        );
        ensure!(
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by DefaultNewImpl"
            }
        );
        let woog = match woog {
            Some(ref woog) => woog,
            _ => unreachable!(),
        };
        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();

        let mut referrers = sarzak_maybe_get_many_r_froms_across_r17!(obj, domain.sarzak());
        referrers.sort_by(|a, b| {
            let obj_a = domain.sarzak().exhume_object(&a.obj_id).unwrap();
            let obj_b = domain.sarzak().exhume_object(&b.obj_id).unwrap();
            obj_a.name.cmp(&obj_b.name)
        });

        // Collect the attributes
        let mut rvals: Vec<RValue> = Vec::new();
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
                rvals.push(RValue::new(attr.as_ident(), &ty));
            }
        }

        // And the referential attributes
        for referrer in &referrers {
            let binary = sarzak_get_one_r_bin_across_r6!(referrer, domain.sarzak());
            let referent = sarzak_get_one_r_to_across_r5!(binary, domain.sarzak());
            let r_obj = sarzak_get_one_obj_across_r16!(referent, domain.sarzak());

            // This determines how a reference is stored in the struct. In this
            // case a reference.
            fields.push(LValue::new(
                referrer.referential_attribute.as_ident(),
                GType::Reference(r_obj.id),
            ));
            params.push(Parameter::new(
                BORROWED,
                None,
                GType::Reference(r_obj.id),
                PUBLIC,
                referrer.referential_attribute.as_ident(),
            ));

            rvals.push(RValue::new(
                referrer.referential_attribute.as_ident(),
                &Type::Reference(reference.id),
            ));
        }

        // Find the method
        // This is going to suck. We don't have cross-domain relationship
        // navigation -- somethinig to be addressed.
        let mut iter = woog.iter_object_method();
        let method = loop {
            match iter.next() {
                Some((_, method)) => {
                    if method.object == obj.id && method.name == "new" {
                        break method;
                    }
                }
                None => {
                    panic!("Unable to find the new method for {}", obj.name);
                }
            }
        };

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
                render_new_instance(
                    buffer,
                    obj,
                    Some(&new),
                    &fields,
                    &rvals,
                    domain.sarzak(),
                    None,
                    &config,
                )?;

                // 🚧 Below is the new code. I'm not sure why the third parameter is None.
                // let rvals = params.iter().map(|p| p.into()).collect();
                // render_new_instance(buffer, obj, None, &fields, &rvals, domain.sarzak())?;

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
        config: &GraceConfig,
        domain: &Domain,
        woog: &Option<&mut WoogStore>,
        imports: &Option<&HashMap<String, Domain>>,
        package: &str,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<GenerationAction> {
        // Output the domain/module documentation/description
        for line in domain.description().lines() {
            emit!(buffer, "//! {}", line);
        }

        buffer.block(
            DirectiveKind::AllowEditing,
            format!("{}-module-definition-file", module),
            |buffer| {
                self.definition
                    .write_code(config, domain, woog, imports, module, obj_id, buffer)?;
                    .write_code(config, domain, woog, package, module, obj_id, buffer)?;

                Ok(())
            },
        )?;

        Ok(GenerationAction::Write)
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
        config: &GraceConfig,
        domain: &Domain,
        _woog: &Option<&mut WoogStore>,
        _imports: &Option<&HashMap<String, Domain>>,
        package: &str,
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
                let objects = objects
                    .iter()
                    .filter(|(id, _)| {
                        // Don't include imported objects
                        !config.is_imported(*id)
                    })
                    .collect::<Vec<_>>();

                for (_, obj) in &objects {
                    emit!(buffer, "pub mod {};", obj.as_ident());
                }
                emit!(buffer, "");
                for (_, obj) in &objects {
                    if object_is_singleton(obj, domain.sarzak())
                        && !object_is_supertype(obj, domain.sarzak())
                    {
                        emit!(
                            buffer,
                            "pub use crate::{}::{}::{};",
                            module,
                            obj.as_ident(),
                            obj.as_const()
                        );
                    } else {
                        emit!(
                            buffer,
                            "pub use crate::{}::{}::{};",
                            module,
                            obj.as_ident(),
                            obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                        );
                    }
                }

                Ok(())
            },
        )?;

        Ok(())
    }
}

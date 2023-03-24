//! Default Struct Handling
//!
//! This is the place to find all the default implementations for generating structs.
//! These are meant to be used in an application domain.
use std::fmt::Write;

use fnv::FnvHashMap as HashMap;
use log;
use sarzak::{
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::types::Object,
    v2::domain::Domain,
    woog::{store::ObjectStore as WoogStore, Ownership, BORROWED, PUBLIC},
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        emit_object_comments,
        generator::{CodeWriter, FileGenerator, GenerationAction},
        get_binary_referrers_sorted, object_is_hybrid, object_is_singleton, object_is_supertype,
        render::{render_attributes, RenderConst, RenderIdent, RenderType},
        render_make_uuid, render_method_definition, render_new_instance,
    },
    options::GraceConfig,
    todo::{GType, LValue, ObjectMethod, Parameter, RValue},
    types::{
        MethodImplementation, ModuleDefinition, TypeDefinition, TypeImplementation, TypeImports,
    },
};

pub(crate) struct DefaultStructBuilder {
    imports: Option<Box<dyn TypeImports>>,
    definition: Option<Box<dyn TypeDefinition>>,
    implementations: Vec<Box<dyn TypeImplementation>>,
}

impl DefaultStructBuilder {
    pub(crate) fn new() -> Self {
        DefaultStructBuilder {
            imports: None,
            definition: None,
            implementations: Vec::new(),
        }
    }

    pub(crate) fn imports(mut self, imports: Box<dyn TypeImports>) -> Self {
        self.imports = Some(imports);

        self
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
        Ok(Box::new(DefaultStructGenerator {
            imports: self.imports,
            definition: self.definition,
            implementations: self.implementations,
        }))
    }
}

/// Default Struct Generator
///
/// Called by the [`Generator`] to write code for a struct.
pub(crate) struct DefaultStructGenerator {
    imports: Option<Box<dyn TypeImports>>,
    definition: Option<Box<dyn TypeDefinition>>,
    implementations: Vec<Box<dyn TypeImplementation>>,
}

impl FileGenerator for DefaultStructGenerator {
    fn generate(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        woog: &Option<&mut WoogStore>,
        imports_map: &Option<&HashMap<String, Domain>>,
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
                if let Some(imports) = self.imports.as_ref() {
                    imports.write_code(
                        config,
                        domain,
                        woog,
                        imports_map,
                        package,
                        module,
                        Some(obj_id),
                        buffer,
                    )?;
                }

                if let Some(definition) = self.definition.as_ref() {
                    definition.write_code(
                        config,
                        domain,
                        woog,
                        imports_map,
                        package,
                        module,
                        Some(obj_id),
                        buffer,
                    )?;
                }

                for implementation in &self.implementations {
                    implementation.write_code(
                        config,
                        domain,
                        woog,
                        imports_map,
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
        woog: &Option<&mut WoogStore>,
        _imports: &Option<&HashMap<String, Domain>>,
        _package: &str,
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
        ensure!(
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by DefaultStruct"
            }
        );
        let woog = woog.as_ref().unwrap();

        let obj = domain.sarzak().exhume_object(obj_id).unwrap();

        let referrers = get_binary_referrers_sorted!(obj, domain.sarzak());
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
                    let binary = referrer.r6_binary(domain.sarzak())[0];
                    let referent = binary.r5_referent(domain.sarzak())[0];
                    let r_obj = referent.r16_object(domain.sarzak())[0];

                    emit!(
                        buffer,
                        "use crate::{}::types::{}::{};",
                        module,
                        r_obj.as_ident(),
                        r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    );

                    emit!(
                        paste,
                        "/// R{}: [`{}`] '{}' [`{}`]",
                        binary.number,
                        obj.as_type(&Ownership::new_borrowed(), woog, domain),
                        referrer.description,
                        r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    );
                    emit!(
                        paste,
                        "pub {}: &'a {},",
                        referrer.referential_attribute.as_ident(),
                        r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
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
                        obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    );
                } else {
                    emit!(
                        buffer,
                        "pub struct {} {{",
                        obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    );
                }

                render_attributes(buffer, obj, woog, domain)?;

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
    methods: Vec<Box<dyn MethodImplementation>>,
}

impl DefaultImplBuilder {
    pub(crate) fn new() -> DefaultImplBuilder {
        Self {
            methods: Vec::new(),
        }
    }

    pub(crate) fn method(mut self, method: Box<dyn MethodImplementation>) -> Self {
        self.methods.push(method);

        self
    }

    pub(crate) fn build(self) -> Box<dyn TypeImplementation> {
        Box::new(DefaultImplementation {
            methods: self.methods,
        })
    }
}

pub(crate) struct DefaultImplementation {
    methods: Vec<Box<dyn MethodImplementation>>,
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
        ensure!(
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by DefaultImplementation"
            }
        );
        let local_woog = woog.as_ref().unwrap();

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-struct-implementation", object.as_ident()),
            |buffer| {
                let obj = domain.sarzak().exhume_object(&obj_id).unwrap();

                let referrers = obj.r17_referrer(domain.sarzak());
                let has_referential_attrs = referrers.len() > 0;

                if has_referential_attrs {
                    emit!(
                        buffer,
                        "impl<'a> {}<'a> {{",
                        obj.as_type(&Ownership::new_borrowed(), local_woog, domain)
                    );
                } else {
                    emit!(
                        buffer,
                        "impl {} {{",
                        obj.as_type(&Ownership::new_borrowed(), local_woog, domain)
                    );
                }

                for method in &self.methods {
                    method.write_code(
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
pub(crate) struct DefaultNewImpl;

impl DefaultNewImpl {
    pub(crate) fn new() -> Box<dyn MethodImplementation> {
        Box::new(Self)
    }
}

impl MethodImplementation for DefaultNewImpl {}

impl CodeWriter for DefaultNewImpl {
    fn write_code(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        woog: &Option<&mut WoogStore>,
        _imports: &Option<&HashMap<String, Domain>>,
        _package: &str,
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

        let referrers = get_binary_referrers_sorted!(obj, domain.sarzak());

        // Collect the attributes
        let mut params: Vec<Parameter> = Vec::new();
        let mut rvals: Vec<RValue> = Vec::new();
        let mut fields: Vec<LValue> = Vec::new();

        let mut attrs = obj.r1_attribute(domain.sarzak());
        attrs.sort_by(|a, b| a.name.cmp(&b.name));
        for attr in attrs {
            // We are going to generate the id, so don't include it in the
            // list of parameters.
            if attr.name != "id" {
                let ty = attr.r2_ty(domain.sarzak())[0];
                fields.push(LValue::new(attr.name.as_ident(), ty.into()));
                params.push(Parameter::new(
                    BORROWED,
                    None,
                    ty.into(),
                    PUBLIC,
                    attr.as_ident(),
                ));
                rvals.push(RValue::new(attr.as_ident(), ty.into()));
            }
        }

        // And the referential attributes
        for referrer in &referrers {
            let binary = referrer.r6_binary(domain.sarzak())[0];
            let referent = binary.r5_referent(domain.sarzak())[0];
            let r_obj = referent.r16_object(domain.sarzak())[0];

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

            // 🚧
            rvals.push(RValue::new(
                referrer.referential_attribute.as_ident(),
                GType::Reference(r_obj.id),
            ));
        }

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

        // Find the method
        // This is going to suck. We don't have cross-domain relationship
        // navigation -- something to be addressed.
        // let mut iter = woog.iter_object_method();
        // let method = loop {
        // match iter.next() {
        // Some((_, method)) => {
        // if method.object == obj.id && method.name == "new" {
        // break method;
        // }
        // }
        // None => {
        // panic!("Unable to find the new method for {}", obj.name);
        // }
        // }
        // };

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
                    obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );

                // Output the top of the function definition
                render_method_definition(buffer, &method, woog, domain)?;

                // Output the code to create the `id`.
                let id = LValue::new("id", GType::Uuid);
                render_make_uuid(buffer, &id, &rvals, domain)?;

                // Output code to create the instance
                render_new_instance(buffer, obj, None, &fields, &rvals, config, woog, domain)?;

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
                self.definition.write_code(
                    config, domain, woog, imports, package, module, obj_id, buffer,
                )?;

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
        woog: &Option<&mut WoogStore>,
        imports: &Option<&HashMap<String, Domain>>,
        _package: &str,
        module: &str,
        _obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by DefaultModule"
            }
        );
        let woog = woog.as_ref().unwrap();

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-module-definition", module),
            |buffer| {
                let mut objects: Vec<&Object> = domain.sarzak().iter_object().collect();
                objects.sort_by(|a, b| a.name.cmp(&b.name));
                let objects = objects
                    .iter()
                    .filter(|obj| {
                        // Don't include imported objects
                        !config.is_imported(&obj.id)
                    })
                    .collect::<Vec<_>>();

                for obj in &objects {
                    emit!(buffer, "pub mod {};", obj.as_ident());
                }
                emit!(buffer, "");
                for obj in &objects {
                    if object_is_singleton(obj, config, imports, domain)?
                        && !object_is_supertype(obj, config, imports, domain)?
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
                            obj.as_type(&Ownership::new_borrowed(), woog, domain)
                        );
                        if object_is_hybrid(obj, config, imports, domain)? {
                            emit!(
                                buffer,
                                "pub use crate::{}::{}::{}Enum;",
                                module,
                                obj.as_ident(),
                                obj.as_type(&Ownership::new_borrowed(), woog, domain)
                            );
                        }
                    }
                }

                Ok(())
            },
        )?;

        Ok(())
    }
}

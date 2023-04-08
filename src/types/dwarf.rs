//! Dwarf File Generation
//!
//! This is where we generate code for use in the next stage of the compiler.
use std::fmt::Write;

use fnv::FnvHashMap as HashMap;
use log;
use sarzak::{
    lu_dog::types::ValueType,
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::types::{Object, Ty},
    v2::domain::Domain,
    woog::{store::ObjectStore as WoogStore, Ownership, BORROWED, PUBLIC},
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        collect_attributes,
        diff_engine::DirectiveKind,
        emit_object_comments,
        generator::{CodeWriter, FileGenerator, GenerationAction},
        get_binary_referrers_sorted, object_is_hybrid, object_is_singleton, object_is_supertype,
        render::{render_attributes, RenderConst, RenderIdent, RenderType},
        render_make_uuid, render_method_definition, render_new_instance, AttributeBuilder,
    },
    options::GraceConfig,
    todo::{GType, LValue, ObjectMethod, Parameter, RValue},
    types::{
        DwarfDefinition, MethodImplementation, TypeDefinition, TypeImplementation, TypeImports,
    },
};

pub(crate) struct DwarfBuilder {
    definition: Option<Box<dyn DwarfDefinition>>,
}

impl DwarfBuilder {
    pub(crate) fn new() -> Self {
        DwarfBuilder { definition: None }
    }

    pub(crate) fn definition(mut self, definition: Box<dyn DwarfDefinition>) -> Self {
        self.definition = Some(definition);

        self
    }

    pub(crate) fn build(self) -> Result<Box<DwarfGenerator>> {
        ensure!(
            self.definition.is_some(),
            CompilerSnafu {
                description: "missing DwarfDefinition"
            }
        );

        Ok(Box::new(DwarfGenerator {
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
pub(crate) struct DwarfGenerator {
    definition: Box<dyn DwarfDefinition>,
}

impl FileGenerator for DwarfGenerator {
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
            emit!(buffer, "// {}", line);
        }

        buffer.block(
            DirectiveKind::AllowEditing,
            format!("{}-dwarf-file", module),
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
pub(crate) struct DwarfModule;

impl DwarfModule {
    pub(crate) fn new() -> Box<dyn DwarfDefinition> {
        Box::new(Self)
    }
}

impl DwarfDefinition for DwarfModule {}

impl CodeWriter for DwarfModule {
    fn write_code(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        woog: &Option<&mut WoogStore>,
        _imports: &Option<&HashMap<String, Domain>>,
        _package: &str,
        module: &str,
        _obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by DwarfModule"
            }
        );
        let woog = woog.as_ref().unwrap();

        struct Attribute {
            pub name: String,
            pub ty: ValueType,
        }

        impl AttributeBuilder<Attribute> for Attribute {
            fn new(name: String, ty: ValueType) -> Self {
                Attribute { name, ty }
            }
        }

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
                    emit!(
                        buffer,
                        "type {} {{",
                        obj.as_type(&Ownership::new_owned(), woog, domain)
                    );

                    let attrs: Vec<Attribute> = collect_attributes(obj, domain);
                    for attr in attrs {
                        let ty = match attr.ty {
                            ValueType::Ty(ref id) => {
                                let ty = domain.sarzak().exhume_ty(id).unwrap();
                                match ty {
                                    Ty::Object(ref id) => {
                                        let obj = domain.sarzak().exhume_object(id).unwrap();
                                        obj.as_ident()
                                    }
                                    Ty::String(_) => "string".to_string(),
                                    Ty::Boolean(_) => "bool".to_string(),
                                    Ty::Integer(_) => "int".to_string(),
                                    Ty::Float(_) => "float".to_string(),
                                    Ty::Uuid(_) => "uuid".to_string(),
                                    Ty::External(_) => "ext_what_to_do".to_string(),
                                }
                            }
                            ValueType::WoogOption(_) => todo!(),
                        };
                        emit!(buffer, "    {}: {},", attr.name, ty);
                    }

                    emit!(buffer, "}}");
                }

                Ok(())
            },
        )?;

        Ok(())
    }
}

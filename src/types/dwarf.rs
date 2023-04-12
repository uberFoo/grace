//! Dwarf File Generation
//!
//! This is where we generate code for use in the next stage of the compiler.
use std::{fmt::Write, sync::RwLock};

use fnv::{FnvHashMap as HashMap, FnvHashSet as HashSet};
use sarzak::{
    lu_dog::store::ObjectStore as LuDogStore,
    lu_dog::types::ValueType,
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::types::{Object, Ty},
    v2::domain::Domain,
    woog::{store::ObjectStore as WoogStore, Ownership},
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
        render::RenderType,
        AttributeBuilder,
    },
    options::GraceConfig,
    types::DwarfDefinition,
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
        lu_dog: &Option<&RwLock<LuDogStore>>,
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
                    config, domain, woog, lu_dog, imports, package, module, obj_id, buffer,
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
        lu_dog: &Option<&RwLock<LuDogStore>>,
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

        ensure!(
            lu_dog.is_some(),
            CompilerSnafu {
                description: "lu_dog is required by DwarfModule"
            }
        );
        let lu_dog = lu_dog.as_ref().unwrap();

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
            format!("{}-dwarf-output", module),
            |buffer| {
                // Add an import statement for each imported domain
                let mut imports = HashSet::default();
                for imported in domain
                    .sarzak()
                    .iter_object()
                    .filter(|obj| config.is_imported(&obj.id))
                {
                    let imported_object = config.get_imported(&imported.id).unwrap();
                    imports.insert(imported_object.domain.as_str());
                }

                for import in imports {
                    emit!(buffer, "import {};", import);
                }

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
                    //
                    // Emit the type definition
                    //
                    emit_object_comments(&obj.description, "//", buffer);
                    emit!(
                        buffer,
                        "struct {} {{",
                        obj.as_type(&Ownership::new_owned(), woog, domain)
                    );

                    let attrs: Vec<Attribute> = collect_attributes(obj, &lu_dog, domain);
                    for attr in &attrs {
                        let ty = value_type_to_string(&attr.ty, lu_dog, woog, domain);
                        emit!(buffer, "    {}: {},", attr.name, ty);
                    }

                    emit!(buffer, "}}");

                    //
                    // Emit the impl block
                    //
                    emit!(
                        buffer,
                        "impl {} {{",
                        obj.as_type(&Ownership::new_owned(), woog, domain)
                    );

                    //
                    // Emit the constructor
                    //
                    write!(buffer, "    fn new(").context(FormatSnafu)?;
                    let mut ft = true;
                    let mut iter = attrs.iter();
                    loop {
                        match iter.next() {
                            Some(attr) => {
                                if attr.name == "id" {
                                    continue;
                                }
                                if !ft {
                                    write!(buffer, ", ").context(FormatSnafu)?;
                                } else {
                                    ft = false;
                                }
                                let ty = value_type_to_string(&attr.ty, lu_dog, woog, domain);
                                write!(buffer, "{}: {}", attr.name, ty).context(FormatSnafu)?;
                            }
                            None => break,
                        }
                    }
                    // for attr in attrs {
                    //     if attr.name == "id" {
                    //         continue;
                    //     }
                    //     let ty = value_type_to_string(&attr.ty, lu_dog, woog, domain);
                    //     write!(buffer, "{}: {}, ", attr.name, ty).context(FormatSnafu)?;
                    // }
                    writeln!(buffer, ") -> Self {{").context(FormatSnafu)?;
                    emit!(buffer, "        let id = Uuid::new();");
                    emit!(buffer, "        Self {{");
                    for attr in &attrs {
                        emit!(buffer, "            {}: {},", attr.name, attr.name);
                    }
                    emit!(buffer, "        }}");
                    emit!(buffer, "    }}");

                    emit!(buffer, "}}");
                    emit!(buffer, "");
                }

                Ok(())
            },
        )?;

        Ok(())
    }
}

fn value_type_to_string(
    ty: &ValueType,
    lu_dog: &RwLock<LuDogStore>,
    woog: &WoogStore,
    domain: &Domain,
) -> String {
    match ty {
        ValueType::Ty(ref id) => {
            let ty = domain.sarzak().exhume_ty(id).unwrap();
            match ty {
                Ty::Object(ref id) => {
                    let obj = domain.sarzak().exhume_object(id).unwrap();
                    obj.as_type(&Ownership::new_owned(), woog, domain)
                }
                Ty::String(_) => "string".to_string(),
                Ty::Boolean(_) => "bool".to_string(),
                Ty::Integer(_) => "int".to_string(),
                Ty::Float(_) => "float".to_string(),
                Ty::Uuid(_) => "Uuid".to_string(),
                Ty::External(_) => "ext_what_to_do".to_string(),
            }
        }
        ValueType::WoogOption(ref id) => {
            let inner = {
                let lu_dog = lu_dog.read().unwrap();
                let some = lu_dog.exhume_some(id).unwrap();
                lu_dog.exhume_value_type(&some.inner_type).unwrap().clone()
            };

            let mut ty = String::new();
            ty.push_str("Option<");
            ty.push_str(&value_type_to_string(&inner, lu_dog, woog, domain));
            ty.push_str(">");

            ty
        }
    }
}

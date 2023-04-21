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

        // buffer.block(
        // DirectiveKind::AllowEditing,
        // format!("{}-dwarf-file", module),
        // |buffer| {
        self.definition.write_code(
            config, domain, woog, lu_dog, imports, package, module, obj_id, buffer,
        )?;

        // Ok(())
        // },
        // )?;

        Ok(GenerationAction::Write)
    }
}

/// Dwarf Generator / CodeWriter
///
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

        // buffer.block(
        //     DirectiveKind::IgnoreOrig,
        //     format!("{}-dwarf-output", module),
        //     |buffer| {
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
        // Insert ourselves
        imports.insert(module);

        for import in imports {
            emit!(buffer, "use {};", import);
        }
        emit!(buffer, "");

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
            let obj_type = obj.as_type(&Ownership::new_owned(), woog, domain);
            emit_object_comments(&obj.description, "// ", "", buffer)?;
            emit!(buffer, "struct {} {{", obj_type,);

            let attrs: Vec<Attribute> = collect_attributes(obj, &lu_dog, domain);
            for attr in &attrs {
                let ty = value_type_to_string(&attr.ty, lu_dog, woog, domain);
                emit!(buffer, "    {}: {},", attr.name, ty);
            }

            emit!(buffer, "}}");

            //
            // Emit the impl block
            //
            emit!(buffer, "impl {} {{", obj_type,);

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
            writeln!(buffer, ") -> {} {{", obj_type).context(FormatSnafu)?;
            emit!(buffer, "        let id = Uuid::new_v4();");
            emit!(buffer, "        {} {{", obj_type);
            for attr in &attrs {
                emit!(buffer, "            {}: {},", attr.name, attr.name);
            }
            emit!(buffer, "        }}");
            emit!(buffer, "    }}");
            emit!(buffer, "");

            emit!(buffer, "    fn help() -> () {{");
            emit_object_comments(
                // What a cheat!
                &obj.description.replace("\"", "\u{201d}"),
                "        print(\"",
                "\\n\");",
                buffer,
            )?;
            emit!(buffer, "    }}");
            emit!(buffer, "");

            emit!(buffer, "    fn info() -> () {{");
            emit!(buffer, "        print(\"struct {} {{\\n\");", obj_type,);
            for attr in &attrs {
                let ty = value_type_to_string(&attr.ty, lu_dog, woog, domain);
                emit!(buffer, "        print(\"    {}: {},\\n\");", attr.name, ty);
            }
            emit!(buffer, "        print(\"}}\\n\");");
            emit!(buffer, "    }}");

            emit!(buffer, "}}");
            emit!(buffer, "");
        }

        //         Ok(())
        //     },
        // )?;

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
        ValueType::Empty(_) => "()".to_string(),
        ValueType::Error(_) => "maybe error type wasn't a good idea".to_string(),
        ValueType::Function(_) => "<function>".to_string(),
        ValueType::List(ref id) => {
            let inner = {
                let lu_dog = lu_dog.read().unwrap();
                let list = lu_dog.exhume_list(id).unwrap();
                list.r36_value_type(&lu_dog)[0].clone()
            };
            format!(
                "Vec<{}>",
                &value_type_to_string(&inner, lu_dog, woog, domain)
            )
        }
        ValueType::Reference(ref id) => {
            let inner = {
                let lu_dog = lu_dog.read().unwrap();
                let reference = lu_dog.exhume_reference(id).unwrap();
                reference.r35_value_type(&lu_dog)[0].clone()
            };

            format!("&{}", &value_type_to_string(&inner, lu_dog, woog, domain))
        }
        ValueType::Ty(ref id) => {
            let ty = domain.sarzak().exhume_ty(id).unwrap();
            match ty {
                Ty::Object(ref id) => {
                    let obj = domain.sarzak().exhume_object(id).unwrap();
                    obj.as_type(&Ownership::new_owned(), woog, domain)
                }
                Ty::SString(_) => "string".to_string(),
                Ty::Boolean(_) => "bool".to_string(),
                Ty::Integer(_) => "int".to_string(),
                Ty::Float(_) => "float".to_string(),
                Ty::SUuid(_) => "Uuid".to_string(),
                Ty::External(_) => "ext_what_to_do".to_string(),
            }
        }
        ValueType::Unknown(_) => "<unknown>".to_string(),
        ValueType::WoogOption(ref id) => {
            let inner = {
                let lu_dog = lu_dog.read().unwrap();
                let option = lu_dog.exhume_woog_option(id).unwrap();
                option.r2_value_type(&lu_dog)[0].clone()
            };

            format!(
                "Option<{}>",
                &value_type_to_string(&inner, lu_dog, woog, domain)
            )
        }
        ValueType::WoogStruct(ref id) => {
            let lu_dog = lu_dog.read().unwrap();
            let woog_struct = lu_dog.exhume_woog_struct(id).unwrap();
            woog_struct
                .name
                .as_type(&Ownership::new_owned(), woog, domain)
        }
        ValueType::ZObjectStore(ref id) => {
            let domain_name = {
                let lu_dog = lu_dog.read().unwrap();
                let zobject_store = lu_dog.exhume_z_object_store(id).unwrap();
                zobject_store.domain.to_owned()
            };

            format!(
                "{}Store",
                domain_name.as_type(&Ownership::new_owned(), woog, domain)
            )
        }
    }
}

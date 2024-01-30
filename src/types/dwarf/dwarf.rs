//! Dwarf File Generation
//!
//! This is where we generate code for use in the next stage of the compiler.
use std::{fmt::Write, sync::Arc};

use heck::ToUpperCamelCase;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use sarzak::{
    lu_dog::types::ValueType,
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::types::{Object, Ty},
    v2::domain::Domain,
    woog::store::ObjectStore as WoogStore,
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        collect_attributes, emit_object_comments,
        generator::{CodeWriter, FileGenerator, GenerationAction},
        get_subtypes_sorted_from_super_obj, object_is_enum, object_is_hybrid, object_is_singleton,
        render::RenderIdent,
        AttributeBuilder,
    },
    options::GraceConfig,
    s_read,
    target::dwarf::LU_DOG,
    types::DwarfDefinition,
    Lock,
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

        emit!(buffer, "");
        emit!(buffer, "use std::prelude::*;");
        emit!(buffer, "");

        // buffer.block(
        // DirectiveKind::AllowEditing,
        // format!("{}-dwarf-file", module),
        // |buffer| {
        self.definition.write_code(
            config, domain, woog, imports, package, module, obj_id, buffer,
        )?;

        // Ok(())
        // },
        // )?;

        Ok(GenerationAction::Write)
    }
}

/// Dwarf Generator / CodeWriter
///
pub(crate) struct DwarfFile;

impl DwarfFile {
    pub(crate) fn new() -> Box<dyn DwarfDefinition> {
        Box::new(Self)
    }
}

impl DwarfDefinition for DwarfFile {}

impl CodeWriter for DwarfFile {
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
                description: "woog is required by DwarfModule"
            }
        );
        let woog = woog.as_ref().unwrap();

        let _lu_dog = &LU_DOG;

        struct Attribute {
            pub name: String,
            pub ty: Arc<Lock<ValueType>>,
        }

        impl AttributeBuilder<Attribute> for Attribute {
            fn new(name: String, ty: Arc<Lock<ValueType>>) -> Self {
                Attribute { name, ty }
            }
        }

        let mut objects: Vec<&Object> = domain.sarzak().iter_object().collect();
        objects.sort_by(|a, b| a.name.cmp(&b.name));
        let _ = objects
            .iter()
            .filter(|obj| config.is_imported(&obj.id))
            .map(|obj| -> Result<()> {
                let imported = config.get_imported(&obj.id).unwrap();
                let domain = imported.domain.split("::").last().unwrap();

                emit!(
                    buffer,
                    "use {domain}::{domain}::{};",
                    obj.name.to_upper_camel_case()
                );
                Ok(())
            })
            .collect::<Result<Vec<_>, _>>();

        emit!(buffer, "");

        let objects = objects
            .iter()
            .filter(|obj| {
                // Don't include imported objects
                !config.is_imported(&obj.id)
            })
            .collect::<Vec<_>>();

        // Generate code for the ObjectStore
        // let store_type = module.as_type(&Ownership::new_owned(), woog, domain);
        let store_type = module.sanitize().to_upper_camel_case();
        emit!(
            buffer,
            r#"// This annotation tells the interpreter that the struct will be a proxy for
// an `ObjectStore` called {module}. It will find the plugin based on the name.
#[store(model = "{module}")]
struct {store_type}Store {{}}

// This is just to keep the type checking happy.
#[store(model = "{module}")]
impl {store_type}Store {{
    // This is a function that exists on the ObjectStore, and the interpreter
    // will invoke it in the plugin.
    #[proxy(store = "{module}", object = "ObjectStore", func = "new")]
    fn new() -> Self;
    #[proxy(store = "{module}", object = "ObjectStore", func = "load")]
    fn load(path: string) -> Self;
    #[proxy(store = "{module}", object = "ObjectStore", func = "persist")]
    fn save(self) -> Self;
"#
        );

        for obj in &objects {
            let is_singleton = object_is_singleton(obj, config, imports, domain)?;

            let obj_type = obj.name.sanitize().to_upper_camel_case();
            let obj_ident = obj.as_ident();

            if is_singleton {
                continue;
            }

            emit!(
                buffer,
                r#"
    #[proxy(store = "{module}", object = "ObjectStore", func = "inter_{obj_ident}")]
    fn inter_{obj_ident}(self, {obj_ident}: {obj_type});
    #[proxy(store = "{module}", object = "ObjectStore", func = "exhume_{obj_ident}")]
    fn exhume_{obj_ident}(self, {obj_ident}: Uuid) -> {obj_type};"#
            );
        }

        emit!(buffer, "}}\n");

        for obj in &objects {
            let is_enum = object_is_enum(obj, config, imports, domain)?;
            let is_hybrid = object_is_hybrid(obj, config, imports, domain)?;
            let is_singleton = object_is_singleton(obj, config, imports, domain)?;

            //
            // Emit the type definition
            //
            let obj_type = obj.name.sanitize().to_upper_camel_case();
            let store_type = obj.name.to_owned();
            emit_object_comments(&obj.description, "// ", "", buffer)?;
            emit!(
                buffer,
                r#"// This tells the interpreter that this struct is a proxy for an object called
// "{obj_type}" in the store named "{module}"; declared above.
#[proxy(store = "{module}", object = "{store_type}")]"#
            );
            emit!(buffer, "struct {} {{", obj_type,);

            let attrs: Vec<Attribute> = collect_attributes(obj, domain);
            for attr in &attrs {
                let ty = value_type_to_string(&attr.ty, woog, domain);
                emit!(buffer, "    {}: {},", attr.name, ty);
            }
            emit!(buffer, "    // Non-formalizing relationships");

            // 🚧 We need to attach reverse pointers for relationships that we
            // don't formalize. We also need to add pointers on the super- and
            // sub-types
            // if is_super || is_hybrid {
            //     emit!(buffer, "    subtype")
            // }

            emit!(buffer, "}}\n");

            //
            // Emit the impl block
            //
            emit!(buffer, "impl {} {{", obj_type,);

            //
            // Emit the constructor
            //
            if !is_enum && !is_hybrid {
                emit!(
                    buffer,
                    r#"    #[proxy(store = "{module}", object = "{store_type}", func = "new")]"#
                );
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
                            let ty = value_type_to_string(&attr.ty, woog, domain);
                            write!(buffer, "{}: {}", attr.name, ty).context(FormatSnafu)?;
                        }
                        None => break,
                    }
                }
                writeln!(buffer, ") -> Self;\n").context(FormatSnafu)?;
            } else {
                let subtypes = get_subtypes_sorted_from_super_obj!(obj, domain.sarzak());

                for subtype in subtypes {
                    let s_obj = subtype.r15_object(domain.sarzak())[0];
                    let s_obj_type = s_obj.name.sanitize().to_upper_camel_case();
                    let s_obj_ident = s_obj.as_ident();

                    emit!(
                        buffer,
                        r#"    #[proxy(store = "{module}", object = "{store_type}", func = "new_{s_obj_ident}")]"#
                    );
                    write!(buffer, "    fn new_{s_obj_ident}(").context(FormatSnafu)?;
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
                                let ty = value_type_to_string(&attr.ty, woog, domain);
                                write!(buffer, "{}: {}", attr.name, ty).context(FormatSnafu)?;
                            }
                            None => break,
                        }
                    }
                    if !is_singleton && !is_enum {
                        if !ft {
                            write!(buffer, ", ").context(FormatSnafu)?;
                        }
                        writeln!(buffer, "{s_obj_ident}: {s_obj_type}").context(FormatSnafu)?;
                    }
                    writeln!(buffer, ") -> Self;\n").context(FormatSnafu)?;
                }
            }

            emit!(
                buffer,
                r#"    #[proxy(store = "{module}", object = "{store_type}", func = "instances")]
    fn instances() -> [Self];
"#
            );

            //
            // Generate the help() method
            //
            emit!(buffer, "    fn help() -> () {{");
            emit_object_comments(
                // What a cheat!
                // Oh, man, what did I do? This was for the original parser,
                // whatever it's for.
                // 🚧 Fix this.
                &obj.description.replace('\"', "\u{201d}"),
                "        print(\"",
                "\\n\");",
                buffer,
            )?;
            emit!(buffer, "    }}");
            emit!(buffer, "");

            //
            // Generate the info() method
            //
            emit!(buffer, "    fn info() -> () {{");
            emit!(buffer, "        print(\"struct {} {{\\n\");", obj_type,);
            for attr in &attrs {
                let ty = value_type_to_string(&attr.ty, woog, domain);
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

fn value_type_to_string(ty: &Arc<Lock<ValueType>>, woog: &WoogStore, domain: &Domain) -> String {
    let lu_dog = &LU_DOG;

    match *s_read!(ty) {
        ValueType::Char(_) => "char".to_owned(),
        ValueType::Empty(_) => "()".to_owned(),
        ValueType::Error(_) => "maybe error type wasn't a good idea".to_owned(),
        ValueType::Function(_) => "<function>".to_owned(),
        ValueType::Import(ref import) => {
            let lu_dog = lu_dog.read().unwrap();
            let import = lu_dog.exhume_import(import).unwrap();
            let import = s_read!(import);
            if import.has_alias {
                import.alias.clone()
            } else {
                import.name.clone()
            }
        }
        ValueType::Lambda(_) => "<lambda>".to_owned(),
        ValueType::List(ref id) => {
            let inner = {
                let lu_dog = lu_dog.read().unwrap();
                let list = lu_dog.exhume_list(id).unwrap();
                let list = s_read!(list);
                list.r36_value_type(&lu_dog)[0].clone()
            };
            format!("Vec<{}>", &value_type_to_string(&inner, woog, domain))
        }
        ValueType::Range(_) => "<range>".to_owned(),
        ValueType::Reference(ref id) => {
            let inner = {
                let lu_dog = lu_dog.read().unwrap();
                let reference = lu_dog.exhume_reference(id).unwrap();
                let reference = s_read!(reference);
                reference.r35_value_type(&lu_dog)[0].clone()
            };

            value_type_to_string(&inner, woog, domain)
        }
        ValueType::Ty(ref id) => {
            let ty = domain.sarzak().exhume_ty(id).unwrap();
            match ty {
                Ty::Object(ref id) => {
                    let obj = domain.sarzak().exhume_object(id).unwrap();
                    obj.name.sanitize().to_upper_camel_case()
                }
                Ty::SString(_) => "string".to_owned(),
                Ty::Boolean(_) => "bool".to_owned(),
                Ty::Integer(_) => "int".to_owned(),
                Ty::Float(_) => "float".to_owned(),
                Ty::SUuid(_) => "Uuid".to_owned(),
                Ty::External(_) => "ext_what_to_do".to_owned(),
            }
        }
        ValueType::Unknown(_) => "<unknown>".to_owned(),
        ValueType::WoogOption(ref id) => {
            let inner = {
                let lu_dog = lu_dog.read().unwrap();
                let option = lu_dog.exhume_woog_option(id).unwrap();
                let option = s_read!(option);
                option.r2_value_type(&lu_dog)[0].clone()
            };

            format!("Option<{}>", &value_type_to_string(&inner, woog, domain))
        }
        ValueType::WoogStruct(ref id) => {
            let lu_dog = lu_dog.read().unwrap();
            let woog_struct = lu_dog.exhume_woog_struct(id).unwrap();
            let woog_struct = s_read!(woog_struct);

            woog_struct.name.sanitize().to_upper_camel_case()
        }
        ValueType::ZObjectStore(ref id) => {
            let domain_name = {
                let lu_dog = lu_dog.read().unwrap();
                let zobject_store = lu_dog.exhume_z_object_store(id).unwrap();
                let zobject_store = s_read!(zobject_store);
                zobject_store.domain.to_owned()
            };

            format!("{}Store", domain_name.sanitize().to_upper_camel_case())
        }
    }
}

trait Sanitize {
    fn sanitize(&self) -> String;
}

impl Sanitize for String {
    fn sanitize(&self) -> String {
        self.as_str().sanitize()
    }
}

impl Sanitize for &str {
    fn sanitize(&self) -> String {
        match *self {
            "Future" => "XFuture".to_owned(),
            _ => self.to_string(),
        }
    }
}

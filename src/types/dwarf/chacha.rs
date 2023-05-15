//! ChaCha File Generation
//!
//! This is where we generate code for use in the next stage of the compiler.
use std::{
    fmt::Write,
    sync::{Arc, RwLock},
};

use fnv::FnvHashMap as HashMap;
use sarzak::{
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
        generator::{CodeWriter, FileGenerator, GenerationAction},
        get_subtypes_sorted_from_super_obj, object_is_enum, object_is_hybrid, object_is_singleton,
        render::{RenderConst, RenderIdent, RenderType},
        AttributeBuilder,
    },
    options::GraceConfig,
    target::dwarf::LU_DOG,
    types::ChaChaDefinition,
};

pub(crate) struct ChaChaBuilder {
    definition: Option<Box<dyn ChaChaDefinition>>,
}

impl ChaChaBuilder {
    pub(crate) fn new() -> Self {
        ChaChaBuilder { definition: None }
    }

    pub(crate) fn definition(mut self, definition: Box<dyn ChaChaDefinition>) -> Self {
        self.definition = Some(definition);

        self
    }

    pub(crate) fn build(self) -> Result<Box<ChaChaGenerator>> {
        ensure!(
            self.definition.is_some(),
            CompilerSnafu {
                description: "missing ChaChaDefinition"
            }
        );

        Ok(Box::new(ChaChaGenerator {
            definition: self.definition.unwrap(),
        }))
    }
}

pub(crate) struct ChaChaGenerator {
    definition: Box<dyn ChaChaDefinition>,
}

impl FileGenerator for ChaChaGenerator {
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
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by ChaChaModule"
            }
        );

        // Output the domain/module documentation/description
        for line in domain.description().lines() {
            emit!(buffer, "//! {}", line);
        }

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

        Ok(GenerationAction::FormatWrite)
    }
}

#[derive(Debug)]
struct Attribute {
    pub name: String,
    pub ty: Arc<RwLock<ValueType>>,
}

impl AttributeBuilder<Attribute> for Attribute {
    fn new(name: String, ty: Arc<RwLock<ValueType>>) -> Self {
        Attribute { name, ty }
    }
}

/// ChaCha Generator / CodeWriter
///
pub(crate) struct ChaChaFile;

impl ChaChaFile {
    pub(crate) fn new() -> Box<dyn ChaChaDefinition> {
        Box::new(Self)
    }
}

impl ChaChaDefinition for ChaChaFile {}

impl CodeWriter for ChaChaFile {
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
                description: "woog is required by ChaChaModule"
            }
        );
        let woog = woog.as_ref().unwrap();

        let _lu_dog = &LU_DOG;

        // buffer.block(
        //     DirectiveKind::IgnoreOrig,
        //     format!("{}-dwarf-output", module),
        //     |buffer| {

        // Add an import statement for each imported domain
        // let mut use_imports = HashSet::default();
        // for imported in domain
        //     .sarzak()
        //     .iter_object()
        //     .filter(|obj| config.is_imported(&obj.id))
        // {
        //     let imported_object = config.get_imported(&imported.id).unwrap();
        //     use_imports.insert(imported_object.domain.as_str());
        // }
        // // Insert ourselves
        // use_imports.insert(module);

        // for import in use_imports {
        //     emit!(buffer, "use {};", import);
        // }

        let mut objects: Vec<&Object> = domain.sarzak().iter_object().collect();
        objects.sort_by(|a, b| a.name.cmp(&b.name));
        let objects = objects
            .iter()
            .filter(|obj| {
                // Don't include imported objects
                !config.is_imported(&obj.id)
                // 🚧 I'd love to figure out how to get rid of the unwrap().
                && !object_is_singleton(obj, config, imports, domain).unwrap()
            })
            .collect::<Vec<_>>();

        emit!(
            buffer,
            "use std::{{any::Any, collections::VecDeque, fmt, sync::{{Arc, RwLock}}}};"
        );
        emit!(buffer, "");
        emit!(buffer, "use ansi_term::Colour;");
        emit!(buffer, "use derivative::Derivative;");
        emit!(buffer, "use lazy_static::lazy_static;");
        emit!(buffer, "use sarzak::{{lu_dog::{{Empty, List, ObjectStore as LuDogStore, ValueType}},sarzak::SUuid}};");
        emit!(buffer, "use uuid::{{uuid, Uuid}};");
        emit!(buffer, "");
        emit!(buffer, "use sarzak::merlin::{{ObjectStore as MerlinStore,");
        for object in &objects {
            emit!(
                buffer,
                "{},",
                object.as_type(&Ownership::new_owned(), woog, domain)
            );
        }
        emit!(buffer, "}};");
        emit!(buffer, "");
        emit!(
            buffer,
            "use crate::{{ChaChaError, Result, StoreProxy, Value}};"
        );
        emit!(buffer, "");

        //
        // This is the static reference to our backing domain.
        let domain_name = domain.name().as_type(&Ownership::new_owned(), woog, domain);

        emit!(buffer, "\nlazy_static! {{");
        emit!(
            buffer,
            "static ref MODEL: Arc<RwLock<{}Store>> = Arc::new(RwLock::new(",
            domain_name
        );
        emit!(
            buffer,
            "{}Store::load(\"{}\").unwrap()));",
            domain_name,
            config
                .get_store_path()
                .unwrap()
                .as_path()
                .canonicalize()
                .unwrap()
                .display()
        );
        emit!(buffer, "}}\n");

        for obj in &objects {
            let obj_type = obj.as_type(&Ownership::new_owned(), woog, domain);
            let obj_ident = obj.as_ident();
            let obj_const = obj.as_const();
            let is_enum = object_is_enum(obj, config, imports, domain)?;
            let is_hybrid = object_is_hybrid(obj, config, imports, domain)?;
            let is_singleton = object_is_singleton(obj, config, imports, domain)?;
            let id = if is_enum || is_singleton {
                "id()"
            } else {
                "id"
            };
            let attrs: Vec<Attribute> = collect_attributes(obj, domain);

            //
            // This is the id of the WoogStruct, generated by dwarfc.
            emit!(buffer, "use crate::woog_structs::{obj_const}_TYPE_UUID;");
            //
            // The id of the object that will be backing the dwarf type.
            emit!(
                buffer,
                "const {obj_const}_STORE_TYPE_UUID: Uuid = uuid!(\"{}\");",
                obj.id
            );

            //
            // Generate the proxy type
            emit!(buffer, "#[derive(Clone, Derivative)]",);
            emit!(buffer, "#[derivative(Debug)]",);
            emit!(buffer, "pub struct {obj_type}Proxy {{");
            emit!(buffer, "self_: Option<Arc<RwLock<{obj_type}>>>,");
            emit!(buffer, "type_: Arc<RwLock<ValueType>>,");
            emit!(buffer, "#[derivative(Debug = \"ignore\")]");
            emit!(buffer, "lu_dog: Arc<RwLock<LuDogStore>>,");
            emit!(buffer, "}}\n");

            //
            // This is the implementation of the proxy type. We only need one
            // method so far, and that's basically a default replacement. Default
            // doesn't work because we need a pointer back to the store.
            emit!(buffer, "impl {obj_type}Proxy {{");
            emit!(
                buffer,
                "pub fn new_type(lu_dog: Arc<RwLock<LuDogStore>>) -> Self {{"
            );
            emit!(
                buffer,
                "let type_ = lu_dog.read().unwrap().exhume_value_type(&{obj_const}_STORE_TYPE_UUID).unwrap();\n",
            );
            emit!(buffer, "Self {{");
            emit!(buffer, "self_: None,");
            emit!(buffer, "type_,");
            emit!(buffer, "lu_dog,");
            emit!(buffer, "}}");
            emit!(buffer, "}}");
            emit!(buffer, "}}\n");

            //
            // This is the StoreProxy implementation
            emit!(buffer, "impl StoreProxy for {obj_type}Proxy {{");

            emit!(
                buffer,
                "/// Return the name of the type for which we proxy. Proxy on baby! 🕺"
            );
            emit!(buffer, "fn name(&self) -> &str {{");
            emit!(buffer, "\"{obj_type}\"");
            emit!(buffer, "}}\n");

            emit!(
                buffer,
                "/// Magic methods to make things appear from thin air. 🪄"
            );
            emit!(buffer, "fn into_any(&self) -> Box<dyn Any> {{");
            emit!(buffer, "Box::new(self.clone())");
            emit!(buffer, "}}\n");

            emit!(
                buffer,
                "/// Return the WoogStruct id of the type using this proxy."
            );
            emit!(buffer, "fn struct_uuid(&self) -> Uuid {{");
            emit!(buffer, "{obj_const}_TYPE_UUID");
            emit!(buffer, "}}\n");

            emit!(
                buffer,
                "/// Return the sarzak Object id of the type for which we are proxying."
            );
            emit!(buffer, "fn store_uuid(&self) -> Uuid {{");
            emit!(buffer, "{obj_const}_STORE_TYPE_UUID");
            emit!(buffer, "}}\n");

            emit!(
                buffer,
                "/// This method acts as the function call proxy for the type."
            );
            emit!(buffer, "fn call(&mut self, method: &str, args: &mut VecDeque<Arc<RwLock<Value>>>) -> Result<(Arc<RwLock<Value>>, Arc<RwLock<ValueType>>)> {{");
            emit!(buffer, "if let Some(self_) = &self.self_ {{");
            emit!(buffer, "match method {{");
            emit!(buffer, "\"id\" => Ok((");
            emit!(
                buffer,
                "Arc::new(RwLock::new(Value::Uuid(self_.read().unwrap().{id}))),"
            );
            emit!(
                buffer,
                "self.lu_dog.read().unwrap().exhume_value_type(&SUuid::new().id()).unwrap(),)),"
            );
            emit!(buffer, " 道 => Ok((");
            emit!(
                buffer,
                "Arc::new(RwLock::new(Value::Error(format!(\"unknown method `{{}}`\", 道)))),"
            );
            emit!(
                buffer,
                "Arc::new(RwLock::new(ValueType::Empty(Empty::new().id()))),"
            );
            emit!(buffer, ")),");
            emit!(buffer, "}}");
            emit!(buffer, "}} else {{");
            emit!(buffer, "let arg = args.pop_front().unwrap();");
            emit!(buffer, "let arg = arg.read().unwrap();");
            emit!(buffer, "match method {{");

            if !is_enum && !is_hybrid {
                render_ctor(
                    "new", obj, None, &attrs, config, imports, woog, domain, buffer,
                )?;
            } else {
                for subtype in get_subtypes_sorted_from_super_obj!(obj, domain.sarzak()) {
                    let s_obj = subtype.r15_object(domain.sarzak())[0];
                    render_ctor(
                        &format!("new_{}", s_obj.as_ident()),
                        &s_obj,
                        Some(&obj),
                        &attrs,
                        config,
                        imports,
                        woog,
                        domain,
                        buffer,
                    )?;
                }
            }
            emit!(buffer, "\"instances\" => {{");
            emit!(
                buffer,
                "let instances = MODEL.read().unwrap().iter_{obj_ident}().map(|{obj_ident}| {{",
            );
            emit!(buffer, "let mut {obj_ident}_proxy = self.clone();");
            emit!(buffer, "{obj_ident}_proxy.self_ = Some({obj_ident});");
            emit!(
                buffer,
                "Arc::new(RwLock::new(Value::ProxyType(Arc::new(RwLock::new({obj_ident}_proxy)))))",
            );
            emit!(buffer, "}})");
            emit!(buffer, ".collect();");
            emit!(buffer, "");
            emit!(
                buffer,
                "let list = List::new(&self.type_, &mut self.lu_dog.write().unwrap());"
            );
            emit!(
                buffer,
                "let ty = ValueType::new_list(&list, &mut self.lu_dog.write().unwrap());"
            );
            emit!(buffer, "");
            emit!(
                buffer,
                "Ok((Arc::new(RwLock::new(Value::Vector(instances))), ty))"
            );
            emit!(buffer, "}}");
            emit!(buffer, "道 => Ok((");
            emit!(
                buffer,
                "Arc::new(RwLock::new(Value::Error(format!(\"unknown static method `{{}}`\", 道)))),"
            );
            emit!(
                buffer,
                "Arc::new(RwLock::new(ValueType::Empty(Empty::new().id()))),"
            );
            emit!(buffer, "))");
            emit!(buffer, "}}");
            emit!(buffer, "}}");
            emit!(buffer, "}}");
            emit!(buffer, "");

            emit!(
                buffer,
                "/// This method acts as the field access proxy for the type.",
            );
            emit!(
                buffer,
                "fn get_attr_value(&self, field: &str) -> Result<Arc<RwLock<Value>>> {{"
            );
            emit!(buffer, "if let Some(self_) = &self.self_ {{");
            emit!(buffer, "match field {{");

            for attr in &attrs {
                let attr_name = attr.name.as_ident();
                let ty = value_type_to_string(&attr.ty, woog, domain).0;

                if attr_name == "id" {
                    emit!(
                        buffer,
                        "\"{attr_name}\" => Ok(Arc::new(RwLock::new(Value::{ty}(self_.read().unwrap().{id})))),",
                    );
                } else {
                    if ty == "UserType" {
                        emit!(buffer, "\"{attr_name}\" => {{");
                        emit!(
                            buffer,
                            "let {attr_name} = MODEL.read().unwrap().exhume_{obj_ident}(&self_.read().unwrap().{attr_name}).unwrap();"
                        );
                        emit!(buffer, "");
                        emit!(
                            buffer,
                            "Ok(Arc::new(RwLock::new(({attr_name}, self.lu_dog.clone()).into())))"
                        );
                        emit!(buffer, "}},");
                    } else if ty == "String" {
                        emit!(
                            buffer,
                            "\"{attr_name}\" => Ok(Arc::new(RwLock::new(Value::{ty}(self_.read().unwrap().{attr_name}.to_owned())))),",
                        );
                    } else {
                        emit!(
                        buffer,
                        "\"{attr_name}\" => Ok(Arc::new(RwLock::new(Value::{ty}(self_.read().unwrap().{attr_name})))),",
                    );
                    }
                }
            }
            emit!(
                buffer,
                "_ => Err(ChaChaError::NoSuchField {{field: field.to_owned()}}),"
            );
            emit!(buffer, "}}");
            emit!(buffer, "}} else {{");
            emit!(buffer, "Err(ChaChaError::NotAnInstance)");
            emit!(buffer, "}}");
            emit!(buffer, "}}");
            emit!(buffer, "}}\n");

            //
            // Write the Display implementation
            emit!(buffer, "impl fmt::Display for {obj_type}Proxy {{");
            emit!(
                buffer,
                "fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{"
            );
            emit!(buffer, "if let Some(self_) = &self.self_ {{");
            emit!(
                buffer,
                "write!(f, \"{obj_type}Proxy({{}})\", self_.read().unwrap().{id})",
            );
            emit!(buffer, "}} else {{");
            emit!(
                buffer,
                "write!(f, \"{{}} {obj_type}Proxy\", Colour::Yellow.underline().paint(\"Type\"))",
            );
            emit!(buffer, "}}");
            emit!(buffer, "}}");
            emit!(buffer, "}}\n");

            //
            // Create a From implementation for Value
            emit!(
                buffer,
                "impl From<(Arc<RwLock<{obj_type}>>, Arc<RwLock<LuDogStore>>)> for Value {{",
            );
            emit!(
                    buffer,
                    "fn from(({obj_ident}, store): (Arc<RwLock<{obj_type}>>, Arc<RwLock<LuDogStore>>)) -> Self {{"
                );
            if is_enum {
                emit!(
                    buffer,
                    "let read_{obj_ident} = {obj_ident}.read().unwrap();\n",
                );
                emit!(buffer, "match *read_{obj_ident} {{");
                for subtype in get_subtypes_sorted_from_super_obj!(obj, domain.sarzak()) {
                    let s_obj = subtype.r15_object(domain.sarzak())[0];
                    let s_obj_type = s_obj.as_type(&Ownership::new_owned(), woog, domain);

                    emit!(buffer, "{obj_type}::{s_obj_type}(_) => {{");
                    emit!(
                        buffer,
                        "let mut {obj_ident}_proxy = {obj_type}Proxy::new_type(store.clone());"
                    );
                    emit!(
                        buffer,
                        "{obj_ident}_proxy.self_ = Some({obj_ident}.clone());"
                    );
                    emit!(
                        buffer,
                        "Value::ProxyType(Arc::new(RwLock::new({obj_ident}_proxy)))"
                    );
                    emit!(buffer, "}}");
                }
                emit!(buffer, "}}");
            } else {
                emit!(
                    buffer,
                    "Value::ProxyType(Arc::new(RwLock::new({obj_type}Proxy {{"
                );
                emit!(buffer, "self_: Some({obj_ident}),");
                emit!(buffer, "type_: store.read().unwrap().exhume_value_type(&{obj_const}_STORE_TYPE_UUID).unwrap(),");
                emit!(buffer, "lu_dog: store.clone(),");
                emit!(buffer, "}})))");
            }
            emit!(buffer, "}}");
            emit!(buffer, "}}\n");

            //
            // Create a TryFrom Value for proxy.
            emit!(buffer, "impl TryFrom<&Value> for {obj_type}Proxy {{",);
            emit!(buffer, "type Error = ChaChaError;");
            emit!(buffer, "");
            emit!(buffer, "fn try_from(value: &Value) -> Result<Self, <{obj_type}Proxy as TryFrom<&Value>>::Error> {{");
            emit!(buffer, "match value {{");
            emit!(buffer, "Value::ProxyType(proxy) => {{");
            emit!(buffer, "let read_proxy = proxy.read().unwrap();");
            emit!(buffer, "");
            emit!(
                buffer,
                "if read_proxy.store_uuid() == {obj_const}_STORE_TYPE_UUID {{"
            );
            emit!(buffer, "let any = (&*read_proxy).into_any();");
            emit!(
                buffer,
                "Ok(any.downcast_ref::<{obj_type}Proxy>().unwrap().clone())"
            );
            emit!(buffer, "}} else {{");
            emit!(buffer, "Err(ChaChaError::Conversion {{");
            emit!(buffer, "src: read_proxy.name().to_owned(),");
            emit!(buffer, "dst: \"{obj_type}Proxy\".to_owned(),");
            emit!(buffer, "}})");
            emit!(buffer, "}}");
            emit!(buffer, "}}");
            emit!(buffer, "_ => Err(ChaChaError::Conversion {{");
            emit!(buffer, "src: value.to_string(),");
            emit!(buffer, "dst: \"{obj_type}Proxy\".to_owned(),");
            emit!(buffer, "}})");
            emit!(buffer, "}}");
            emit!(buffer, "}}");
            emit!(buffer, "}}\n");
        }

        //         Ok(())
        //     },
        // )?;

        Ok(())
    }
}

fn render_ctor(
    method_name: &str,
    obj: &Object,
    parent_obj: Option<&Object>,
    attrs: &[Attribute],
    config: &GraceConfig,
    imports: &Option<&HashMap<String, Domain>>,
    woog: &WoogStore,
    domain: &Domain,
    buffer: &mut Buffer,
) -> Result<()> {
    let obj_ident = obj.as_ident();
    let obj_type = obj.as_type(&Ownership::new_owned(), woog, domain);
    let is_singleton = object_is_singleton(obj, config, imports, domain)?;

    let mut args = String::new();

    emit!(buffer, "\"{}\" => {{", method_name);
    for attr in attrs {
        if attr.name != "id" {
            let attr_ident = attr.name.as_ident();
            let ty = value_type_to_string(&attr.ty, woog, domain);
            let ref_name = ty.1;

            if ty.0 == "UserType" {
                args.extend([format!("&{attr_ident}, ")]);
                emit!(
                    buffer,
                    "let {attr_ident}: {ref_name}Proxy = (&*arg).try_into()?;"
                );
                emit!(buffer, "let {attr_ident} = {attr_ident}.self_.unwrap();");
            } else {
                args.extend([format!("{attr_ident}, ")]);
                emit!(buffer, "let {attr_ident} = (*&arg).try_into()?;");
            }
        }
    }
    emit!(buffer, "");
    emit!(buffer, "let mut model = MODEL.write().unwrap();");

    if parent_obj.is_none() {
        let model = if is_singleton { "" } else { "&mut model" };
        emit!(buffer, "let {obj_ident} = {obj_type}::new({args}{model});");
        emit!(buffer, "");
    }
    emit!(buffer, "let mut {obj_ident}_proxy = self.clone();");

    let thing = if let Some(supertype) = parent_obj {
        let supertype = supertype.as_type(&Ownership::new_owned(), woog, domain);
        format!("{supertype}::{method_name}({args}&mut model)",)
    } else {
        obj_ident.clone()
    };

    emit!(buffer, "{obj_ident}_proxy.self_ = Some({thing});");
    emit!(buffer, "");
    emit!(buffer, "Ok((");
    emit!(
        buffer,
        "Arc::new(RwLock::new(Value::ProxyType(Arc::new(RwLock::new({obj_ident}_proxy))))),"
    );
    emit!(buffer, "self.type_.clone(),");
    emit!(buffer, "))");
    emit!(buffer, "}}");

    Ok(())
}

fn value_type_to_string<'a>(
    ty: &Arc<RwLock<ValueType>>,
    woog: &WoogStore,
    domain: &Domain,
) -> (&'a str, String) {
    let lu_dog = &LU_DOG;

    match ty.read().unwrap().clone() {
        ValueType::Reference(ref id) => {
            let inner = {
                let lu_dog = lu_dog.read().unwrap();
                let reference = lu_dog.exhume_reference(id).unwrap().read().unwrap().clone();
                reference.r35_value_type(&lu_dog)[0].clone()
            };

            ("UserType", value_type_to_string(&inner, woog, domain).1)
        }
        ValueType::Ty(ref id) => {
            let ty = domain.sarzak().exhume_ty(id).unwrap();
            match ty {
                Ty::Object(ref id) => {
                    let obj = domain.sarzak().exhume_object(id).unwrap();
                    ("Object", obj.as_type(&Ownership::new_owned(), woog, domain))
                }
                Ty::SString(_) => ("String", "".to_owned()),
                Ty::Boolean(_) => ("Boolean", "".to_owned()),
                Ty::Integer(_) => ("Integer", "".to_owned()),
                Ty::Float(_) => ("Float", "".to_owned()),
                Ty::SUuid(_) => ("Uuid", "".to_owned()),
                Ty::External(_) => ("ext_what_to_do", "".to_owned()),
            }
        }
        ValueType::WoogOption(_) => ("Option", "".to_owned()),
        ValueType::WoogStruct(ref id) => {
            let lu_dog = lu_dog.read().unwrap();
            let woog_struct = lu_dog
                .exhume_woog_struct(id)
                .unwrap()
                .read()
                .unwrap()
                .clone();
            (
                "WoogStruct",
                woog_struct
                    .name
                    .as_type(&Ownership::new_owned(), woog, domain),
            )
        }
        oops => {
            dbg!(oops);
            unimplemented!();
        }
    }
}

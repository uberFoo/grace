//! ChaCha File Generation
//!
//! This is where we generate code for use in the next stage of the compiler.
use std::{fmt::Write, sync::Arc};

use rustc_hash::FxHashMap as HashMap;
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
        object_is_supertype,
        render::{RenderConst, RenderIdent, RenderType},
        AttributeBuilder,
    },
    options::GraceConfig,
    s_read,
    target::dwarf::LU_DOG,
    types::ChaChaDefinition,
    Lock,
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
    pub ty: Arc<Lock<ValueType>>,
}

impl AttributeBuilder<Attribute> for Attribute {
    fn new(name: String, ty: Arc<Lock<ValueType>>) -> Self {
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
        _module: &str,
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

        let domain_name = domain.name().as_ident();
        let domain_type = domain.name().as_type(&Ownership::new_owned(), woog, domain);

        let mut objects: Vec<&Object> = domain.sarzak().iter_object().collect();
        objects.sort_by(|a, b| a.name.cmp(&b.name));

        emit!(
            buffer,
            "use std::{{cell::RefCell, path::Path, fmt::{{self, Display}}, rc::Rc}};"
        );
        emit!(buffer, "");

        emit!(buffer, "use abi_stable::{{export_root_module, prefix_type::PrefixTypeTrait, sabi_extern_fn, sabi_trait::prelude::{{TD_CanDowncast, TD_Opaque}}, std_types::{{RErr, ROk, RResult, RStr, RString, RVec}}}};");
        emit!(buffer, "use dwarf::{{chacha::value::{{FfiProxy, FfiValue, Value}}, plug_in::{{Error, Plugin, PluginModRef, PluginModule, PluginType, Plugin_TO}}}};");
        emit!(buffer, "use log::debug;");
        emit!(buffer, "use uuid::{{uuid, Uuid}};");
        emit!(buffer, "");

        emit!(buffer, "pub mod store;");
        emit!(buffer, "pub mod types;");
        emit!(buffer, "pub use store::ObjectStore;");
        emit!(buffer, "pub use types::*;");
        if config.is_sarzak() {
            emit!(
                buffer,
                r#"pub const MODEL: &[u8] = include_bytes!("../models/sarzak.bin");"#
            );
        }
        // emit!(buffer, "use crate::{domain_name}::{{ObjectStore,");
        // for object in &objects {
        //     if config.is_imported(&object.id) {
        //         continue;
        //     }

        //     if object_is_hybrid(object, config, imports, domain)? {
        //         emit!(
        //             buffer,
        //             "{}Enum,",
        //             object.as_type(&Ownership::new_owned(), woog, domain)
        //         );
        //     }
        //     if object_is_singleton(object, config, imports, domain)? {
        //         emit!(buffer, "{},", object.as_const());
        //     }
        //     emit!(
        //         buffer,
        //         "{},",
        //         object.as_type(&Ownership::new_owned(), woog, domain)
        //     );
        // }
        // emit!(buffer, "}};");
        emit!(buffer, "");

        emit!(
            buffer,
            r#"/// Exports the root module of this library.
///
/// This code isn't run until the layout of the type it returns is checked."#
        );
        if !config.is_meta_model() {
            emit!(buffer, "#[export_root_module]");
        }
        emit!(
            buffer,
            r#"pub fn instantiate_root_module() -> PluginModRef {{
    PluginModule {{ name, id, new }}.leak_into_prefix()
}}
"#
        );

        emit!(
            buffer,
            r#"#[sabi_extern_fn]
pub fn name() -> RStr<'static> {{
    "{domain_name}".into()
}}
"#
        );

        emit!(
            buffer,
            r#"#[sabi_extern_fn]
pub fn id() -> RStr<'static> {{
    "{domain_name}".into()
}}
"#
        );

        emit!(
            buffer,
            r#"/// Instantiates the plugin.
#[sabi_extern_fn]
pub fn new(args: RVec<FfiValue>) -> RResult<PluginType, Error> {{
    match (|| {{
        if args.len() == 0 {{
            Ok({domain_type}Store {{
                store: Rc::new(RefCell::new(ObjectStore::new())),
            }})
        }} else if args.len() == 1 {{
            if let FfiValue::String(path) = &args[0] {{
                let store = ObjectStore::load(Path::new(&path.as_str())).unwrap();
                Ok({domain_type}Store {{
                    store: Rc::new(RefCell::new(store)),
                }})
            }} else {{
                Err(Error::Uber("Invalid arguments".into()))
            }}
        }} else {{
            Err(Error::Uber("Invalid arguments".into()))
        }}
    }})() {{
        Ok(this) => ROk(Plugin_TO::from_value(this, TD_Opaque)),
        Err(e) => RErr(e.into()),
    }}
}}
"#
        );

        emit!(
            buffer,
            r#"#[derive(Clone, Debug)]
struct {domain_type}Store {{
    store: Rc<RefCell<ObjectStore>>,
}}
"#
        );

        emit!(
            buffer,
            r#"impl Display for {domain_type}Store {{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
        write!(f, "{{:?}}", self.store)
    }}
}}
"#
        );

        emit!(
            buffer,
            r#"impl Plugin for {domain_type}Store {{
    fn invoke_func(
        &mut self,
        module: RStr<'_>,
        ty: RStr<'_>,
        func: RStr<'_>,
        mut args: RVec<FfiValue>,
    ) -> RResult<FfiValue, Error> {{
        (|| -> Result<FfiValue, Error> {{
            let ty = ty.as_str();
            let func = func.as_str();
            debug!("type: {{ty}}, func: {{func}}, args: {{args:?}}");
            match ty {{
                "ObjectStore" => match func {{
                    "persist" => {{
                        if args.len() != 1 {{
                            return Err(Error::Uber("Expected 1 argument".into()));
                        }}

                        if let FfiValue::String(path) = args.pop().unwrap() {{
                            self.store.borrow().persist(Path::new(&path.as_str())).unwrap();
                            Ok(FfiValue::Empty)
                        }} else {{
                            Err(Error::Uber("Invalid path".into()))
                        }}
                    }}
"#
        );

        // These are the inter_ and exhume_ functions on the ObjectStore.
        for obj in &objects {
            let is_imported = config.is_imported(&obj.id);
            let is_singleton = object_is_singleton(obj, config, imports, domain)?;
            let is_enum = object_is_enum(obj, config, imports, domain)?;

            if is_imported || is_singleton {
                continue;
            }

            let obj_type = obj.as_type(&Ownership::new_owned(), woog, domain);
            let obj_ident = obj.as_ident();
            let obj_const = obj.as_const();

            let id = if is_enum { "id()" } else { "id" };

            emit!(
                buffer,
                r#""inter_{obj_ident}" => {{
                        if args.len() != 1 {{
                            return Err(Error::Uber("Expected 1 argument".into()));
                        }}

                        if let FfiValue::PlugIn({obj_ident}) = args.pop().unwrap() {{
                            let {obj_ident} = {obj_ident}.obj.downcast_into::<{obj_type}Proxy>().unwrap();
                            self.store.borrow_mut()
                                .inter_{obj_ident}({obj_ident}.inner.clone());
                            Ok(FfiValue::Empty)
                        }} else {{
                            Err(Error::Uber("Invalid {obj_type}".into()))
                        }}
                    }}"#
            );

            emit!(
                buffer,
                r#""exhume_{obj_ident}" => {{
                if args.len() != 1 {{
                            return Err(Error::Uber("Expected 1 argument".into()));
                        }}
                        if let FfiValue::Uuid(id) = args.pop().unwrap() {{
                            let {obj_ident} = self.store.borrow().exhume_{obj_ident}(&id.into()).unwrap();
                            let {obj_ident}_proxy = {obj_type}Proxy {{
                                // 🚧 This bothers me deeply. I know that I've given
                                // this some thought already, and I really need to
                                // document the outcome so that I can stop worrying
                                // over it.
                                inner: {obj_ident}.clone(),
                                store: self.store.clone(),
                            }};
                            let plugin = Plugin_TO::from_value({obj_ident}_proxy, TD_CanDowncast);
                            let proxy = FfiProxy {{
                                module: module.into(),
                                uuid: {obj_const}_ID.into(),
                                id: {obj_ident}.borrow().{id}.into(), // a
                                plugin: plugin.clone(),
                            }};

                            Ok(FfiValue::ProxyType(proxy))
                        }} else {{
                            Err(Error::Uber("Invalid id".into()))
                        }}
                    }}"#
            );
        }

        emit!(
            buffer,
            r#"func => Err(Error::Uber(format!("Invalid function: {{func:?}}").into())),
                }},
"#
        );

        // This is for the constructor function(s) on the ObjectStore.
        for obj in &objects {
            let is_hybrid = object_is_hybrid(obj, config, imports, domain)?;
            let is_imported = config.is_imported(&obj.id);
            let is_singleton = object_is_singleton(obj, config, imports, domain)?;
            let is_enum = object_is_enum(obj, config, imports, domain)?;
            let is_super = object_is_supertype(obj, config, imports, domain)?;

            if is_imported || is_singleton {
                continue;
            }

            let obj_type = obj.as_type(&Ownership::new_owned(), woog, domain);
            let obj_ident = obj.as_ident();
            let obj_const = obj.as_const();

            let id = if is_enum || is_singleton {
                "id()"
            } else {
                "id"
            };

            let attrs: Vec<Attribute> = collect_attributes(obj, domain);

            emit!(buffer, r#""{obj_type}" => match func {{"#);

            // In this case we have to generate constructors for each subtype.
            if is_hybrid || is_super {
                // 🚧 This is funky. I'm excluding imported objects above, so what's this
                // about? This is what I get for not starting over with the second incarnation
                // of the dwarf/stare interface.
                let (subtypes, domain) = if is_imported {
                    let imported = config.get_imported(&obj.id).unwrap();

                    let domain = imports.unwrap().get(&imported.domain).unwrap();

                    (
                        get_subtypes_sorted_from_super_obj!(obj, domain.sarzak()),
                        domain,
                    )
                } else {
                    (
                        get_subtypes_sorted_from_super_obj!(obj, domain.sarzak()),
                        domain,
                    )
                };

                for subtype in subtypes {
                    let s_obj = subtype.r15_object(domain.sarzak())[0];

                    render_ctor(
                        &format!("new_{}", s_obj.as_ident()),
                        s_obj,
                        Some(obj),
                        &attrs,
                        config,
                        imports,
                        woog,
                        domain,
                        buffer,
                    )?;
                }
            } else {
                // This is a plain-jane new function
                render_ctor(
                    "new", obj, None, &attrs, config, imports, woog, domain, buffer,
                )?;
            }

            // if !is_enum {
            emit!(
                buffer,
                r#""instances" => {{
                        let mut instances = Vec::new();
                        for {obj_ident} in self.store.borrow().iter_{obj_ident}() {{
                            let this = {obj_type}Proxy {{
                                inner: {obj_ident}.clone(),
                                store: self.store.clone(),
                            }};
                            let plugin = Plugin_TO::from_value(this, TD_CanDowncast);
                            let proxy = FfiProxy {{
                                module: module.into(),
                                uuid: {obj_const}_ID.into(),
                                id: {obj_ident}.borrow().{id}.into(), // b
                                plugin: plugin.clone(),
                            }};

                            instances.push(FfiValue::ProxyType(proxy));
                        }}
                        Ok(FfiValue::Vector(instances.into()))
                    }}"#
            );
            // }

            emit!(
                buffer,
                r#"func => Err(Error::Uber(format!("Invalid function: {{func:?}}").into())),
                }},
"#
            );
        }

        emit!(
            buffer,
            r#"ty => Err(Error::Uber(format!("Invalid type {{ty:?}}").into())),
            }}
        }})()
        .into()
    }}

    fn name(&self) -> RStr<'_> {{
        "merlin".into()
    }}

    fn close(self) {{}}
}}
"#
        );

        for obj in &objects {
            let obj_type = obj.as_type(&Ownership::new_owned(), woog, domain);
            let obj_const = obj.as_const();
            let is_enum = object_is_enum(obj, config, imports, domain)?;
            let is_singleton = object_is_singleton(obj, config, imports, domain)?;
            let is_imported = config.is_imported(&obj.id);

            if is_imported {
                continue;
            }

            let id = if is_enum || is_singleton {
                "id()"
            } else {
                "id"
            };

            let attrs: Vec<Attribute> = collect_attributes(obj, domain);

            // The id of the object that will be backing the dwarf type.
            emit!(
                buffer,
                "const {obj_const}_ID: Uuid = uuid!(\"{}\");\n",
                obj.id
            );

            //
            // Generate the proxy type
            emit!(buffer, "#[derive(Clone, Debug)]",);
            emit!(buffer, "pub struct {obj_type}Proxy {{");
            emit!(buffer, "inner: Rc<RefCell<{obj_type}>>,");
            emit!(buffer, "store: Rc<RefCell<ObjectStore>>,");
            emit!(buffer, "}}\n");

            //
            // This is the implementation of the proxy type. We only need one
            // method so far, and that's basically a default replacement. Default
            // doesn't work because we need a pointer back to the store.
            emit!(
                buffer,
                r#"impl Plugin for {obj_type}Proxy {{
    fn invoke_func(
        &mut self,
        module: RStr<'_>,
        ty: RStr<'_>,
        func: RStr<'_>,
        mut args: RVec<FfiValue>,
    ) -> RResult<FfiValue, Error> {{
        (|| -> Result<FfiValue, Error> {{
            let ty = ty.as_str();
            let func = func.as_str();
            debug!("type: {{ty}}, func: {{func}}, args: {{args:?}}");
            match ty {{
                "self" => match func {{
                    "get_field_value" => {{
                        if args.len() != 1 {{
                            return Err(Error::Uber("Expected 1 argument".into()));
                        }}

                        if let FfiValue::String(field) = args.pop().unwrap() {{
                            match field.as_str() {{
"#
            );

            for attr in &attrs {
                let attr_name = attr.name.as_ident();

                let (ty, ty_ty) = value_type_to_string(&attr.ty, woog, config, domain);

                emit!(buffer, r#""{attr_name}" => "#);
                if attr_name == "id" {
                    emit!(
                        buffer,
                        "Ok(FfiValue::Uuid(self.inner.borrow().{id}.into())),"
                    );
                } else {
                    match ty {
                        "Boolean" => emit!(
                            buffer,
                            "Ok(FfiValue::Boolean(self.inner.borrow().{attr_name}.into())),"
                        ),
                        "Float" => emit!(
                            buffer,
                            "Ok(FfiValue::Float(self.inner.borrow().{attr_name}.into())),"
                        ),
                        "Imported" => emit!(
                            buffer,
                            r#"Err(Error::Uber("Imported object not supported.".into())),"#
                        ),
                        "Integer" => emit!(
                            buffer,
                            "Ok(FfiValue::Integer(self.inner.borrow().{attr_name}.into())),"
                        ),
                        "Option" => emit!(
                            buffer,
                            "Ok(FfiValue::Option(self.inner.borrow().{attr_name}.into())),"
                        ),
                        "String" => emit!(
                            buffer,
                            "Ok(FfiValue::String(self.inner.borrow().{attr_name}.clone().into())),"
                        ),
                        "UserType" => {
                            let type_const = ty_ty.as_const();
                            let type_ident = ty_ty.as_ident();

                            emit!(
                                buffer,
                                r#"{{let {attr_name} =
                                        self.store.borrow().exhume_{type_ident}(&self.inner.borrow().{attr_name}).unwrap();

                                    let this = {ty_ty}Proxy {{
                                        inner: {attr_name},
                                        store: self.store.clone(),
                                    }};
                                    let plugin = Plugin_TO::from_value(this, TD_CanDowncast);
                                    let proxy = FfiProxy {{
                                        module: module.into(),
                                        uuid: {type_const}_ID.into(),
                                        id: self.inner.borrow().{id}.into(), // c
                                        plugin: plugin.clone(),
                                    }};
                                    Ok(FfiValue::ProxyType(proxy))
                                }}"#
                            );
                        }
                        "Uuid" => {
                            emit!(
                                buffer,
                                "Ok(FfiValue::Uuid(self.inner.borrow().{attr_name}.into())),"
                            )
                        }
                        foo => {
                            dbg!(foo);
                            // unreachable!()
                        }
                    }
                }
            }

            emit!(buffer, "_ => Err(Error::Uber(\"Invalid field\".into())),");
            emit!(buffer, "}}");
            emit!(
                buffer,
                r#"}} else {{
                            Err(Error::Uber("Invalid Object".into()))
                        }}
                    }}
                    "set_field_value" => {{
                        if args.len() != 2 {{
                            return Err(Error::Uber("Expected 2 arguments".into()));
                        }}

                        args.reverse();
                        let field = args.pop().unwrap();

                        if let FfiValue::String(field) = field {{
                            let value: Value = args.pop().unwrap().into();
                            match field.as_str() {{
"#
            );

            for attr in &attrs {
                if attr.name == "id" {
                    continue;
                }

                let attr_ident = attr.name.as_ident();

                emit!(
                    buffer,
                    r#""{attr_ident}" => {{
                                    self.inner.borrow_mut().{attr_ident} = value.try_into().map_err(|e| {{
                                        Error::Uber(
                                            format!("Error converting value: {{e}}").into(),
                                        )
                                    }})?
                                }}"#
                );
            }

            emit!(
                buffer,
                r#"field => {{
                                    return Err(Error::Uber(
                                        format!("Invalid field {{field}}").into(),
                                    ))
                                }}
                            }}

                            Ok(FfiValue::Empty)
                        }} else {{
                            Err(Error::Uber(
                                format!("Invalid field type: {{field:?}}").into(),
                            ))
                        }}
                    }}
                    func => Err(Error::Uber(format!("Invalid function: {{func:?}}").into())),
                }},
                ty => Err(Error::Uber(format!("Invalid type {{ty:?}}").into())),
            }}
        }})()
        .into()
    }}

    fn name(&self) -> RStr<'_> {{
        "{obj_type}".into()
    }}

    fn close(self) {{}}
}}
"#
            );

            //
            // Write the Display implementation
            emit!(buffer, "impl Display for {obj_type}Proxy {{");
            emit!(
                buffer,
                "fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{"
            );
            // write a line for each attribute
            for (idx, attr) in attrs.iter().enumerate() {
                let attr_name = attr.name.as_ident();

                if idx == 0 {
                    emit!(buffer, "writeln!(f, \"{obj_type}({{{{\")?;",);
                } else {
                    emit!(buffer, "?;");
                }

                if attr_name == "id" {
                    write!(
                        buffer,
                        "writeln!(f, \"\t{attr_name}: {{:?}},\", self.inner.borrow().{id})",
                    )
                    .context(FormatSnafu)?;
                } else {
                    write!(
                        buffer,
                        "writeln!(f, \"\t{attr_name}: {{:?}},\", self.inner.borrow().{attr_name})",
                    )
                    .context(FormatSnafu)?;
                }
            }
            emit!(buffer, "?;");
            emit!(buffer, "writeln!(f, \"}}}})\")");
            emit!(buffer, "}}");
            emit!(buffer, "}}\n");
        }

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
    let obj_const = obj.as_const();
    let is_singleton = object_is_singleton(obj, config, imports, domain)?;
    let is_enum = object_is_enum(obj, config, imports, domain)?;

    let is_hybrid = if let Some(parent) = parent_obj {
        object_is_hybrid(parent, config, imports, domain)?
    } else {
        false
    };

    emit!(buffer, "\"{}\" => {{", method_name);

    let len = if let Some(parent) = parent_obj {
        if object_is_enum(parent, config, imports, domain)? {
            1
        } else {
            attrs.len() - 1
        }
    } else {
        attrs.len() - 1
    };

    emit!(
        buffer,
        r#"if args.len() != {len} {{
                            return Err(Error::Uber("Expected {len} arguments".into()));
                        }}"#
    );

    if len > 0 {
        emit!(
            buffer,
            r#"                 let mut value_args: Vec<Value> = Vec::new();
                        args.reverse();
                        for arg in args.into_iter() {{
                            value_args.push(arg.into());
                        }}"#
        );
    }

    // dbg!(&obj_ident, is_enum, is_singleton, is_hybrid);

    if is_enum || is_singleton && !is_hybrid {
        if let Some(parent) = parent_obj {
            let parent_type = parent.as_type(&Ownership::new_owned(), woog, domain);
            let parent_ident = parent.as_ident();

            emit!(
                buffer,
                r#"match (|| -> Result<Rc<RefCell<{parent_type}>>, Error> {{
                            let {obj_ident} = self.store.borrow().exhume_{parent_ident}(&{obj_const}).unwrap();

                            Ok({obj_ident})
                        }})() {{
                            Ok({obj_ident}) => {{
                                let this = {parent_type}Proxy {{ inner: {obj_ident}.clone(), store: self.store.clone() }};
                                let plugin = Plugin_TO::from_value(this, TD_CanDowncast);
                                let proxy = FfiProxy {{
                                    module: module.into(),
                                    uuid: {obj_const}.into(),
                                    id: {obj_ident}.borrow().id().into(),
                                    plugin: plugin.clone(),
                                }};
                                Ok(FfiValue::ProxyType(proxy))
                            }}
                            Err(e) => Err(e),
                        }}
                    }}"#
            );
        }
    } else {
        if let Some(parent) = parent_obj {
            let is_singleton = object_is_singleton(parent, config, imports, domain)?;
            let is_enum = object_is_enum(parent, config, imports, domain)?;

            // dbg!(&parent, is_singleton, is_enum);

            let parent_type = parent.as_type(&Ownership::new_owned(), woog, domain);
            // let prelude = format!(
            //     "let id = Uuid::new_v4();
            //  let {obj_ident} = {parent_type} {{
            //     id,"
            // );
            let prelude = if is_enum {
                format!("let {obj_ident} = {parent_type}")
            } else {
                format!(
                    "let id = Uuid::new_v4();
             let {obj_ident} = {parent_type} {{
                id,"
                )
            };

            emit!(
                buffer,
                "match (|| -> Result<{parent_type}, Error> {{
                {prelude}"
            );

            //                                                 v yes! v
            // If this is a singleton (should we be checking enum too?) then we don't
            // expect the subtype to be passed in. This is actually sort of silly because
            // it's making things overly complex here. I could move that to the dwarf
            // code, but it's generated too. Probably a wash?
            if is_singleton {
                emit!(
                    buffer,
                    "subtype: {parent_type}Enum::{obj_type}({obj_const}),"
                )
            } else if is_enum {
                emit!(
                    buffer,
                    r#"::{obj_type}(value_args.pop().unwrap().try_into().map_err(|e| {{
                        Error::Uber(format!("Error converting value: {{e}}").into())
                    }})?);"#
                );
            } else {
                emit!(
                    buffer,
                    r#"subtype: {parent_type}Enum::{obj_type}(value_args.pop().unwrap().try_into().map_err(|e| {{
                        Error::Uber(format!("Error converting value: {{e}}").into())
                    }})?),"#
                );
            }
        } else {
            // singletons/enums don't have an id attribute.
            let prelude = if is_singleton || is_enum {
                format!("let {obj_ident} = {obj_type} {{ //kts")
            } else {
                format!(
                    "let id = Uuid::new_v4();
             let {obj_ident} = {obj_type} {{
                id,"
                )
            };
            emit!(
                buffer,
                "match (|| -> Result<{obj_type}, Error> {{
                        {prelude}"
            );
        }

        for attr in attrs {
            if attr.name != "id" {
                let attr_ident = attr.name.as_ident();

                emit!(
                    buffer,
                    r#"{attr_ident}: value_args.pop().unwrap().try_into().map_err(|e| {{
                                    Error::Uber(format!("Error converting value: {{e}}").into())
                                }})?,"#
                );
            }
        }

        // This is the case where we are a subtype, and we need to create the parent
        // object.
        if let Some(parent) = parent_obj {
            let parent_type = parent.as_type(&Ownership::new_owned(), woog, domain);
            let parent_const = parent.as_const();
            let parent_ident = parent.as_ident();
            let is_enum = object_is_enum(parent, config, imports, domain)?;
            let is_singleton = object_is_singleton(parent, config, imports, domain)?;

            if !is_enum {
                emit!(buffer, "}};");
            }

            let id = if is_singleton || is_enum {
                "id()"
            } else {
                "id"
            };

            emit!(
                buffer,
                r#"
                    Ok({obj_ident})
                        }})() {{
                            Ok({obj_ident}) => {{
                                let {obj_ident} = Rc::new(RefCell::new({obj_ident}));
                                self.store.borrow_mut().inter_{parent_ident}({obj_ident}.clone());
                                let this = {parent_type}Proxy {{ inner: {obj_ident}.clone(), store: self.store.clone() }};
                                let plugin = Plugin_TO::from_value(this, TD_CanDowncast);
                                let proxy = FfiProxy {{
                                    module: module.into(),
                                    uuid: {parent_const}_ID.into(),
                                    id: {obj_ident}.borrow().{id}.into(), // d
                                    plugin: plugin.clone(),
                                }};

                                Ok(FfiValue::ProxyType(proxy))
                            }}
                            Err(e) => Err(e),
                        }}}},"#
            );
        } else {
            let id = if is_enum || is_singleton {
                "id()"
            } else {
                "id"
            };

            emit!(
                buffer,
                r#"}};

                            Ok({obj_ident})
                        }})() {{
                            Ok({obj_ident}) => {{
                                let {obj_ident} = Rc::new(RefCell::new({obj_ident}));
                                self.store.borrow_mut().inter_{obj_ident}({obj_ident}.clone());
                                let this = {obj_type}Proxy {{ inner: {obj_ident}.clone(), store: self.store.clone() }};
                                let plugin = Plugin_TO::from_value(this, TD_CanDowncast);
                                let proxy = FfiProxy {{
                                    module: module.into(),
                                    uuid: {obj_const}_ID.into(),
                                    id: {obj_ident}.borrow().{id}.into(), // e
                                    plugin: plugin.clone(),
                                }};

                                Ok(FfiValue::ProxyType(proxy))
                            }}
                            Err(e) => Err(e),
                        }}}},"#
            );
        }
    } // it is_enum

    Ok(())
}

fn value_type_to_string<'a>(
    ty: &Arc<Lock<ValueType>>,
    woog: &WoogStore,
    config: &GraceConfig,
    domain: &Domain,
) -> (&'a str, String) {
    let lu_dog = &LU_DOG;

    match &*s_read!(ty) {
        ValueType::Reference(ref id) => {
            let inner = {
                let lu_dog = lu_dog.read().unwrap();
                let reference = lu_dog.exhume_reference(id).unwrap();
                let reference = s_read!(reference);
                reference.r35_value_type(&lu_dog)[0].clone()
            };

            let (ty, ty_ty) = value_type_to_string(&inner, woog, config, domain);
            // dbg!(ty, ty_ty);
            if ty == "Imported" {
                ("Imported", ty_ty)
            } else {
                ("UserType", ty_ty)
            }
            // (
            //     "UserType",
            //     value_type_to_string(&inner, woog, config, domain).1,
            // )
        }
        ValueType::Ty(ref id) => {
            let ty = domain.sarzak().exhume_ty(id).unwrap();
            match ty {
                Ty::Object(ref id) => {
                    let obj = domain.sarzak().exhume_object(id).unwrap();
                    if config.is_imported(id) {
                        (
                            "Imported",
                            obj.as_type(&Ownership::new_owned(), woog, domain),
                        )
                    } else {
                        ("Object", obj.as_type(&Ownership::new_owned(), woog, domain))
                    }
                }
                Ty::SString(_) => ("String", "".to_owned()),
                Ty::Boolean(_) => ("Boolean", "".to_owned()),
                Ty::Integer(_) => ("Integer", "".to_owned()),
                Ty::Float(_) => ("Float", "".to_owned()),
                Ty::SUuid(_) => ("Uuid", "".to_owned()),
                Ty::External(_) => ("ext_what_to_do", "".to_owned()),
            }
        }
        ValueType::WoogOption(ref id) => {
            let inner = {
                let lu_dog = lu_dog.read().unwrap();
                let option = lu_dog.exhume_woog_option(id).unwrap();
                let option = s_read!(option);
                option.r2_value_type(&lu_dog)[0].clone()
            };
            (
                "Option",
                value_type_to_string(&inner, woog, config, domain).1,
            )
        }
        ValueType::WoogStruct(ref id) => {
            let lu_dog = lu_dog.read().unwrap();
            let woog_struct = lu_dog.exhume_woog_struct(id).unwrap();
            let woog_struct = s_read!(woog_struct);
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

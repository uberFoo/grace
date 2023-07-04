//! Domain Enum Generation
//!
//! Here we are.
use std::fmt::Write;

use fnv::{FnvHashMap as HashMap, FnvHashSet as HashSet};
use sarzak::{
    mc::{CompilerSnafu, FormatSnafu, Result},
    v2::domain::Domain,
    woog::{store::ObjectStore as WoogStore, Ownership},
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        emit_object_comments, find_store, get_assoc_referrer_obj_from_obj_via_assoc_referent,
        get_binary_referents_sorted, get_binary_referrers_sorted,
        get_objs_for_assoc_referrers_sorted, get_objs_for_binary_referents_sorted,
        get_objs_for_binary_referrers_sorted, get_subtypes_sorted,
        get_subtypes_sorted_from_super_obj, object_is_enum, object_is_hybrid, object_is_singleton,
        object_is_supertype,
        render::{RenderConst, RenderIdent, RenderType},
    },
    options::{GraceConfig, UberStoreOptions},
    types::{
        domain::rels::{
            generate_assoc_referent_rels, generate_assoc_referrer_rels,
            generate_binary_referent_rels, generate_binary_referrer_rels, generate_subtype_rels,
        },
        CodeWriter, MethodImplementation, TypeDefinition,
    },
};

/// Domain Enum Generator / CodeWriter
///
pub(crate) struct Enum;

impl Enum {
    pub(crate) fn new() -> Box<dyn TypeDefinition> {
        Box::new(Self)
    }
}

impl TypeDefinition for Enum {}

impl CodeWriter for Enum {
    fn write_code(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        woog: &Option<&mut WoogStore>,
        imports: &Option<&HashMap<String, Domain>>,
        _package: &str,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by Enum"
            }
        );
        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();
        ensure!(
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by Enum"
            }
        );
        let woog = woog.as_ref().unwrap();

        // These need to be sorted, as they are output as attributes and we require
        // stable output.
        let mut referrer_objs = get_objs_for_binary_referrers_sorted!(obj, domain.sarzak());
        referrer_objs.append(&mut get_assoc_referrer_obj_from_obj_via_assoc_referent!(
            obj,
            domain.sarzak()
        ));
        let referrer_objs: HashSet<_> = referrer_objs.into_iter().collect();
        // Remove ourselves, should that happen. Spoiler alert: it does.
        let referrer_objs: HashSet<_> = referrer_objs
            .into_iter()
            .filter(|r_obj| r_obj.id != obj.id)
            .collect();

        let mut referent_objs = get_objs_for_binary_referents_sorted!(obj, domain.sarzak());
        referent_objs.append(&mut get_objs_for_assoc_referrers_sorted!(
            obj,
            domain.sarzak()
        ));
        let referent_objs: HashSet<_> = referent_objs.into_iter().collect();
        // Remove ourselves, should that happen. Spoiler alert: it does.
        let referent_objs: HashSet<_> = referent_objs
            .into_iter()
            .filter(|r_obj| r_obj.id != obj.id)
            .collect();

        let subtypes = get_subtypes_sorted_from_super_obj!(obj, domain.sarzak());

        // Output the use statements.
        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-use-statements", obj.as_ident()),
            |buffer| {
                let mut stores = HashSet::default();
                let mut uses = HashSet::default();
                let mut import_store = false;

                if config.is_uber_store() {
                    use UberStoreOptions::*;
                    match config.get_uber_store().unwrap() {
                        Disabled => unreachable!(),
                        AsyncRwLock => {
                            emit!(buffer, "use async_std::sync::Arc;");
                            emit!(buffer, "use async_std::sync::RwLock;");
                            // emit!(buffer, "use futures::{{future::OptionFuture, stream::{{self, StreamExt}}}};");
                        }
                        NDRwLock => {
                            emit!(buffer, "use std::sync::Arc;");
                            emit!(buffer, "use no_deadlocks::RwLock;");
                        }
                        Single => {
                            emit!(buffer, "use std::cell::RefCell;");
                            emit!(buffer, "use std::rc::Rc;")
                        }
                        StdRwLock => {
                            emit!(buffer, "use std::sync::Arc;");
                            emit!(buffer, "use std::sync::RwLock;")
                        }
                        StdMutex => {
                            emit!(buffer, "use std::sync::Arc;");
                            emit!(buffer, "use std::sync::Mutex;")
                        }
                        ParkingLotRwLock => {
                            emit!(buffer, "use std::sync::Arc;");
                            emit!(buffer, "use parking_lot::RwLock;")
                        }
                        ParkingLotMutex => {
                            emit!(buffer, "use std::sync::Arc;");
                            emit!(buffer, "use parking_lot::Mutex;")
                        }
                    };
                    uses.insert("use tracy_client::span;".to_owned());
                }

                // Everything has an `id`, everything needs this.
                uses.insert("use uuid::Uuid;".to_owned());

                // Add the use statements from the options.
                if let Some(use_paths) = config.get_use_paths(&obj.id) {
                    for path in use_paths {
                        uses.insert(format!("use {};", path));
                    }
                }

                // 🚧 I don't think that this will ever apply, otherwise it would
                // be a hybrid. This use-statement logic can probably be refactored.
                //
                // Add use statements for all the referrers.
                for r_obj in &referrer_objs {
                    if config.is_imported(&r_obj.id) {
                        let imported_object = config.get_imported(&r_obj.id).unwrap();
                        stores.insert(imported_object.domain.as_str());
                        uses.insert(format!(
                            "use crate::{}::types::{}::{};",
                            imported_object.domain,
                            r_obj.as_ident(),
                            r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                        ));
                    } else {
                        import_store = true;
                        uses.insert(format!(
                            "use crate::{}::types::{}::{};",
                            module,
                            r_obj.as_ident(),
                            r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                        ));
                    }
                }

                // Add use statements for all the referents.
                for r_obj in &referent_objs {
                    import_store = true;
                    uses.insert(format!(
                        "use crate::{}::types::{}::{};",
                        module,
                        r_obj.as_ident(),
                        r_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    ));
                }

                // Add use statements for supertypes.
                for subtype in get_subtypes_sorted!(obj, domain.sarzak()) {
                    let isa = subtype.r27_isa(domain.sarzak())[0];
                    let supertype = isa.r13_supertype(domain.sarzak())[0];
                    let s_obj = supertype.r14_object(domain.sarzak())[0];

                    import_store = true;

                    uses.insert(format!(
                        "use crate::{}::types::{}::{};",
                        module,
                        s_obj.as_ident(),
                        s_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    ));

                    if object_is_hybrid(s_obj, config, imports, domain)? {
                        uses.insert(format!(
                            "use crate::{}::types::{}::{}Enum;",
                            module,
                            s_obj.as_ident(),
                            s_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                        ));
                    }
                }

                let mut only_singletons = true;
                for subtype in &subtypes {
                    let s_obj = subtype.r15_object(domain.sarzak())[0];

                    let is_singleton = object_is_singleton(s_obj, config, imports, domain)?;
                    let is_supertype = object_is_supertype(s_obj, config, imports, domain)?;

                    if config.is_imported(&s_obj.id) {
                        let imported_object = config.get_imported(&s_obj.id).unwrap();
                        if is_singleton && !is_supertype {
                            uses.insert(format!(
                                "use crate::{}::types::{}::{};",
                                imported_object.domain,
                                s_obj.as_ident(),
                                s_obj.as_const()
                            ));
                        } else {
                            only_singletons = false;
                            uses.insert(format!(
                                "use crate::{}::types::{}::{};",
                                imported_object.domain,
                                s_obj.as_ident(),
                                s_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                            ));
                        }
                    } else if is_singleton && !is_supertype {
                        uses.insert(format!(
                            "use crate::{}::types::{}::{};",
                            module,
                            s_obj.as_ident(),
                            s_obj.as_const()
                        ));
                    } else {
                        only_singletons = false;
                        uses.insert(format!(
                            "use crate::{}::types::{}::{};",
                            module,
                            s_obj.as_ident(),
                            s_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                        ));
                    }
                }

                // Add the use statements, plus the use for any imported objects.
                for use_path in uses {
                    emit!(buffer, "{}", use_path);
                }

                // Add the ObjectStore, plus the store for any imported objects.
                if import_store || !only_singletons {
                    stores.insert(module);
                }
                for import in stores {
                    let store = find_store(import, woog, domain);
                    emit!(buffer, "use {} as {};", store.path, store.name);
                }

                Ok(())
            },
        )?;

        emit!(buffer, "");

        log::debug!("writing Enum Definition for {}", obj.name);

        // Documentation
        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-enum-documentation", obj.as_ident()),
            |buffer| emit_object_comments(obj.description.as_str(), "/// ", "", buffer),
        )?;

        // Enum Definition
        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-enum-definition", obj.as_ident()),
            |buffer| {
                if let Some(derives) = config.get_derives(&obj.id) {
                    // So our enums are Copy, which is sweet. I just wish I'd
                    // noticed earlier. Probably I wasn't as good as I am now,
                    // and it never even crossed my event horizon.
                    write!(buffer, "#[derive(Copy, ").context(FormatSnafu)?;
                    for d in derives {
                        write!(buffer, "{},", d).context(FormatSnafu)?;
                    }
                    emit!(buffer, ")]");
                }

                emit!(
                    buffer,
                    "pub enum {} {{",
                    obj.as_type(&Ownership::new_borrowed(), woog, domain)
                );

                if let crate::options::OptimizationLevel::Vec = config.get_optimization_level() {
                    // We need to plumb the depths of this tree to see if any of our
                    // subtypes are also supertypes, and if they consist of only
                    // singletons. If that's the case, we need to expand them out here,
                    // so that
                    let all_single = subtypes.iter().all(|subtype| {
                        let s_obj = subtype.r15_object(domain.sarzak())[0];
                        // These unwraps really bug me, and I don't know what else to do
                        // inside of an `all` adaptor.
                        let is_singleton =
                            object_is_singleton(s_obj, config, imports, domain).unwrap();
                        let is_supertype =
                            object_is_supertype(s_obj, config, imports, domain).unwrap();

                        is_singleton && !is_supertype
                    });
                    for (i, subtype) in subtypes.iter().enumerate() {
                        let s_obj = subtype.r15_object(domain.sarzak())[0];

                        if all_single {
                            emit!(
                                buffer,
                                "{} = {i},",
                                s_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                            );
                        } else {
                            emit!(
                                buffer,
                                "{}(usize),",
                                s_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                            );
                        }
                    }
                } else {
                    for subtype in &subtypes {
                        let s_obj = subtype.r15_object(domain.sarzak())[0];

                        emit!(
                            buffer,
                            "{}(Uuid),",
                            s_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                        );
                    }
                }
                emit!(buffer, "}}");
                Ok(())
            },
        )?;

        Ok(())
    }
}

pub(crate) struct EnumGetIdImpl;

impl EnumGetIdImpl {
    pub(crate) fn new() -> Box<dyn MethodImplementation> {
        Box::new(Self)
    }
}

impl MethodImplementation for EnumGetIdImpl {}

impl CodeWriter for EnumGetIdImpl {
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
                description: "obj_id is required by EnumGetIdImpl"
            }
        );
        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();
        ensure!(
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by EnumGetIdImpl"
            }
        );
        let woog = woog.as_ref().unwrap();

        let subtypes = get_subtypes_sorted_from_super_obj!(obj, domain.sarzak());

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-get-id-impl", obj.as_ident()),
            |buffer| {
                if let crate::options::OptimizationLevel::Vec = config.get_optimization_level() {
                    emit!(buffer, "pub fn id(&self) -> usize {{");
                } else {
                    emit!(buffer, "pub fn id(&self) -> Uuid {{");
                }
                emit!(buffer, "match self {{");
                for subtype in subtypes {
                    let s_obj = subtype.r15_object(domain.sarzak())[0];
                    if let crate::options::OptimizationLevel::Vec = config.get_optimization_level()
                    {
                        emit!(
                            buffer,
                            "Self::{} => Self::{} as usize,",
                            s_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                            s_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                        );
                    } else {
                        emit!(
                            buffer,
                            "Self::{}(id) => *id,",
                            s_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                        );
                    }
                }
                emit!(buffer, "}}");
                emit!(buffer, "}}");
                Ok(())
            },
        )?;

        Ok(())
    }
}

pub(crate) struct EnumRelNavImpl;

impl EnumRelNavImpl {
    pub(crate) fn new() -> Box<dyn MethodImplementation> {
        Box::new(Self)
    }
}

impl MethodImplementation for EnumRelNavImpl {}

impl CodeWriter for EnumRelNavImpl {
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
                description: "obj_id is required by EnumRelNavImpl"
            }
        );
        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();
        ensure!(
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by EnumRelNavImpl"
            }
        );
        let woog = woog.as_ref().unwrap();

        generate_binary_referrer_rels(buffer, config, module, obj, woog, domain)?;
        generate_binary_referent_rels(buffer, config, module, obj, "id()", woog, domain)?;
        generate_assoc_referrer_rels(buffer, config, module, obj, woog, domain)?;
        generate_assoc_referent_rels(buffer, config, module, obj, "id()", woog, domain)?;
        generate_subtype_rels(buffer, config, module, obj, woog, domain)?;

        Ok(())
    }
}

pub(crate) struct EnumNewImpl;

impl EnumNewImpl {
    pub(crate) fn new() -> Box<dyn MethodImplementation> {
        Box::new(Self)
    }
}

impl MethodImplementation for EnumNewImpl {}

impl CodeWriter for EnumNewImpl {
    fn write_code(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        woog: &Option<&mut WoogStore>,
        imports: &Option<&HashMap<String, Domain>>,
        _package: &str,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by EnumNewImpl"
            }
        );
        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();
        let obj_ident = obj.as_ident();

        ensure!(
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by EnumNewImpl"
            }
        );
        let woog = woog.as_ref().unwrap();

        let store = find_store(module, woog, domain);
        let subtypes = get_subtypes_sorted_from_super_obj!(obj, domain.sarzak());

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-new-impl", obj.as_ident()),
            |buffer| {
                let is_uber = config.is_uber_store() && !config.is_imported(&obj.id);

                for subtype in subtypes {
                    let s_obj = subtype.r15_object(domain.sarzak())[0];
                    let s_obj_ident = s_obj.as_ident();

                    let is_singleton = object_is_singleton(s_obj, config, imports, domain)?;
                    let is_supertype = object_is_supertype(s_obj, config, imports, domain)?;
                    let _is_imported = config.is_imported(&s_obj.id);

                    emit!(
                        buffer,
                        "/// Create a new instance of {}::{}",
                        obj.as_type(&Ownership::new_borrowed(), woog, domain),
                        s_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                    );

                    if is_singleton && !is_supertype {
                        if is_uber {
                            use UberStoreOptions::*;
                            let ret_type = match config.get_uber_store().unwrap() {
                                Disabled => unreachable!(),
                                Single => "Rc<RefCell<Self>>",
                                StdRwLock |
                                ParkingLotRwLock |
                                AsyncRwLock |
                                NDRwLock => "Arc<RwLock<Self>>",
                                StdMutex | ParkingLotMutex => "Arc<Mutex<Self>>",
                            };
                            if let AsyncRwLock = config.get_uber_store().unwrap() {
                                emit!(
                                    buffer,
                                    "pub async fn new_{s_obj_ident}(store: &{}) -> {ret_type} {{",
                                    store.name
                                );
                                emit!(
                                    buffer,
                                    "// This is already in the store."
                                );
                                emit!(
                                    buffer,
                                    "store.exhume_{obj_ident}(&{}).await.unwrap()",
                                    s_obj.as_const()
                                );
                            } else {
                                if let crate::options::OptimizationLevel::Vec = config.get_optimization_level()
                                {
                                    emit!(
                                        buffer,
                                        "pub fn new_{s_obj_ident}(store: &{}) -> {ret_type} {{",
                                        store.name
                                    );
                                    emit!(
                                        buffer,
                                        "// This is already in the store."
                                    );
                                    emit!(
                                        buffer,
                                        "store.exhume_{obj_ident}(Self::{} as usize).unwrap()",
                                        s_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                                    );
                                } else {
                                    emit!(
                                        buffer,
                                        "pub fn new_{s_obj_ident}(store: &{}) -> {ret_type} {{",
                                        store.name
                                    );
                                    emit!(
                                        buffer,
                                        "// This is already in the store."
                                    );
                                    emit!(
                                        buffer,
                                        "store.exhume_{obj_ident}(&{}).unwrap()",
                                        s_obj.as_const()
                                    );
                                }
                            }

                        } else {
                            emit!(buffer, "pub fn new_{s_obj_ident}() -> Self {{");
                            emit!(
                                buffer,
                                "// This is already in the store, see associated function `new` above."
                            );
                            emit!(
                                buffer,
                                "Self::{}({})",
                                s_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                                s_obj.as_const()
                            );
                        }
                        emit!(buffer, "}}");
                        emit!(buffer, "");
                    } else {
                        if is_uber {
                            // if is_imported {
                            //     emit!(
                            //         buffer,
                            //         "pub fn new_{}({}: &{}, store: &mut {}) -> Arc<RwLock<Self>> {{",
                            //         s_obj.as_ident(),
                            //         s_obj.as_ident(),
                            //         s_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                            //         store.name
                            //     );
                            // } else {
                                use UberStoreOptions::*;
                                let ret_type = match config.get_uber_store().unwrap() {
                                    Disabled => unreachable!(),
                                    Single => "Rc<RefCell<Self>>",
                                    StdRwLock |
                                    ParkingLotRwLock |
                                    AsyncRwLock |
                                    NDRwLock => "Arc<RwLock<Self>>",
                                    StdMutex | ParkingLotMutex => "Arc<Mutex<Self>>",
                                };
                                let store_type = match config.get_uber_store().unwrap() {
                                    Disabled => unreachable!(),
                                    Single => format!(
                                        "Rc<RefCell<{}>>",
                                        s_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                                    ),
                                    StdRwLock |
                                    ParkingLotRwLock |
                                    AsyncRwLock |
                                    NDRwLock => format!(
                                        "Arc<RwLock<{}>>",
                                        s_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                                    ),
                                    StdMutex | ParkingLotMutex => format!(
                                        "Arc<Mutex<{}>>",
                                        s_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                                    ),
                                };
                                if let AsyncRwLock = config.get_uber_store().unwrap() {
                                    emit!(
                                        buffer,
                                        "pub async fn new_{s_obj_ident}({s_obj_ident}: &{store_type}, store: &mut {}) -> {ret_type} {{",
                                        store.name
                                    );
                                } else {
                                    emit!(
                                        buffer,
                                        "pub fn new_{s_obj_ident}({s_obj_ident}: &{store_type}, store: &mut {}) -> {ret_type} {{",
                                        store.name
                                    );
                                }

                            // }
                        } else {
                            emit!(
                                buffer,
                                "pub fn new_{s_obj_ident}({s_obj_ident}: &{}, store: &mut {}) -> Self {{",
                                s_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                                store.name
                            );
                        }

                        // I feel sort of gross doing this, but also sort of not. Part of me feels
                        // like I should move this, and the same idea in codegen::render_new_instance,
                        // into a function. Refactor the bits. But then the other part of me wants to
                        // see how this plays out once woog comes into play. I have a feeling that
                        // I should be able to build the let statement in terms of woog and then
                        // have it write itself. So for now, here we are. I'm only here because I'm
                        // trying to get woog working, so that's sort of funny.
                        let id = if object_is_enum(s_obj, config, imports, domain)? {
                            "id()"
                        } else {
                            "id"
                        };

                        if is_uber {
                            // if is_imported {
                            //     emit!(
                            //         buffer, "if let Some({}) = store.exhume_{}(&{}.{id}) {{",
                            //         s_obj.as_ident(),
                            //         obj.as_ident(),
                            //         s_obj.as_ident()
                            //     );
                            //     emit!(buffer, "{}", s_obj.as_ident());
                            //     emit!(buffer, "}} else {{");
                            //     emit!(
                            //         buffer,
                            //         "let new = Arc::new(RwLock::new(Self::{}({}.{id})));",
                            //         s_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                            //         s_obj.as_ident()
                            //     );
                            //     emit!(buffer, "store.inter_{}(new.clone());", obj.as_ident());
                            //     emit!(buffer, "new");
                            //     emit!(buffer, "}}");
                            // } else {
                                use UberStoreOptions::*;
                                let read = match config.get_uber_store().unwrap() {
                                    Disabled => unreachable!(),
                                    AsyncRwLock => ".read().await",
                                    NDRwLock => ".read().unwrap()",
                                    Single => ".borrow()",
                                    StdRwLock => ".read().unwrap()",
                                    StdMutex => ".lock().unwrap()",
                                    ParkingLotRwLock => ".read()",
                                    ParkingLotMutex => ".lock()",
                                };
                                let (ctor, tail) = match config.get_uber_store().unwrap() {
                                    Disabled => unreachable!(),
                                    Single => (format!(
                                        "Rc::new(RefCell::new(Self::{}",
                                        s_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                                    ), "))"),
                                    StdRwLock |
                                    ParkingLotRwLock |
                                    AsyncRwLock |
                                    NDRwLock => (format!(
                                        "Arc::new(RwLock::new(Self::{}",
                                        s_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                                    ), "))"),
                                    StdMutex | ParkingLotMutex => (format!(
                                        "Arc::new(Mutex::new(Self::{}",
                                        s_obj.as_type(&Ownership::new_borrowed(), woog, domain)
                                    ), "))"),
                                };

                                emit!(buffer, "let id = {s_obj_ident}{read}.{id};");

                                if let AsyncRwLock = config.get_uber_store().unwrap() {
                                    emit!(
                                        buffer, "if let Some({s_obj_ident}) = store.exhume_{obj_ident}(&id).await {{"
                                    );
                                    emit!(buffer, "{s_obj_ident}");
                                    emit!(buffer, "}} else {{");
                                    emit!(
                                        buffer,
                                        "let new = {ctor}(id){tail};"
                                    );
                                    emit!(buffer, "store.inter_{obj_ident}(new.clone()).await;");
                                } else {
                                    use UberStoreOptions::*;
                                    match config.get_uber_store().unwrap() {
                                        StdRwLock => {
                                            emit!(
                                                buffer, "if let Some({s_obj_ident}) = store.exhume_{obj_ident}(&id) {{"
                                            );
                                            emit!(buffer, "{s_obj_ident}");
                                            emit!(buffer, "}} else {{");
                                            emit!(
                                                buffer,
                                                "let new = {ctor}(id){tail};"
                                            );
                                            emit!(buffer, "store.inter_{obj_ident}(new.clone());");
                                            emit!(buffer, "new");
                                        }
                                        Single => {
                                            emit!(
                                                buffer, "if let Some({s_obj_ident}) = store.exhume_{obj_ident}(id) {{"
                                            );
                                            emit!(buffer, "{s_obj_ident}");
                                            emit!(buffer, "}} else {{");
                                            emit!(buffer, "store.inter_{obj_ident}(|id| {{");
                                            emit!(
                                                buffer,
                                                "{ctor}(id){tail}"
                                            );
                                            emit!(buffer, "}})");
                                        }
                                        store => panic!("{store} is not currently supported"),
                                    }
                                }

                                emit!(buffer, "}}");
                            // }
                        } else {
                            emit!(
                                buffer,
                                "let new = Self::{}({s_obj_ident}.{id});",
                                s_obj.as_type(&Ownership::new_borrowed(), woog, domain),
                            );
                            emit!(buffer, "store.inter_{}(new.clone());", obj.as_ident());
                            emit!(buffer, "new");
                        };

                        emit!(buffer, "}}");
                        emit!(buffer, "");
                    }
                }
                Ok(())
            },
        )?;

        Ok(())
    }
}

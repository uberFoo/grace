//! Domain Enum with extras Generation
//!
//! Here we are.
use std::fmt::Write;

use fnv::{FnvHashMap as HashMap, FnvHashSet as HashSet};
use sarzak::{
    mc::{CompilerSnafu, FormatSnafu, Result},
    v2::domain::Domain,
    woog::{store::ObjectStore as WoogStore, types::SHARED},
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
        get_objs_for_binary_referrers_sorted, get_subtypes_sorted, object_is_singleton,
        object_is_supertype,
        render::{
            render_associative_attributes, render_attributes, render_referential_attributes,
            RenderConst, RenderIdent, RenderType,
        },
        render_methods,
    },
    options::GraceConfig,
    types::{CodeWriter, MethodImplementation, TypeDefinition},
};

const SUBTYPE_ATTR: &str = "subtype";

/// Domain Hybrid Generator / CodeWriter
///
pub(crate) struct Hybrid;

impl Hybrid {
    pub(crate) fn new() -> Box<dyn TypeDefinition> {
        Box::new(Self)
    }
}

impl TypeDefinition for Hybrid {}

impl CodeWriter for Hybrid {
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
                description: "obj_id is required by Hybrid"
            }
        );
        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();
        ensure!(
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by DomainStore"
            }
        );
        let woog = woog.as_ref().unwrap();

        let subtypes = get_subtypes_sorted!(obj, domain.sarzak());

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

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-use-statements", obj.as_ident()),
            |buffer| {
                let mut imported_domains = HashSet::default();
                let mut uses = HashSet::default();

                // Everything has an `id`, everything needs this.
                emit!(buffer, "use uuid::Uuid;");
                emit!(buffer, "");

                // Add the use statements from the options.
                if let Some(use_paths) = config.get_use_paths(&obj.id) {
                    for path in use_paths {
                        uses.insert(format!("use {};", path));
                    }
                }

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
                            uses.insert(format!(
                                "use crate::{}::types::{}::{};",
                                imported_object.domain,
                                s_obj.as_ident(),
                                s_obj.as_type(
                                    &woog
                                        .exhume_ownership(
                                            &woog.exhume_borrowed(&SHARED).unwrap().id()
                                        )
                                        .unwrap(),
                                    woog,
                                    domain
                                )
                            ));
                        }
                    } else {
                        if is_singleton && !is_supertype {
                            uses.insert(format!(
                                "use crate::{}::types::{}::{};",
                                module,
                                s_obj.as_ident(),
                                s_obj.as_const()
                            ));
                        } else {
                            uses.insert(format!(
                                "use crate::{}::types::{}::{};",
                                module,
                                s_obj.as_ident(),
                                s_obj.as_type(
                                    &woog
                                        .exhume_ownership(
                                            &woog.exhume_borrowed(&SHARED).unwrap().id()
                                        )
                                        .unwrap(),
                                    woog,
                                    domain
                                )
                            ));
                        }
                    }
                }

                // Add use statements for all the referrers.
                for r_obj in &referrer_objs {
                    if config.is_imported(&r_obj.id) {
                        let imported_object = config.get_imported(&r_obj.id).unwrap();
                        imported_domains.insert(imported_object.domain.as_str());
                        uses.insert(format!(
                            "use crate::{}::types::{}::{};",
                            imported_object.domain,
                            r_obj.as_ident(),
                            r_obj.as_type(
                                &woog
                                    .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                                    .unwrap(),
                                woog,
                                domain
                            )
                        ));
                    } else {
                        uses.insert(format!(
                            "use crate::{}::types::{}::{};",
                            module,
                            r_obj.as_ident(),
                            r_obj.as_type(
                                &woog
                                    .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                                    .unwrap(),
                                woog,
                                domain
                            )
                        ));
                    }
                }

                // Add use statements for all the referents.
                for r_obj in &referent_objs {
                    uses.insert(format!(
                        "use crate::{}::types::{}::{};",
                        module,
                        r_obj.as_ident(),
                        r_obj.as_type(
                            &woog
                                .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                                .unwrap(),
                            woog,
                            domain
                        )
                    ));
                }

                // Ad use statements for supertypes.
                for subtype in obj.r15_subtype(domain.sarzak()) {
                    let isa = subtype.r27_isa(domain.sarzak())[0];
                    let supertype = isa.r13_supertype(domain.sarzak())[0];
                    let s_obj = supertype.r14_object(domain.sarzak())[0];

                    uses.insert(format!(
                        "use crate::{}::types::{}::{};",
                        module,
                        s_obj.as_ident(),
                        s_obj.as_type(
                            &woog
                                .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                                .unwrap(),
                            woog,
                            domain
                        )
                    ));
                }

                // Add the ObjectStore, plus the store for any imported objects.
                for use_path in uses {
                    emit!(buffer, "{}", use_path);
                }

                imported_domains.insert(module);
                emit!(buffer, "");
                for import in imported_domains {
                    let store = find_store(import, woog, domain);
                    emit!(buffer, "use {} as {};", store.path, store.name);
                }

                Ok(())
            },
        )?;
        emit!(buffer, "");

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-hybrid-documentation", obj.as_ident()),
            |buffer| emit_object_comments(obj.description.as_str(), "///", buffer),
        )?;

        log::debug!("writing Struct Definition for {}", obj.name);
        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-hybrid-struct-definition", obj.as_ident()),
            |buffer| {
                if let Some(derives) = config.get_derives(&obj.id) {
                    write!(buffer, "#[derive(").context(FormatSnafu)?;
                    for d in derives {
                        write!(buffer, "{},", d).context(FormatSnafu)?;
                    }
                    emit!(buffer, ")]");
                }

                emit!(
                    buffer,
                    "pub struct {} {{",
                    obj.as_type(
                        &woog
                            .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                            .unwrap(),
                        woog,
                        domain
                    )
                );

                emit!(
                    buffer,
                    "pub {}: {}Enum,",
                    SUBTYPE_ATTR,
                    obj.as_type(
                        &woog
                            .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                            .unwrap(),
                        woog,
                        domain
                    )
                );

                render_attributes(buffer, obj, woog, domain)?;

                render_referential_attributes(buffer, obj, woog, domain)?;

                render_associative_attributes(buffer, obj, woog, domain)?;

                emit!(buffer, "}}");
                Ok(())
            },
        )?;

        log::debug!("writing Enum Definition for {}", obj.name);
        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-hybrid-enum-definition", obj.as_ident()),
            |buffer| {
                if let Some(derives) = config.get_derives(&obj.id) {
                    write!(buffer, "#[derive(").context(FormatSnafu)?;
                    for d in derives {
                        write!(buffer, "{},", d).context(FormatSnafu)?;
                    }
                    emit!(buffer, ")]");
                }

                emit!(
                    buffer,
                    "pub enum {}Enum {{",
                    obj.as_type(
                        &woog
                            .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                            .unwrap(),
                        woog,
                        domain
                    )
                );
                for subtype in &subtypes {
                    let s_obj = subtype.r15_object(domain.sarzak())[0];
                    emit!(
                        buffer,
                        "{}(Uuid),",
                        s_obj.as_type(
                            &woog
                                .exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id())
                                .unwrap(),
                            woog,
                            domain
                        ),
                    );
                }
                emit!(buffer, "}}");
                Ok(())
            },
        )?;

        Ok(())
    }
}

/// Hybrid New Implementation
///
/// This generates new implementations for hybrid objects. Plural. One for each
/// subtype. This is sort of lame. Ideally, I think we would have a single
/// implementation that takes the enum that is our subtypes. However, because
/// this is a single object in the model, we have no way to distinguish between
/// the Hybrid enum, and struct. So we have multiple new methods and we never
/// surface the existence of the enum
///
/// __NB__ --- this implies that the lexicographical sum of it's attributes,
/// across all instances, must be unique.
pub(crate) struct HybridNewImpl;

impl HybridNewImpl {
    pub(crate) fn new() -> Box<dyn MethodImplementation> {
        Box::new(Self)
    }
}

impl MethodImplementation for HybridNewImpl {}

impl CodeWriter for HybridNewImpl {
    fn write_code(
        &self,
        config: &GraceConfig,
        domain: &Domain,
        woog: &Option<&mut WoogStore>,
        imports: &Option<&HashMap<String, Domain>>,
        _package: &str,
        _module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by HybridNewImpl"
            }
        );
        ensure!(
            woog.is_some(),
            CompilerSnafu {
                description: "woog is required by HybridNewImpl"
            }
        );
        let woog = match woog {
            Some(ref woog) => woog,
            None => unreachable!(),
        };
        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();

        // let subtypes = get_subtypes_sorted!(obj, domain.sarzak());

        // for subtype in subtypes {
        render_methods(buffer, obj, config, imports, woog, domain)?;
        // }
        Ok(())
    }
}

//! Domain Enum Generation
//!
//! Here we are.
use std::{collections::HashMap, fmt::Write};

use sarzak::{
    mc::{CompilerSnafu, FormatSnafu, Result},
    v2::domain::Domain,
    woog::{store::ObjectStore as WoogStore, Mutability, BORROWED},
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        emit_object_comments, find_store, get_subtypes_sorted, object_is_singleton,
        render::{RenderConst, RenderIdent, RenderType},
    },
    options::GraceConfig,
    types::{CodeWriter, MethodImplementation, TypeDefinition},
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
        _woog: &Option<&mut WoogStore>,
        _imports: &Option<&HashMap<String, Domain>>,
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

        let subtypes = get_subtypes_sorted!(obj, domain.sarzak());

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-use-statements", obj.as_ident()),
            |buffer| {
                // Everything has an `id`, everything needs this.
                emit!(buffer, "use uuid::Uuid;");
                emit!(buffer, "");

                // Add the use statements from the options.
                if let Some(use_paths) = config.get_use_paths(&obj.id) {
                    for path in use_paths {
                        emit!(buffer, "use {};", path);
                    }
                    emit!(buffer, "");
                }

                let mut only_singletons = true;
                for subtype in &subtypes {
                    let s_obj = subtype.r15_object(domain.sarzak())[0];
                    if object_is_singleton(s_obj, domain) {
                        emit!(
                            buffer,
                            "use crate::{}::types::{}::{};",
                            module,
                            s_obj.as_ident(),
                            s_obj.as_const()
                        );
                    } else {
                        only_singletons = false;
                        emit!(
                            buffer,
                            "use crate::{}::types::{}::{};",
                            module,
                            s_obj.as_ident(),
                            s_obj.as_type(&Mutability::Borrowed(BORROWED), domain)
                        );
                    }
                }

                if !only_singletons {
                    let store = find_store(module, domain);
                    emit!(buffer, "use {} as {};", store.path, store.name);
                }

                Ok(())
            },
        )?;

        emit!(buffer, "");

        log::debug!("writing Enum Definition for {}", obj.name);

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-enum-documentation", obj.as_ident()),
            |buffer| emit_object_comments(obj.description.as_str(), "///", buffer),
        )?;

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-enum-definition", obj.as_ident()),
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
                    "pub enum {} {{",
                    obj.as_type(&Mutability::Borrowed(BORROWED), domain)
                );
                for subtype in &subtypes {
                    let s_obj = subtype.r15_object(domain.sarzak())[0];
                    emit!(
                        buffer,
                        "{}(Uuid),",
                        s_obj.as_type(&Mutability::Borrowed(BORROWED), domain),
                    );
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
        _config: &GraceConfig,
        domain: &Domain,
        _woog: &Option<&mut WoogStore>,
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

        let subtypes = get_subtypes_sorted!(obj, domain.sarzak());

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-get-id-impl", obj.as_ident()),
            |buffer| {
                emit!(buffer, "pub fn id(&self) -> Uuid {{");
                emit!(buffer, "match self {{");
                for subtype in subtypes {
                    let s_obj = subtype.r15_object(domain.sarzak())[0];
                    emit!(
                        buffer,
                        "{}::{}(id) => *id,",
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain),
                        s_obj.as_type(&Mutability::Borrowed(BORROWED), domain),
                    );
                }
                emit!(buffer, "}}");
                emit!(buffer, "}}");
                Ok(())
            },
        )?;

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
        _config: &GraceConfig,
        domain: &Domain,
        _woog: &Option<&mut WoogStore>,
        _imports: &Option<&HashMap<String, Domain>>,
        _package: &str,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        ensure!(
            obj_id.is_some(),
            CompilerSnafu {
                description: "obj_id is required by DomainNewImpl"
            }
        );
        let obj_id = obj_id.unwrap();
        let obj = domain.sarzak().exhume_object(obj_id).unwrap();

        let store = find_store(module, domain);
        let subtypes = get_subtypes_sorted!(obj, domain.sarzak());

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-new-impl", obj.as_ident()),
            |buffer| {
                for subtype in subtypes {
                    let s_obj = subtype.r15_object(domain.sarzak())[0];
                    emit!(
                        buffer,
                        "/// Create a new instance of {}::{}",
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain),
                        s_obj.as_type(&Mutability::Borrowed(BORROWED), domain)
                    );
                    if object_is_singleton(s_obj, domain) {
                        emit!(buffer, "pub fn new_{}() -> Self {{", s_obj.as_ident());
                        emit!(
                            buffer,
                            "// This is already in the store, see associated function `new` above."
                        );
                        emit!(
                            buffer,
                            "Self::{}({})",
                            s_obj.as_type(&Mutability::Borrowed(BORROWED), domain),
                            s_obj.as_const()
                        );
                    } else {
                        emit!(
                            buffer,
                            "pub fn new_{}({}: &{}, store: &mut {}) -> Self {{",
                            s_obj.as_ident(),
                            s_obj.as_ident(),
                            s_obj.as_type(&Mutability::Borrowed(BORROWED), domain),
                            store.name
                        );
                        emit!(
                            buffer,
                            "let new = Self::{}({}.id);",
                            s_obj.as_type(&Mutability::Borrowed(BORROWED), domain),
                            s_obj.as_ident()
                        );
                        emit!(buffer, "store.inter_{}(new.clone());", obj.as_ident());
                        emit!(buffer, "new");
                    }
                    emit!(buffer, "}}");
                    emit!(buffer, "");
                }
                Ok(())
            },
        )?;

        Ok(())
    }
}

//! Generate ObjectStore for use in sarzak Domain
//!
use std::fmt::Write;

use sarzak::{
    domain::Domain,
    mc::{CompilerSnafu, FormatSnafu, Result},
    sarzak::{
        macros::{sarzak_get_many_as_across_r1, sarzak_maybe_get_many_r_sups_across_r14},
        types::{Attribute, Object, Supertype},
    },
    woog::{
        store::ObjectStore as WoogStore,
        types::{Mutability, BORROWED},
    },
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::{emit, Buffer},
        diff_engine::DirectiveKind,
        generator::{CodeWriter, FileGenerator},
        render::{RenderIdent, RenderType},
    },
    options::GraceCompilerOptions,
    types::ObjectStoreDefinition,
};

pub(crate) struct DomainStoreBuilder {
    definition: Option<Box<dyn ObjectStoreDefinition>>,
}

impl DomainStoreBuilder {
    pub(crate) fn new() -> Self {
        Self { definition: None }
    }

    pub(crate) fn definition(mut self, definition: Box<dyn ObjectStoreDefinition>) -> Self {
        self.definition = Some(definition);

        self
    }

    pub(crate) fn build(&mut self) -> Result<Box<DomainStoreGenerator>> {
        ensure!(
            self.definition.is_some(),
            CompilerSnafu {
                description: "DomainStoreBuilder::build called before definition".to_owned()
            }
        );

        Ok(Box::new(DomainStoreGenerator {
            definition: self.definition.take().unwrap(),
        }))
    }
}

pub(crate) struct DomainStoreGenerator {
    definition: Box<dyn ObjectStoreDefinition>,
}

impl FileGenerator for DomainStoreGenerator {
    fn generate(
        &self,
        options: &GraceCompilerOptions,
        domain: &Domain,
        woog: &mut WoogStore,
        module: &str,
        obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        // Output the domain/module documentation/description
        emit!(buffer, "//! {} Object Store", module);
        emit!(buffer, "//!");
        emit!(
            buffer,
            "//! The ObjectStore contains instances of objects in the domain."
        );
        emit!(
            buffer,
            "//! The instances are stored in a hash map, keyed by the object's UUID."
        );
        emit!(
            buffer,
            "//! This is used during code generation, and probably not useful elsewhere."
        );
        let mut objects: Vec<(&Uuid, &Object)> = domain.sarzak().iter_object().collect();
        objects.sort_by(|a, b| a.1.name.cmp(&b.1.name));
        let objects = objects
            .iter()
            .filter(|(_, obj)| {
                let attrs = sarzak_get_many_as_across_r1!(obj, domain.sarzak());
                let is_super = sarzak_maybe_get_many_r_sups_across_r14!(obj, domain.sarzak());
                attrs.len() > 1 || is_super.len() > 0
            })
            .collect::<Vec<_>>();

        // We don't want this to be edited -- there's no reason.
        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-object-store-file", module),
            |buffer| {
                emit!(buffer, "//!");
                emit!(buffer, "//! # Contents:");
                emit!(buffer, "//!");
                for (_, obj) in &objects {
                    emit!(
                        buffer,
                        "//! * [`{}`]",
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );
                }

                self.definition
                    .write_code(options, domain, woog, module, obj_id, buffer)?;

                Ok(())
            },
        )?;

        Ok(())
    }
}

pub(crate) struct DomainStore;

impl DomainStore {
    pub(crate) fn new() -> Box<dyn ObjectStoreDefinition> {
        Box::new(Self)
    }
}

impl ObjectStoreDefinition for DomainStore {}

impl CodeWriter for DomainStore {
    fn write_code(
        &self,
        _options: &GraceCompilerOptions,
        domain: &Domain,
        _woog: &mut WoogStore,
        module: &str,
        _obj_id: Option<&Uuid>,
        buffer: &mut Buffer,
    ) -> Result<()> {
        let mut objects: Vec<(&Uuid, &Object)> = domain.sarzak().iter_object().collect();
        objects.sort_by(|a, b| a.1.name.cmp(&b.1.name));
        let objects = objects
            .iter()
            .filter(|(_, obj)| {
                let attrs = sarzak_get_many_as_across_r1!(obj, domain.sarzak());
                let is_super = sarzak_maybe_get_many_r_sups_across_r14!(obj, domain.sarzak());
                attrs.len() > 1 || is_super.len() > 0
            })
            .collect::<Vec<_>>();

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-object-store-definition", module),
            |buffer| {
                emit!(buffer, "use std::collections::HashMap;");
                emit!(buffer, "");
                emit!(buffer, "use serde::{{Deserialize, Serialize}};");
                emit!(buffer, "use uuid::Uuid;");
                emit!(buffer, "");
                emit!(buffer, "use crate::{}::types::{{", module);

                for (_, obj) in &objects {
                    emit!(
                        buffer,
                        "{},",
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );
                }
                emit!(buffer, "}};");
                emit!(buffer, "");
                emit!(buffer, "#[derive(Clone, Debug, Deserialize, Serialize)]");
                emit!(buffer, "pub struct ObjectStore {{");
                for (_, obj) in &objects {
                    emit!(
                        buffer,
                        "{}: HashMap<Uuid,{}>,",
                        obj.as_ident(),
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );
                }
                emit!(buffer, "}}");
                emit!(buffer, "");
                emit!(buffer, "impl ObjectStore {{");
                emit!(buffer, "pub fn new() -> Self {{");
                emit!(buffer, "Self {{");
                for (_, obj) in &objects {
                    emit!(buffer, "{}: HashMap::new(),", obj.as_ident());
                }
                emit!(buffer, "}}");
                emit!(buffer, "}}");
                emit!(buffer, "");
                for (_, obj) in &objects {
                    emit!(
                        buffer,
                        "/// Inter [`{}`] into the store.",
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );
                    emit!(buffer, "///");
                    emit!(
                        buffer,
                        "pub fn inter_{}(&mut self, {}: {}) {{",
                        obj.as_ident(),
                        obj.as_ident(),
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );

                    let is_super = sarzak_maybe_get_many_r_sups_across_r14!(obj, domain.sarzak());
                    if is_super.len() > 0 {
                        emit!(
                            buffer,
                            "self.{}.insert({}.id(), {});",
                            obj.as_ident(),
                            obj.as_ident(),
                            obj.as_ident()
                        );
                    } else {
                        emit!(
                            buffer,
                            "self.{}.insert({}.id, {});",
                            obj.as_ident(),
                            obj.as_ident(),
                            obj.as_ident()
                        );
                    }
                    emit!(buffer, "}}");
                    emit!(buffer, "");
                    emit!(
                        buffer,
                        "/// Exhume [`{}`] from the store.",
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );
                    emit!(buffer, "///");
                    emit!(
                        buffer,
                        "pub fn exhume_{}(&self, id: &Uuid) -> Option<&{}> {{",
                        obj.as_ident(),
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );
                    emit!(buffer, "self.{}.get(id)", obj.as_ident());
                    emit!(buffer, "}}");
                    emit!(
                        buffer,
                        "/// Get an iterator over the internal `HashMap<&Uuid, {}>`.",
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );
                    emit!(buffer, "//");
                    emit!(
                        buffer,
                        "pub fn iter_{}(&self) -> impl Iterator<Item = (&Uuid, &{})> {{",
                        obj.as_ident(),
                        obj.as_type(&Mutability::Borrowed(BORROWED), domain.sarzak())
                    );
                    emit!(buffer, "self.{}.iter()", obj.as_ident());
                    emit!(buffer, "}}");
                }
                emit!(buffer, "}}");

                Ok(())
            },
        )?;

        Ok(())
    }
}

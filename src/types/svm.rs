//! Svm File Generation
//!
//! This is where we generate code for use in the next stage of the compiler.
use std::{fmt::Write, sync::RwLock};

use fnv::FnvHashMap as HashMap;
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
    types::TypeDefinition,
};

pub(crate) struct SvmBuilder {
    definition: Option<Box<dyn TypeDefinition>>,
}

impl SvmBuilder {
    pub(crate) fn new() -> Self {
        SvmBuilder { definition: None }
    }

    pub(crate) fn definition(mut self, definition: Box<dyn TypeDefinition>) -> Self {
        self.definition = Some(definition);

        self
    }

    pub(crate) fn build(self) -> Result<Box<SvmGenerator>> {
        ensure!(
            self.definition.is_some(),
            CompilerSnafu {
                description: "missing TypeDefinition"
            }
        );

        Ok(Box::new(SvmGenerator {
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
pub(crate) struct SvmGenerator {
    definition: Box<dyn TypeDefinition>,
}

impl FileGenerator for SvmGenerator {
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
            format!("{}-svm-file", module),
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
pub(crate) struct SvmModule;

impl SvmModule {
    pub(crate) fn new() -> Box<dyn TypeDefinition> {
        Box::new(Self)
    }
}

impl TypeDefinition for SvmModule {}

impl CodeWriter for SvmModule {
    fn write_code(
        &self,
        _config: &GraceConfig,
        _domain: &Domain,
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
                description: "woog is required by SvmModule"
            }
        );
        let _woog = woog.as_ref().unwrap();

        ensure!(
            lu_dog.is_some(),
            CompilerSnafu {
                description: "lu_dog is required by SvmModule"
            }
        );
        let _lu_dog = lu_dog.as_ref().unwrap();

        buffer.block(
            DirectiveKind::IgnoreOrig,
            format!("{}-svm-output", module),
            |_buffer| {
                // Load up lu dog and see what imports we have to deal with

                // Generate use statements for the imports

                // Generate the main function

                Ok(())
            },
        )?;

        Ok(())
    }
}

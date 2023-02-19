use std::{
    fs,
    path::{Path, PathBuf},
};

use sarzak::{
    mc::{FileSnafu, ModelCompilerError, Result},
    sarzak::types::Object,
    woog::store::ObjectStore as WoogStore,
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{generator::GeneratorBuilder, render::RenderIdent},
    options::{GraceCompilerOptions, GraceConfig},
    targets::Target,
    types::default::{
        DefaultImplBuilder, DefaultModule, DefaultModuleBuilder, DefaultStruct,
        DefaultStructBuilder, DefaultStructNewImpl,
    },
    RS_EXT, TYPES,
};

pub(crate) struct ApplicationTarget<'a> {
    config: GraceConfig,
    _package: &'a str,
    module: &'a str,
    src_path: &'a Path,
    domain: sarzak::v1::domain::Domain,
    woog: WoogStore,
    _test: bool,
}

impl<'a> ApplicationTarget<'a> {
    pub(crate) fn new(
        options: &'a GraceCompilerOptions,
        _package: &'a str,
        module: &'a str,
        src_path: &'a Path,
        domain: sarzak::domain::DomainBuilder,
        woog: WoogStore,
        _test: bool,
    ) -> Box<dyn Target + 'a> {
        let domain = domain.build_v1().expect("Failed to build domain");
        let config: GraceConfig = (options, &domain).into();

        Box::new(Self {
            config,
            _package,
            module,
            src_path: src_path.as_ref(),
            domain,
            woog,
            _test,
        })
    }
}

impl<'a> Target for ApplicationTarget<'a> {
    fn compile(&mut self) -> Result<(), ModelCompilerError> {
        // ✨Generate Types✨

        // Build a path to src/types
        let mut types = PathBuf::from(self.src_path);
        types.push(self.module);
        types.push(TYPES);
        fs::create_dir_all(&types).context(FileSnafu { path: &types })?;
        types.push("discard");

        // Sort the objects -- I need to figure out how to do this automagically.
        let mut objects: Vec<(&Uuid, &Object)> = self.domain.sarzak().iter_object().collect();
        objects.sort_by(|a, b| a.1.name.cmp(&b.1.name));

        // Iterate over the objects, generating an implementation for file each.
        for (id, obj) in objects {
            types.set_file_name(obj.as_ident());
            types.set_extension(RS_EXT);

            // Here's the generation.
            GeneratorBuilder::new()
                .config(&self.config)
                // Where to write
                .path(&types)?
                // Domain/Store
                .domain(&self.domain)
                // Compiler Domain
                .compiler_domain(&mut self.woog)
                // Module name
                .module(self.module)
                .obj_id(&id)
                // What to write
                .generator(
                    // Struct
                    DefaultStructBuilder::new()
                        // Definition type
                        .definition(DefaultStruct::new())
                        // Implementation
                        .implementation(
                            DefaultImplBuilder::new()
                                .implementation(DefaultStructNewImpl::new())
                                .build(),
                        )
                        // Go!
                        .build()?,
                )
                // Really go!
                .generate()?;
        }

        // Generate a "types.rs" module file containing all of the types.
        // This needs to be done after the types are generated so that rustfmt
        // doesn't complain at us.
        let mut types = PathBuf::from(self.src_path);
        types.push(self.module);
        types.push("discard");
        types.set_file_name(TYPES);
        types.set_extension(RS_EXT);

        GeneratorBuilder::new()
            .config(&self.config)
            .path(&types)?
            .domain(&self.domain)
            .compiler_domain(&mut self.woog)
            .module(self.module)
            .generator(
                DefaultModuleBuilder::new()
                    .definition(DefaultModule::new())
                    .build()?,
            )
            .generate()?;

        Ok(())
    }

    fn domain(&self) -> &str {
        self.domain.domain()
    }
}

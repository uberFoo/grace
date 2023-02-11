use std::{
    fs,
    path::{Path, PathBuf},
};

use sarzak::{
    mc::{FileSnafu, ModelCompilerError, Result},
    sarzak::types::{External, Object, Type},
    woog::{
        store::ObjectStore as WoogStore,
        types::{Mutability, BORROWED},
    },
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        generator::GeneratorBuilder,
        render::{RenderIdent, RenderType},
    },
    options::GraceCompilerOptions,
    target::Target,
    types::{
        default::{DefaultModule, DefaultModuleBuilder, DefaultStructBuilder},
        domain::{
            store::{DomainStore, DomainStoreBuilder},
            structs::{DomainImplBuilder, DomainNewImpl, DomainStruct},
        },
    },
    RS_EXT, TYPES,
};

pub(crate) struct DomainTarget<'a> {
    options: &'a GraceCompilerOptions,
    _package: &'a str,
    module: &'a str,
    src_path: &'a Path,
    domain: sarzak::domain::Domain,
    woog: WoogStore,
    _test: bool,
}

impl<'a> DomainTarget<'a> {
    pub(crate) fn new(
        options: &'a GraceCompilerOptions,
        _package: &'a str,
        module: &'a str,
        src_path: &'a Path,
        domain: sarzak::domain::DomainBuilder,
        woog: WoogStore,
        _test: bool,
    ) -> Box<dyn Target + 'a> {
        let domain = {
            let module = module.to_owned();
            domain
                .post_load(move |sarzak, _| {
                    let store_type = Type::External(
                        External::new(
                            sarzak,
                            format!(
                                "{}Store",
                                module.as_type(&Mutability::Borrowed(BORROWED), &sarzak)
                            ),
                            format!("crate::{}::store::ObjectStore", module.as_ident(),),
                        )
                        .id,
                    );

                    sarzak.inter_ty(store_type);
                })
                .build()
                .unwrap()
        };

        Box::new(Self {
            options,
            _package,
            module,
            src_path: src_path.as_ref(),
            domain,
            woog,
            _test,
        })
    }
}

impl<'a> Target for DomainTarget<'a> {
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
                .options(&self.options)
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
                        .definition(DomainStruct::new())
                        // Implementation
                        .implementation(
                            DomainImplBuilder::new()
                                .implementation(DomainNewImpl::new())
                                .build(),
                        )
                        // Go!
                        .build()?,
                )
                // Really go!
                .generate()?;
        }

        // Generate the store.rs file
        let mut store = PathBuf::from(self.src_path);
        store.push(self.module);
        store.push("store.rs");

        GeneratorBuilder::new()
            .options(&self.options)
            .path(&store)?
            .domain(&self.domain)
            .compiler_domain(&mut self.woog)
            .module(self.module)
            .generator(
                DomainStoreBuilder::new()
                    .definition(DomainStore::new())
                    .build()?,
            )
            .generate()?;

        // Generate a "types.rs" module file containing all of the types.
        // This needs to be done after the types are generated so that rustfmt
        // doesn't complain at us.
        let mut types = PathBuf::from(self.src_path);
        types.push(self.module);
        types.push("discard");
        types.set_file_name(TYPES);
        types.set_extension(RS_EXT);

        GeneratorBuilder::new()
            .options(&self.options)
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

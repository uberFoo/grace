use std::{
    fs,
    path::{Path, PathBuf},
};

use sarzak::{
    mc::{FileSnafu, ModelCompilerError, Result},
    sarzak::{
        macros::{sarzak_get_many_as_across_r1, sarzak_maybe_get_many_r_sups_across_r14},
        types::{Attribute, External, Object, Supertype, Type},
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
        generator::GeneratorBuilder,
        object_is_singleton, object_is_supertype,
        render::{RenderIdent, RenderType},
    },
    options::GraceCompilerOptions,
    targets::Target,
    types::{
        default::{DefaultModule, DefaultModuleBuilder, DefaultStructBuilder},
        domain::{
            consts::DomainConst,
            enums::{DomainEnum, DomainEnumGetIdImpl},
            store::{DomainStore, DomainStoreBuilder},
            structs::{DomainImplBuilder, DomainRelNavImpl, DomainStruct, DomainStructNewImpl},
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
        // This post_load script creates an external entity of the ObjectStore so that
        // we can use it from within the domain. Remember that the ObjectStore is a
        // generated construct, and appears as if it was an external library to the
        // domain. Now, if it were modeled, we'd probably include some aspect of it's
        // model as an imported object, and we wouldn't need this. We'd probably need
        // something else...
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

    fn generate_types(&mut self) -> Result<(), ModelCompilerError> {
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
        // Now things get tricky. We need to generate an enum if the objects is
        // a supertype. For now, we just ignore any attributes on a supertype.
        for (id, obj) in objects {
            types.set_file_name(obj.as_ident());
            types.set_extension(RS_EXT);

            // Test if the object is a supertype. Those we generate as enums.
            let generator = if object_is_supertype(obj, &self.domain) {
                DefaultStructBuilder::new()
                    .definition(DomainEnum::new())
                    .implementation(
                        DomainImplBuilder::new()
                            .method(DomainEnumGetIdImpl::new())
                            .build(),
                    )
                    .build()?
            } else {
                // Look for naked objects, and generate a singleton for them.
                if object_is_singleton(obj, &self.domain) {
                    eprintln!("Generating singleton for {}", obj.name);
                    DefaultStructBuilder::new()
                        .definition(DomainConst::new())
                        .build()?
                } else {
                    DefaultStructBuilder::new()
                        // Definition type
                        .definition(DomainStruct::new())
                        .implementation(
                            DomainImplBuilder::new()
                                // New implementation
                                .method(DomainStructNewImpl::new())
                                // Relationship navigation implementations
                                .method(DomainRelNavImpl::new())
                                .build(),
                        )
                        // Go!
                        .build()?
                }
            };

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
                .generator(generator)
                // Go!
                .generate()?;
        }

        Ok(())
    }

    fn generate_store(&mut self) -> Result<(), ModelCompilerError> {
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

        Ok(())
    }

    fn generate_types_module(&mut self) -> Result<(), ModelCompilerError> {
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
}

impl<'a> Target for DomainTarget<'a> {
    fn compile(&mut self) -> Result<(), ModelCompilerError> {
        // ✨Generate Types✨
        self.generate_types()?;

        // Generate the store.rs file
        self.generate_store()?;

        // Generate a "types.rs" module file containing all of the types.
        // This needs to be done after the types are generated so that rustfmt
        // doesn't complain at us.
        self.generate_types_module()?;

        Ok(())
    }

    fn domain(&self) -> &str {
        self.domain.domain()
    }
}

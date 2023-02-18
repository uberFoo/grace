use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use sarzak::{
    domain::DomainBuilder,
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
        object_is_singleton, object_is_supertype,
        render::{RenderIdent, RenderType},
    },
    options::{parse_config_value, GraceCompilerOptions, GraceConfig},
    targets::Target,
    types::{
        default::{DefaultModule, DefaultModuleBuilder, DefaultStructBuilder},
        domain::{
            consts::DomainConst,
            enums::{DomainEnum, DomainEnumGetIdImpl},
            store::{DomainStore, DomainStoreBuilder},
            structs::{DomainImplBuilder, DomainRelNavImpl, DomainStruct, DomainStructNewImpl},
        },
        null::NullGenerator,
    },
    RS_EXT, TYPES,
};

pub(crate) struct DomainTarget<'a> {
    config: GraceConfig,
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
            let module = module.replace("/", "::");
            let options = options.clone();
            domain
                .post_load(move |sarzak, _| {
                    let mut external = HashSet::new();
                    external.insert(module.clone());

                    if let Some(domains) = options.imported_domains.as_ref() {
                        for domain in domains {
                            external.insert(domain.replace("/", "::"));
                        }
                    }

                    for store in external {
                        // Store is a rust path at this point. That's fine for the path,
                        // but not so good for the name.
                        let name = store
                            .split("::")
                            .last()
                            // 🚧 Sigh, another expect.
                            .expect("Failed to get last part of path")
                            .to_string();

                        log::debug!("Adding ObjectStore for {}", name);
                        let store_type = Type::External(
                            External::new(
                                sarzak,
                                format!(
                                    "{}Store",
                                    name.as_type(&Mutability::Borrowed(BORROWED), sarzak)
                                ),
                                format!("crate::{}::store::ObjectStore", store,),
                            )
                            .id,
                        );

                        sarzak.inter_ty(store_type);
                    }
                })
                .build()
                // 🚧 Blow up here -- we don't return a result. We could fix this,
                // but I'm not sure it's worth it.
                .expect("Failed to build domain")
        };

        // This is boss. Who says boss anymore?
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
        // a supertype.
        //
        // For now, we just ignore any attributes on a supertype,
        // since enums don't have fields like structs. In the future I can see
        // creating a type with an enum field that is used to track it's subtype
        // status.
        //
        // Talk about tricky? Now things are going to get tricky. If the object
        // is imported, we are going to suck it in and generate a module for
        // it! Whoohoo! Note also, that we have a NullGenerator that does nothing.
        for (id, obj) in objects {
            types.set_file_name(obj.as_ident());
            types.set_extension(RS_EXT);

            println!(
                "Generating code for: {} ... output path: {}",
                obj.name,
                types.display(),
            );

            // Test if the object is a supertype. Those we generate as enums.
            let generator = if object_is_supertype(obj, self.domain.sarzak()) {
                DefaultStructBuilder::new()
                    .definition(DomainEnum::new())
                    .implementation(
                        DomainImplBuilder::new()
                            .method(DomainEnumGetIdImpl::new())
                            .build(),
                    )
                    .build()?
            } else if self.config.is_imported(id) {
                // If the object is imported, we don't generate anything...here.
                // But before we finish this, let's take a detour. Let's generate
                // code for the imported domain. What do you say? Sound fun?
                // Let's do it!
                let imported = self.config.get_imported(id).unwrap();

                // 🚧 Snafu is fighting me here, so unwrap it is. Figure it out
                // later. It's important though, because this is likely to get
                // screwed up.
                // This belongs elsewhere...
                let imported_domain = DomainBuilder::new()
                    .cuckoo_model(&imported.model_file)
                    .expect("Failed to build imported domain")
                    .build()
                    .expect("Failed to build imported domain");

                NullGenerator::new()
            } else if object_is_singleton(obj, self.domain.sarzak()) {
                // Look for naked objects, and generate a singleton for them.

                log::debug!("Generating singleton for {}", obj.name);
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
            };

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
            .config(&self.config)
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

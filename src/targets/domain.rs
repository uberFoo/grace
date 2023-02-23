use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use heck::ToUpperCamelCase;
use sarzak::{
    domain::DomainBuilder,
    mc::{FileSnafu, ModelCompilerError, Result},
    sarzak::types::{External, Object, Type},
    woog::store::ObjectStore as WoogStore,
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    codegen::{
        generator::GeneratorBuilder, object_is_singleton, object_is_supertype, render::RenderIdent,
    },
    init_woog::init_woog,
    options::{FromDomain, GraceCompilerOptions, GraceConfig},
    targets::Target,
    types::{
        default::{DefaultModule, DefaultModuleBuilder, DefaultStructBuilder},
        domain::{
            consts::DomainConst,
            enums::{Enum, EnumGetIdImpl, EnumNewImpl},
            from::{DomainFromBuilder, DomainFromImpl},
            store::{DomainStore, DomainStoreBuilder},
            structs::{DomainImplBuilder, DomainNewImpl, DomainRelNavImpl, DomainStruct},
        },
        null::NullGenerator,
    },
    RS_EXT, TYPES,
};

const FROM: &str = "from";

pub(crate) struct DomainTarget<'a> {
    config: GraceConfig,
    package: &'a str,
    module: &'a str,
    src_path: &'a Path,
    domain: sarzak::v1::domain::Domain,
    imports: HashMap<String, sarzak::v1::domain::Domain>,
    woog: WoogStore,
    _test: bool,
}

impl<'a> DomainTarget<'a> {
    pub(crate) fn new(
        options: &'a GraceCompilerOptions,
        package: &'a str,
        module: &'a str,
        src_path: &'a Path,
        domain: sarzak::domain::DomainBuilder,
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
                                // 🚧 Hmmm. Well, I don't have a domain here, and I think that
                                // as_type should take one. So I'm going to do the gross thing,
                                // and hardcode what I need. It's really not that gross, since
                                // all the as_type stuff is overly complicated, in order to be
                                // used to generate code in places that maybe I don't know
                                // what it should be. Like here.
                                format!(
                                    "{}Store",
                                    // name.as_type(&Mutability::Borrowed(BORROWED), sarzak)
                                    name.to_upper_camel_case()
                                ),
                                format!("crate::{}::store::ObjectStore", store,),
                            )
                            .id,
                        );

                        sarzak.inter_ty(store_type);
                    }
                })
                .build_v1()
                // 🚧 Blow up here -- we don't return a result. We could fix this,
                // but I'm not sure it's worth it.
                .expect("Failed to build domain")
        };

        // This is boss. Who says boss anymore?
        let config: GraceConfig = (options, &domain).into();

        // Suck in the imported domains for later use.
        let mut imported_domains = HashMap::new();
        // Include the from domain, if there is one.
        if let Some(from_domain) = config.get_from_domain() {
            let domain = DomainBuilder::new()
                .cuckoo_model(&from_domain.path)
                .expect(format!("Failed to load domain {}", &from_domain.path.display()).as_str())
                .build_v1()
                .expect("Failed to build domain");

            log::debug!("Loaded imported domain {}", &from_domain.path.display());
            let domain_name = from_domain
                .module
                .split("::")
                .last()
                .expect("Failed to get last part of path")
                .to_string();
            imported_domains.insert(domain_name, domain);
        }

        for (id, _) in domain.sarzak().iter_object() {
            if config.is_imported(&id) {
                let io = config.get_imported(&id).unwrap();
                // Only import the domain once.
                if !imported_domains.contains_key(&io.domain) {
                    let domain = DomainBuilder::new()
                        .cuckoo_model(&io.model_file)
                        .expect(
                            format!("Failed to load domain {}", io.model_file.display()).as_str(),
                        )
                        .build_v1()
                        .expect("Failed to build domain");

                    log::debug!("Loaded imported domain {}", io.domain);
                    imported_domains.insert(io.domain.clone(), domain);
                }
            }
        }

        // Create our local compiler domain.
        let woog = init_woog(module, &options, &domain.sarzak());

        Box::new(Self {
            config,
            package,
            module,
            src_path: src_path.as_ref(),
            domain,
            imports: imported_domains,
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
            let generator = if object_is_supertype(obj, &self.domain) {
                DefaultStructBuilder::new()
                    .definition(Enum::new())
                    .implementation(
                        DomainImplBuilder::new()
                            .method(EnumNewImpl::new())
                            .method(EnumGetIdImpl::new())
                            .build(),
                    )
                    .build()?
            } else if self.config.is_imported(id) {
                // If the object is imported, we don't generate anything...here.

                NullGenerator::new()
            } else if object_is_singleton(obj, &self.domain) {
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
                            .method(DomainNewImpl::new())
                            // Relationship navigation implementations
                            .method(DomainRelNavImpl::new())
                            .build(),
                    )
                    // Go!
                    .build()?
            };

            // Here's the generation.
            GeneratorBuilder::new()
                .package(&self.package)
                .config(&self.config)
                // Where to write
                .path(&types)?
                // Domain/Store
                .domain(&self.domain)
                // Compiler Domain
                .compiler_domain(&mut self.woog)
                // Imported domains
                .imports(&self.imports)
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
            .package(&self.package)
            .config(&self.config)
            .path(&store)?
            .domain(&self.domain)
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
            .package(&self.package)
            .config(&self.config)
            .path(&types)?
            .domain(&self.domain)
            .module(self.module)
            .generator(
                DefaultModuleBuilder::new()
                    .definition(DefaultModule::new())
                    .build()?,
            )
            .generate()?;

        Ok(())
    }

    fn generate_from_module(&self, domain: &FromDomain) -> Result<(), ModelCompilerError> {
        let mut from = PathBuf::from(self.src_path);
        from.push(self.module);
        from.push("discard");
        from.set_file_name(FROM);
        from.set_extension(RS_EXT);

        GeneratorBuilder::new()
            .package(&self.package)
            .config(&self.config)
            .path(&from)?
            .domain(&self.domain)
            .module(self.module)
            .imports(&self.imports)
            .generator(
                DomainFromBuilder::new()
                    .domain(domain.clone())
                    .definition(DomainFromImpl::new())
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

        // Generate From trait implementations
        if let Some(domain) = self.config.get_from_domain() {
            self.generate_from_module(&domain)?;
        }

        Ok(())
    }

    fn domain(&self) -> &str {
        self.domain.domain()
    }
}

use std::{
    fs,
    path::{Path, PathBuf},
};

use fnv::{FnvHashMap as HashMap, FnvHashSet as HashSet};
use heck::ToUpperCamelCase;
use sarzak::{
    domain::DomainBuilder,
    mc::{FileSnafu, ModelCompilerError, Result},
    sarzak::types::{External, Object, Ty},
    v2::domain::Domain,
    woog::store::ObjectStore as WoogStore,
};
use snafu::prelude::*;

use crate::{
    codegen::{generator::GeneratorBuilder, render::RenderIdent},
    options::{GraceCompilerOptions, GraceConfig},
    targets::Target,
    types::default::{
        DefaultImplBuilder, DefaultModule, DefaultModuleBuilder, DefaultNewImpl, DefaultStruct,
        DefaultStructBuilder,
    },
    woog::{init_woog, populate_woog},
    RS_EXT, TYPES,
};

pub(crate) struct ApplicationTarget<'a> {
    config: GraceConfig,
    package: &'a str,
    module: &'a str,
    src_path: &'a Path,
    domain: sarzak::v2::domain::Domain,
    imports: HashMap<String, Domain>,
    woog: WoogStore,
    _test: bool,
}

impl<'a> ApplicationTarget<'a> {
    pub(crate) fn new(
        options: &'a GraceCompilerOptions,
        package: &'a str,
        module: &'a str,
        src_path: &'a Path,
        mut domain: Domain,
        _test: bool,
    ) -> Result<Box<dyn Target + 'a>> {
        // This creates an external entity of the ObjectStore so that
        // we can use it from within the domain. Remember that the ObjectStore is a
        // generated construct, and appears as if it was an external library to the
        // domain. Now, if it were modeled, we'd probably include some aspect of it's
        // model as an imported object, and we wouldn't need this. We'd probably need
        // something else...
        let mut external = HashSet::default();

        // This is the object store for _this_ domain.
        external.insert(module.replace("/", "::"));

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
            let external = External::new(
                // 🚧 Hmmm. Well, I don't have a domain here, and I think that
                // as_type should take one. So I'm going to do the gross thing,
                // and hardcode what I need. It's really not that gross, since
                // all the as_type stuff is overly complicated, in order to be
                // used to generate code in places that maybe I don't know
                // what it should be. Like here.
                "new".to_owned(),
                format!(
                    "{}Store",
                    // name.as_type(&woog.exhume_ownership(&woog.exhume_borrowed(&SHARED).unwrap().id()).unwrap(), &woog, &domain)
                    name.to_upper_camel_case()
                ),
                format!("crate::{}::store::ObjectStore", store,),
                domain.sarzak_mut(),
            );

            Ty::new_external(&external, domain.sarzak_mut());
        }

        // This is boss. Who says boss anymore?
        let config: GraceConfig = (options, &domain).into();

        // Create an external entity for any objects with the annotation.
        let borrow_checker_pleasure_yourself =
            domain.sarzak().iter_object().cloned().collect::<Vec<_>>();
        for obj in borrow_checker_pleasure_yourself {
            if config.is_external(&obj.id) {
                let external = config.get_external(&obj.id).unwrap();
                let external = External::new(
                    external.ctor.clone(),
                    external.name.clone(),
                    external.path.clone(),
                    domain.sarzak_mut(),
                );
                Ty::new_external(&external, domain.sarzak_mut());
            }
        }

        // Suck in the imported domains for later use.
        let mut imported_domains = HashMap::default();
        // Include the from domain, if there is one.
        if let Some(from_domain) = config.get_from_domain() {
            let domain = DomainBuilder::new()
                .cuckoo_model(&from_domain.path)
                .expect(format!("Failed to load domain {}", &from_domain.path.display()).as_str())
                .build_v2()
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

        // These are imports in the object descriptions.
        for obj in domain.sarzak().iter_object() {
            if config.is_imported(&obj.id) {
                let io = config.get_imported(&obj.id).unwrap();
                // Only import the domain once.
                if !imported_domains.contains_key(&io.domain) {
                    let domain = DomainBuilder::new()
                        .cuckoo_model(&io.model_file)
                        .expect(
                            format!("Failed to load domain {}", io.model_file.display()).as_str(),
                        )
                        .build_v2()
                        .expect("Failed to build domain");

                    log::debug!("Loaded imported domain {}", io.domain);
                    imported_domains.insert(io.domain.clone(), domain);
                }
            }
        }

        // Create our local compiler domain.
        let mut woog = init_woog(src_path, &config, &domain);
        populate_woog(module, &config, &imported_domains, &mut woog, &domain)?;

        Ok(Box::new(Self {
            config,
            package,
            module,
            src_path: src_path.as_ref(),
            domain,
            imports: imported_domains,
            woog,
            _test,
        }))
    }
}

impl<'a> Target for ApplicationTarget<'a> {
    fn compile(&mut self) -> Result<(), ModelCompilerError> {
        // ✨Generate Types✨

        // Build a path to src/types
        let mut types = PathBuf::from(self.src_path);
        types.push(self.module);
        types.push(TYPES);
        fs::create_dir_all(&types).context(FileSnafu {
            path: &types,
            description: "creating types directory".to_owned(),
        })?;
        types.push("discard");

        // Sort the objects -- I need to figure out how to do this automagically.
        let mut objects: Vec<&Object> = self.domain.sarzak().iter_object().collect();
        objects.sort_by(|a, b| a.name.cmp(&b.name));

        // Iterate over the objects, generating an implementation for file each.
        for obj in objects {
            types.set_file_name(obj.as_ident());
            types.set_extension(RS_EXT);

            // Here's the generation.
            GeneratorBuilder::new()
                .imports(&self.imports)
                .package(&self.package)
                .config(&self.config)
                // Where to write
                .path(&types)?
                // Domain/Store
                .domain(&self.domain)
                // Compiler Domain
                .compiler_domain(&mut self.woog)
                // Module name
                .module(self.module)
                .obj_id(&obj.id)
                // What to write
                .generator(
                    // Struct
                    DefaultStructBuilder::new()
                        // Definition type
                        .definition(DefaultStruct::new())
                        // Implementation
                        .implementation(
                            DefaultImplBuilder::new()
                                .method(DefaultNewImpl::new())
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
            .package(&self.package)
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

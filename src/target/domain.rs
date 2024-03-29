use std::{
    env, fs,
    path::{Path, PathBuf},
};

use ansi_term::Colour;
use heck::ToUpperCamelCase;
use rayon::prelude::*;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use sarzak::{
    domain::DomainBuilder,
    mc::{FileSnafu, ModelCompilerError, Result},
    sarzak::types::{External, Object, Ty},
    woog::{
        store::ObjectStore as WoogStore,
        types::{GenerationUnit, TimeStamp},
    },
};
use snafu::prelude::*;

use crate::{
    codegen::{
        generator::{FileGenerator, GeneratorBuilder},
        local_object_is_hybrid, local_object_is_singleton, local_object_is_supertype,
        object_is_enum,
        render::RenderIdent,
    },
    options::{FromDomain, GraceCompilerOptions, GraceConfig, OptimizationLevel},
    target::Target,
    types::{
        default::{DefaultModule, DefaultModuleBuilder, DefaultStructBuilder},
        domain::{
            consts::DomainConst,
            enums::{Enum, EnumGetIdImpl, EnumNewImpl, EnumRelNavImpl},
            from::{DomainFromBuilder, DomainFromImpl},
            hybrid::{Hybrid, HybridNewImpl},
            store::{DomainStore, DomainStoreBuilder},
            store_vec::DomainStoreVec,
            structs::{
                DomainImplBuilder, EqImpl, Imports, Struct, StructNewImpl, StructRelNavImpl,
            },
        },
        external::ExternalBuilder,
        null::NullGenerator,
    },
    woog::{init_woog, populate_woog},
    RS_EXT, TYPES,
};

macro_rules! display_output {
    ($obj:expr, $types:expr, $color:expr, $out:literal) => {
        println!(
            "Generating code for: {:<42} ({}) \t output: {}",
            // This is goofy, and necessary.
            format!("{}", Colour::Blue.paint(&$obj.name)),
            $color.italic().paint($out),
            Colour::White.dimmed().paint($types.display().to_string()),
        );
    };
}

const FROM: &str = "from";

pub(crate) struct DomainTarget<'a> {
    config: GraceConfig,
    package: &'a str,
    module: &'a str,
    src_path: &'a Path,
    domain: sarzak::v2::domain::Domain,
    imports: HashMap<String, sarzak::v2::domain::Domain>,
    woog: WoogStore,
    _test: bool,
}

impl<'a> DomainTarget<'a> {
    pub(crate) fn new(
        options: &'a GraceCompilerOptions,
        package: &'a str,
        module: &'a str,
        src_path: &'a Path,
        mut domain: sarzak::v2::domain::Domain,
        _test: bool,
    ) -> Result<Box<dyn Target + 'a>> {
        // This is boss. Who says boss anymore?
        let config: GraceConfig = (options, &domain).into();

        // Create our local compiler domain.
        let mut woog = init_woog(src_path, &config, &domain);

        // This creates an external entity of the ObjectStore so that
        // we can use it from within the domain. Remember that the ObjectStore is a
        // generated construct, and appears as if it was an external library to the
        // domain. Now, if it were modeled, we'd probably include some aspect of it's
        // model as an imported object, and we wouldn't need this. We'd probably need
        // something else...
        let mut external = HashSet::default();

        // This is the object store for _this_ domain.
        let mut import = module.replace('/', "::");
        import = ["crate::", import.as_str()].concat();
        external.insert(import);

        if let Some(domains) = options.imported_domains.as_ref() {
            for domain in domains {
                external.insert(domain.replace('/', "::"));
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
                // what it should be.
                "new".to_owned(),
                format!(
                    "{}Store",
                    // name.as_type(&Ownership::new_borrowed(), &woog, &domain)
                    name.to_upper_camel_case()
                ),
                format!("{store}::store::ObjectStore",),
                domain.sarzak_mut(),
            );

            Ty::new_external(&external, domain.sarzak_mut());
        }

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
                .unwrap_or_else(|_| panic!("Failed to load domain {}", &from_domain.path.display()))
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
                        .unwrap_or_else(|_| {
                            panic!("Failed to load domain {}", io.model_file.display())
                        })
                        .build_v2()
                        .expect("Failed to build domain");

                    log::debug!("Loaded imported domain {}", io.domain);
                    imported_domains.insert(io.domain.clone(), domain);
                }
            }
        }

        if let OptimizationLevel::Vec = config.get_optimization_level() {
            let objects = domain.sarzak().iter_object().cloned().collect::<Vec<_>>();
            // Find all the enums and add an id attribute so that they become hybrids.
            for obj in objects {
                if object_is_enum(&obj, &config, &Some(&imported_domains), &domain)? {
                    log::debug!("Turning {} into a hybrid", &obj.name);
                    sarzak::sarzak::Attribute::new(
                        "hack".to_owned(),
                        &obj,
                        &Ty::new_s_uuid(),
                        &mut domain.sarzak_mut(),
                    );
                }
            }
        }

        populate_woog(module, &config, &imported_domains, &mut woog, &domain)?;

        Ok(Box::new(Self {
            config,
            package,
            module,
            src_path,
            domain,
            imports: imported_domains,
            woog,
            _test,
        }))
    }

    fn generate_types(&mut self) -> Result<usize, ModelCompilerError> {
        // Build a path to src/types
        let mut types = PathBuf::from(self.src_path);
        types.push(self.module);
        types.push(TYPES);
        fs::create_dir_all(&types).context(FileSnafu {
            description: "creating type directory".to_owned(),
            path: &types,
        })?;
        types.push("discard");

        let cwd = env::current_dir().unwrap();

        let gen_partial_eq = !(self.config.get_optimization_level() == &OptimizationLevel::None);

        // Sort the objects -- I need to figure out how to do this automagically.
        let mut objects: Vec<&Object> = self.domain.sarzak().iter_object().collect();
        objects.sort_by(|a, b| a.name.cmp(&b.name));

        // Iterate over the objects, generating an implementation file for each.
        // Now things get tricky. We need to generate an enum if the objects is
        // a supertype.
        //
        // If we only want to compile things that have changed, then we need to
        // store a timestamp in woog, and then on subsequent runs, compare it
        // to the timestamp from the store.
        //
        // We can also look for things in woog that don't show up here -- those
        // we need to delete.
        // for obj in objects {
        objects
            .par_iter()
            .map(|obj| {
                // if !is_object_stale(obj, &self.woog, &self.domain) {
                // log::debug!("Skipping object {}", obj.name);
                // continue;
                // }
                let mut types = types.clone();
                types.set_file_name(obj.as_ident());
                types.set_extension(RS_EXT);
                types = if let Ok(types) = types.strip_prefix(&cwd) {
                    types.to_owned()
                } else {
                    types
                };

                // Test if the object is a supertype. For those we generate as enums.
                let generator: Box<dyn FileGenerator> =
                    if local_object_is_supertype(obj, &self.config, &self.domain) {
                        // Unless it's got referential attributes. Then we generate what
                        // I now dub, a _hybrid_. What about regular attributes you ask?
                        // Well, I don't have a use case for that at the moment, so they
                        // will be done in due time.
                        if local_object_is_hybrid(obj, &self.config, &self.domain) {
                            display_output!(obj, &types, Colour::Cyan, "hybrid");

                            let builder = DefaultStructBuilder::new()
                                .definition(Hybrid::new())
                                .implementation(
                                    DomainImplBuilder::new()
                                        .method(HybridNewImpl::new())
                                        // The struct implementation suffices -- thankfully. Reuse FTW!
                                        .method(StructRelNavImpl::new())
                                        .build(),
                                );

                            let builder = if gen_partial_eq {
                                builder.implementation(
                                    DomainImplBuilder::new()
                                        .make_trait("PartialEq")
                                        .method(EqImpl::new())
                                        .build(),
                                )
                            } else {
                                builder
                            };

                            builder.build()?
                        } else {
                            display_output!(obj, &types, Colour::Green, "enumeration");

                            let builder = DefaultStructBuilder::new()
                                .definition(Enum::new())
                                .implementation(
                                    DomainImplBuilder::new()
                                        .method(EnumNewImpl::new())
                                        .method(EnumGetIdImpl::new())
                                        .method(EnumRelNavImpl::new())
                                        .build(),
                                );

                            let builder = if gen_partial_eq {
                                builder.implementation(
                                    DomainImplBuilder::new()
                                        .make_trait("PartialEq")
                                        .method(EqImpl::new())
                                        .build(),
                                )
                            } else {
                                builder
                            };

                            builder.build()?
                        }
                    } else if self.config.is_imported(&obj.id) {
                        // If the object is imported, we don't generate anything...here.
                        // I'd like to amend this position. Wouldn't it be cool if we could
                        // generate relationship navigation methods for imported objects?
                        // I think we can.
                        // We can create an implementation of the relationship navigation
                        // methods. We'd need to make sure that the names don't collide.
                        // They won't because the store would be different.
                        // DefaultStructBuilder::new()
                        //     .imports(Imports::new())
                        //     .implementation(
                        //         DomainImplBuilder::new()
                        //             .method(StructRelNavImpl::new())
                        //             .build(),
                        //     )
                        //     .build()?
                        NullGenerator::new()
                    } else if self.config.is_external(&obj.id) {
                        // If the object is external, we create a newtype to wrap it.
                        display_output!(obj, &types, Colour::Red, "external");

                        if gen_partial_eq {
                            ExternalBuilder::new().implementation(
                                DomainImplBuilder::new()
                                    .make_trait("PartialEq")
                                    .method(EqImpl::new())
                                    .build(),
                            )
                        } else {
                            ExternalBuilder::new()
                        }
                        .build()?
                    } else if local_object_is_singleton(obj, &self.config, &self.domain) {
                        // Look for naked objects, and generate a singleton for them.
                        display_output!(obj, &types, Colour::Purple, "constant");

                        log::debug!("Generating singleton for {}", obj.name);
                        DefaultStructBuilder::new()
                            .definition(DomainConst::new())
                            .build()?
                    } else {
                        display_output!(obj, &types, Colour::Yellow, "struct");

                        let builder = DefaultStructBuilder::new()
                            .imports(Imports::new())
                            // Definition type
                            .definition(Struct::new())
                            .implementation(
                                DomainImplBuilder::new()
                                    // New implementation
                                    .method(StructNewImpl::new())
                                    // Relationship navigation implementations
                                    .method(StructRelNavImpl::new())
                                    .build(),
                            );

                        let builder = if gen_partial_eq {
                            builder.implementation(
                                DomainImplBuilder::new()
                                    .make_trait("PartialEq")
                                    .method(EqImpl::new())
                                    .build(),
                            )
                        } else {
                            builder
                        };

                        // Go!
                        builder.build()?
                    };

                let mut woog = self.woog.clone();

                // Here's the generation.
                GeneratorBuilder::new()
                    .package(self.package)
                    .config(&self.config)
                    // Where to write
                    .path(&types)?
                    // Domain/Store
                    .domain(&self.domain)
                    // Compiler Domain
                    .woog(&mut woog)
                    // Imported domains
                    .imports(&self.imports)
                    // Module name
                    .module(self.module)
                    .obj_id(&obj.id)
                    // What to write
                    .generator(generator)
                    // Go!
                    .generate()?;

                // Update the timestamp in woog.
                let ts = TimeStamp::new(&mut woog);
                let _ = GenerationUnit::new(obj, &ts, &mut woog);

                Ok(())
            })
            .collect::<Result<Vec<_>, ModelCompilerError>>()?;

        println!("Generated code for {} objects.", objects.len());

        Ok(objects.len())
    }

    fn generate_store(&mut self) -> Result<(), ModelCompilerError> {
        let mut store = PathBuf::from(self.src_path);
        store.push(self.module);
        store.push("store.rs");

        // 🚧 I should have a store that's persistent, and one that isn't.
        let generator = match self.config.get_optimization_level() {
            crate::options::OptimizationLevel::None => DomainStoreBuilder::new()
                .definition(DomainStore::new())
                .build()?,

            crate::options::OptimizationLevel::Vec => DomainStoreBuilder::new()
                .definition(DomainStoreVec::new())
                .build()?,

            _ => todo!(),
        };

        GeneratorBuilder::new()
            .package(self.package)
            .config(&self.config)
            .path(&store)?
            .domain(&self.domain)
            .module(self.module)
            .woog(&mut self.woog)
            .generator(generator)
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
            .package(self.package)
            .config(&self.config)
            .path(&types)?
            .domain(&self.domain)
            .module(self.module)
            .woog(&mut self.woog)
            .generator(
                DefaultModuleBuilder::new()
                    .definition(DefaultModule::new())
                    .build()?,
            )
            .generate()?;

        Ok(())
    }

    fn generate_from_module(&mut self, domain: &FromDomain) -> Result<(), ModelCompilerError> {
        let mut from = PathBuf::from(self.src_path);
        from.push(self.module);
        from.push("discard");
        from.set_file_name(FROM);
        from.set_extension(RS_EXT);

        GeneratorBuilder::new()
            .package(self.package)
            .config(&self.config)
            .path(&from)?
            .domain(&self.domain)
            .module(self.module)
            .imports(&self.imports)
            .woog(&mut self.woog)
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
    fn compile(&mut self) -> Result<usize, ModelCompilerError> {
        // ✨Generate Types✨
        let count = self.generate_types()?;

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

        // persist_woog(&self.woog, self.src_path, &self.domain)?;

        Ok(count)
    }

    fn domain(&self) -> &str {
        self.domain.domain()
    }
}

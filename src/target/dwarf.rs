use std::{
    fs,
    path::{Path, PathBuf},
    sync::RwLock,
};

use lazy_static::lazy_static;
// use rayon::prelude::*;
use rustc_hash::FxHashMap as HashMap;
use sarzak::{
    domain::DomainBuilder,
    lu_dog::store::ObjectStore as LuDogStore,
    mc::{FileSnafu, ModelCompilerError, Result},
    sarzak::types::Object,
    woog::store::ObjectStore as WoogStore,
};
use snafu::prelude::*;

use crate::{
    codegen::{generator::GeneratorBuilder, render::RenderIdent},
    options::{GraceCompilerOptions, GraceConfig},
    target::Target,
    types::dwarf::{ChaChaBuilder, ChaChaFile, DwarfBuilder, DwarfFile},
    woog::init_woog,
    BUILD_DIR, RS_EXT, TARGET_DIR,
};

// pub(crate) const DWARF_EXT: &str = "道";
pub(crate) const DWARF_EXT: &str = "tao";

lazy_static! {
    //
    // This is the global LuDog store. It's got it's own locking, but this still needs
    // to be behind an RwLock. So, maybe using a global isn't such a great idea.
    //
    pub(crate) static ref LU_DOG: RwLock<LuDogStore> = RwLock::new(LuDogStore::new());
}

pub(crate) struct DwarfTarget<'a> {
    config: GraceConfig,
    package: &'a str,
    module: &'a str,
    src_path: &'a Path,
    domain: sarzak::v2::domain::Domain,
    woog: WoogStore,
    _test: bool,
}

impl<'a> DwarfTarget<'a> {
    pub(crate) fn new(
        options: &'a GraceCompilerOptions,
        package: &'a str,
        module: &'a str,
        src_path: &'a Path,
        domain: sarzak::v2::domain::Domain,
        _test: bool,
    ) -> Result<Box<dyn Target + 'a>> {
        let config: GraceConfig = (options, &domain).into();

        // Create our local compiler domain.
        let woog = init_woog(src_path, &config, &domain);

        // This needs to become a global someplace.
        let _lu_dog = LuDogStore::new();
        // let lu_dog = RwLock::new(lu_dog);

        Ok(Box::new(Self {
            config,
            package,
            module,
            src_path,
            domain,
            woog,
            _test,
        }))
    }
}

impl<'a> Target for DwarfTarget<'a> {
    fn compile(&mut self) -> Result<usize, ModelCompilerError> {
        let mut path = PathBuf::from(self.src_path);
        path.pop();
        path.push(TARGET_DIR);
        path.push(BUILD_DIR);
        path.push(self.domain.name());

        fs::create_dir_all(&path).context(FileSnafu {
            description: "creating dwarf output directory".to_owned(),
            path: &path,
        })?;

        let mut dwarf_file = path.clone();
        dwarf_file.push("discard");
        dwarf_file.set_file_name(self.domain.name().as_ident());
        dwarf_file.set_extension(DWARF_EXT);

        let mut chacha_file = path.clone();
        chacha_file.push("discard");
        chacha_file.set_file_name(self.domain.name().as_ident());
        chacha_file.set_extension(RS_EXT);

        // Sort the objects -- I need to figure out how to do this automagically.
        let objects: Vec<&Object> = self.domain.sarzak().iter_object().collect();
        // objects.sort_by(|a, b| a.name.cmp(&b.name));

        let mut imported_domains = HashMap::default();
        for obj in &objects {
            if self.config.is_imported(&obj.id) {
                let io = self.config.get_imported(&obj.id).unwrap();
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

        // objects
        // .par_iter()
        // .map(|_obj| {
        let mut woog = self.woog.clone();

        GeneratorBuilder::new()
            .path(&dwarf_file)?
            .package(self.package)
            .config(&self.config)
            .domain(&self.domain)
            .module(self.module)
            .woog(&mut woog)
            .generator(DwarfBuilder::new().definition(DwarfFile::new()).build()?)
            .generate()?;

        GeneratorBuilder::new()
            .path(&chacha_file)?
            .package(self.package)
            .config(&self.config)
            .domain(&self.domain)
            .module(self.module)
            .woog(&mut woog)
            .generator(ChaChaBuilder::new().definition(ChaChaFile::new()).build()?)
            .imports(&imported_domains)
            .generate()?;

        // Ok(())
        // })
        // .collect::<Result<Vec<_>, _>>()?;

        Ok(objects.len())
    }

    fn domain(&self) -> &str {
        self.domain.domain()
    }
}

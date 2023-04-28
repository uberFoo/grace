use std::{
    fs,
    path::{Path, PathBuf},
    sync::RwLock,
};

use lazy_static::lazy_static;
use rayon::prelude::*;
use sarzak::{
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
    types::dwarf::{DwarfBuilder, DwarfModule},
    woog::init_woog,
    BUILD_DIR, TARGET_DIR,
};

pub(crate) const DWARF_EXT: &str = "道";

lazy_static! {
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
            src_path: src_path.as_ref(),
            domain,
            woog,
            _test,
        }))
    }
}

impl<'a> Target for DwarfTarget<'a> {
    fn compile(&mut self) -> Result<(), ModelCompilerError> {
        let mut path = PathBuf::from(self.src_path);
        path.pop();
        path.push(TARGET_DIR);
        path.push(BUILD_DIR);
        path.push(self.domain.name());

        fs::create_dir_all(&path).context(FileSnafu {
            description: "creating type directory".to_owned(),
            path: &path,
        })?;

        path.push("discard");
        path.set_file_name(self.domain.name().as_ident());
        path.set_extension(DWARF_EXT);

        // Sort the objects -- I need to figure out how to do this automagically.
        let mut objects: Vec<&Object> = self.domain.sarzak().iter_object().collect();
        objects.sort_by(|a, b| a.name.cmp(&b.name));

        objects
            .par_iter()
            .map(|_obj| {
                let mut woog = self.woog.clone();

                GeneratorBuilder::new()
                    .path(&path)?
                    .package(&self.package)
                    .config(&self.config)
                    .domain(&self.domain)
                    .module(self.module)
                    .woog(&mut woog)
                    .generator(DwarfBuilder::new().definition(DwarfModule::new()).build()?)
                    .generate()?;

                Ok(())
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(())
    }

    fn domain(&self) -> &str {
        self.domain.domain()
    }
}

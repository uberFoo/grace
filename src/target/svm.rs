use std::{
    path::{Path, PathBuf},
    sync::RwLock,
};


use sarzak::{
    lu_dog::store::ObjectStore as LuDogStore,
    mc::{FileSnafu, ModelCompilerError, Result},
    woog::store::ObjectStore as WoogStore,
};
use snafu::prelude::*;

use crate::{
    codegen::{render::RenderIdent},
    options::{GraceCompilerOptions, GraceConfig},
    target::Target,
    woog::init_woog,
    BIN, BUILD_DIR, RS_EXT, SVM, TARGET_DIR,
};

pub(crate) struct SvmTarget<'a> {
    config: GraceConfig,
    package: &'a str,
    module: &'a str,
    src_path: &'a Path,
    domain: sarzak::v2::domain::Domain,
    woog: WoogStore,
    lu_dog: RwLock<LuDogStore>,
    _test: bool,
}

impl<'a> SvmTarget<'a> {
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
        let lu_dog = LuDogStore::new();
        let lu_dog = RwLock::new(lu_dog);

        Ok(Box::new(Self {
            config,
            package,
            module,
            src_path: src_path.as_ref(),
            domain,
            woog,
            lu_dog,
            _test,
        }))
    }
}

impl<'a> Target for SvmTarget<'a> {
    fn compile(&mut self) -> Result<(), ModelCompilerError> {
        let mut lu_dog_path = PathBuf::from(self.src_path);
        lu_dog_path.pop();
        lu_dog_path.push(TARGET_DIR);
        lu_dog_path.push(BUILD_DIR);
        lu_dog_path.push(self.domain.name());

        // Load up the populated LuDog store, which contains instances from
        // parsing a dwarf file.
        let lu_dog = LuDogStore::load(&lu_dog_path).context(FileSnafu {
            description: "loading Lu-Dog store",
            path: lu_dog_path,
        })?;

        let mut output_path = PathBuf::from(self.src_path);
        output_path.push(BIN);
        output_path.push("discard");
        output_path.set_file_name(format!("{}_{}", self.domain.name().as_ident(), SVM));
        output_path.set_extension(RS_EXT);

        for import in lu_dog.iter_import() {
            dbg!(import);
        }

        for struct_ in lu_dog.iter_woog_struct() {
            dbg!(struct_);
        }

        for func in lu_dog.iter_function() {
            dbg!(func);
        }

        // // Sort the objects -- I need to figure out how to do this automagically.
        // let mut objects: Vec<&Object> = self.domain.sarzak().iter_object().collect();
        // objects.sort_by(|a, b| a.name.cmp(&b.name));

        // objects
        //     .par_iter()
        //     .map(|obj| {
        //         let mut woog = self.woog.clone();

        //         GeneratorBuilder::new()
        //             .path(&path)?
        //             .package(&self.package)
        //             .config(&self.config)
        //             .domain(&self.domain)
        //             .module(self.module)
        //             .woog(&mut woog)
        //             .lu_dog(&self.lu_dog)
        //             .generator(SvmBuilder::new().definition(SvmModule::new()).build()?)
        //             .generate()?;

        //         Ok(())
        //     })
        //     .collect::<Result<Vec<_>, _>>()?;

        Ok(())
    }

    fn domain(&self) -> &str {
        self.domain.domain()
    }
}

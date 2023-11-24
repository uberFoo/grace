use std::path::Path;

use sarzak::mc::ModelCompilerOptions;

mod codegen;
pub mod options;
mod target;
mod todo;
mod types;
mod woog;

pub use options::{
    DomainConfig, DwarfConfig, GraceCompilerOptions, OptimizationLevel, Target, UberStoreOptions,
};
pub use sarzak::mc::{FileSnafu, ModelCompilerError, SarzakModelCompiler};

use target::{
    application::ApplicationTarget, domain::DomainTarget, dwarf::DwarfTarget, svm::SvmTarget,
};

type Lock<T> = std::sync::RwLock<T>;
// type Lock<T> = parking_lot::Mutex<T>;

#[macro_export]
macro_rules! s_read {
    ($arg:expr) => {
        $arg.read().unwrap()
    };
}

#[macro_export]
macro_rules! s_write {
    ($arg:expr) => {
        $arg.write().unwrap()
    };
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_TIME: &str = include!(concat!(env!("OUT_DIR"), "/timestamp.txt"));

pub(crate) const BIN: &str = "bin";
pub(crate) const BUILD_DIR: &str = "sarzak";
pub(crate) const LIB_NAME: &str = "lib";
pub(crate) const RS_EXT: &str = "rs";
pub(crate) const SVM: &str = "svm";
pub(crate) const TARGET_DIR: &str = "target";
pub(crate) const TYPES: &str = "types";

#[derive(Default)]
pub struct ModelCompiler {}

impl SarzakModelCompiler for ModelCompiler {
    fn compile<P: AsRef<Path>>(
        &self,
        domain: sarzak::v2::domain::Domain,
        package: &str,
        module: &str,
        src_path: P,
        options: Box<&dyn ModelCompilerOptions>,
        test: bool,
        _verbosity: u8,
    ) -> Result<usize, ModelCompilerError> {
        // Extract our options
        let options = match options.as_any().downcast_ref::<GraceCompilerOptions>() {
            Some(options) => options.clone(),
            None => GraceCompilerOptions::default(),
        };

        let mut target = match options.target {
            Target::Domain(_) => {
                DomainTarget::new(&options, package, module, src_path.as_ref(), domain, test)?
            }
            Target::Application => {
                ApplicationTarget::new(&options, package, module, src_path.as_ref(), domain, test)?
            }
            Target::Dwarf(_) => {
                DwarfTarget::new(&options, package, module, src_path.as_ref(), domain, test)?
            }
            Target::Svm => {
                SvmTarget::new(&options, package, module, src_path.as_ref(), domain, test)?
            }
        };

        println!(
            "grace model compiler version: {}. Born on: {}",
            VERSION, BUILD_TIME
        );

        log::debug!(
            "compile invoked with model: {}, package: {}, module: {}, src_path: {}, options: {:?}, test: {}",
            target.domain(),
            package,
            module,
            src_path.as_ref().display(),
            options,
            test
        );

        target.compile()
    }
}

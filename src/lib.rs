use std::path::Path;

use sarzak::mc::ModelCompilerOptions;

mod codegen;
mod init_woog;
pub mod options;
mod targets;
mod todo;
mod types;

pub use options::{DomainConfig, GraceCompilerOptions, Target};
pub use sarzak::{
    mc::{FileSnafu, ModelCompilerError, SarzakModelCompiler},
    sarzak::types::{External, Ty},
    woog::types::{Mutability, BORROWED},
};

use targets::{application::ApplicationTarget, domain::DomainTarget};

pub(crate) const RS_EXT: &str = "rs";
pub(crate) const TYPES: &str = "types";

#[derive(Default)]
pub struct ModelCompiler {}

impl SarzakModelCompiler for ModelCompiler {
    fn compile<P: AsRef<Path>>(
        &self,
        domain: sarzak::domain::DomainBuilder,
        package: &str,
        module: &str,
        src_path: P,
        options: Box<&dyn ModelCompilerOptions>,
        test: bool,
    ) -> Result<(), ModelCompilerError> {
        // Extract our options
        let options = match options.as_any().downcast_ref::<GraceCompilerOptions>() {
            Some(options) => options.clone(),
            None => GraceCompilerOptions::default(),
        };

        let mut target = match options.target {
            Target::Domain(_) => {
                DomainTarget::new(&options, package, module, src_path.as_ref(), domain, test)
            }
            Target::Application => {
                ApplicationTarget::new(&options, package, module, src_path.as_ref(), domain, test)
            }
        };

        log::debug!(
            "compile invoked with model: {}, module: {}, src_path: {}, options: {:?}, test: {}",
            target.domain(),
            module,
            src_path.as_ref().display(),
            options,
            test
        );

        target.compile()?;

        Ok(())
    }
}

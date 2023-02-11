use std::path::Path;

use sarzak::mc::ModelCompilerOptions;

mod codegen;
pub mod options;
mod target;
mod todo;
mod types;

pub use options::{GraceCompilerOptions, Target};
pub use sarzak::{
    mc::{FileSnafu, ModelCompilerError, SarzakModelCompiler},
    sarzak::types::{External, Type},
    woog::types::{Mutability, BORROWED},
};

use sarzak::woog::store::ObjectStore as WoogStore;
use target::{application::ApplicationTarget, domain::DomainTarget};

pub(crate) const RS_EXT: &str = "rs";
pub(crate) const TYPES: &str = "types";

#[derive(Default)]
pub struct ModelCompiler {}

impl SarzakModelCompiler for ModelCompiler {
    fn compile<P: AsRef<Path>>(
        &self,
        domain: sarzak::domain::DomainBuilder,
        _package: &str,
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

        // Create our local compiler domain
        let mut woog = WoogStore::new();
        sarzak::woog::init_instances(&mut woog);

        let mut target = match options.target {
            Target::Domain => DomainTarget::new(
                &options,
                _package,
                module,
                src_path.as_ref(),
                domain,
                woog,
                test,
            ),
            Target::Application => ApplicationTarget::new(
                &options,
                _package,
                module,
                src_path.as_ref(),
                domain,
                woog,
                test,
            ),
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

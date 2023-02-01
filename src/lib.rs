use std::{
    fs,
    path::{Path, PathBuf},
};

use sarzak::mc::ModelCompilerOptions;
use snafu::prelude::*;
use uuid::Uuid;

mod codegen;
pub mod options;
mod types;

pub use options::GraceCompilerOptions;
pub use sarzak::mc::{FileSnafu, ModelCompilerError, SarzakModelCompiler};

use codegen::{generator::GeneratorBuilder, render::RenderIdent};
use sarzak::sarzak::types::Object;
use types::{
    default::{
        DefaultImplBuilder, DefaultModule, DefaultModuleBuilder, DefaultNewImpl, DefaultStruct,
        DefaultStructBuilder,
    },
    domain::{DomainImplBuilder, DomainNewImpl, DomainStruct},
};

const RS_EXT: &str = "rs";
const TYPES: &str = "types";

#[derive(Default)]
pub struct ModelCompiler {}

impl SarzakModelCompiler for ModelCompiler {
    fn compile<P: AsRef<Path>>(
        &self,
        domain: &sarzak::domain::Domain,
        module: &str,
        src_path: P,
        options: Box<&dyn ModelCompilerOptions>,
        _test: bool,
    ) -> Result<(), ModelCompilerError> {
        log::debug!(
            "compile invoked with model: {}, module: {}, src_path: {}, options: {:?}, test: {}",
            domain.domain(),
            module,
            src_path.as_ref().display(),
            options,
            _test
        );
        // ✨Generate Types✨
        // Extract our options
        let options = match options.as_any().downcast_ref::<GraceCompilerOptions>() {
            Some(options) => options.clone(),
            None => GraceCompilerOptions::default(),
        };

        // Build a path to src/types
        let mut types = PathBuf::from(src_path.as_ref());
        types.push(module);
        types.push(TYPES);
        fs::create_dir_all(&types).context(FileSnafu { path: &types })?;
        types.push("discard");

        // Sort the objects -- I need to figure out how to do this automagically.
        let mut objects: Vec<(&Uuid, &Object)> = domain.sarzak().iter_object().collect();
        objects.sort_by(|a, b| a.1.name.cmp(&b.1.name));

        // Iterate over the objects, generating an implementation for file each.
        for (id, obj) in objects {
            types.set_file_name(obj.as_ident());
            types.set_extension(RS_EXT);

            let (struct_writer, impl_writer) = if options.generate_domain {
                (
                    DomainStruct::new(),
                    DomainImplBuilder::new()
                        .implementation(DomainNewImpl::new())
                        .build(),
                )
            } else {
                (
                    DefaultStruct::new(),
                    DefaultImplBuilder::new()
                        .implementation(DefaultNewImpl::new())
                        .build(),
                )
            };

            // Here's the generation.
            GeneratorBuilder::new()
                .options(&options)
                // Where to write
                .path(&types)?
                // Domain/Store
                .domain(&domain)
                // Module name
                .module(module)
                .obj_id(&id)
                // What to write
                .generator(
                    // Struct
                    DefaultStructBuilder::new()
                        // Definition type
                        .definition(struct_writer)
                        // Implementation
                        .implementation(impl_writer)
                        .build()?,
                )
                .generate()?;
        }

        let mut types = PathBuf::from(src_path.as_ref());
        types.push(module);
        types.push("discard");
        types.set_file_name(TYPES);
        types.set_extension(RS_EXT);

        // Generate a "types.rs" module file containing all of the types.
        // This needs to be done after the types are generated so that rustfmt
        // doesn't complain an us.
        GeneratorBuilder::new()
            .options(&options)
            .path(&types)?
            .domain(&domain)
            .module(module)
            .generator(
                DefaultModuleBuilder::new()
                    .definition(DefaultModule::new())
                    .build()?,
            )
            .generate()?;

        // // Generate macros.rs
        // let mut types = PathBuf::from(src_path.as_ref());
        // types.push(module);
        // types.push("macros.rs");

        // GeneratorBuilder::new()
        //     .path(&types)?
        //     .generate()?
        //     .generate()?;

        // // Generate store.rs
        // let mut types = PathBuf::from(src_path.as_ref());
        // types.push(module);
        // types.push("store.rs");

        // GeneratorBuilder::new()
        //     .path(&types)?
        //     .generate()?
        //     .generate()?;

        Ok(())
    }
}

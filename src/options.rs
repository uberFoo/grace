//! Configuration Options for the grace model compiler
//!
//! This is very much a 1.0 sort of thing. Maybe pre-1.0, since I don't have any
//! idea what 1.0 would even look like. My point is that I plan on generating
//! the config structs. But to do that, I need a model compiler, and there's no
//! way that I'm hacking more on nut.
//!
//! So maybe, the first feature to come out of this is an implementation to
//! generate options files! Boy, that sure would be pretty limited wouldn't it?
//! Maybe not. Looking at what I need to implement below, It's actually just
//! a clap Args derive, with inserted attributes. This may actually be a good
//! test case.
//!
//! I need te generate a struct definition anyway. All generating this would
//! be is modifying a struct definition. And that's just the sort of problem that
//! I should solve early.
use std::any::Any;

use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};

use sarzak::mc::ModelCompilerOptions;

const DEFAULT_TARGET: Target = Target::Application;
const DEFAULT_DERIVE: &'static [&'static str] = &["Debug", "PartialEq"];
const DEFAULT_USE_PATHS: Option<Vec<String>> = None;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Subcommand)]
pub enum Target {
    /// Target Domain Infrastructure
    ///
    /// This target is used by model compilers to generate code.
    Domain,
    /// Target Application Code
    ///
    /// This target is intended to be run as an application.
    Application,
}

#[derive(Args, Clone, Debug, Deserialize, Serialize)]
pub struct GraceCompilerOptions {
    /// Code Generation Target
    ///
    /// This determines how objects are rendered into structs and enums.
    #[command(subcommand)]
    pub target: Target,
    /// Derive macros
    ///
    /// A comma separated list of derive macros to be added to each generated
    /// item, globally.
    ///
    /// Note that this option is available on a per-object basis using the
    /// description coloring option: `// 🐶 {"derive": ["macro", ...]}`.
    #[arg(long, short, use_value_delimiter = true, value_delimiter = ',')]
    pub derive: Option<Vec<String>>,
    /// Use Paths
    ///
    /// These are paths to be issued as `use` statements.
    #[arg(long, short, use_value_delimiter = true, value_delimiter = ',')]
    pub use_paths: Option<Vec<String>>,
}

impl ModelCompilerOptions for GraceCompilerOptions {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Default for GraceCompilerOptions {
    fn default() -> Self {
        Self {
            target: DEFAULT_TARGET,
            derive: Some(DEFAULT_DERIVE.iter().map(|&x| x.to_owned()).collect()),
            use_paths: DEFAULT_USE_PATHS,
        }
    }
}

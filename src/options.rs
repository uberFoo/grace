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

use clap::Args;
use serde::{Deserialize, Serialize};

use nut::sarzak::mc::ModelCompilerOptions;

#[derive(Args, Clone, Debug, Deserialize, Serialize)]
pub struct GraceCompilerOptions {}

impl ModelCompilerOptions for GraceCompilerOptions {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Default for GraceCompilerOptions {
    fn default() -> Self {
        Self {}
    }
}

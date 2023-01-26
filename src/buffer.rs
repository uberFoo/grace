//! A buffer for building files
//!
use std::{
    fs::File,
    path::{Path, PathBuf},
};

use sarzak::mc::{IOSnafu, ModelSnafu, Result};
use snafu::prelude::*;

pub struct BufferBuilder {
    path: Option<PathBuf>,
}

impl BufferBuilder {
    pub fn new() -> Self {
        BufferBuilder { path: None }
    }

    pub fn path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.path = Some(path.as_ref().to_path_buf());

        self
    }

    pub fn build(self) -> Result<Buffer> {
        ensure!(
            self.path.is_some(),
            ModelSnafu {
                description: "missing path"
            }
        );

        Ok(Buffer {
            path: self.path.unwrap(),
        })
    }
}

pub struct Buffer {
    path: PathBuf,
}

impl Buffer {
    pub fn write_file(&self) -> Result<()> {
        File::create(&self.path).context(IOSnafu {
            path: self.path.clone(),
        })?;

        Ok(())
    }
}

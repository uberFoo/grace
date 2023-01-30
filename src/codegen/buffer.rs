//! A buffer for building files
//!
use std::{
    fmt::{self, Write},
    ops::AddAssign,
};

use sarzak::mc::{FormatSnafu, Result};
use serde_json;
use snafu::prelude::*;

use crate::codegen::{DirectiveComment, DirectiveKind};

pub(crate) struct Buffer {
    buffer: String,
}

impl Buffer {
    pub(crate) fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub(crate) fn block<S, F>(&mut self, directive: DirectiveKind, tag: S, block: F) -> Result<()>
    where
        S: AsRef<str>,
        F: FnOnce(&mut Self) -> Result<()>,
    {
        let mut inner = Self::new();

        block(&mut inner)?;

        // Don't do anything if nothing happened.
        if inner.buffer.len() != 0 {
            let start_comment = serde_json::to_string(&DirectiveComment::start(
                directive.clone(),
                tag.as_ref().to_owned(),
            ))
            .expect("serde_json failed");
            let end_comment = serde_json::to_string(&DirectiveComment::end(directive))
                .expect("serde_json failed");

            writeln!(self.buffer, "// {}", start_comment).context(FormatSnafu)?;
            writeln!(inner, "// {}", end_comment).context(FormatSnafu)?;
            *self += inner;
        }

        Ok(())
    }

    pub(crate) fn dump(&self) -> &String {
        &self.buffer
    }
}

impl fmt::Write for Buffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.buffer.write_str(s)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.buffer.write_char(c)
    }
}

impl AddAssign for Buffer {
    fn add_assign(&mut self, rhs: Self) {
        self.buffer += rhs.buffer.as_str();
    }
}

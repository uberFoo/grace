//! A buffer for building files
//!
use std::{
    fmt::{self, Write},
    ops::AddAssign,
};

use sarzak::mc::{FormatSnafu, Result};
use serde::Serialize;
use serde_json;
use snafu::prelude::*;

const MAGIC: char = '';

#[derive(Serialize)]
pub(crate) enum Directive {
    #[serde(rename = "provenance")]
    Provenance,
    #[serde(rename = "ignore")]
    Ignore,
    #[serde(rename = "prefer-new")]
    PreferNewCommentOld,
    #[serde(rename = "prefer-old")]
    PreferOldCommentNew,
}

#[derive(Serialize)]
struct DirectiveComment {
    magic: char,
    directive: Directive,
    tag: String,
}

pub(crate) struct Buffer {
    buffer: String,
}

impl Buffer {
    pub(crate) fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub(crate) fn block<S, F>(&mut self, directive: Directive, tag: S, mut block: F) -> Result<()>
    where
        S: AsRef<str>,
        F: FnOnce(&mut Self) -> Result<()>,
    {
        let mut inner = Self::new();

        block(&mut inner)?;

        // Don't do anything if nothing happened.
        if inner.buffer.len() != 0 {
            let comment_directive = DirectiveComment {
                magic: MAGIC,
                directive,
                tag: tag.as_ref().to_owned(),
            };
            let comment = serde_json::to_string(&comment_directive).expect("serde_json failed");

            writeln!(self.buffer, "// {}", comment).context(FormatSnafu)?;
            writeln!(inner, "// {}", comment).context(FormatSnafu)?;
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

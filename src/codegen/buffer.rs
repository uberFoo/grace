//! A buffer for building files
//!
use std::{fmt, ops::AddAssign};

pub(crate) struct Buffer {
    buffer: String,
}

impl Buffer {
    pub(crate) fn new() -> Self {
        Self {
            buffer: String::new(),
        }
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

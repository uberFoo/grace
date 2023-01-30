//! Things necessary for code generation
//!

use diff;
use serde::{Deserialize, Serialize};

pub(crate) mod buffer;
pub(crate) mod generator;
pub(crate) mod render;
mod rustfmt;

const MAGIC: char = '';
// const UBER: char = "❌";

/// Diff Directives
///
/// These describe diff behavior. They are all from the perspective of the
/// original file. So, orig is the source file, and new is the generated
/// code.
///
/// Each output code block will be wrapped in a pair of these. For all lines
/// in the wrapped pair, the behavior of the diff engine is defined as...
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub(crate) enum DirectiveKind {
    /// Comment Original
    ///
    /// The intent of this is to comment lines in the original that don't
    /// occur in the generated code.
    #[serde(rename = "comment-orig")]
    CommentOrig,
    /// Comment Generated
    ///
    /// This means that incoming changes, generated code, that does not exist in
    /// the original source will be output as commented code.
    #[serde(rename = "comment-gen")]
    CommentGenerated,
    /// Ignore Generated
    ///
    /// Generated code will not be output is this section.
    #[serde(rename = "ignore-gen")]
    IgnoreGenerated,
    /// Ignore Original
    ///
    /// This implies that anything added to this section will be eradicated
    /// by code gen.
    #[serde(rename = "ignore-orig")]
    IgnoreOrig,
}

#[derive(Debug, Deserialize, Serialize)]
enum Directive {
    Start {
        directive: DirectiveKind,
        tag: String,
    },
    End {
        directive: DirectiveKind,
    },
}

#[derive(Debug, Deserialize, Serialize)]
struct DirectiveComment {
    magic: char,
    directive: Directive,
}

impl DirectiveComment {
    fn start(directive: DirectiveKind, tag: String) -> Self {
        Self {
            magic: MAGIC,
            directive: Directive::Start { directive, tag },
        }
    }

    fn end(directive: DirectiveKind) -> Self {
        Self {
            magic: MAGIC,
            directive: Directive::End { directive },
        }
    }
}

/// Diff Entry Point
///
/// Given to strings, diff according to the rules, which are defined using
/// directives embedded in the source file.
fn process_diff(orig: &str, incoming: &str, directive: DirectiveKind) -> String {
    log::trace!("diffing buffers");
    let mut diff = diff::lines(orig, incoming);

    // Reverse the diff so that we can just pop lines off the end when we process
    // the list.
    diff.reverse();

    process_diff_not_recursive_after_all(&mut diff, directive)
}

/// Process the Diff
///
/// Process each line of the diff, and do the right thing. This was meant to be
/// recursive, but the borrow checker would have none it. So now it isn't.
///
/// This function is a bit complicated. It processes each line as an insertion,
/// from the original Left), an insertion from the generated (Right), or both
/// buffers contain the line (Both).
///
/// Since we need to process directives, we parse them as we see them. However,
/// we only parse the directive if it comes from the file. So, in Left, and
/// Both. We don't want to process directives in Right because we don't want it
/// to interfere with any directives in the file. This is slightly less than
/// optimal, because it means that sometimes a directive end is output. This
/// can happen if a block type is changed in the source from the generated. I'm
/// not quite sure what to do about it.
///
/// In fact, I'm not convinced that this is the correct approach. I need to do
/// some serious thinking about this.
fn process_diff_not_recursive_after_all<'a>(
    lines: &'a mut Vec<diff::Result<&'a str>>,
    directive: DirectiveKind,
) -> String {
    let mut stack = Vec::new();
    let mut directive = directive;
    let mut output = String::new();

    while lines.len() > 0 {
        let line = lines.pop().expect("lines.pop()");
        match line {
            diff::Result::Left(orig) => {
                // Parse directive
                match parse_directive(orig) {
                    Some(d) => match d {
                        Directive::Start {
                            directive: d,
                            tag: _,
                        } => {
                            // Write the line -- always write the directive
                            output.push_str(orig);
                            output.push('\n');

                            // Instead of recursion...
                            stack.push(directive);
                            directive = d;
                        }
                        Directive::End { directive: d } => {
                            assert_eq!(d, directive);

                            // Write the line -- always write the directive
                            output.push_str(orig);
                            output.push('\n');

                            directive = stack.pop().expect("unbalanced directives")
                        }
                    },
                    None => {
                        // Process line
                        write_left(orig, &mut output, &directive);
                    }
                };
            }
            diff::Result::Both(both, _) => {
                // Parse directive
                match parse_directive(both) {
                    Some(d) => match d {
                        Directive::Start {
                            directive: d,
                            tag: _,
                        } => {
                            // The directive will be written below.
                            // Instead of recursion...
                            stack.push(directive);
                            directive = d;
                        }
                        Directive::End { directive: d } => {
                            assert_eq!(d, directive);
                            // The directive will be written below.
                            directive = stack.pop().expect("unbalanced directives")
                        }
                    },
                    None => {}
                };

                // Process line
                // If it's in both, we always just write it.
                output.push_str(both);
                output.push('\n');
            }
            diff::Result::Right(new) => {
                // If we processed directives here, we may have a chance of
                // catching trailing end directives that should not be written.

                // Process line
                write_right(new, &mut output, &directive);
            }
        }
    }

    output
}

/// Write a line that exists in the file, but not the generated code.
///
fn write_left(line: &str, output: &mut String, directive: &DirectiveKind) {
    match directive {
        // Ignoring new means that we write the line
        DirectiveKind::IgnoreGenerated => {
            output.push_str(line);
            output.push('\n');
        }
        // This implies that we write the original line
        DirectiveKind::CommentGenerated => {
            output.push_str(line);
            output.push('\n');
        }
        // This means that we should comment this out.
        DirectiveKind::CommentOrig => {
            output.push('/');
            output.push('/');
            output.push(' ');
            output.push_str(line);
            output.push('\n');
        }
        _ => {}
    }
}

/// Write a line that exists in the generated code, but not the file.
fn write_right(line: &str, output: &mut String, directive: &DirectiveKind) {
    match directive {
        // Ignoring orig means that we write the line
        DirectiveKind::IgnoreOrig => {
            output.push_str(line);
            output.push('\n');
        }
        // Prefer new means that we write the line
        DirectiveKind::CommentOrig => {
            output.push_str(line);
            output.push('\n');
        }
        // This means that we should comment this out.
        DirectiveKind::CommentGenerated => {
            output.push('/');
            output.push('/');
            output.push(' ');
            output.push_str(line);
            output.push('\n');
        }
        _ => {}
    }
}

/// Parse a &str looking for ✨ MAGIC ✨
///
/// Directives are always behind comments.
///
/// There is the opportunity for an override: ☯️. I'm just not quite sure how
/// I want to use it.
fn parse_directive(line: &str) -> Option<Directive> {
    let mut test = String::from(line);
    test = test.trim_start().to_owned();
    if test.starts_with("//") {
        test.replace_range(..3, "");
        if let Ok(directive_comment) = serde_json::from_str::<DirectiveComment>(test.as_str()) {
            let directive = directive_comment.directive;
            log::trace!("found directive: {:?}", directive);
            Some(directive)
        } else {
            None
        }
    } else {
        None
    }
}

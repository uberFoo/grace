//! The Famous Diff Engine
//!
//! Makes me think of "Feersum Endjinn"...
//!
//! This takes care of writing the output to the file, hopefully without clobbering
//! what's already there. While also hopefully actually doing it's job.
use diff;
use serde::{Deserialize, Serialize};

const MAGIC: char = '';
const UBER: char = '❌';

/// Diff Directives
///
/// These describe diff behavior. They are all from the perspective of the
/// original file. So, orig is the source file, and new is the generated
/// code.
///
/// Each output code block will be wrapped in a pair of these. For all lines
/// in the wrapped pair, the behavior of the diff engine is defined as...
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
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
    /// Allow Editing
    ///
    /// This may be one of two of these that I actually need. This simply states
    /// that unless otherwise restricted, editing is allowed in this section.
    /// By default the entire file should be marked with this, with generated
    /// code bracketed by the other one I need. CommentOrig, I think.
    #[serde(rename = "allow-editing")]
    AllowEditing,
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
pub(crate) struct DirectiveComment {
    magic: char,
    directive: Directive,
}

impl DirectiveComment {
    pub(crate) fn start(directive: DirectiveKind, tag: String) -> Self {
        Self {
            magic: MAGIC,
            directive: Directive::Start { directive, tag },
        }
    }

    pub(crate) fn end(directive: DirectiveKind) -> Self {
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
pub(crate) fn process_diff(orig: &str, incoming: &str, directive: DirectiveKind) -> String {
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
                            output.extend([orig, "\n"]);

                            // Instead of recursion...
                            stack.push(directive);
                            directive = d;
                        }
                        Directive::End { directive: d } => {
                            if d == directive {
                                // Write the line
                                output.extend([orig, "\n"]);
                            } else {
                                // Don't output an un-balanced directive.
                            }

                            directive = stack.pop().expect("unbalanced directives")
                        }
                    },
                    None => {
                        // Process line
                        write_orig_only(orig, &mut output, &directive);
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
                            if d != directive {
                                log::error!("unbalanced directives: {:?} != {:?}", d, directive);
                            }
                            // The directive will be written below.
                            directive = stack.pop().expect("unbalanced directives")
                        }
                    },
                    None => {}
                };

                // Process line
                // If it's in both, we always just write it.
                output.extend([both, "\n"]);
            }
            diff::Result::Right(new) => {
                // If we processed directives here, we may have a chance of
                // catching trailing end directives that should not be written.

                // Process line
                write_generated_only(new, &mut output, &directive);
            }
        }
    }

    output
}

/// Write a line that exists in the file, but not the generated code.
///
fn write_orig_only(line: &str, output: &mut String, directive: &DirectiveKind) {
    match directive {
        // Ignoring new means that we write the line
        DirectiveKind::IgnoreGenerated | DirectiveKind::AllowEditing => {
            output.extend([line, "\n"]);
        }
        // This implies that we write the original line
        DirectiveKind::CommentGenerated => {
            output.extend([line, "\n"]);
        }
        // This means that we should comment this out.
        DirectiveKind::CommentOrig => {
            output.extend([comment_line(line), "\n".to_owned()]);
        }
        _ => log::trace!("orig unhandled directive {:?}", directive),
    }
}

/// Write a line that exists in the generated code, but not the file.
fn write_generated_only(line: &str, output: &mut String, directive: &DirectiveKind) {
    match directive {
        // Ignoring orig means that we write the line
        DirectiveKind::IgnoreOrig | DirectiveKind::AllowEditing => {
            output.extend([line, "\n"]);
        }
        // Prefer new means that we write the line
        DirectiveKind::CommentOrig => {
            output.extend([line, "\n"]);
        }
        // This means that we should comment this out.
        DirectiveKind::CommentGenerated => {
            output.extend([comment_line(line), "\n".to_owned()]);
        }
        // Ignore this line.
        DirectiveKind::IgnoreGenerated => {}
    }
}

fn comment_line(line: &str) -> String {
    let test = line.trim();
    if test.starts_with("//") {
        line.to_owned()
    } else {
        format!("// {}", line)
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
        test.replace_range(..2, "");
        if let Ok(directive_comment) = serde_json::from_str::<DirectiveComment>(test.as_str()) {
            let directive = match directive_comment.magic {
                UBER => match directive_comment.directive {
                    Directive::Start { directive, ref tag } => match directive {
                        DirectiveKind::IgnoreGenerated => {
                            log::trace!("overriding IgnoreGenerated start with IgnoreOrig start");
                            Directive::Start {
                                directive: DirectiveKind::IgnoreOrig,
                                tag: tag.to_owned(),
                            }
                        }
                        _ => directive_comment.directive,
                    },
                    Directive::End { directive } => match directive {
                        DirectiveKind::IgnoreGenerated => {
                            log::trace!("overriding IgnoreGenerated end with IgnoreOrig end");
                            Directive::End {
                                directive: DirectiveKind::IgnoreOrig,
                            }
                        }
                        _ => directive_comment.directive,
                    },
                },
                MAGIC => directive_comment.directive,
                _ => panic!("bad voodoo: {}", directive_comment.magic),
            };
            log::trace!("found directive: {:?}", directive);
            Some(directive)
        } else {
            None
        }
    } else {
        None
    }
}

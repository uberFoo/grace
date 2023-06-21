use std::{fs, io::prelude::*, path::Path, process};

use sarzak::mc::{CompilerSnafu, IOSnafu, Result};
use snafu::prelude::*;
use tempfile::NamedTempFile;

/// Format
///
/// We call rustfmt to do our formatting. This may need to change, because we
/// are doing a lot of this now.
///
/// If there is an error, we will optionally display the offending file.
pub(crate) fn format(path: &Path, display_err: bool) -> Result<()> {
    log::trace!("running `rustfmt --emit files {}`", path.display());

    // Run rustfmt on the file
    let child = process::Command::new("rustfmt")
        // .arg(&path.to_str().expect("this is a pain in the dick"))
        .args([
            "--edition",
            "2021",
            "--emit",
            "files",
            format!("{}", path.display()).as_str(),
        ])
        .stderr(process::Stdio::piped())
        .spawn()
        .context(IOSnafu {
            description: "spawning rustfmt".to_owned(),
        })?;

    // Wait for the process to finish.
    let output = child.wait_with_output().context(IOSnafu {
        description: "waiting for rustfmt to complete".to_owned(),
    })?;
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() && display_err {
        if cfg!(feature = "vscode") {
            // Save the file off
            let mut fail_file = NamedTempFile::new().context(IOSnafu {
                description: "getting temp file".to_owned(),
            })?;
            fail_file
                .write_all(
                    fs::read_to_string(path)
                        .expect("read_to_string")
                        .as_bytes(),
                )
                .context(IOSnafu {
                    description: "writing file with erroneous code".to_owned(),
                })?;

            let (_, fail_path) = fail_file.keep().expect("error with temporary file");

            let mut child = process::Command::new("code")
                .args(["-w", format!("{}", fail_path.display()).as_str()])
                .stdin(process::Stdio::piped())
                .spawn()
                .context(IOSnafu {
                    description: "spawning vscode".to_owned(),
                })?;

            child.wait().context(IOSnafu {
                description: "waiting for vscode to complete".to_owned(),
            })?;
        } else {
            eprintln!("😱 rustfmt failed with:");
            eprintln!("{}", stderr);
        }
    }

    // We've already dealt with the failure case. We just use this to throw the
    // error.
    ensure!(
        output.status.success(),
        CompilerSnafu {
            description: format!("😱 rustfmt failed: {}", stderr)
        }
    );

    Ok(())
}

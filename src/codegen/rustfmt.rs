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
        .args(["--emit", "files", format!("{}", path.display()).as_str()])
        .stderr(process::Stdio::piped())
        .spawn()
        .context(IOSnafu)?;

    // Wait for the process to finish.
    let output = child.wait_with_output().context(IOSnafu)?;
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() && display_err {
        if cfg!(feature = "vscode") {
            // Save the file off
            let mut fail_file = NamedTempFile::new().context(IOSnafu)?;
            fail_file
                .write_all(
                    fs::read_to_string(&path)
                        .expect("read_to_string")
                        .as_bytes(),
                )
                .context(IOSnafu)?;

            let (_, fail_path) = fail_file.keep().expect("error with temporary file");

            process::Command::new("code")
                .args([format!("{}", fail_path.display())])
                .stdin(process::Stdio::piped())
                .spawn()
                .context(IOSnafu)?;
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

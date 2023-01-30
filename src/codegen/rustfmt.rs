use std::{fs, io::prelude::*, path::Path, process};

use sarzak::mc::{CompilerSnafu, IOSnafu, Result};
use snafu::prelude::*;
use tempfile::NamedTempFile;

pub(crate) fn format(path: &Path) -> Result<()> {
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

    if !output.status.success() {
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

            let path = path.to_path_buf();
            let mut to = path.clone();

            // Borrow these from path so that we can mutate to.
            let stem = path.file_stem().expect("can't get file stem");
            let ext = path.extension().expect("can't get file extension");

            to.set_file_name(format!(
                "{}_fail",
                stem.to_str().expect("can't turn it to a &str")
            ));
            to.set_extension(ext);

            // log::trace!("moving {} to {}", path.display(), to.display());
            // fs::rename(&path, &to).context(IOSnafu)?;

            process::Command::new("code")
                .args([format!("{}", fail_file.path().display())])
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

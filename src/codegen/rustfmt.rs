use std::{io::Write, process};

use sarzak::mc::{CompilerSnafu, IOSnafu, Result};
use snafu::prelude::*;

use tempfile::NamedTempFile;

pub(crate) fn format(buffer: &String) -> Result<String> {
    log::trace!("running `rustfmt --emit stdout`");

    // Write the buffer to a temporary file
    let mut file = NamedTempFile::new().context(IOSnafu)?;
    file.write_all(buffer.as_bytes()).context(IOSnafu)?;

    // Run rustfmt on the file
    let child = process::Command::new("rustfmt")
        // .arg(&path.to_str().expect("this is a pain in the dick"))
        .args(["--emit", "stdout", &file.path().to_string_lossy()])
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn()
        .context(IOSnafu)?;

    // Wait for the process to finish, and then read it's output buffer.
    let output = child.wait_with_output().context(IOSnafu)?;
    let buffer = String::from_utf8_lossy(&output.stdout);

    // Need to figure out what to do with the failed output. Maybe squirt it
    // to vscode? That would actually be really useful...
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);

        if cfg!(feature = "vscode") {
            // Just spray and pray.
            let mut vscode = process::Command::new("code")
                .args(["-"])
                .stdin(process::Stdio::piped())
                .spawn()
                .context(IOSnafu)?;

            let mut stdin = vscode.stdin.take().context(CompilerSnafu {
                description: "foo".to_owned(),
            })?;
            writeln!(stdin, "😱 rustfmt failed with:").context(IOSnafu)?;
            stdin.write_all(&stderr.as_bytes()).context(IOSnafu)?;
            writeln!(stdin, "😱 here is any offending output:",).context(IOSnafu)?;
            stdin.write_all(&buffer.as_bytes()).context(IOSnafu)?;
        } else {
            eprintln!("😱 rustfmt failed with:");
            eprintln!("{}", stderr);
            eprintln!("😱 here is any offending output:",);
            eprintln!("{}", buffer);
        }
    }

    // We've already dealt with the failure case. We just use this to throw the
    // error.
    ensure!(
        output.status.success(),
        CompilerSnafu {
            description: format!("rustfmt exited with status: {}", output.status)
        }
    );

    // Some junk get's appended to the top of stdout.
    let mut iter = buffer.splitn(3, '\n');
    iter.next();
    iter.next();

    Ok(iter.next().unwrap().into())
}

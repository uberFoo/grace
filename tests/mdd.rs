use std::process::{self, ExitCode};

use grace::{GraceCompilerOptions, ModelCompiler, SarzakModelCompiler};
use sarzak::domain::DomainBuilder;

/// Model Driven Development
///
/// This function builds the domains in the mdd package and runs cargo test on
/// the package.
#[test]
fn compile_and_test() -> Result<ExitCode, std::io::Error> {
    let options = GraceCompilerOptions::default();
    let grace = ModelCompiler::default();

    // Build the domains
    let domain = DomainBuilder::new()
        .cuckoo_model("tests/mdd/models/everything.json")
        .unwrap()
        .build()
        .unwrap();

    grace
        .compile(
            &domain,
            "everything",
            "tests/mdd/src",
            Box::new(&options),
            false,
        )
        .unwrap();

    // Run cargo test
    // Hopefully I can just pass the errors along...
    let mut child = process::Command::new("cargo")
        .arg("test")
        .arg("--")
        .arg("--nocapture")
        .current_dir("tests/mdd")
        .spawn()?;

    match child.wait() {
        Ok(e) => Ok(ExitCode::from(e.code().unwrap() as u8)),
        Err(e) => Err(e),
    }
}

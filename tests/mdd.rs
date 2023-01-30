use std::process::{self, ExitCode};

use env_logger;
use grace::{GraceCompilerOptions, ModelCompiler, SarzakModelCompiler};
use log;
use sarzak::domain::DomainBuilder;

/// Model Driven Development
///
/// This function builds the domains in the mdd package and runs cargo test on
/// the package.
#[test]
fn compile_and_test_default() -> Result<ExitCode, std::io::Error> {
    let _ = env_logger::builder().is_test(true).try_init();

    let options = GraceCompilerOptions::default();
    let grace = ModelCompiler::default();

    // Build the domains
    log::debug!("Testing everything domain.");
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

#[test]
fn compile_and_test_domain() -> Result<ExitCode, std::io::Error> {
    let _ = env_logger::builder().is_test(true).try_init();

    let mut options = GraceCompilerOptions::default();
    options.generate_domain = true;
    let grace = ModelCompiler::default();

    // Build the domains
    log::debug!("Testing everything domain.");
    let domain = DomainBuilder::new()
        .cuckoo_model("tests/mdd/models/everything.json")
        .unwrap()
        .build()
        .unwrap();

    grace
        .compile(
            &domain,
            "everything_domain",
            "tests/mdd/src",
            Box::new(&options),
            false,
        )
        .unwrap();

    // Run cargo test
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

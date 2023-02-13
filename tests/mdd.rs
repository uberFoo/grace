//! Model Driven Development FTW! ✨
//!
use std::process::{self, ExitCode};

use env_logger;
use grace::{GraceCompilerOptions, ModelCompiler, SarzakModelCompiler, Target};
use log;
use sarzak::domain::DomainBuilder;

macro_rules! test_target_domain {
    ($name:ident, $domain:literal) => {
        #[test]
        fn $name() -> Result<ExitCode, std::io::Error> {
            let _ = env_logger::builder().is_test(true).try_init();

            let mut options = GraceCompilerOptions::default();
            options.target = Target::Domain;
            if let Some(ref mut derive) = options.derive {
                derive.push("Clone".to_string());
                derive.push("Deserialize".to_string());
                derive.push("Serialize".to_string());
            }
            options.use_paths = Some(vec!["serde::{Deserialize, Serialize}".to_string()]);

            let grace = ModelCompiler::default();

            // Build the domains
            log::debug!("Testing domain: {},  target: Domain.", $domain);
            let domain = DomainBuilder::new()
                .cuckoo_model(format!("tests/mdd/models/{}.json", $domain))
                .unwrap();

            grace
                .compile(
                    domain,
                    "mdd",
                    format!("{}_domain", $domain).as_str(),
                    "tests/mdd/src",
                    Box::new(&options),
                    false,
                )
                .unwrap();

            // Run cargo test
            let mut child = process::Command::new("cargo")
                .arg("test")
                .arg(format!("{}_domain", $domain))
                .arg("--")
                .arg("--nocapture")
                .current_dir("tests/mdd")
                .spawn()?;

            match child.wait() {
                Ok(e) => Ok(ExitCode::from(e.code().unwrap() as u8)),
                Err(e) => Err(e),
            }
        }
    };
}

macro_rules! test_target_application {
    ($name:ident, $domain:literal) => {
        #[test]
        fn $name() -> Result<ExitCode, std::io::Error> {
            let _ = env_logger::builder().is_test(true).try_init();

            let mut options = GraceCompilerOptions::default();
            let grace = ModelCompiler::default();

            // Build the domains
            log::debug!("Testing domain: {},  target: Domain.", $domain);
            let domain = DomainBuilder::new()
                .cuckoo_model(format!("tests/mdd/models/{}.json", $domain))
                .unwrap();

            grace
                .compile(
                    domain,
                    "mdd",
                    $domain,
                    "tests/mdd/src",
                    Box::new(&options),
                    false,
                )
                .unwrap();

            // Run cargo test
            let mut child = process::Command::new("cargo")
                .arg("test")
                .arg(format!("{}::", $domain))
                .arg("--")
                .arg("--nocapture")
                .current_dir("tests/mdd")
                .spawn()?;

            match child.wait() {
                Ok(e) => Ok(ExitCode::from(e.code().unwrap() as u8)),
                Err(e) => Err(e),
            }
        }
    };
}

test_target_domain!(everything_domain, "everything");
test_target_domain!(one_to_one_domain, "one_to_one");
test_target_domain!(one_to_many_domain, "one_to_many");
test_target_application!(everything_application, "everything");

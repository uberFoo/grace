//! Model Driven Development FTW! ✨
//!
use std::process::{self, ExitCode};

use env_logger;
use grace::{
    DomainConfig, DwarfConfig, GraceCompilerOptions, ModelCompiler, OptimizationLevel,
    SarzakModelCompiler, Target, UberStoreOptions,
};
use log;
use sarzak::domain::DomainBuilder;
use tracy_client::Client;

macro_rules! test_target_domain {
    ($name:ident, $domain:literal, $path:literal) => {
        #[test]
        fn $name() -> Result<ExitCode, std::io::Error> {
            let _ = env_logger::builder().is_test(true).try_init();
            let _ = Client::start();

            let mut options = GraceCompilerOptions::default();
            options.target = Target::Domain(DomainConfig {
                persist: true,
                ..Default::default()
            });
            if let Some(ref mut derive) = options.derive {
                derive.push("Clone".to_string());
                derive.push("Deserialize".to_string());
                derive.push("Serialize".to_string());
            }
            options.use_paths = Some(vec!["serde::{Deserialize, Serialize}".to_string()]);
            options.always_process = Some(true);

            let grace = ModelCompiler::default();

            // Build the domains
            log::debug!(
                "Testing domain: {},  target: {:?}.",
                $domain,
                options.target
            );
            let domain = DomainBuilder::new()
                .cuckoo_model($path)
                .unwrap()
                .build_v2()
                .unwrap();

            grace
                .compile(
                    domain,
                    "mdd",
                    format!("domain/{}", $domain).as_str(),
                    "tests/mdd/src",
                    Box::new(&options),
                    false,
                ).map_err(|e| {
                    println!("Compiler exited with: {}", e);
                    std::io::Error::new(std::io::ErrorKind::Other, "Compiler exited with error")
                })?;

            // Run cargo test
            let mut child = process::Command::new("cargo")
                .arg("test")
                .arg(format!("domain::{}::tests", $domain))
                // .arg("--")
                // .arg("--nocapture")
                .current_dir("tests/mdd")
                .spawn()?;

            match child.wait() {
                Ok(e) => Ok(ExitCode::from(e.code().unwrap() as u8)),
                Err(e) => Err(e),
            }
        }
    };
    ($name:ident, $domain:literal, $path:literal, $($imports:literal),+) => {
        #[test]
        /// This one handles imports
        fn $name() -> Result<ExitCode, std::io::Error> {
            let _ = env_logger::builder().is_test(true).try_init();
            let _ = Client::start();

            let mut options = GraceCompilerOptions::default();
            options.target = Target::Domain(DomainConfig {
                persist: true,
                ..Default::default()
            });
            if let Some(ref mut derive) = options.derive {
                derive.push("Clone".to_string());
                derive.push("Deserialize".to_string());
                derive.push("Serialize".to_string());
            }
            options.use_paths = Some(vec!["serde::{Deserialize, Serialize}".to_string()]);
            let mut imports = Vec::new();
            $(
                imports.push($imports.to_string());
            )*
            options.imported_domains = Some(imports);
            options.always_process = Some(true);

            let grace = ModelCompiler::default();

            // Build the domains
            log::debug!(
                "Testing domain: {},  target: {:?}.",
                $domain,
                options.target
            );
            let domain = DomainBuilder::new()
                .cuckoo_model($path)
                .unwrap()
                .build_v2()
                .unwrap();

            grace
                .compile(
                    domain,
                    "mdd",
                    format!("domain/{}", $domain).as_str(),
                    "tests/mdd/src",
                    Box::new(&options),
                    false,
                )
                .map_err(|e| {
                    println!("Compiler exited with: {}", e);
                    std::io::Error::new(std::io::ErrorKind::Other, "Compiler exited with error")
                })?;

            // Run cargo test
            let mut child = process::Command::new("cargo")
                .arg("test")
                .arg(format!("domain::{}::tests", $domain))
                // .arg("--")
                // .arg("--nocapture")
                .current_dir("tests/mdd")
                .spawn()?;

            match child.wait() {
                Ok(e) => Ok(ExitCode::from(e.code().unwrap() as u8)),
                Err(e) => Err(e),
            }
        }
    };
}

macro_rules! test_target_domain_vec_store {
    ($name:ident, $domain:literal, $path:literal) => {
        #[test]
        fn $name() -> Result<ExitCode, std::io::Error> {
            let _ = env_logger::builder().is_test(true).try_init();
            let _ = Client::start();

            let mut options = GraceCompilerOptions::default();
            options.target = Target::Domain(DomainConfig {
                persist: true,
                optimization_level: OptimizationLevel::Vec,
                uber_store: UberStoreOptions::Single,
                ..Default::default()
            });
            if let Some(ref mut derive) = options.derive {
                derive.push("Clone".to_string());
                derive.push("Deserialize".to_string());
                derive.push("Serialize".to_string());
            }
            options.use_paths = Some(vec!["serde::{Deserialize, Serialize}".to_string()]);
            options.always_process = Some(true);

            let grace = ModelCompiler::default();

            // Build the domains
            log::debug!(
                "Testing domain: {},  target: {:?}.",
                $domain,
                options.target
            );
            let domain = DomainBuilder::new()
                .cuckoo_model($path)
                .unwrap()
                .build_v2()
                .unwrap();

            grace
                .compile(
                    domain,
                    "mdd",
                    format!("domain/{}", $domain).as_str(),
                    "tests/mdd/src",
                    Box::new(&options),
                    false,
                ).map_err(|e| {
                    println!("Compiler exited with: {}", e);
                    std::io::Error::new(std::io::ErrorKind::Other, "Compiler exited with error")
                })?;

            // Run cargo test
            let mut child = process::Command::new("cargo")
                .arg("test")
                .arg(format!("domain::{}::tests", $domain))
                // .arg("--")
                // .arg("--nocapture")
                .current_dir("tests/mdd")
                .spawn()?;

            match child.wait() {
                Ok(e) => Ok(ExitCode::from(e.code().unwrap() as u8)),
                Err(e) => Err(e),
            }
        }
    };
    ($name:ident, $domain:literal, $path:literal, $($imports:literal),+) => {
        #[test]
        /// This one handles imports
        fn $name() -> Result<ExitCode, std::io::Error> {
            let _ = env_logger::builder().is_test(true).try_init();
            let _ = Client::start();

            let mut options = GraceCompilerOptions::default();
            options.target = Target::Domain(DomainConfig {
                persist: true,
                ..Default::default()
            });
            if let Some(ref mut derive) = options.derive {
                derive.push("Clone".to_string());
                derive.push("Deserialize".to_string());
                derive.push("Serialize".to_string());
            }
            options.use_paths = Some(vec!["serde::{Deserialize, Serialize}".to_string()]);
            let mut imports = Vec::new();
            $(
                imports.push($imports.to_string());
            )*
            options.imported_domains = Some(imports);
            options.always_process = Some(true);

            let grace = ModelCompiler::default();

            // Build the domains
            log::debug!(
                "Testing domain: {},  target: {:?}.",
                $domain,
                options.target
            );
            let domain = DomainBuilder::new()
                .cuckoo_model($path)
                .unwrap()
                .build_v2()
                .unwrap();

            grace
                .compile(
                    domain,
                    "mdd",
                    format!("domain/{}", $domain).as_str(),
                    "tests/mdd/src",
                    Box::new(&options),
                    false,
                )
                .map_err(|e| {
                    println!("Compiler exited with: {}", e);
                    std::io::Error::new(std::io::ErrorKind::Other, "Compiler exited with error")
                })?;

            // Run cargo test
            let mut child = process::Command::new("cargo")
                .arg("test")
                .arg(format!("domain::{}::tests", $domain))
                // .arg("--")
                // .arg("--nocapture")
                .current_dir("tests/mdd")
                .spawn()?;

            match child.wait() {
                Ok(e) => Ok(ExitCode::from(e.code().unwrap() as u8)),
                Err(e) => Err(e),
            }
        }
    };
}

macro_rules! test_target_domain_rwlock_vec_store {
    ($name:ident, $domain:literal, $path:literal) => {
        #[test]
        fn $name() -> Result<ExitCode, std::io::Error> {
            let _ = env_logger::builder().is_test(true).try_init();
            let _ = Client::start();

            let mut options = GraceCompilerOptions::default();
            options.target = Target::Domain(DomainConfig {
                persist: true,
                optimization_level: OptimizationLevel::Vec,
                uber_store: UberStoreOptions::StdRwLock,
                ..Default::default()
            });
            if let Some(ref mut derive) = options.derive {
                derive.push("Clone".to_string());
                derive.push("Deserialize".to_string());
                derive.push("Serialize".to_string());
            }
            options.use_paths = Some(vec!["serde::{Deserialize, Serialize}".to_string()]);
            options.always_process = Some(true);

            let grace = ModelCompiler::default();

            // Build the domains
            log::debug!(
                "Testing domain: {},  target: {:?}.",
                $domain,
                options.target
            );
            let domain = DomainBuilder::new()
                .cuckoo_model($path)
                .unwrap()
                .build_v2()
                .unwrap();

            grace
                .compile(
                    domain,
                    "mdd",
                    format!("domain/{}", $domain).as_str(),
                    "tests/mdd/src",
                    Box::new(&options),
                    false,
                ).map_err(|e| {
                    println!("Compiler exited with: {}", e);
                    std::io::Error::new(std::io::ErrorKind::Other, "Compiler exited with error")
                })?;

            // Run cargo test
            let mut child = process::Command::new("cargo")
                .arg("test")
                .arg(format!("domain::{}::tests", $domain))
                // .arg("--")
                // .arg("--nocapture")
                .current_dir("tests/mdd")
                .spawn()?;

            match child.wait() {
                Ok(e) => Ok(ExitCode::from(e.code().unwrap() as u8)),
                Err(e) => Err(e),
            }
        }
    };
    ($name:ident, $domain:literal, $path:literal, $($imports:literal),+) => {
        #[test]
        /// This one handles imports
        fn $name() -> Result<ExitCode, std::io::Error> {
            let _ = env_logger::builder().is_test(true).try_init();
            let _ = Client::start();

            let mut options = GraceCompilerOptions::default();
            options.target = Target::Domain(DomainConfig {
                persist: true,
                ..Default::default()
            });
            if let Some(ref mut derive) = options.derive {
                derive.push("Clone".to_string());
                derive.push("Deserialize".to_string());
                derive.push("Serialize".to_string());
            }
            options.use_paths = Some(vec!["serde::{Deserialize, Serialize}".to_string()]);
            let mut imports = Vec::new();
            $(
                imports.push($imports.to_string());
            )*
            options.imported_domains = Some(imports);
            options.always_process = Some(true);

            let grace = ModelCompiler::default();

            // Build the domains
            log::debug!(
                "Testing domain: {},  target: {:?}.",
                $domain,
                options.target
            );
            let domain = DomainBuilder::new()
                .cuckoo_model($path)
                .unwrap()
                .build_v2()
                .unwrap();

            grace
                .compile(
                    domain,
                    "mdd",
                    format!("domain/{}", $domain).as_str(),
                    "tests/mdd/src",
                    Box::new(&options),
                    false,
                )
                .map_err(|e| {
                    println!("Compiler exited with: {}", e);
                    std::io::Error::new(std::io::ErrorKind::Other, "Compiler exited with error")
                })?;

            // Run cargo test
            let mut child = process::Command::new("cargo")
                .arg("test")
                .arg(format!("domain::{}::tests", $domain))
                // .arg("--")
                // .arg("--nocapture")
                .current_dir("tests/mdd")
                .spawn()?;

            match child.wait() {
                Ok(e) => Ok(ExitCode::from(e.code().unwrap() as u8)),
                Err(e) => Err(e),
            }
        }
    };
}

macro_rules! test_target_domain_rwlock {
    ($name:ident, $domain:literal, $path:literal) => {
        #[test]
        fn $name() -> Result<ExitCode, std::io::Error> {
            let _ = env_logger::builder().is_test(true).try_init();
            let _ = Client::start();

            let mut options = GraceCompilerOptions::default();
            options.target = Target::Domain(DomainConfig {
                persist: true,
                uber_store: UberStoreOptions::StdRwLock,
                ..Default::default()
            });
            if let Some(ref mut derive) = options.derive {
                derive.push("Clone".to_string());
                derive.push("Deserialize".to_string());
                derive.push("Serialize".to_string());
            }
            options.use_paths = Some(vec!["serde::{Deserialize, Serialize}".to_string()]);
            options.always_process = Some(true);

            let grace = ModelCompiler::default();

            // Build the domains
            log::debug!(
                "Testing domain: {},  target: {:?}.",
                $domain,
                options.target
            );
            let domain = DomainBuilder::new()
                .cuckoo_model($path)
                .unwrap()
                .build_v2()
                .unwrap();

            grace
                .compile(
                    domain,
                    "mdd",
                    format!("domain/{}", $domain).as_str(),
                    "tests/mdd/src",
                    Box::new(&options),
                    false,
                ).map_err(|e| {
                    println!("Compiler exited with: {}", e);
                    std::io::Error::new(std::io::ErrorKind::Other, "Compiler exited with error")
                })?;

            // Run cargo test
            let mut child = process::Command::new("cargo")
                .arg("test")
                .arg(format!("domain::{}::tests", $domain))
                // .arg("--")
                // .arg("--nocapture")
                .current_dir("tests/mdd")
                .spawn()?;

            match child.wait() {
                Ok(e) => Ok(ExitCode::from(e.code().unwrap() as u8)),
                Err(e) => Err(e),
            }
        }
    };
    ($name:ident, $domain:literal, $path:literal, $($imports:literal),+) => {
        #[test]
        /// This one handles imports
        fn $name() -> Result<ExitCode, std::io::Error> {
            let _ = env_logger::builder().is_test(true).try_init();
            let _ = Client::start();

            let mut options = GraceCompilerOptions::default();
            options.target = Target::Domain(DomainConfig {
                persist: true,
                ..Default::default()
            });
            if let Some(ref mut derive) = options.derive {
                derive.push("Clone".to_string());
                derive.push("Deserialize".to_string());
                derive.push("Serialize".to_string());
            }
            options.use_paths = Some(vec!["serde::{Deserialize, Serialize}".to_string()]);
            let mut imports = Vec::new();
            $(
                imports.push($imports.to_string());
            )*
            options.imported_domains = Some(imports);
            options.always_process = Some(true);

            let grace = ModelCompiler::default();

            // Build the domains
            log::debug!(
                "Testing domain: {},  target: {:?}.",
                $domain,
                options.target
            );
            let domain = DomainBuilder::new()
                .cuckoo_model($path)
                .unwrap()
                .build_v2()
                .unwrap();

            grace
                .compile(
                    domain,
                    "mdd",
                    format!("domain/{}", $domain).as_str(),
                    "tests/mdd/src",
                    Box::new(&options),
                    false,
                )
                .map_err(|e| {
                    println!("Compiler exited with: {}", e);
                    std::io::Error::new(std::io::ErrorKind::Other, "Compiler exited with error")
                })?;

            // Run cargo test
            let mut child = process::Command::new("cargo")
                .arg("test")
                .arg(format!("domain::{}::tests", $domain))
                // .arg("--")
                // .arg("--nocapture")
                .current_dir("tests/mdd")
                .spawn()?;

            match child.wait() {
                Ok(e) => Ok(ExitCode::from(e.code().unwrap() as u8)),
                Err(e) => Err(e),
            }
        }
    };
}

macro_rules! test_target_domain_timestamps {
    ($name:ident, $domain:literal, $path:literal) => {
        #[test]
        fn $name() -> Result<ExitCode, std::io::Error> {
            let _ = env_logger::builder().is_test(true).try_init();
            let _ = Client::start();

            let mut options = GraceCompilerOptions::default();
            options.target = Target::Domain(DomainConfig {
                persist: true,
                persist_timestamps: true,
                ..Default::default()
            });
            if let Some(ref mut derive) = options.derive {
                derive.push("Clone".to_string());
                derive.push("Deserialize".to_string());
                derive.push("Serialize".to_string());
            }
            options.use_paths = Some(vec!["serde::{Deserialize, Serialize}".to_string()]);
            options.always_process = Some(true);

            let grace = ModelCompiler::default();

            // Build the domains
            log::debug!(
                "Testing domain: {},  target: {:?}.",
                $domain,
                options.target
            );
            let domain = DomainBuilder::new()
                .cuckoo_model($path)
                .unwrap()
                .build_v2()
                .unwrap();

            grace
                .compile(
                    domain,
                    "mdd",
                    format!("domain/{}", $domain).as_str(),
                    "tests/mdd/src",
                    Box::new(&options),
                    false,
                )
                .map_err(|e| {
                    println!("Compiler exited with: {}", e);
                    std::io::Error::new(std::io::ErrorKind::Other, "Compiler exited with error")
                })?;

            // Run cargo test
            let mut child = process::Command::new("cargo")
                .arg("test")
                .arg(format!("domain::{}::tests", $domain))
                // .arg("--")
                // .arg("--nocapture")
                .current_dir("tests/mdd")
                .spawn()?;

            match child.wait() {
                Ok(e) => Ok(ExitCode::from(e.code().unwrap() as u8)),
                Err(e) => Err(e),
            }
        }
    };
    ($name:ident, $domain:literal, $path:literal, $($imports:literal),+) => {
        #[test]
        /// This one handles imports
        fn $name() -> Result<ExitCode, std::io::Error> {
            let _ = env_logger::builder().is_test(true).try_init();
            let _ = Client::start();

            let mut options = GraceCompilerOptions::default();
            options.target = Target::Domain(DomainConfig {
                persist: true,
                persist_timestamps: true,
                ..Default::default()
            });
            if let Some(ref mut derive) = options.derive {
                derive.push("Clone".to_string());
                derive.push("Deserialize".to_string());
                derive.push("Serialize".to_string());
            }
            options.use_paths = Some(vec!["serde::{Deserialize, Serialize}".to_string()]);
            let mut imports = Vec::new();
            $(
                imports.push($imports.to_string());
            )*
            options.imported_domains = Some(imports);
            options.always_process = Some(true);

            let grace = ModelCompiler::default();

            // Build the domains
            log::debug!(
                "Testing domain: {},  target: {:?}.",
                $domain,
                options.target
            );
            let domain = DomainBuilder::new()
                .cuckoo_model($path)
                .unwrap()
                .build_v2()
                .unwrap();

            grace
                .compile(
                    domain,
                    "mdd",
                    format!("domain/{}", $domain).as_str(),
                    "tests/mdd/src",
                    Box::new(&options),
                    false,
                )
                .map_err(|e| {
                    println!("Compiler exited with: {}", e);
                    std::io::Error::new(std::io::ErrorKind::Other, "Compiler exited with error")
                })?;

            // Run cargo test
            let mut child = process::Command::new("cargo")
                .arg("test")
                .arg(format!("domain::{}::tests", $domain))
                // .arg("--")
                // .arg("--nocapture")
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
    ($name:ident, $domain:literal, $path:literal) => {
        #[test]
        fn $name() -> Result<ExitCode, std::io::Error> {
            let _ = env_logger::builder().is_test(true).try_init();
            let _ = Client::start();

            let mut options = GraceCompilerOptions::default();
            options.always_process = Some(true);
            let grace = ModelCompiler::default();

            // Build the domains
            log::debug!(
                "Testing domain: {},  target: {:?}.",
                $domain,
                options.target
            );
            let domain = DomainBuilder::new()
                .cuckoo_model($path)
                .unwrap()
                .build_v2()
                .unwrap();

            grace
                .compile(
                    domain,
                    "mdd",
                    format!("app/{}", $domain).as_str(),
                    "tests/mdd/src",
                    Box::new(&options),
                    false,
                )
                .map_err(|e| {
                    println!("Compiler exited with: {}", e);
                    std::io::Error::new(std::io::ErrorKind::Other, "Compiler exited with error")
                })?;

            // Run cargo test
            let mut child = process::Command::new("cargo")
                .arg("test")
                .arg(format!("app::{}::tests", $domain))
                // .arg("--")
                // .arg("--nocapture")
                .current_dir("tests/mdd")
                .spawn()?;

            match child.wait() {
                Ok(e) => Ok(ExitCode::from(e.code().unwrap() as u8)),
                Err(e) => Err(e),
            }
        }
    };
}

macro_rules! test_target_dwarf {
    ($name:ident, $domain:literal, $path:literal) => {
        #[test]
        fn $name() -> Result<(), std::io::Error> {
            let _ = env_logger::builder().is_test(true).try_init();
            let _ = Client::start();

            let mut options = GraceCompilerOptions::default();
            options.target = Target::Dwarf(DwarfConfig {
                store_path: $path.into(),
            });

            options.always_process = Some(true);
            let grace = ModelCompiler::default();

            // Build the domains
            log::debug!(
                "Testing domain: {},  target: {:?}.",
                $domain,
                options.target
            );
            let domain = DomainBuilder::new()
                .cuckoo_model($path)
                .unwrap()
                .build_v2()
                .unwrap();

            grace
                .compile(
                    domain,
                    "mdd",
                    format!("dwarf/{}", $domain).as_str(),
                    "tests/mdd/src",
                    Box::new(&options),
                    false,
                )
                .map_err(|e| {
                    println!("Compiler exited with: {}", e);
                    std::io::Error::new(std::io::ErrorKind::Other, "Compiler exited with error")
                })?;

            // Run dwarfc?
            Ok(())
        }
    };
}

// macro_rules! test_domain {
//     ($domain:tt) => {
//         test_target_domain!(
//             $domain,
//             stringify!($domain),
//             "tests/mdd/models/$domain.json",
//         );
//         test_target_domain_rwlock!(
//             format!("{}_rwlock", $domain),
//             format!("\"{}_rwlock\"", $domain),
//             format!("tests/mdd/models/{}.json", $domain).as_str()
//         );
//         test_target_domain_rwlock_ts!(
//             format!("{}_rwlock_ts", $domain),
//             format!("\"{}_rwlock_ts\"", $domain),
//             format!("tests/mdd/models/{}.json", $domain).as_str()
//         );
//         test_target_domain_vec_store!(
//             format!("{}_vec_store", $domain),
//             format!("\"{}_vec_store\"", $domain),
//             format!("tests/mdd/models/{}.json", $domain).as_str()
//         );
//         test_target_domain_timestamps!(
//             format!("{}_ts", $domain),
//             format!("\"{}_ts\"", $domain),
//             format!("tests/mdd/models/{}.json", $domain).as_str()
//         );
//         test_target_domain_dwarf!(
//             format!("{}_dwarf", $domain),
//             format!("\"{}_dwarf\"", $domain),
//             format!("tests/mdd/models/{}.json", $domain).as_str()
//         );
//     };
// }

// This is an imported domain that we need to build, so get it done early.
test_target_domain!(sarzak, "sarzak", "../sarzak/models/sarzak.json");
test_target_domain_rwlock!(
    sarzak_rwlock,
    "sarzak_rwlock",
    "../sarzak/models/sarzak.json"
);
test_target_domain_vec_store!(sarzak_vec, "sarzak_vec", "../sarzak/models/sarzak.json");
test_target_domain_timestamps!(sarzak_ts, "sarzak_ts", "../sarzak/models/sarzak.json");
test_target_dwarf!(sarzak_dwarf, "sarzak_dwarf", "../sarzak/models/sarzak.json");

// Domain Target Tests
test_target_domain!(
    everything_domain,
    "everything",
    "tests/mdd/models/everything.json"
);
test_target_domain_rwlock!(
    everything_domain_rwlock,
    "everything_rwlock",
    "tests/mdd/models/everything.json"
);
test_target_domain_vec_store!(
    everything_domain_vec,
    "everything_vec",
    "tests/mdd/models/everything.json"
);
test_target_domain_rwlock_vec_store!(
    everything_domain_rwlock_vec,
    "everything_rwlock_vec",
    "tests/mdd/models/everything.json"
);
test_target_domain_timestamps!(
    everything_domain_ts,
    "everything_ts",
    "tests/mdd/models/everything.json"
);
test_target_dwarf!(
    everything_domain_dwarf,
    "everything_dwarf",
    "tests/mdd/models/everything.json"
);

test_target_domain!(
    one_to_one_domain,
    "one_to_one",
    "tests/mdd/models/one_to_one.json"
);
test_target_domain_rwlock!(
    one_to_one_domain_rwlock,
    "one_to_one_rwlock",
    "tests/mdd/models/one_to_one.json"
);
test_target_domain_vec_store!(
    one_to_one_domain_vec,
    "one_to_one_vec",
    "tests/mdd/models/one_to_one.json"
);
test_target_domain_timestamps!(
    one_to_one_domain_ts,
    "one_to_one_ts",
    "tests/mdd/models/one_to_one.json"
);
test_target_dwarf!(
    one_to_one_domain_dwarf,
    "one_to_one_dwarf",
    "tests/mdd/models/one_to_one.json"
);

test_target_domain!(
    one_to_many_domain,
    "one_to_many",
    "tests/mdd/models/one_to_many.json"
);
test_target_domain_rwlock!(
    one_to_many_domain_rwlock,
    "one_to_many_rwlock",
    "tests/mdd/models/one_to_many.json"
);
test_target_domain_vec_store!(
    one_to_many_domain_vec,
    "one_to_many_vec",
    "tests/mdd/models/one_to_many.json"
);
test_target_domain_timestamps!(
    one_to_many_domain_ts,
    "one_to_many_ts",
    "tests/mdd/models/one_to_many.json"
);
test_target_dwarf!(
    one_to_many_domain_dwarf,
    "one_to_many_dwarf",
    "tests/mdd/models/one_to_many.json"
);

test_target_domain!(isa_domain, "isa", "tests/mdd/models/isa.json");
test_target_domain_rwlock!(isa_domain_rwlock, "isa_rwlock", "tests/mdd/models/isa.json");
test_target_domain_vec_store!(isa_domain_vec, "isa_vec", "tests/mdd/models/isa.json");
test_target_domain_timestamps!(isa_domain_ts, "isa_ts", "tests/mdd/models/isa.json");
test_target_dwarf!(isa_domain_dwarf, "isa", "tests/mdd/models/isa.json");

test_target_domain!(
    associative_domain,
    "associative",
    "tests/mdd/models/associative.json"
);
test_target_domain_rwlock!(
    associative_domain_rwlock,
    "associative_rwlock",
    "tests/mdd/models/associative.json"
);
test_target_domain_vec_store!(
    associative_domain_vec,
    "associative_vec",
    "tests/mdd/models/associative.json"
);
test_target_domain_timestamps!(
    associative_domain_ts,
    "associative_ts",
    "tests/mdd/models/associative.json"
);
test_target_dwarf!(
    associative_domain_dwarf,
    "associative_dwarf",
    "tests/mdd/models/associative.json"
);

// This one has imports
test_target_domain!(
    imported_object_domain,
    "imported_object",
    "tests/mdd/models/imported_object.json",
    "domain/sarzak",
    "domain/isa"
);
test_target_domain_rwlock!(
    imported_object_domain_rwlock,
    "imported_object_rwlock",
    "tests/mdd/models/imported_object.json",
    "domain/sarzak",
    "domain/isa"
);
test_target_domain_vec_store!(
    imported_object_domain_vec,
    "imported_object_vec",
    "tests/mdd/models/imported_object.json",
    "domain/sarzak",
    "domain/isa"
);
test_target_domain_timestamps!(
    imported_object_domain_ts,
    "imported_object_ts",
    "tests/mdd/models/imported_object.json",
    "domain/sarzak",
    "domain/isa"
);
// test_target_dwarf!(
//     imported_object_domain_dwarf,
//     "imported_object",
//     "tests/mdd/models/imported_object.json"
// );

test_target_domain!(external, "external", "tests/mdd/models/external.json");
test_target_domain_rwlock!(
    external_rwlock,
    "external_rwlock",
    "tests/mdd/models/external.json"
);
test_target_domain_vec_store!(
    external_vec,
    "external_vec",
    "tests/mdd/models/external.json"
);
test_target_domain_timestamps!(external_ts, "external_ts", "tests/mdd/models/external.json");
test_target_dwarf!(
    external_dwarf,
    "external_dwarf",
    "tests/mdd/models/external.json"
);

// I just don't care about application any more. Not now anyway.
// // Application Target Tests
// test_target_application!(
//     everything_application,
//     "everything",
//     "tests/mdd/models/everything.json"
// );

#[test]
fn test_from_extrude() -> Result<ExitCode, std::io::Error> {
    let _ = env_logger::builder().is_test(true).try_init();
    let _ = Client::start();

    let mut options = GraceCompilerOptions::default();
    options.target = Target::Domain(DomainConfig {
        from_module: Some("domain::isa".to_string()),
        from_path: Some("tests/mdd/models/isa.json".into()),
        persist: true,
        // Get some cheap code coverage this way, I think.
        persist_timestamps: true,
        ..Default::default()
    });
    if let Some(ref mut derive) = options.derive {
        derive.push("Clone".to_string());
        derive.push("Deserialize".to_string());
        derive.push("Serialize".to_string());
    }
    options.use_paths = Some(vec!["serde::{Deserialize, Serialize}".to_string()]);
    options.always_process = Some(true);

    let grace = ModelCompiler::default();

    // Build the domains
    let domain = DomainBuilder::new()
        .cuckoo_model("tests/mdd/models/isa.json")
        .unwrap()
        .build_v2()
        .unwrap();

    log::debug!(
        "Testing domain: {} from extrusion,  target: {:?}.",
        domain.name(),
        options.target
    );

    grace
        .compile(
            domain,
            "mdd",
            "domain/isa_clone",
            "tests/mdd/src",
            Box::new(&options),
            false,
        )
        .map_err(|e| {
            println!("Compiler exited with: {}", e);
            std::io::Error::new(std::io::ErrorKind::Other, "Compiler exited with error")
        })?;

    Ok(ExitCode::SUCCESS)
}

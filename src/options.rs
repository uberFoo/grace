//! Configuration Options for the grace model compiler
//!
//! This is very much a 1.0 sort of thing. Maybe pre-1.0, since I don't have any
//! idea what 1.0 would even look like. My point is that I plan on generating
//! the config structs. But to do that, I need a model compiler, and there's no
//! way that I'm hacking more on nut.
//!
//! So maybe, the first feature to come out of this is an implementation to
//! generate options files! Boy, that sure would be pretty limited wouldn't it?
//! Maybe not. Looking at what I need to implement below, It's actually just
//! a clap Args derive, with inserted attributes. This may actually be a good
//! test case.
//!
//! I need te generate a struct definition anyway. All generating this would
//! be is modifying a struct definition. And that's just the sort of problem that
//! I should solve early.
use std::{any::Any, collections::HashMap, path::PathBuf};

use clap::{Args, Subcommand};
use sarzak::{domain::Domain, mc::ModelCompilerOptions};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const DEFAULT_TARGET: Target = Target::Application;
const DEFAULT_DERIVE: &'static [&'static str] = &["Debug", "PartialEq"];
const DEFAULT_USE_PATHS: Option<Vec<String>> = None;
const DEFAULT_IMPORTED_OBJECTS: bool = false;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Subcommand)]
pub enum Target {
    /// Target Domain Infrastructure
    ///
    /// This target is used by model compilers to generate code.
    Domain,
    /// Target Application Code
    ///
    /// This target is intended to be run as an application.
    Application,
}

#[derive(Args, Clone, Debug, Deserialize, Serialize)]
pub struct GraceCompilerOptions {
    /// Code Generation Target
    ///
    /// This determines how objects are rendered into structs and enums.
    #[command(subcommand)]
    pub target: Target,
    /// Derive Macros
    ///
    /// A comma separated list of derive macros to be added to each generated
    /// item, globally. Don't put a space after the comma. This is a bug, IMO.
    ///
    /// Note that this option is available on a per-object basis using the
    /// description coloring option:
    ///
    /// `🐶 {"derive": ["macro", ...]}`.
    #[arg(long, short, use_value_delimiter = true, value_delimiter = ',')]
    pub derive: Option<Vec<String>>,
    /// Use Paths
    ///
    /// These are a comma separated list of paths to be issued as `use` statements.
    /// Again, no spaces after the comma.
    ///
    /// If specified as an option, the paths will be added to the top of the
    /// file generated for each type. This may also be applied to specific
    /// objects by using the description coloring option:
    ///
    /// `🐶 {"use_paths": ["path", ...]}`.
    #[arg(long, short, use_value_delimiter = true, value_delimiter = ',')]
    pub use_paths: Option<Vec<String>>,
    /// Imported Objects
    ///
    /// This is a useless, do nothing option that's sole purpose is to taunt you.
    /// If I can figure out how to make it work, I'll do so.
    ///
    /// Until then, if you want to indicate that an object is imported, you will
    /// need to do so in the object's description like so:
    ///
    /// `🐶 {"imported_object": {"domain": "Super-Awesome Domain", "package": "my-app", "model_file": "../sarzak/models/sarzak.json"}}`
    ///
    /// "package" is also known as crate. It's the root of the rust project.
    /// "model_file" a path to the domain file, relative to the package root.
    #[arg(long, short)]
    pub imported_objects: bool,
}

impl ModelCompilerOptions for GraceCompilerOptions {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Default for GraceCompilerOptions {
    fn default() -> Self {
        Self {
            target: DEFAULT_TARGET,
            derive: Some(DEFAULT_DERIVE.iter().map(|&x| x.to_owned()).collect()),
            use_paths: DEFAULT_USE_PATHS,
            imported_objects: DEFAULT_IMPORTED_OBJECTS,
        }
    }
}

/// Grace Compiler Configuration
///
/// This is the main configuration for the compiler. It is assembled from the
/// [`GraceCompilerOptions`] and the [`ConfigValue`]s that are generated from
/// the object descriptions.
///
/// This is passed to code generation implementations within the compiler.
///
/// The config is just a map from object id to [`ConfigValue`], therefore each
/// object in the domain has a configuration.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct GraceConfig {
    inner: HashMap<Uuid, ConfigValue>,
}

impl GraceConfig {
    pub(crate) fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub(crate) fn insert(&mut self, key: Uuid, config_value: ConfigValue) {
        self.inner.insert(key, config_value);
    }

    fn get(&self, key: Uuid) -> Option<&ConfigValue> {
        self.inner.get(&key)
    }

    pub(crate) fn get_use_paths(&self, key: &Uuid) -> Option<&Vec<String>> {
        if let Some(config_value) = self.get(*key) {
            if let Some(ref use_paths) = config_value.use_paths {
                Some(&use_paths)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub(crate) fn get_derives(&self, key: &Uuid) -> Option<&Vec<String>> {
        if let Some(config_value) = self.get(*key) {
            if let Some(ref derive) = config_value.derive {
                Some(&derive)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub(crate) fn get_imported(&self, key: &Uuid) -> Option<&ImportedObject> {
        if let Some(config_value) = self.get(*key) {
            if let Some(ref imported_object) = config_value.imported_object {
                Some(&imported_object)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub(crate) fn get_imported_objects(&self) -> Vec<(&Uuid, &ImportedObject)> {
        self.inner
            .iter()
            .filter_map(|(key, config_value)| {
                if let Some(ref imported_object) = config_value.imported_object {
                    Some((key, imported_object))
                } else {
                    None
                }
            })
            .collect()
    }

    pub(crate) fn is_imported(&self, key: &Uuid) -> bool {
        self.get_imported(key).is_some()
    }
}
impl From<(&GraceCompilerOptions, &Domain)> for GraceConfig {
    fn from((options, domain): (&GraceCompilerOptions, &Domain)) -> Self {
        let mut config = Self::new();

        for (key, object) in domain.sarzak().iter_object() {
            // We want to load the initial value from the object description, and then
            // layer the defaults on top, without overwriting what came from the
            // description.
            //
            // Hmmm... I'm not sure if we want to layer, or just take what's in the
            // description. 🤔
            //
            // Something to worry about later. For now, I'm not layering.
            let mut config_value = parse_config_value(object.description.as_str());

            if let Some(ref derive) = options.derive {
                if config_value.derive.is_none() {
                    config_value.derive = Some(derive.clone());
                }
            }

            if let Some(ref use_paths) = options.use_paths {
                if config_value.use_paths.is_none() {
                    config_value.use_paths = Some(use_paths.clone());
                }
            }

            config.insert(*key, config_value);
        }

        config
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct ConfigValue {
    pub(crate) imported_object: Option<ImportedObject>,
    pub(crate) derive: Option<Vec<String>>,
    pub(crate) use_paths: Option<Vec<String>>,
}

impl ConfigValue {
    pub(crate) fn new() -> Self {
        Self {
            imported_object: None,
            derive: None,
            use_paths: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct ImportedObject {
    pub domain: String,
    pub package: String,
    pub model_file: PathBuf,
}

fn parse_config_value(input: &str) -> ConfigValue {
    if input.contains("🐶") {
        let mut iter = input.split("🐶");
        iter.next();
        if let Some(input) = iter.next() {
            if let Ok(config_value) = serde_json::from_str(input) {
                config_value
            } else {
                panic!("error parsing config value {}", input);
            }
        } else {
            ConfigValue::new()
        }
    } else {
        ConfigValue::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_imported_object() {
        let input = "Testing, 1, 2, 3...\nIt can handle junk at the beginning of the line, but not the end. 🐶 {\"imported_object\": {\"domain\": \"Super-Awesome Domain\", \"package\": \"my-app\", \"model_file\": \"../sarzak/models/sarzak.json\"}}";
        let expected = ImportedObject {
            domain: "Super-Awesome Domain".to_owned(),
            package: "my-app".to_owned(),
            model_file: PathBuf::from("../sarzak/models/sarzak.json"),
        };

        let actual: ConfigValue = parse_config_value(input);
        assert_eq!(actual.imported_object, Some(expected));
    }

    #[test]
    fn test_parse_derive() {
        let input = "🐶 {\"derive\": [\"Debug\", \"PartialEq\"]}";
        let expected = vec!["Debug".to_owned(), "PartialEq".to_owned()];

        let actual: ConfigValue = parse_config_value(input);
        assert_eq!(actual.derive, Some(expected));
    }

    #[test]
    fn test_parse_use_paths() {
        let input = "🐶 {\"use_paths\": [\"crate::models::super_awesome_domain::SuperAwesomeDomain\", \"serde::Deserialize\"]}";
        let expected = vec![
            "crate::models::super_awesome_domain::SuperAwesomeDomain".to_owned(),
            "serde::Deserialize".to_owned(),
        ];

        let actual: ConfigValue = parse_config_value(input);
        assert_eq!(actual.use_paths, Some(expected));
    }

    #[test]
    fn test_config_from_options() {
        use sarzak::domain::DomainBuilder;

        let mut options = GraceCompilerOptions::default();
        options.derive = Some(vec![
            "Clone".to_owned(),
            "Deserialize".to_owned(),
            "Serialize".to_owned(),
        ]);
        options.use_paths = Some(vec!["serde::{Deserialize, Serialize}".to_string()]);

        let domain = DomainBuilder::new()
            .cuckoo_model("tests/mdd/models/one_to_one.json")
            .unwrap()
            .build()
            .unwrap();

        let config: GraceConfig = (&options, &domain).into();

        for (id, _) in domain.sarzak().iter_object() {
            let config_value = config.get(*id).unwrap();
            assert_eq!(
                config_value.derive,
                Some(vec![
                    "Clone".to_string(),
                    "Deserialize".to_string(),
                    "Serialize".to_string()
                ])
            );
            assert_eq!(
                config_value.use_paths,
                Some(vec!["serde::{Deserialize, Serialize}".to_string()])
            );
        }
    }

    #[test]
    fn test_config_from_description() {
        use sarzak::domain::DomainBuilder;

        let options = GraceCompilerOptions::default();

        let domain = DomainBuilder::new()
            .cuckoo_model("tests/mdd/models/imported_object.json")
            .unwrap()
            .build()
            .unwrap();

        let config: GraceConfig = (&options, &domain).into();

        for (id, obj) in domain.sarzak().iter_object() {
            let config_value = config.get(*id).unwrap();

            if obj.name == "Object" {
                assert_eq!(
                    config_value.imported_object,
                    Some(ImportedObject {
                        domain: "sarzak".to_string(),
                        package: "sarzak".to_string(),
                        model_file: PathBuf::from("../../../sarzak/models/sarzak_✨.json"),
                    })
                );
            } else {
                assert_eq!(config_value.imported_object, None);
            }

            // Below are the defaults for compiler options.
            assert_eq!(
                config_value.derive,
                Some(vec!["Debug".to_string(), "PartialEq".to_string(),])
            );
            assert_eq!(config_value.use_paths, None);
        }
    }
}

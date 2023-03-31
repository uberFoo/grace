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
use std::{any::Any, path::PathBuf};

use clap::{ArgAction, Args, Subcommand};
use fnv::FnvHashMap as HashMap;
use sarzak::{mc::ModelCompilerOptions, v2::domain::Domain};
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

const _TARGET_: Uuid = uuid!("a42b46ea-820a-5132-b1d2-b1a6363e4cc1");

/// Compiler Target
///
/// Currently grace supports two targets: Domain and Application.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Subcommand)]
#[serde(tag = "target")]
#[serde(rename_all = "lowercase")]
pub enum Target {
    /// Target Domain Infrastructure
    ///
    /// This target is used by model compilers to generate code.
    Domain(DomainConfig),
    /// Target Application Code
    ///
    /// This target is intended to be run as an application.
    Application,
}

/// Domain Target Configuration
///
/// The domain target has the following, target-specific, configuration options.
#[derive(Args, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DomainConfig {
    /// Generate `From` trait implementations
    ///
    /// From implementations are generated for each object in the domain,
    /// if there is a corresponding object in the source domain.
    ///
    /// The source domain to be use for extrusion, using the `from` trait,
    /// is specified using the `--from-path` option, which is required if
    /// using this option.
    ///
    /// This is the path to a module, e.g., `generated::sarzak`.
    #[arg(long, requires = "from_path")]
    pub from_module: Option<String>,
    /// Path to the source domain's model file
    ///
    /// When generating From implementations, the model file is loaded and
    /// inspected for sources for the `From` trait.
    ///
    /// This is a file system path, relative to the current package.
    ///
    /// This option requires the `--from-module` option.
    #[arg(long, requires = "from_module")]
    pub from_path: Option<PathBuf>,
    /// Persist ObjectStore
    ///
    /// Wheen this option is specified, code will be generated that will persist
    /// the ObjectStore to disk.
    ///
    /// This is used to persist model files. It may be useful for persisting
    /// user domains.
    #[arg(long, short, action=ArgAction::SetTrue)]
    pub persist: bool,
    /// Persist with timestamps
    ///
    /// Store time stamps along side the type in the ObjectStore. The time stamp
    /// is the time that the object was interred into the store. This is useful
    /// to the compiler so that it can only generate code for types that have
    /// changed.
    #[arg(long, short = 't', action=ArgAction::SetTrue, requires = "persist")]
    pub persist_timestamps: bool,
    /// This Domain is Sarzak
    ///
    /// There can be only one! 💥😱🤣
    #[arg(long, action=ArgAction::SetFalse)]
    pub is_sarzak: bool,
    /// This Domain is a Meta Model
    ///
    /// With capital letters.
    ///
    /// Don't use this unless you mean it.
    #[arg(long, action=ArgAction::SetFalse)]
    pub is_meta_model: bool,
}

const DOMAIN_FROM_MODULE: Option<String> = None;
const DOMAIN_FROM_PATH: Option<PathBuf> = None;
const DOMAIN_PERSIST: bool = false;
const DOMAIN_PERSIST_TIMESTAMPS: bool = false;
const DOMAIN_IS_SARZAK: bool = false;
const DOMAIN_IS_META_MODEL: bool = false;

/// Default implementation for DomainConfig
///
/// We select defaults that are appropriate for applications that aren't using
/// the domain as the backend for a model compiler. Put another way, the defaults
/// are most appropriate for domains that aren't meta-models.
impl Default for DomainConfig {
    fn default() -> Self {
        DomainConfig {
            from_module: DOMAIN_FROM_MODULE,
            from_path: DOMAIN_FROM_PATH,
            persist: DOMAIN_PERSIST,
            persist_timestamps: DOMAIN_PERSIST_TIMESTAMPS,
            is_sarzak: DOMAIN_IS_SARZAK,
            is_meta_model: DOMAIN_IS_META_MODEL,
        }
    }
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
    /// Imported Domains
    ///
    /// A domain may be imported, and it's objects referenced by the importing
    /// domain.  The values in this list need to correspond to the output location
    /// of the imported domain, i.e., the path to the generated module.
    ///
    /// To indicate that an object is imported, you will need to do so in the
    /// object's description like so:
    ///
    /// `🐶 {"imported_object": {"domain": "Super-Awesome Domain", "model_file": "../sarzak/models/sarzak.json"}}`
    ///
    /// "domain" is perhaps a misnomer. It is in fact the location of the generated
    /// domain's module in this package.
    /// "model_file" a path to the domain file, relative to the package root.
    ///
    /// Note that the domain parameters _must_ match.
    #[arg(long, short, use_value_delimiter = true, value_delimiter = ',')]
    pub imported_domains: Option<Vec<String>>,
    /// Generate Document Tests
    ///
    /// Document tests are generated for all generated functions.
    #[arg(long, short = 't')]
    pub doc_test: Option<bool>,
    /// Disable Optimized Generation
    ///
    /// By default the compiler will only generate code for changed model
    /// artifacts. Enabling this option will disable this behavior, and all
    /// model elements will be processed.
    #[arg(long, short)]
    pub always_process: Option<bool>,
}

impl ModelCompilerOptions for GraceCompilerOptions {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

const DEFAULT_TARGET: Target = Target::Application;
const DEFAULT_DERIVE: &'static [&'static str] = &["Debug", "PartialEq"];
const DEFAULT_USE_PATHS: Option<Vec<String>> = None;
const DEFAULT_IMPORTED_DOMAINS: Option<Vec<String>> = None;
const DEFAULT_DOC_TEST: bool = true;
const DEFAULT_ALWAYS_PROCESS: bool = false;

impl Default for GraceCompilerOptions {
    fn default() -> Self {
        Self {
            target: DEFAULT_TARGET,
            derive: Some(DEFAULT_DERIVE.iter().map(|&x| x.to_owned()).collect()),
            use_paths: DEFAULT_USE_PATHS,
            imported_domains: DEFAULT_IMPORTED_DOMAINS,
            doc_test: Some(DEFAULT_DOC_TEST),
            always_process: Some(DEFAULT_ALWAYS_PROCESS),
        }
    }
}

/// Grace Compiler Configuration (internal)
///
/// This is the main configuration for the compiler. It is assembled from the
/// [`GraceCompilerOptions`] and the [`ConfigValue`]s that are generated from
/// the object descriptions.
///
/// This is passed to code generation implementations within the compiler.
///
/// The config is just a map from object id to [`ConfigValue`], therefore each
/// object in the domain has a configuration.
///
/// There is a special key in the map, that corresponds to the target: _TARGET_.
/// This entry contains any target specific options.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct GraceConfig {
    inner: HashMap<Uuid, ConfigValue>,
}

impl GraceConfig {
    pub(crate) fn new() -> Self {
        Self {
            inner: HashMap::default(),
        }
    }

    pub(crate) fn insert(&mut self, key: Uuid, config_value: ConfigValue) {
        self.inner.insert(key, config_value);
    }

    fn get(&self, key: Uuid) -> Option<&ConfigValue> {
        self.inner.get(&key)
    }

    pub(crate) fn get_target(&self) -> &Target {
        if let Some(config_value) = self.get(_TARGET_) {
            if let Some(ref target) = config_value.target {
                &target
            } else {
                &DEFAULT_TARGET
            }
        } else {
            &DEFAULT_TARGET
        }
    }

    /// Get the `from_domain` value for the target.
    ///
    /// This is sort of a special purpose function, because the target is assumed
    /// to be Target::Domain.
    pub(crate) fn get_from_domain(&self) -> Option<FromDomain> {
        match self.get_target() {
            Target::Domain(config) => {
                if let Some(module) = config.from_module.clone() {
                    if let Some(path) = config.from_path.clone() {
                        Some(FromDomain { module, path })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Get the `persist` value for the target.
    ///
    /// As above, this is sort of a special purpose function.
    pub(crate) fn get_persist(&self) -> Option<bool> {
        match self.get_target() {
            Target::Domain(config) => Some(config.persist),
            _ => None,
        }
    }

    /// Get the `persist_timestamps` value for the target.
    ///
    pub(crate) fn get_persist_timestamps(&self) -> Option<bool> {
        match self.get_target() {
            Target::Domain(config) => Some(config.persist_timestamps),
            _ => None,
        }
    }

    pub(crate) fn is_sarzak(&self) -> bool {
        match self.get_target() {
            Target::Domain(config) => config.is_sarzak,
            _ => false,
        }
    }

    pub(crate) fn is_meta_model(&self) -> bool {
        match self.get_target() {
            Target::Domain(config) => config.is_meta_model,
            _ => false,
        }
    }

    pub(crate) fn get_doc_test(&self) -> bool {
        if let Some(config_value) = self.get(_TARGET_) {
            if let Some(doc_test) = config_value.doc_test {
                doc_test
            } else {
                DEFAULT_DOC_TEST
            }
        } else {
            DEFAULT_DOC_TEST
        }
    }

    pub(crate) fn get_always_process(&self) -> bool {
        if let Some(config_value) = self.get(_TARGET_) {
            if let Some(always_process) = config_value.always_process {
                always_process
            } else {
                DEFAULT_ALWAYS_PROCESS
            }
        } else {
            DEFAULT_ALWAYS_PROCESS
        }
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

    pub(crate) fn is_imported(&self, key: &Uuid) -> bool {
        self.get_imported(key).is_some()
    }

    pub(crate) fn get_external(&self, key: &Uuid) -> Option<&ExternalEntity> {
        if let Some(config_value) = self.get(*key) {
            if let Some(ref external_entity) = config_value.external_entity {
                Some(&external_entity)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub(crate) fn is_external(&self, key: &Uuid) -> bool {
        self.get_external(key).is_some()
    }
}

/// Create a GraceConfig from GraceCompilerOptions and a Domain
///
/// How slick is this?
impl From<(&GraceCompilerOptions, &Domain)> for GraceConfig {
    fn from((options, domain): (&GraceCompilerOptions, &Domain)) -> Self {
        let mut config = Self::new();

        // 🚧 I'm not sure that _TARGET_ is the best name for this now that we
        // are also storing non-target options here.
        config.insert(_TARGET_, ConfigValue::from(options));

        for object in domain.sarzak().iter_object() {
            // We want to load the initial value from the object description, and then
            // layer the defaults on top, without overwriting what came from the
            // description.
            //
            // Hmmm... I'm not sure if we want to layer, or just take what's in the
            // description. 🤔
            //
            // Something to worry about later. For now, I'm not layering.
            let mut config_value = parse_config_value(object.description.as_str());

            // Except that this appears to be taking the default if it's not
            // in the description. 🤔
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

            config.insert(object.id, config_value);
        }

        config
    }
}

/// A configuration value for a single object.
///
/// These are built in the From GraceConfig implementation for GraceCompilerOptions.
///
/// Note that there is the special key _TARGET_, which is used to pass target
/// specific options. We also use this key to store compiler-wide options.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct ConfigValue {
    pub(crate) target: Option<Target>,
    pub(crate) imported_object: Option<ImportedObject>,
    pub(crate) external_entity: Option<ExternalEntity>,
    pub(crate) derive: Option<Vec<String>>,
    pub(crate) use_paths: Option<Vec<String>>,
    pub(crate) doc_test: Option<bool>,
    pub(crate) always_process: Option<bool>,
}

impl ConfigValue {
    pub(crate) fn new() -> Self {
        Self {
            target: None,
            imported_object: None,
            external_entity: None,
            derive: None,
            use_paths: None,
            doc_test: None,
            always_process: None,
        }
    }
}

/// Turn an Option to a Config
///
/// Here is where we merge the compiler options into the compiler configuration.
/// Note that we are storing the target, as well as the global options.
impl From<&GraceCompilerOptions> for ConfigValue {
    fn from(options: &GraceCompilerOptions) -> Self {
        Self {
            target: Some(options.target.clone()),
            imported_object: None,
            external_entity: None,
            derive: options.derive.clone(),
            use_paths: options.use_paths.clone(),
            doc_test: options.doc_test.clone(),
            always_process: options.always_process.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct FromDomain {
    pub module: String,
    pub path: PathBuf,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct ImportedObject {
    /// The source domain
    ///
    /// This is actually a path. The path separator is a rust `::`. I don't think
    /// that it's actually used for anything at all, besides pulling off the last
    /// bit and using it to get the name of the ObjectStore to use. Maybe it's
    /// cruft. No, it's used someplace, because we output a path to the domain,
    /// relative to the crate.
    ///
    ///  However...
    ///
    /// You can also use a `/` as a path separator in case you are doing
    /// things with the module system, like the `sarzak.toml` in sarzak does.
    pub domain: String,
    /// A path to the model file
    ///
    /// This is a file system path to the json "file" that contains the model.
    pub model_file: PathBuf,
    /// The UUID of the object in the domain.
    ///
    /// This isn't used, but it is checked. Pretty stupid.
    pub id: Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub(crate) struct ExternalEntity {
    pub name: String,
    pub ctor: String,
    pub path: String,
}

pub(crate) fn parse_config_value(input: &str) -> ConfigValue {
    if input.contains("🐶") {
        let mut iter = input.split("🐶");
        iter.next();
        if let Some(input) = iter.next() {
            let value = serde_json::from_str(input)
                .map_err(|e| panic!("error {}\nparsing config value {}", e, input))
                .unwrap();
            log::debug!("parsed config value: {:?}", value);
            value
        } else {
            panic!(
                "error parsing config value {}, no value after marker: 🐶",
                input
            );
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
        let input = "Testing, 1, 2, 3...\nIt can handle junk at the beginning of the line, but not the end. 🐶 {\"imported_object\": {\"domain\": \"Super-Awesome Domain\", \"model_file\": \"../sarzak/models/sarzak.json\", \"id\": \"00000000-0000-0000-0000-000000000000\"}}";
        let expected = ImportedObject {
            domain: "Super-Awesome Domain".to_owned(),
            model_file: PathBuf::from("../sarzak/models/sarzak.json"),
            id: Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap(),
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
    fn test_external_entity() {
        let input = "🐶 {\"external_entity\": {\"ctor\": \"now\", \"name\": \"SystemTime\", \"path\": \"std::time\"}}";
        let expected = ExternalEntity {
            ctor: "now".to_owned(),
            name: "SystemTime".to_owned(),
            path: "std::time".to_owned(),
        };

        let actual: ConfigValue = parse_config_value(input);
        assert_eq!(actual.external_entity, Some(expected));
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
            .build_v2()
            .unwrap();

        let config: GraceConfig = (&options, &domain).into();

        for obj in domain.sarzak().iter_object() {
            let config_value = config.get(obj.id).unwrap();
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
            .build_v2()
            .unwrap();

        let config: GraceConfig = (&options, &domain).into();

        for obj in domain.sarzak().iter_object() {
            let config_value = config.get(obj.id).unwrap();

            if obj.name == "Object" {
                assert!(config.is_imported(&obj.id));
                assert_eq!(
                    config_value.imported_object,
                    Some(ImportedObject {
                        domain: "domain::sarzak".to_string(),
                        model_file: PathBuf::from("../sarzak/models/sarzak.json"),
                        id: Uuid::parse_str("7178e7a4-5131-504b-a7b3-c2c0cfedf343").unwrap(),
                    })
                );
            } else if obj.name == "Simple Supertype" {
                assert!(config.is_imported(&obj.id));
                assert_eq!(
                    config_value.imported_object,
                    Some(ImportedObject {
                        domain: "domain::isa".to_string(),
                        model_file: PathBuf::from("tests/mdd/models/isa.json"),
                        id: Uuid::parse_str("6339b18b-3929-51ae-ad1a-f0cb4dc73362").unwrap(),
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

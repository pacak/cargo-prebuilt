use std::collections::HashSet;
use std::path::PathBuf;
use bpaf::batteries::cargo_helper;
use bpaf::Parser;
use directories::ProjectDirs;
use indexmap::IndexSet;
use crate::{APPLICATION, DEFAULT_INDEX, ORG, QUALIFIER, TARGET};
use crate::types::{ReportType, SigType, SpecType, TypingError, VerifyType};

static CONFIG_FILE: &str = "config.toml";

// TODO: Color? Require config file?
pub struct Config {
    pub spec: SpecType,
    pub index: String,
    pub public_keys: HashSet<String>,
    pub auth_token: Option<String>,
    pub verify_index_with: SigType,
    pub verify_blob_with: IndexSet<VerifyType>,
    pub safe: bool,
    pub path: PathBuf,
    pub report_path: PathBuf,
    pub reports: HashSet<ReportType>,
    pub ci: bool,
    pub out: bool,
    pub target: String,
    pub packages: IndexSet<String>,
}

// TODO: Switch to bpaf derive?
struct Arguments {
    config_file_path: PathBuf,
    require_config_file: bool,
    index_key: Option<String>,
    spec: SpecType,
    index: String,
    public_keys: HashSet<String>,
    auth_token: Option<String>,
    verify_index_with: SigType,
    verify_blob_with: IndexSet<VerifyType>,
    safe: bool,
    path: PathBuf,
    report_path: PathBuf,
    reports: HashSet<ReportType>,
    ci: bool,
    out: bool,
    target: String,
    packages: IndexSet<String>,
    color: bool,
    no_color: bool,
}

fn parse_args() -> Arguments {
    use bpaf::*;

    let p_dirs = ProjectDirs::from(QUALIFIER, ORG, APPLICATION).expect("Could not get project dirs. This should never happen!");

    let target = long("target")
        .env("PREBUILT_TARGET")
        .help(format!("Target of the binary to download. (Default: {})", TARGET).as_str())
        .argument::<String>("TARGET")
        .fallback(TARGET.to_string());

    let config_file_path = long("config")
        .env("PREBUILT_CONFIG")
        .help(format!("Path to the config file (Default: See https://github.com/cargo-prebuilt/cargo-prebuilt/blob/v{}/docs/PATHS.md#config)", env!("CARGO_PKG_VERSION")).as_str())
        .argument::<PathBuf>("CONFIG_PATH")
        .fallback(p_dirs.config_local_dir().with_file_name(CONFIG_FILE));

    let require_config_file = long("require-config").short('r')

        .env("PREBUILT_REQUIRE_CONFIG")
        .help("Require a config file to be used. (--ci overrides this)")
        .switch();

    let index_key = long("index-key")
        .env("PREBUILT_INDEX_KEY")
        .help("Index to use, pulling from config file. Overrides --index.")
        .argument::<String>("INDEX_KEY")
        .optional();

    let spec = long("spec")
        .env("PREBUILT_SPEC")
        .help("The index spec to use. (Default: gh-pub)")
        .argument::<String>("SPEC")
        .parse(|s| {
            s.parse::<SpecType>()
        })
        .fallback(SpecType::default());

    let index = long("index")
        .env("PREBUILT_INDEX")
        .help(format!("Index to use. (Default: {DEFAULT_INDEX})").as_str())
        .argument::<String>("INDEX")
        .fallback(DEFAULT_INDEX.to_string());

    let public_keys = long("pub-key")
        .short('p')
        .env("PREBUILT_PUB_KEY")
        .help("A public verifying key encoded as base64. Must be used with --index.")
        .argument::<String>("PUB_KEY")
        .collect()
        .fallback(HashSet::from([include_str!("../keys/cargo-prebuilt-index.pub").to_string()]));

    let auth_token = long("auth")
        .env("PREBUILT_AUTH")
        .help("Auth token to use for private indexes.")
        .argument::<String>("TOKEN")
        .optional();

    let verify_index_with = long("sig-with")
        .env("PREBUILT_SIG_WITH")
        .help("Method to verify the index with. (Default: minisign)")
        .argument::<String>("SIG")
        .parse(|s| {
            s.parse::<SigType>()
        })
        .fallback(SigType::default());

    let verify_blob_with = long("verify-with")
        .short('v')
        .env("PREBUILT_VERIFY_WITH")
        .help("Method to verify blobs with. Runs the 'best' one first, if possible. (Default: sha3_512, sha3_256, sha512, sha256)")
        .argument::<String>("VERIFY")
        .parse(|s| {
            s.parse::<VerifyType>()
        })
        .collect()
        .fallback(VerifyType::get_defaults());

    let safe = long("safe")
        .short('s')
        .env("PREBUILT_SAFE")
        .help("Do not overwrite binaries that already exist.")
        .switch();

    let path = long("path")
        .env("PREBUILT_PATH")
        .help("Path to the folder where downloaded binaries will be installed. (Default: $CARGO_HOME/bin)")
        .argument::<PathBuf>("PATH")
        .fallback(home::cargo_home().expect("Could not find $CARGO_HOME, please set the env var.").join("bin"));

    let report_path = long("report-path")
        .env("PREBUILT_REPORT_PATH")
        .help(format!("Path to the folder where the reports will be put (Default: See https://github.com/cargo-prebuilt/cargo-prebuilt/blob/v{}/docs/PATHS.md#reports)", env!("CARGO_PKG_VERSION")).as_str())
        .argument::<PathBuf>("REPORT_PATH")
        .fallback(p_dirs.data_local_dir().join("reports"));

    let reports = long("reports")
        .env("PREBUILT_REPORTS")
        .help(format!("Reports to be downloaded in a CSV format (Default: license_dl) (See: See https://github.com/cargo-prebuilt/cargo-prebuilt/blob/v{}/docs/CONFIG.md#report-types)", env!("CARGO_PKG_VERSION")).as_str())
        .argument::<String>("REPORTS")
        .parse(|s| {
            let mut v = HashSet::new();
            for i in s.split(',') {
                v.insert(s.parse::<ReportType>()?);
            }
            Ok::<HashSet<ReportType>, TypingError>(v)
        })
        .fallback(ReportType::get_defaults());

    let ci = long("ci")
        .env("PREBUILT_CI")
        .help("Do not download reports, do not check for a config file, and always overwrite files.")
        .switch();

    let out = long("out")
        .env("PREBUILT_OUT")
        .help("Output events.")
        .switch();

    let color = long("color")
        .env("FORCE_COLOR")
        .help("Force color to be turned on.")
        .switch();

    let no_color = long("no-color")
        .env("FORCE_NO_COLOR")
        .help("Force color to be turned off.")
        .switch();

    let packages = positional::<String>("PKGS")
        .help("A CSV list of packages with optional @VERSION")
        .parse(|s| {
            let mut v = IndexSet::new();
            for i in s.split(',') {
                v.insert(i.to_string());
            }
            Ok::<IndexSet<String>, String>(v)
        });

    let parser = construct!(Arguments {
        target,
        config_file_path,
        require_config_file,
        index_key,
        spec,
        index,
        public_keys,
        auth_token,
        verify_index_with,
        verify_blob_with,
        safe,
        path,
        report_path,
        reports,
        ci,
        out,
        color,
        no_color,
        packages,
    });

    cargo_helper("prebuilt", parser)
        .to_options()
        .version(env!("CARGO_PKG_VERSION"))
        .run()
}

pub fn get() -> anyhow::Result<Config>{
    let args = parse_args();
    todo!()
}

use crate::{
    color,
    data::{ConfigFileV1, ReportType, SigKeys},
    DEFAULT_INDEX, TARGET,
};
use bpaf::*;
use home::{cargo_home, home_dir};
use indexmap::IndexSet;
use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

#[derive(Clone, Debug)]
pub struct Config {
    pub target: String,
    pub index: String,
    pub auth: Option<String>,
    pub path: PathBuf,
    pub report_path: PathBuf,
    pub ci: bool,
    pub no_create_path: bool,
    pub reports: IndexSet<ReportType>,
    pub sigs: Vec<String>,
    pub no_verify: bool,
    pub safe: bool,
    pub pkgs: IndexSet<String>,
}

#[derive(Clone, Debug)]
struct Arguments {
    target: Option<String>,
    index: Option<String>,
    auth: Option<String>,
    path: Option<PathBuf>,
    report_path: Option<PathBuf>,
    ci: bool,
    no_create_path: bool,
    reports: Option<IndexSet<ReportType>>,
    pub_key: Option<String>,
    no_verify: bool,
    safe: bool,
    color: bool,
    no_color: bool,
    pkgs: IndexSet<String>,
}

fn parse_args() -> Arguments {
    let pkgs = positional::<String>("PKGS")
        .help("A CSV list of packages with optional @VERSION")
        .parse(|s| {
            let mut v = IndexSet::new();
            for i in s.split(',') {
                v.insert(i.to_string());
            }
            Ok::<IndexSet<String>, String>(v)
        });

    let target = long("target")
        .env("PREBUILT_TARGET")
        .help("Target of the binary to download. (Defaults to target of cargo-prebuilt)")
        .argument::<String>("TARGET")
        .optional();

    let index = long("index")
        .env("PREBUILT_INDEX")
        .help(format!("Index to use. (Default: {DEFAULT_INDEX})").as_str())
        .argument::<String>("INDEX")
        .optional();

    let auth = long("auth")
        .env("PREBUILT_AUTH")
        .help("Auth token to use for private indexes.")
        .argument::<String>("TOKEN")
        .optional();

    let path = long("path")
        .env("PREBUILT_PATH")
        .help("Path to the folder where downloaded binaries will be installed. (Default: $CARGO_HOME)")
        .argument::<PathBuf>("PATH")
        .optional();

    let report_path = long("report-path")
        .env("PREBUILT_REPORT_PATH")
        .help("Path to the folder where the reports will be put. (Default: $HOME/.prebuilt)")
        .argument::<PathBuf>("REPORT_PATH")
        .optional();

    let ci = long("ci")
        .env("PREBUILT_CI")
        .help("Do not download reports, create a .prebuilt directory, check for a config file, or allow safe mode.")
        .switch();

    let no_create_path = long("no-create-path")
        .env("PREBUILT_NO_CREATE_PATH")
        .help("Do not create the report and/or bin folder if it is missing.")
        .switch();

    let reports = long("reports")
        .env("PREBUILT_REPORTS")
        .help("A CSV list of reports types. (license, deps, audit)")
        .argument::<String>("REPORTS")
        .parse(|s| {
            let mut v = IndexSet::new();
            if !s.eq("") {
                for i in s.split(',') {
                    match TryInto::<ReportType>::try_into(i) {
                        Ok(d) => {
                            let _ = v.insert(d);
                        }
                        Err(_) => return Err(format!("{i} is not a report type.")),
                    }
                }
            }
            Ok(v)
        })
        .optional();

    let pub_key = long("pub-key")
        .env("PREBUILT_PUB_KEY")
        .help("A public verifying key encoded as base64. Must be used with --index.")
        .argument::<String>("PUB_KEY")
        .optional();

    let no_verify = long("no-verify")
        .env("PREBUILT_NO_VERIFY")
        .help("Do not verify downloaded info.json's and hashes.json's.")
        .switch();

    let safe = short('s')
        .long("safe")
        .env("PREBUILT_SAFE")
        .help("Do not overwrite binaries that already exist.")
        .switch();

    let color = long("color")
        .env("FORCE_COLOR")
        .help("Force color to be turned on.")
        .switch();

    let no_color = long("no-color")
        .env("NO_COLOR")
        .help("Force color to be turned off.")
        .switch();

    let parser = construct!(Arguments {
        target,
        index,
        auth,
        path,
        report_path,
        ci,
        no_create_path,
        reports,
        pub_key,
        no_verify,
        safe,
        color,
        no_color,
        pkgs,
    });

    cargo_helper("prebuilt", parser)
        .to_options()
        .version(env!("CARGO_PKG_VERSION"))
        .run()
}

fn fill_from_file(args: &mut Arguments, sig_keys: &mut SigKeys) {
    match home_dir() {
        Some(mut conf) => {
            conf.push(".config/cargo-prebuilt/config.toml");
            if conf.exists() {
                let mut file = File::open(conf).expect("Could not open config file.");
                let mut str = String::new();
                file.read_to_string(&mut str)
                    .expect("Could not read config file.");

                let config: Result<ConfigFileV1, toml::de::Error> = toml::from_str(&str);
                match config {
                    Ok(config) => {
                        if let Some(mut keys) = config.key {
                            for (_, v) in keys.iter_mut() {
                                if sig_keys.contains_key(&(v.index)) {
                                    sig_keys
                                        .get_mut(&(v.index))
                                        .unwrap()
                                        .push(v.pub_key.clone());
                                }
                                else {
                                    sig_keys.insert(v.index.clone(), vec![v.pub_key.clone()]);
                                }
                            }
                        }

                        if let Some(prebuilt) = config.prebuilt {
                            // TODO: Way to not clone?
                            macro_rules! file_convert {
                                ($($x:ident), *) => {
                                    {
                                        $(args.$x = args.$x.clone().or(prebuilt.$x);)*
                                    }
                                };
                            }
                            macro_rules! file_convert_switch {
                                ($($x:ident), *) => {
                                    {
                                        $(if !args.$x {
                                            if let Some(opt) = prebuilt.$x {
                                                args.$x = opt;
                                            }
                                        })*
                                    }
                                };
                            }

                            file_convert![target, index, auth, path, report_path, reports];
                            file_convert_switch![no_create_path, no_verify, safe, color];
                        }
                    }
                    Err(err) => eprintln!("Failed to parse config file.\n{err}"),
                }
            }
        }
        None => eprintln!("Could not find home directory! Config file will be ignored."),
    }
}

fn convert(args: Arguments, mut sigs: SigKeys) -> Config {
    let target = match args.target {
        Some(val) => val,
        None => TARGET.to_owned(),
    };

    let index = match args.index {
        Some(val) => val,
        None => DEFAULT_INDEX.to_string(),
    };

    let auth = args.auth;

    let path = match args.path {
        Some(val) => val,
        None => {
            let mut cargo_home = cargo_home().expect("Could not find cargo home directory, please set CARGO_HOME or use PREBUILT_PATH or --path");
            if !cargo_home.ends_with("bin") {
                cargo_home.push("bin");
            }
            cargo_home
        }
    };

    let report_path = match args.report_path {
        Some(val) => val,
        None => {
            let mut prebuilt_home = home_dir().expect("Could not find home directory, please set HOME or use PREBUILT_REPORT_PATH or --report-path");
            prebuilt_home.push(".prebuilt");
            prebuilt_home
        }
    };

    let ci = args.ci;

    let no_create_path = args.no_create_path;

    let reports = match args.reports {
        Some(val) => val,
        None => IndexSet::from([ReportType::LicenseDL]),
    };

    let no_verify = args.no_verify;

    let safe = args.safe;

    let sigs = sigs.remove(&index).unwrap_or_else(|| {
        if no_verify {
            eprintln!("Expected to find public key(s) for index {index}, but there was none.");
            std::process::exit(403);
        }
        Vec::new()
    });

    match (args.color, args.no_color) {
        (true, false) => color::set_override(true),
        (_, true) => color::set_override(false),
        _ => color::from_stream(),
    }

    let pkgs = args.pkgs;

    Config {
        target,
        index,
        auth,
        path,
        report_path,
        ci,
        no_create_path,
        reports,
        sigs,
        no_verify,
        safe,
        pkgs,
    }
}

pub fn get() -> Config {
    // arguments and env vars
    let mut args = parse_args();
    #[cfg(debug_assertions)]
    dbg!(&args);

    // Check if sig is used with index.
    if args.pub_key.is_some() && args.index.is_none() {
        eprintln!("pub_key must be used with index.");
        std::process::exit(502);
    }

    let mut keys: SigKeys = HashMap::with_capacity(1);
    keys.insert(
        DEFAULT_INDEX.to_string(),
        vec![include_str!("../keys/cargo-prebuilt-index.pub").to_string()],
    );

    // Add sig key from args
    if let Some(k) = &args.pub_key {
        keys.insert(args.index.clone().unwrap(), vec![k.clone()]);
    }

    // config file
    if !args.ci {
        fill_from_file(&mut args, &mut keys);
        #[cfg(debug_assertions)]
        dbg!(&args);
    }

    convert(args, keys)
}

#[cfg(test)]
mod test {
    use minisign_verify::{PublicKey, Signature};

    #[test]
    fn test_minisign1() {
        let data = include_bytes!("../test/pubdata.test");
        let sig = include_str!("../test/pubdata.test.minisig");
        let pubkey = include_str!("../test/pubdata.pub");

        let signature = Signature::decode(sig).unwrap();
        let pk = PublicKey::from_base64(pubkey).unwrap();
        pk.verify(data, &signature, false).unwrap();
    }
}
mod color;
mod config;
mod data;
mod events;
mod get;
mod interact;

use flate2::read::GzDecoder;
use std::{
    fs::{self, create_dir_all, File},
    io::{Read, Write},
    path::Path,
    str,
};
use tar::Archive;

use crate::{
    color::{err_color_print, PossibleColor},
    get::Fetcher,
};

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

static DEFAULT_INDEX: &str = "gh-pub:github.com/cargo-prebuilt/index";
static TARGET: &str = env!("TARGET");

fn main() -> Result<(), String> {
    let config = config::get();
    #[cfg(debug_assertions)]
    dbg!(&config);

    // Check if a needed feature was excluded.
    should_error();

    if !config.no_create_path && create_dir_all(&config.path).is_err() {
        eprintln!("Could not create the directories {:?}.", config.path);
        std::process::exit(44);
    }
    else if !Path::new(&config.path).exists() {
        eprintln!("Directories do not exist! {:?}.", config.path);
        std::process::exit(45);
    }

    if !config.no_create_path && create_dir_all(&config.report_path).is_err() {
        eprintln!("Could not create the directories {:?}.", config.report_path);
        std::process::exit(44);
    }
    else if !Path::new(&config.report_path).exists() {
        eprintln!("Directories do not exist! {:?}.", config.report_path);
        std::process::exit(45);
    }

    // Build ureq agent
    let agent = create_agent();

    // Create Fetcher which is used to fetch items from index.
    let mut fetcher = Fetcher::new(&config, agent);

    // Get pkgs
    for pkg in config.pkgs.iter() {
        let mut id = pkg.as_str();
        let mut version = None; // None will pull the latest version

        // If there is a version string get it
        if let Some((i, j)) = id.split_once('@') {
            id = i;
            version = Some(j)
        }

        // Init fetcher for this crate and get latest version if needed
        fetcher.load(id, version);

        // Get version that fetcher is using
        let version = fetcher.get_version();

        events::target(id, &version, &config);

        // Download and hash tar
        let tar_bytes = fetcher.download(&config);

        // Extract Tar
        let reader = std::io::Cursor::new(tar_bytes);
        let mut archive = Archive::new(GzDecoder::new(reader));
        match archive.entries() {
            Ok(es) => {
                eprintln!(
                    "{} {id}@{version}...",
                    err_color_print("Extracting", PossibleColor::BrightBlue)
                );

                for e in es {
                    let mut e = e.expect("Malformed entry in tarball.");

                    let mut blob_data = Vec::new();
                    e.read_to_end(&mut blob_data)
                        .expect("Could not extract binary from archive.");

                    let bin_path = e.path().expect("Could not extract path from archive.");
                    let str_name = bin_path
                        .clone()
                        .into_owned()
                        .into_os_string()
                        .into_string()
                        .expect("Archive has non utf-8 path.");

                    // Make sure there are no path separators since this will be appended
                    if str_name.contains(std::path::is_separator) {
                        eprintln!(
                            "{} path separator in archive for {id}@{version}",
                            err_color_print("Illegal", PossibleColor::BrightRed)
                        );
                        std::process::exit(488);
                    }

                    if !fetcher.is_bin(&str_name) {
                        eprintln!(
                            "{} binary ({str_name}) in archive for {id}@{version}",
                            err_color_print("Illegal", PossibleColor::BrightRed)
                        );
                        std::process::exit(499);
                    }

                    let mut path = config.path.clone();
                    path.push(bin_path);

                    if config.safe && !config.ci && path.exists() {
                        eprintln!(
                            "Binary {str_name} {} for {id}@{version}",
                            err_color_print("already exists", PossibleColor::BrightRed)
                        );
                        std::process::exit(4091);
                    }

                    let mut file =
                        File::create(&path).expect("Could not open file to write binary to.");
                    file.write_all(&blob_data)
                        .expect("Could not write binary to file.");

                    // Add +x permission on unix platforms.
                    #[cfg(target_family = "unix")]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        fs::set_permissions(&path, fs::Permissions::from_mode(0o755))
                            .expect("Could not set permissions.");
                    }

                    let abs = fs::canonicalize(path).expect("Could not canonicalize install path.");

                    eprintln!(
                        "{} {abs:?}.",
                        err_color_print("Installed", PossibleColor::BrightPurple)
                    );

                    events::binary_installed(id, &version, &config, abs.as_path());
                }
            }
            Err(_) => {
                eprintln!("Cannot get entries from downloaded tar.");
                std::process::exit(13);
            }
        }

        // Reports
        if !config.ci {
            fetcher.reports(&config);
        }

        eprintln!(
            "{} {id}@{version}.",
            err_color_print("Installed", PossibleColor::BrightGreen)
        );

        // Prepare for next crate.
        fetcher.reset();
    }

    eprintln!("{}", err_color_print("Done!", PossibleColor::Green));

    Ok(())
}

fn should_error() {
    // No TLS
    #[cfg(not(any(feature = "native", feature = "rustls")))]
    {
        eprintln!("cargo-prebuilt only supports https and was built without the 'native' or 'rustls' feature.");
        std::process::exit(400);
    }

    // No Indexes
    #[cfg(not(any(feature = "github-public", feature = "github-private")))]
    {
        eprintln!("cargo-prebuilt was not built with any indexes, try the 'indexes' feature.");
        std::process::exit(222);
    }
}

fn create_agent() -> ureq::Agent {
    #[cfg(feature = "native")]
    let agent = ureq::AgentBuilder::new().tls_connector(std::sync::Arc::new(
        native_tls::TlsConnector::new().expect("Could not create TlsConnector"),
    ));

    #[cfg(feature = "rustls")]
    let agent = ureq::AgentBuilder::new();

    #[cfg(any(feature = "native", feature = "rustls"))]
    let agent = agent
        .https_only(true)
        .user_agent(format!("cargo-prebuilt_cli {}", env!("CARGO_PKG_VERSION")).as_str())
        .build();

    // Allows for any feature set to be built for, even though this is unsupported.
    #[cfg(not(any(feature = "native", feature = "rustls")))]
    let agent = ureq::agent();

    agent
}

use std::path::Path;

use crate::config::Config;

const EVENT_VERSION: u8 = 1;

pub fn info_verify(id: &str, version: &str, config: &Config, verified: bool) {
    if config.out {
        println!("{{\"crate\":\"{id}\",\"version\":\"{version}\",\"event_version\":\"{EVENT_VERSION}\",\"event\":\"info_verify\",\"data\":\"{verified}\"}}");
    }
}

pub fn hashes_verify(id: &str, version: &str, config: &Config, verified: bool) {
    if config.out {
        println!("{{\"crate\":\"{id}\",\"version\":\"{version}\",\"event_version\":\"{EVENT_VERSION}\",\"event\":\"hashes_verify\",\"data\":\"{verified}\"}}");
    }
}

pub fn target(id: &str, version: &str, config: &Config) {
    if config.out {
        println!("{{\"crate\":\"{id}\",\"version\":\"{version}\",\"event_version\":\"{EVENT_VERSION}\",\"event\":\"target\",\"data\":\"{}\"}}", config.target);
    }
}

pub fn binary_installed(id: &str, version: &str, config: &Config, path: &Path) {
    if config.out {
        println!("{{\"crate\":\"{id}\",\"version\":\"{version}\",\"event_version\":\"{EVENT_VERSION}\",\"event\":\"bin_installed\",\"data\":{path:?}}}");
    }
}

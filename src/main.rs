#![allow(unused)] // TODO: Remove!

use std::collections::HashSet;
use std::string::ToString;
use indexmap::IndexSet;
use crate::types::SpecType;

mod config;
mod types;
mod events;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

static QUALIFIER: &str = "tech";
static ORG: &str = "harmless";
static APPLICATION: &str = "cargo-prebuilt";

static DEFAULT_INDEX: &str = "https://github.com/cargo-prebuilt/index";
static TARGET: &str = env!("TARGET");

fn main() -> anyhow::Result<()> {
    let config = config::get()?;

    todo!()
}

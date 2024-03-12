#![allow(unused)]

use once_cell::sync::Lazy;
use std::path::Path;

/// Directory containing project test data.
///
/// See: [`config.toml`](../.cargo/config.toml)
pub static DATA_DIR: Lazy<&Path> = Lazy::new(|| Path::new(env!("TEST_DATA_DIR")));

/// Directory for build/testing artifacts, appropriate for writing results.
///
/// See: [`config.toml`](../.cargo/config.toml)
pub static TARGET_DIR: Lazy<&Path> = Lazy::new(|| Path::new(env!("CARGO_TARGET_DIR")));

/// Read a text file from the test data directory.
pub fn read_test_file(filename: &str) -> String {
    std::fs::read_to_string(DATA_DIR.join(filename))
        .expect(&format!("Data file contents for {filename}"))
}

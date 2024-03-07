use crate::utils::Conv;
use autocxx::prelude::*;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Config {
    pub version: String,
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub sha: String,
    pub plugin_path: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        Self {
            version: Conv(crate::ffi::pdal::Config::versionString()).into(),
            major: crate::ffi::pdal::Config::versionMajor().into(),
            minor: crate::ffi::pdal::Config::versionMinor().into(),
            patch: crate::ffi::pdal::Config::versionPatch().into(),
            sha: Conv(crate::ffi::pdal::Config::sha1()).into(),
            plugin_path: Conv(crate::ffi::pdal::Config::pluginInstallPath()).into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_pdal_config() {
        let conf = Config::new();
        dbg!(conf);
    }
}

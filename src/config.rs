use crate::error::Result;
use crate::utils::{fetch_int, fetch_string_with_buffer, Conv};
use pdal_sys::{
    PDALPluginInstallPath, PDALSha1, PDALVersionMajor, PDALVersionMinor, PDALVersionPatch,
    PDALVersionString,
};
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
    pub fn new() -> Result<Self> {
        Ok(Self {
            version: Conv(fetch_string_with_buffer::<256>(PDALVersionString)?).try_into()?,
            major: fetch_int(PDALVersionMajor)?,
            minor: fetch_int(PDALVersionMinor)?,
            patch: fetch_int(PDALVersionPatch)?,
            sha: Conv(fetch_string_with_buffer::<256>(PDALSha1)?).try_into()?,
            plugin_path: Conv(fetch_string_with_buffer::<1024>(PDALPluginInstallPath)?)
                .try_into()?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_pdal_config() {
        let conf = Config::new().expect("PDAL Config");
        dbg!(conf);
    }
}

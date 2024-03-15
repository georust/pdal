// MIT License
//
// Copyright (c) 2024 NUVIEW, Inc. <simeon.fitch@nuview.space>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT
// OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

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
        assert_eq!(conf.major, 2);
    }
}

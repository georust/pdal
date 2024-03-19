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

#![allow(dead_code)]
#[cxx::bridge]
pub mod ffi {
    #[namespace = "pdal::Config"]
    #[repr(u32)]
    enum Feature {
        LAZPERF,
        ZSTD,
        ZLIB,
        LZMA,
        LIBXML2
    }
    
    #[namespace = "pdal::Config"]
    unsafe extern "C++" {
        include!("pdal-sys/src/config/config.hpp");
        type Feature;
        fn hasFeature(f: Feature) -> bool;
        fn versionInteger() -> i32;
        fn versionMajor() -> i32;
        fn versionMinor() -> i32;
        fn versionPatch() -> i32;
    }
    #[namespace = "pdal_sys::Config"]
    unsafe extern "C++" {
        include!("pdal-sys/src/config/config.hpp");
        fn fullVersionString() -> String;
        fn versionString() -> String;
        fn sha1() -> String;
        fn debugInformation() -> String;
        fn pluginInstallPath() -> String;
    }
}

#[cfg(test)]
mod tests {
    use super::ffi::Feature;

    #[test]
    fn test_version() {
        let major = super::ffi::versionMajor();
        let minor = super::ffi::versionMinor();
        let patch = super::ffi::versionPatch();
        let ver_int = super::ffi::versionInteger();
        assert_eq!(ver_int, major * 10000 + minor * 100 + patch);
    }

    #[test]
    fn test_version_string() {
        let full_version = super::ffi::fullVersionString();
        let version = super::ffi::versionString();
        let sha1 = super::ffi::sha1();
        assert!(!full_version.is_empty());
        assert!(!version.is_empty());
        assert!(!sha1.is_empty());
        assert!(full_version.contains(&sha1));
    }
    
    #[test]
    fn test_feature() {
        // Expect at least one of these to be true
        assert!(
            super::ffi::hasFeature(Feature::LAZPERF) ||
                super::ffi::hasFeature(Feature::ZSTD) ||
                super::ffi::hasFeature(Feature::ZLIB) ||
                super::ffi::hasFeature(Feature::LZMA) ||
                super::ffi::hasFeature(Feature::LIBXML2)
        );
    }
}

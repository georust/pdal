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
mod ffi {
    #[namespace = "pdal::Config"]
    #[repr(u32)]
    enum Feature {
        LAZPERF,
        ZSTD,
        ZLIB,
        LZMA,
        LIBXML2,
    }

    // These functions are bound directly since they return primitives.
    #[namespace = "pdal::Config"]
    unsafe extern "C++" {
        include!("pdal-sys/src/config/config.hpp");
        type Feature;
        #[cxx_name = "hasFeature"]
        fn has_feature(f: Feature) -> bool;
        #[cxx_name = "versionInteger"]
        fn version_integer() -> i32;
        #[cxx_name = "versionMajor"]
        fn version_major() -> i32;
        #[cxx_name = "versionMinor"]
        fn version_minor() -> i32;
        #[cxx_name = "versionPatch"]
        fn version_patch() -> i32;
    }

    // These functions require wrappers to translate the Rust types to C++
    #[namespace = "pdal_sys::Config"]
    unsafe extern "C++" {
        include!("pdal-sys/src/config/config.hpp");
        #[cxx_name = "fullVersionString"]
        fn full_version_string() -> String;
        #[cxx_name = "versionString"]
        fn version_string() -> String;
        fn sha1() -> String;
        #[cxx_name = "debugInformation"]
        fn debug_information() -> String;
        #[cxx_name = "pluginInstallPath"]
        fn plugin_install_path() -> String;
    }
}

pub use ffi::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let major = version_major();
        let minor = version_minor();
        let patch = version_patch();
        let ver_int = version_integer();
        assert_eq!(ver_int, major * 10000 + minor * 100 + patch);
    }

    #[test]
    fn test_version_string() {
        let full_version = full_version_string();
        let version = version_string();
        let sha1 = sha1();
        assert!(!full_version.is_empty());
        assert!(!version.is_empty());
        assert!(!sha1.is_empty());
        assert!(full_version.contains(&sha1));
    }

    #[test]
    fn test_feature() {
        // Expect at least one of these to be true
        assert!(
            has_feature(Feature::LAZPERF)
                || has_feature(Feature::ZSTD)
                || has_feature(Feature::ZLIB)
                || has_feature(Feature::LZMA)
                || has_feature(Feature::LIBXML2)
        );
    }
}

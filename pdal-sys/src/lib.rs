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
        LIBXML2
    }
    
    #[namespace = "pdal::Config"]
    unsafe extern "C++" {
        include!("pdal-sys/src/bridge.hpp");
        type Feature;
        fn hasFeature(f: Feature) -> bool;
        fn versionInteger() -> i32;
        fn versionMajor() -> i32;
        fn versionMinor() -> i32;
        fn versionPatch() -> i32;
    }
    #[namespace = "pdal_sys::Config"]
    unsafe extern "C++" {
        include!("pdal-sys/src/bridge.hpp");
        fn fullVersionString() -> String;
        fn versionString() -> String;
        fn sha1() -> String;
        fn debugInformation() -> String;
        fn pluginInstallPath() -> String;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_version() {
        let major = crate::ffi::versionMajor();
        let minor = crate::ffi::versionMinor();
        let patch = crate::ffi::versionPatch();
        let ver_int = crate::ffi::versionInteger();
        assert_eq!(ver_int, major * 10000 + minor * 100 + patch);
    }

    #[test]
    fn test_version_string() {
        let full_version = crate::ffi::fullVersionString();
        let version = crate::ffi::versionString();
        let sha1 = crate::ffi::sha1();
        assert!(!full_version.is_empty());
        assert!(!version.is_empty());
        assert!(!sha1.is_empty());
        assert!(full_version.contains(&sha1));
    }
    
    #[test]
    fn test_feature() {
        // Expect at least one of these to be true
        assert!(
            crate::ffi::hasFeature(crate::ffi::Feature::LAZPERF) ||
                crate::ffi::hasFeature(crate::ffi::Feature::ZSTD) ||
                crate::ffi::hasFeature(crate::ffi::Feature::ZLIB) ||
                crate::ffi::hasFeature(crate::ffi::Feature::LZMA) ||
                crate::ffi::hasFeature(crate::ffi::Feature::LIBXML2)
        );
    }
}

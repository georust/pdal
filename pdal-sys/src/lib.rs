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

/// Low-level bindings to PDAL, a point cloud processing library.
/// See the `pdal` crate for the idiomatic interface.
pub mod config;
pub mod core;
pub mod layout;
pub mod pipeline_manager;
pub mod point_view;

#[cfg(test)]
pub(crate) mod testkit {
    use once_cell::sync::Lazy;
    use std::path::Path;
    pub static DATA_DIR: Lazy<&Path> = Lazy::new(|| Path::new(env!("TEST_DATA_DIR")));
    pub static TEST_WD: Lazy<&Path> = Lazy::new(|| Path::new(env!("PKG_DIR")));

    pub fn data_file_path(name: &str) -> String {
        DATA_DIR.join(name).to_string_lossy().to_string()
    }
}

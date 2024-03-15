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

pub type TestResult = crate::error::Result<()>;

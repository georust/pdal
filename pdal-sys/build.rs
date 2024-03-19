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

use cxx_build::CFG;
use std::error::Error;
use std::path::PathBuf;

static MODULES: &[&str] = &["config", "options", "pipeline_manager"];

// See https://github.com/alexcrichton/curl-rust/blob/0.4.34/curl-sys/build.rs
// for inspiration.
fn main() -> Result<(), Box<dyn Error>> {
    // std::env::set_var("RUST_LOG", "bindgen=info");
    // let _ = env_logger::builder().is_test(true).try_init()?;
    let mut pdal_pkg_config = pkg_config::Config::new().probe("pdal")?;
    // For some reason pkg-config reports a path like `/foo/bar/include/pdal`, but
    // internal C++ references assume that the path is `/foo/bar/include`.
    if let Some(pdal_inc) = pdal_pkg_config
        .include_paths
        .iter()
        .find(|&path| path.ends_with("pdal"))
    {
        if let Some(parent) = pdal_inc.parent() {
            pdal_pkg_config.include_paths.push(parent.to_path_buf());
        }
    }

    CFG.exported_header_dirs
        .extend(pdal_pkg_config.include_paths.iter().map(|p| p.as_path()));

    let module_files = MODULES
        .iter()
        .map(|&m| PathBuf::from(format!("src/{m}/mod.rs")))
        .collect::<Vec<_>>();

    let mut builder = cxx_build::bridges(module_files);
    builder
        .flag_if_supported("-std=c++14")
        .cargo_warnings(false);

    for m in MODULES {
        builder.file(format!("src/{m}/{m}.cpp"));
    }

    builder.compile("pdal-sys");

    for m in MODULES {
        println!("cargo:rerun-if-changed=src/{m}/mod.rs");
        println!("cargo:rerun-if-changed=src/{m}/{m}.cpp");
        println!("cargo:rerun-if-changed=src/{m}/{m}.hpp");
    }

    Ok(())
}

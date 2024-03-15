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

use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

// See https://github.com/alexcrichton/curl-rust/blob/0.4.34/curl-sys/build.rs
// and https://github.com/rust-lang/cargo/issues/5077
// for inspiration.
fn main() -> Result<(), Box<dyn Error>> {
    // std::env::set_var("RUST_LOG", "bindgen=info");
    // let _ = env_logger::builder().is_test(true).try_init()?;

    let here = Path::new(env!("CARGO_MANIFEST_DIR"));

    if !here.join("vendor").join("CMakeLists.txt").exists() {
        println!("cargo:warning=Checking out PDAL/CAPI as a submodule");
        let r = Command::new("git")
            .current_dir(&here)
            .args(&[
                "submodule",
                "update",
                "--force",
                "--init",
                "--recursive",
                "vendor",
            ])
            .status()
            .expect("`git submodule update --init --recursive vendor` failed.");

        if !r.success() {
            panic!("Error running `git`: {r}")
        }
    }

    let dst = cmake::Config::new(here.join("vendor"))
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CONDA_BUILD", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF") // not working
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=pdalc");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(&[
            "-I",
            &dst.join("include").to_string_lossy(),
            "-fretain-comments-from-system-headers",
        ])
        .allowlist_function("PDAL.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(&env::var("OUT_DIR")?).join("bindings.rs");
    //println!("cargo:warning={}", out_path.to_string_lossy());
    bindings.write_to_file(out_path)?;

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

    Ok(())
}

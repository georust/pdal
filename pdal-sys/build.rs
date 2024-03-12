use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "bindgen=info");
    let _ = env_logger::builder().is_test(true).try_init()?;

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

    Ok(())
}

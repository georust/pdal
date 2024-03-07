use pkg_config::Config;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut gdal_pkg_config = Config::new().probe("pdal")?;

    // For some reason pkg-config reports a path like `/foo/bar/include/pdal`, but
    // internal C++ references assume that the path is `/foo/bar/include`.
    if let Some(pdal_inc) = gdal_pkg_config
        .include_paths
        .iter()
        .find(|&path| path.ends_with("pdal"))
    {
        if let Some(parent) = pdal_inc.parent() {
            gdal_pkg_config.include_paths.push(parent.to_path_buf());
        }
    }

    let mut b = autocxx_build::Builder::new("src/lib.rs", gdal_pkg_config.include_paths).build()?;
    b.cargo_warnings(false);
    b.flag_if_supported("-std=c++17");
    b.compile("pdal");
    println!("cargo:rerun-if-changed=src/lib.rs");
    Ok(())
}

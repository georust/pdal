use autocxx::prelude::*;

include_cpp! {
    #include "pdal/pdal_config.hpp"
    safety!(unsafe_ffi)
    generate_ns!("pdal::Config")
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_pdal_config() {
        let s = ffi::pdal::Config::fullVersionString();
        dbg!(s.to_string_lossy());
    }
}

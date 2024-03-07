mod config;
mod error;
pub(crate) mod utils;

use autocxx::prelude::*;

include_cpp! {
    #include "pdal/pdal_config.hpp"
    #include "pdal/StageFactory.hpp"
    safety!(unsafe_ffi)
    generate_ns!("pdal::Config")
    generate!("pdal::StageFactory")
}

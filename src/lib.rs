mod config;
mod driver;
mod error;
mod pipeline;
pub(crate) mod utils;

use autocxx::prelude::*;

include_cpp! {
    #include "pdal/pdal.hpp"
    #include "pdal/PipelineManager.hpp"
    #include "pdal/PipelineWriter.hpp"
    #include "pdal/Stage.hpp"
    #include "shims.hpp"
    safety!(unsafe_ffi)
    generate_ns!("pdal::Config")
    generate!("pdal::PipelineManager")
}

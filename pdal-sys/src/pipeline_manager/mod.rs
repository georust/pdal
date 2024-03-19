#![allow(dead_code)]

use cxx::UniquePtr;
use std::fmt::Debug;

#[cxx::bridge]
pub mod ffi {
    #[namespace = "pdal_sys"]
    unsafe extern "C++" {
        include!("pdal-sys/src/pipeline_manager/pipeline_manager.hpp");
        type PipelineManager;
        #[cxx_name = "createPipelineManager"]
        fn create_pipeline_manager() -> UniquePtr<PipelineManager>;
        #[cxx_name = "readPipeline"]
        fn read_pipeline(self: Pin<&mut PipelineManager>, pipeline: &str) -> Result<()>;
        #[cxx_name = "readPipelineFromFile"]
        fn read_pipeline_from_file(self: Pin<&mut PipelineManager>, path: &str) -> Result<()>;
        #[cxx_name = "pipelineStreamable"]
        fn pipeline_streamable(&self) -> bool;
        fn execute(self: Pin<&mut PipelineManager>) -> Result<usize>;
        #[cxx_name = "executeStreamed"]
        fn execute_streamed(self: Pin<&mut PipelineManager>) -> Result<()>;
    }
}

pub type PipelineManager = ffi::PipelineManager;
impl PipelineManager {
    pub fn new() -> UniquePtr<ffi::PipelineManager> {
        ffi::create_pipeline_manager()
    }
}
pub type PipelineManagerPtr = UniquePtr<ffi::PipelineManager>;

impl Debug for PipelineManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipelineManager").finish()
    }
}

#[cfg(test)]
mod tests {
    const PIPELINE_FILE: &str = "tests/data/info.json";

    #[test]
    fn test_parse_pipeline() {
        let good =
            r#"{ "pipeline": [ {"filename":"foobar.las", "spatialreference":"EPSG:2993" } ] }"#;
        let bad = r#"{"pipeline":[{"blah":"foobar.las"}]}"#;

        let mut mgr = super::ffi::create_pipeline_manager();
        let r = mgr.pin_mut().read_pipeline(good);
        assert!(r.is_ok(), "Error: {:?}", r.err());

        let mut mgr = super::ffi::create_pipeline_manager();

        let r = mgr.pin_mut().read_pipeline(bad);
        assert!(r.is_err());
        assert!(r.err().unwrap().to_string().contains("reader"));
    }

    #[test]
    fn test_read_pipeline() {
        std::env::set_current_dir("..").unwrap();
        let mut mgr = super::ffi::create_pipeline_manager();
        let r = mgr.pin_mut().read_pipeline_from_file(PIPELINE_FILE);
        assert!(r.is_ok(), "Error: {:?}", r.err());
        let r = mgr.pin_mut().execute();
        assert_eq!(r.unwrap(), 110000);
    }
}

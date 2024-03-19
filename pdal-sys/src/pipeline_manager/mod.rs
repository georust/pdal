#![allow(dead_code)]
#[cxx::bridge]
pub mod ffi {
    #[namespace = "pdal_sys"]   
    unsafe extern "C++" {
        include!("pdal-sys/src/pipeline_manager/pipeline_manager.hpp");
        type PipelineManager;
        fn create_pipeline_manager() -> UniquePtr<PipelineManager>;
        fn readPipeline(self: Pin<&mut PipelineManager>, pipeline: &str) -> Result<()>;
        fn readPipelineFromFile(self: Pin<&mut PipelineManager>, path: &str) -> Result<()>;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_pipeline() {
        let good = r#"{ "pipeline": [ {"filename":"foobar.las", "spatialreference":"EPSG:2993" } ] }"#;
        let bad = r#"{"pipeline":[{"blah":"foobar.las"}]}"#;
        
        let mut mgr = super::ffi::create_pipeline_manager();
        let r = mgr.pin_mut().readPipeline(good);
        assert!(r.is_ok(), "Error: {:?}", r.err());

        let mut mgr = super::ffi::create_pipeline_manager();

        let r = mgr.pin_mut().readPipeline(bad);
        assert!(r.is_err());
        assert!(r.err().unwrap().to_string().contains("reader"));
    }

    #[test]
    fn test_read_pipeline() {
        let mut mgr = super::ffi::create_pipeline_manager();
        let r = mgr.pin_mut().readPipelineFromFile("../tests/data/info.json");
        assert!(r.is_ok(), "Error: {:?}", r.err());
    }
}

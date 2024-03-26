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

#![allow(dead_code)]

use cxx::UniquePtr;
use std::fmt::Debug;

#[cxx::bridge]
mod ffi {
    #[namespace = "pdal_sys"]
    unsafe extern "C++" {
        include!("pdal-sys/src/pipeline_manager/pipeline_manager.hpp");
        type PipelineManager;
        #[namespace = "pdal_sys::point_view_set"]
        type PointViewSet = crate::point_view::PointViewSet;
        fn createPipelineManager() -> UniquePtr<PipelineManager>;
        fn readPipeline(self: Pin<&mut PipelineManager>, pipeline: &str) -> Result<()>;
        fn readPipelineFromFile(self: Pin<&mut PipelineManager>, path: &str) -> Result<()>;
        fn pipelineStreamable(self: &PipelineManager) -> bool;
        fn execute(self: Pin<&mut PipelineManager>) -> Result<usize>;
        fn executeStreamed(self: Pin<&mut PipelineManager>) -> Result<()>;
        fn views(self: &PipelineManager) -> Result<&PointViewSet>;
        fn metadata(self: &PipelineManager) -> Result<String>;
        fn schema(self: &PipelineManager) -> Result<String>;
        fn pipeline(self: &PipelineManager) -> Result<String>;
    }
}
pub use ffi::{createPipelineManager, PipelineManager};

impl PipelineManager {
    pub fn new() -> UniquePtr<ffi::PipelineManager> {
        createPipelineManager()
    }
}
pub type PipelineManagerPtr = UniquePtr<PipelineManager>;

impl Debug for PipelineManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipelineManager").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::ffi::createPipelineManager;
    use crate::testkit::*;

    #[test]
    fn test_parse_pipeline() {
        let good =
            r#"{ "pipeline": [ {"filename":"foobar.las", "spatialreference":"EPSG:2993" } ] }"#;
        let bad = r#"{"pipeline":[{"blah":"foobar.las"}]}"#;

        let mut mgr = createPipelineManager();
        let r = mgr.pin_mut().readPipeline(good);
        assert!(r.is_ok(), "Error: {:?}", r.err());

        let mut mgr = createPipelineManager();

        let r = mgr.pin_mut().readPipeline(bad);
        assert!(r.is_err());
        assert!(r.err().unwrap().to_string().contains("reader"));
    }

    #[test]
    fn test_read_pipeline() {
        std::env::set_current_dir(TEST_WD.to_path_buf()).unwrap();
        let mut mgr = createPipelineManager();
        let r = mgr
            .pin_mut()
            .readPipelineFromFile(&data_file_path("info.json"));
        assert!(r.is_ok(), "Error: {:?}", r.err());
        let r = mgr.pin_mut().execute();
        assert_eq!(r.unwrap(), 110000);
    }
}

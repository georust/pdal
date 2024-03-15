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

use crate::error::Result;
use crate::json::PdalJson;
use crate::utils::fetch_string_from_handle_with_buffer;
use crate::PointViewIter;

use pdal_sys::PDALPipelineIsStreamable;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Pipeline(pdal_sys::PDALPipelinePtr);

impl Pipeline {
    pub fn as_ptr(&self) -> pdal_sys::PDALPipelinePtr {
        self.0
    }

    /// Construct a new pipeline.
    pub fn new<J: Into<PdalJson>>(pdal_json: J) -> Result<Self> {
        let json: PdalJson = pdal_json.try_into()?;
        let pipeline = unsafe { pdal_sys::PDALCreatePipeline(json.as_ptr()) };
        if pipeline.is_null() {
            return Err("PDAL pipeline creation failed".into());
        } else {
            Ok(Self(pipeline))
        }
    }

    /// Determine if the pipeline is streamable.
    pub fn is_streamable(&self) -> bool {
        unsafe { PDALPipelineIsStreamable(self.as_ptr()) }
    }

    /// Execute the pipeline.
    ///
    /// Returns the number of points produced.
    pub fn execute(self) -> Result<ExecutedPipeline> {
        let p_count = unsafe { pdal_sys::PDALExecutePipeline(self.as_ptr()) };
        if p_count <= 0 {
            return Err("PDAL pipeline execution failed".into());
        }
        Ok(ExecutedPipeline::new(self, p_count as usize))
    }

    /// Executes a pipeline as a streamable pipeline. Will run as non-streamed pipeline if the pipeline is not streamable.
    ///
    /// Note: number of points produced is not available when run in streamed mode.
    pub fn execute_streamed(self) -> Result<ExecutedPipeline> {
        let success = unsafe { pdal_sys::PDALExecutePipelineAsStream(self.as_ptr()) };
        if !success {
            return Err("PDAL pipeline execution failed".into());
        }
        Ok(ExecutedPipeline::new(self, 0))
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe { pdal_sys::PDALDisposePipeline(self.as_ptr()) }
    }
}

#[derive(Debug)]
pub struct ExecutedPipeline {
    pipeline: Pipeline,
    points: usize,
}

impl ExecutedPipeline {
    pub(crate) fn new(pipeline: Pipeline, points: usize) -> Self {
        Self { pipeline, points }
    }

    /// Get the number of points produced by the pipeline.
    pub fn point_count(&self) -> usize {
        self.points
    }

    /// Get an iterator over the point views produced by the pipeline.
    pub fn point_views(&self) -> Result<PointViewIter> {
        let iter_handle = unsafe { pdal_sys::PDALGetPointViews(self.pipeline.as_ptr()) };
        if iter_handle.is_null() {
            return Err("Failed to get point view iterator".into());
        }
        Ok(PointViewIter::new(iter_handle))
    }

    /// Retrieves a pipeline's computed metadata
    pub fn metadata(&self) -> Result<String> {
        let s = fetch_string_from_handle_with_buffer::<8096, _>(
            self.pipeline.as_ptr(),
            pdal_sys::PDALGetPipelineMetadata,
        )?;
        if s.is_empty() {
            return Err("Unable to extract metadata".into());
        }
        Ok(s.to_string_lossy().into_owned())
    }

    /// Retrieves the full json string representation of an execute pipeline
    pub fn pipeline_json(&self) -> Result<String> {
        let s = fetch_string_from_handle_with_buffer::<8096, _>(
            self.pipeline.as_ptr(),
            pdal_sys::PDALGetPipelineAsString,
        )?;
        if s.is_empty() {
            return Err("Unable to extract pipeline JSON".into());
        }
        Ok(s.to_string_lossy().into_owned())
    }

    /// Retrieves a pipeline's computed schema.
    pub fn schema(&self) -> Result<String> {
        let s = fetch_string_from_handle_with_buffer::<8096, _>(
            self.pipeline.as_ptr(),
            pdal_sys::PDALGetPipelineSchema,
        )?;
        if s.is_empty() {
            return Err("Unable to extract pipeline results schema".into());
        }
        Ok(s.to_string_lossy().into_owned())
    }
}

impl Display for ExecutedPipeline {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pipeline_json().map_err(|_| std::fmt::Error)?)
    }
}

#[cfg(test)]
mod test {
    use crate::testkit::{read_test_file, TestResult};
    use crate::Pipeline;
    #[test]
    fn test_validate_pipeline() -> TestResult {
        let json = read_test_file("invalid.json");
        let pipeline = Pipeline::new(json);
        assert!(pipeline.is_err());

        let json = read_test_file("stats.json");
        let pipeline = Pipeline::new(json);
        assert!(pipeline.is_ok());

        pipeline.unwrap().is_streamable();
        Ok(())
    }

    #[test]
    fn test_pipeline_execution() -> TestResult {
        let json = read_test_file("stats.json");
        let pipeline = Pipeline::new(json)?;
        let result = pipeline.execute()?;
        assert_eq!(result.point_count(), 110000);
        Ok(())
    }

    #[test]
    fn test_pipeline_metadata() -> TestResult {
        let json = read_test_file("stats.json");
        let pipeline = Pipeline::new(json)?;
        let results = pipeline.execute()?;
        let md = results.metadata()?;
        assert!(!md.is_empty());
        assert!(md.contains("average"));
        Ok(())
    }

    #[test]
    fn test_pipeline_schema() -> TestResult {
        let json = read_test_file("stats.json");
        let pipeline = Pipeline::new(json)?;
        let results = pipeline.execute()?;
        let schema = results.schema()?;
        assert!(schema.contains("dimensions"));
        Ok(())
    }
}

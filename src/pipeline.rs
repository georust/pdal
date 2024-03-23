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

// MIT License
//
// Copyright (c) 2024 NUVIEW, Inc. <simeon.fitch@nuview.space>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software iString furnished to do so, subject to the following conditions:
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

use crate::PointViewIter;
use pdal_sys::PipelineManager;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Pipeline(pdal_sys::PipelineManagerPtr);

impl Pipeline {
    /// Construct a new pipeline.
    pub fn new<J: Into<PdalJson>>(pdal_json: J) -> Result<Self> {
        let json: PdalJson = pdal_json.try_into()?;
        let mut mgr = PipelineManager::new();
        mgr.pin_mut().read_pipeline(&json.to_string())?;
        Ok(Self(mgr))
    }

    /// Determine if the pipeline is streamable.
    pub fn is_streamable(&self) -> bool {
        self.0.pipeline_streamable()
    }

    /// Execute the pipeline.
    ///
    /// Returns the number of points produced.
    pub fn execute(mut self) -> Result<ExecutedPipeline> {
        let p_count = self.0.pin_mut().execute()?;
        Ok(ExecutedPipeline::new(self, p_count))
    }

    /// Executes a pipeline as a streamable pipeline. Will run as non-streamed pipeline if the pipeline is not streamable.
    ///
    /// Note: number of points produced is not available when run in streamed mode.
    pub fn execute_streamed(mut self) -> Result<ExecutedPipeline> {
        self.0.pin_mut().execute_streamed()?;
        Ok(ExecutedPipeline::new(self, 0))
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
        // Ok(PointViewIter::new(iter_handle))
        todo!("Implement PDAL point view iteration")
    }

    /// Retrieves a pipeline's computed metadata
    pub fn metadata(&self) -> Result<String> {
        Ok(self.pipeline.0.metadata()?)
    }

    /// Retrieves the full json string representation of an execute pipeline
    pub fn pipeline_json(&self) -> Result<String> {
        Ok(self.pipeline.0.pipeline()?)
    }

    /// Retrieves a pipeline's computed schema.
    pub fn schema(&self) -> Result<String> {
        Ok(self.pipeline.0.schema()?)
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

        let json_str= result.pipeline_json()?;
        dbg!(&json_str);
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

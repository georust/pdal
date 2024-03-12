use crate::error::Result;
use crate::json::PdalJson;
use crate::utils::fetch_string_from_handle_with_buffer;
use crate::PointViewIter;

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Pipeline(pdal_sys::PDALPipelinePtr);

impl Pipeline {
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

    pub(crate) fn as_ptr(&self) -> pdal_sys::PDALPipelinePtr {
        self.0
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

    pub fn point_views(&self) -> Result<PointViewIter> {
        let iter_handle = unsafe { pdal_sys::PDALGetPointViews(self.pipeline.as_ptr()) };
        if iter_handle.is_null() {
            return Err("Failed to get point view iterator".into());
        }
        Ok(PointViewIter::new(iter_handle))
    }

    pub fn metadata(&self) -> Result<String> {
        let s = fetch_string_from_handle_with_buffer::<8096, _>(
            self.pipeline.as_ptr(),
            pdal_sys::PDALGetPipelineMetadata,
        )?;
        if s.is_empty() {
            return Err("Pipeline must be executed to access metadata".into());
        }
        Ok(s.to_string_lossy().into_owned())
    }

    pub fn pipeline_json(&self) -> Result<String> {
        let s = fetch_string_from_handle_with_buffer::<8096, _>(
            self.pipeline.as_ptr(),
            pdal_sys::PDALGetPipelineAsString,
        )?;
        if s.is_empty() {
            return Err("Pipeline must be executed before rendered to JSON".into());
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
        let json = read_test_file("stats.json");
        let pipeline = Pipeline::new(json);
        assert!(pipeline.is_ok());

        let json = read_test_file("invalid.json");
        let pipeline = Pipeline::new(json);
        assert!(pipeline.is_err());
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
}

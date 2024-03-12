use crate::error::Result;
use crate::json::PdalJson;
use crate::utils::fetch_string_from_handle_with_buffer;
use pdal_sys::{PDALCreatePipeline, PDALDisposePipeline, PDALGetPipelineAsString, PDALPipelinePtr};

pub struct Pipeline(PDALPipelinePtr);

impl Pipeline {
    pub fn new<J: Into<PdalJson>>(pdal_json: J) -> Result<Self> {
        let json: PdalJson = pdal_json.try_into()?;
        let pipeline = unsafe { PDALCreatePipeline(json.as_ptr()) };
        if pipeline.is_null() {
            return Err("PDAL pipeline creation failed".into());
        } else {
            Ok(Self(pipeline))
        }
    }

    pub fn execute(&mut self) -> Result<usize> {
        let p_count = unsafe { pdal_sys::PDALExecutePipeline(self.0) };
        if p_count <= 0 {
            return Err("PDAL pipeline execution failed".into());
        }
        Ok(p_count as usize)
    }

    pub fn pipeline_json(&self) -> Result<String> {
        let s = fetch_string_from_handle_with_buffer::<8096>(self.0, PDALGetPipelineAsString)?;
        if s.is_empty() {
            return Err("Pipeline must be executed before rendered to JSON".into());
        }
        Ok(s.to_string_lossy().into_owned())
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe { PDALDisposePipeline(self.0) }
    }
}

#[cfg(test)]
mod test {
    use crate::error::Result;
    use crate::testkit::read_test_file;
    #[test]
    fn test_pipeline() -> Result<()> {
        let json = read_test_file("stats.json");
        let mut pipeline = super::Pipeline::new(json)?;
        let points = pipeline.execute()?;
        assert_eq!(points, 1065);
        println!("{}\n{}", points, pipeline.pipeline_json()?);
        Ok(())
    }
}

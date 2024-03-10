#[cfg(test)]
mod tests {
    #[test]
    fn simple_pipeline() {
        let pipeline = crate::ffi::pdal::PipelineExecutor::new();
        assert_eq!(pipeline.run(), Ok(()));
    }
}

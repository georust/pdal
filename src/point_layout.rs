use crate::error::Result;
use crate::DimensionTypeList;
use std::ffi::CString;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct PointLayout<'a>(pdal_sys::PDALPointLayoutPtr, PhantomData<&'a ()>);

impl<'a> PointLayout<'a> {
    pub(crate) fn new(handle: pdal_sys::PDALPointLayoutPtr) -> Self {
        // The layout is owned by the point view, so we don't need to free it, but use
        // a phantom lifetime to ensure that the layout cannot outlive the point view.
        Self(handle, PhantomData::<&'a ()>)
    }

    pub fn as_ptr(&self) -> pdal_sys::PDALPointLayoutPtr {
        self.0
    }

    /// Get the size in bytes of a point in this layout.
    pub fn point_size(&self) -> usize {
        let s = unsafe { pdal_sys::PDALGetPointSize(self.as_ptr()) };
        s as usize
    }

    /// Returns the list of dimension types used by the layout.
    pub fn dimension_types(&self) -> Result<DimensionTypeList> {
        let handle = unsafe { pdal_sys::PDALGetPointLayoutDimTypes(self.as_ptr()) };
        Ok(DimensionTypeList::new(handle))
    }

    /// Get the size in bytes of the given dimension in this layout.
    pub fn dimension_size(&self, name: &str) -> Result<usize> {
        let name = CString::new(name)?;
        let s = unsafe { pdal_sys::PDALGetDimSize(self.as_ptr(), name.as_ptr()) };
        if s <= 0 {
            return Err(format!("dimension size for {name:?} not found").into());
        }
        Ok(s as usize)
    }
}

#[cfg(test)]
mod tests {
    use crate::testkit::{read_test_file, TestResult};

    #[test]
    fn test_get_layout() -> TestResult {
        let json = read_test_file("stats.json");
        let pipeline = crate::Pipeline::new(json)?;
        let result = pipeline.execute()?;
        let view = result.point_views()?.next().ok_or("no point view")?;
        let layout = view.layout()?;
        assert_eq!(layout.point_size(), 56);
        Ok(())
    }
}

use crate::error::Result;
use crate::utils::{fetch_string_from_handle_with_buffer, Conv};
use crate::PointLayout;
use pdal_sys::size_t;

#[derive(Debug)]
pub struct PointView(pdal_sys::PDALPointViewPtr);

impl PointView {
    pub(crate) fn new(handle: pdal_sys::PDALPointViewPtr) -> Self {
        Self(handle)
    }
    pub fn as_ptr(&self) -> pdal_sys::PDALPointViewPtr {
        self.0
    }

    /// Get the point view ID.
    pub fn id(&self) -> i32 {
        unsafe { pdal_sys::PDALGetPointViewId(self.as_ptr()) }
    }

    /// Determine if the point view is empty.
    pub fn is_empty(&self) -> bool {
        unsafe { pdal_sys::PDALIsPointViewEmpty(self.as_ptr()) }
    }

    /// Get the number of points in the view.
    pub fn point_count(&self) -> usize {
        unsafe { pdal_sys::PDALGetPointViewSize(self.as_ptr()) as usize }
    }

    /// Get the CRS as a Proj4 string.
    pub fn proj4(&self) -> Result<String> {
        let s = fetch_string_from_handle_with_buffer::<1024, _>(
            self.as_ptr(),
            pdal_sys::PDALGetPointViewProj4,
        )?;
        Ok(Conv(s).try_into()?)
    }

    /// Get the CRS as a Proj4 string.
    pub fn wkt(&self) -> Result<String> {
        extern "C" fn pretty_wkt(
            handle: pdal_sys::PDALPointViewPtr,
            buf: *mut ::std::ffi::c_char,
            size: size_t,
        ) -> size_t {
            unsafe { pdal_sys::PDALGetPointViewWkt(handle, buf, size, true) }
        }

        let s = fetch_string_from_handle_with_buffer::<1024, _>(self.as_ptr(), pretty_wkt)?;
        Ok(Conv(s).try_into()?)
    }

    /// Get the point view layout.
    pub fn layout(&self) -> Result<PointLayout> {
        let layout = unsafe { pdal_sys::PDALGetPointViewLayout(self.as_ptr()) };
        if layout.is_null() {
            return Err("PDAL point view layout retrieval failed".into());
        }
        Ok(PointLayout::new(layout))
    }

    // TODO:
    // - PDALGetPackedPoint
    // - PDALGetAllPackedPoints
    // - PDALGetMeshSize
    // - PDALGetAllTriangles
}

impl Drop for PointView {
    fn drop(&mut self) {
        unsafe { pdal_sys::PDALDisposePointView(self.as_ptr()) }
    }
}

impl Clone for PointView {
    fn clone(&self) -> Self {
        let handle = unsafe { pdal_sys::PDALClonePointView(self.as_ptr()) };
        Self::new(handle)
    }
}

/// Iterator over point views.
pub struct PointViewIter(pdal_sys::PDALPointViewIteratorPtr);

impl PointViewIter {
    pub(crate) fn new(handle: pdal_sys::PDALPointViewIteratorPtr) -> Self {
        Self(handle)
    }

    fn has_next(&self) -> bool {
        unsafe { pdal_sys::PDALHasNextPointView(self.as_ptr()) }
    }

    fn next(&mut self) -> pdal_sys::PDALPointViewPtr {
        unsafe { pdal_sys::PDALGetNextPointView(self.as_ptr()) }
    }

    // why?
    // fn reset(&mut self) -> () {
    //     unsafe { pdal_sys::PDALResetPointViewIterator(self.as_ptr()) }
    // }

    pub fn as_ptr(&self) -> pdal_sys::PDALPointViewIteratorPtr {
        self.0
    }
}

impl Drop for PointViewIter {
    fn drop(&mut self) {
        unsafe { pdal_sys::PDALDisposePointViewIterator(self.as_ptr()) }
    }
}

impl Iterator for PointViewIter {
    type Item = PointView;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_next() {
            return None;
        }
        let next = self.next();
        if next.is_null() {
            None
        } else {
            Some(PointView::new(next))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testkit::{read_test_file, TestResult};
    use crate::Pipeline;

    #[test]
    fn test_iterator() -> TestResult {
        let json = read_test_file("stats.json");
        let pipeline = Pipeline::new(json)?;
        let result = pipeline.execute()?;

        let views = result.point_views()?.collect::<Vec<_>>();
        assert_eq!(views.len(), 1);
        let view = &views[0];
        assert_eq!(view.point_count(), 110000);
        assert_eq!(view.proj4()?, "+proj=lcc +lat_0=41.75 +lon_0=-120.5 +lat_1=43 +lat_2=45.5 +x_0=400000 +y_0=0 +ellps=GRS80 +units=ft +no_defs");
        assert!(view
            .wkt()?
            .contains("NAD_1983_HARN_Lambert_Conformal_Conic"));
        Ok(())
    }
}

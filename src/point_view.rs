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
use crate::utils::{fetch_string_from_handle_with_buffer, Conv, Elided};
use crate::{DimensionTypeList, PointLayout};
use pdal_sys::size_t;
use std::ffi::c_char;
use std::fmt::{Debug, Formatter};

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
            buf: *mut c_char,
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

    /// Retrieves data for a point based on the provided dimension list.
    pub fn get_packed_point(&self, dims: &DimensionTypeList, index: usize) -> Result<PackedPoint> {
        let layout = self.layout()?;
        let p_size = layout.point_size();
        let mut buf = vec![u8::default(); p_size];

        let size = unsafe {
            pdal_sys::PDALGetPackedPoint(
                self.as_ptr(),
                dims.as_ptr(),
                index as size_t,
                buf.as_mut_ptr() as *mut c_char,
            ) as usize
        };

        dbg!(&buf);

        if size != p_size {
            return Err("Expected point size of '{}'; got '{}'".into());
        }

        Ok(PackedPoint(buf))
    }

    // TODO:
    // - PDALGetAllPackedPoints
    // - PDALGetMeshSize
    // - PDALGetAllTriangles
}

impl Drop for PointView {
    fn drop(&mut self) {
        unsafe { pdal_sys::PDALDisposePointView(self.as_ptr()) }
    }
}

// TODO: This doesn't actually clone the points, so need to understand semantics better.
// impl Clone for PointView {
//     fn clone(&self) -> Self {
//         let handle = unsafe { pdal_sys::PDALClonePointView(self.as_ptr()) };
//         Self::new(handle)
//     }
// }

impl Debug for PointView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PointView")
            .field("id", &self.id())
            .field("point_count", &self.point_count())
            .field("proj4", &Elided(&self.proj4().unwrap_or_default()))
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct PackedPoint(Vec<u8>);

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
    use crate::{error::Result, ExecutedPipeline, Pipeline, PointView};

    #[test]
    fn test_lifetime() {
        let json = read_test_file("copy.json");
        let pipeline = Pipeline::new(json)?;
        let result = pipeline.execute()?;
    }

    fn fixture() -> Result<ExecutedPipeline> {
        let json = read_test_file("copy.json");
        let pipeline = Pipeline::new(json)?;
        pipeline.execute()
    }

    fn read_view() -> Result<PointView> {
        let result = fixture()?;
        let view = result.point_views()?.next().ok_or("no point view")?;
        Ok(view)
    }

    #[test]
    fn test_iterator() -> TestResult {
        let view = read_view()?;
        assert_eq!(view.point_count(), 110000);
        assert_eq!(view.proj4()?, "+proj=lcc +lat_0=41.75 +lon_0=-120.5 +lat_1=43 +lat_2=45.5 +x_0=400000 +y_0=0 +ellps=GRS80 +units=ft +no_defs");
        assert!(view
            .wkt()?
            .contains("NAD_1983_HARN_Lambert_Conformal_Conic"));
        Ok(())
    }

    #[test]
    fn test_packed_point() -> TestResult {
        let view = read_view()?;
        dbg!(&view);
        let layout = view.layout()?;
        dbg!(&layout);
        let dims = layout.dimension_types()?;
        let point = view.get_packed_point(&dims, 0)?;
        assert_eq!(point.0.len(), 56);
        Ok(())
    }
}

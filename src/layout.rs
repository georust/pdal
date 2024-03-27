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
// use crate::{DimensionType, DimensionTypeList};
use std::fmt::Debug;

/// Point layout definition
pub struct PointLayout<'pv>(pub(crate) &'pv pdal_sys::layout::PointLayout);

impl<'a> PointLayout<'a> {
    /// Get the size in bytes of a point in this layout.
    pub fn point_size(&self) -> usize {
        self.0.pointSize()
    }

    // /// Returns the list of dimension types used by the layout.
    // pub fn dimension_types(&self) -> Result<DimensionTypeList> {
    //     todo!("dimension_types")
    // }
    //
    // /// Lookup the dimension type by name.
    // ///
    // /// Returns Ok(DimensionType::invalid()) if a dimension with given name is not found.
    // pub fn dimension(&self, name: &str) -> Result<DimensionType> {
    //     todo!("dimension")
    // }

    /// Get the size in bytes of the given dimension in this layout.
    pub fn dimension_size(&self, name: &str) -> Result<usize> {
        todo!("dimension_size")
    }

    /// Returns the byte offset of a dimension type with the given name.
    pub fn dimension_offset(&self, name: &str) -> Result<usize> {
        todo!("dimension_offset")
    }
}

impl Debug for PointLayout<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PointLayout")
            .field("point_size", &self.point_size())
            // .field(
            //     "dimension_types",
            //     &self.dimension_types().map_err(|_| std::fmt::Error)?,
            // )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::testkit::{read_test_file, TestResult};

    #[test]
    fn test_layout() -> TestResult {
        let json = read_test_file("stats.json");
        let pipeline = crate::Pipeline::new(json)?;
        let result = pipeline.execute()?;
        let views = result.point_views()?;
        let view = views.first().ok_or("no point view")?;
        let layout = view.layout()?;

        assert_eq!(layout.point_size(), 56);
        //        assert_eq!(layout.dimension("X")?.interpretation()?, "double");
        Ok(())
    }
}

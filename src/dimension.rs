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

/// Dimension type identifier. E.g. X, Y, Z, Red, Green, Blue, etc.
pub type DimTypeId = pdal_sys::core::DimTypeId;

/// How the dimension is encoded. E.g. `Unsiged8`, `Signed64`, `Double`, etc.
pub type DimTypeEncoding = pdal_sys::core::DimTypeEncoding;

/// A dimension in a point layout.
pub struct LayoutDimension<'view>(
    pub(crate) &'view crate::PointLayout<'view>,
    pub(crate) DimTypeId,
);

impl<'view> LayoutDimension<'view> {
    pub fn id(&self) -> DimTypeId {
        self.1
    }

    /// Size of the dimension in bytes.
    pub fn size_bytes(&self) -> usize {
        self.0.dimension_size(self.1)
    }

    pub fn offset(&self) -> usize {
        self.0.dimension_offset(self.1)
    }

    pub fn encoding(&self) -> DimTypeEncoding {
        self.0
             .0
            .dim_types()
            .find(|dt| dt.id() == self.1)
            .expect("dimension exists in parent layout")
            .encoding()
    }
}

#[cfg(test)]
mod tests {
    use crate::testkit::{read_test_file, TestResult};
    use crate::DimTypeId;

    #[test]
    fn test_get_layout() -> TestResult {
        let json = read_test_file("stats.json");
        let pipeline = crate::Pipeline::new(json)?;
        let result = pipeline.execute()?;
        let views = result.point_views()?;
        let view = views.first().ok_or("no point view")?;
        let layout = view.layout()?;
        assert_eq!(layout.dimension_count(), 20);

        let dim = layout
            .dimension_types()
            .find(|dt| dt.id() == DimTypeId::Blue)
            .ok_or("Blue dimension not found")?;

        assert_eq!(dim.encoding(), crate::DimTypeEncoding::Unsigned16);

        Ok(())
    }
}

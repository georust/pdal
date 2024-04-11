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

use crate::{DimTypeId, LayoutDimension};
use std::fmt::Debug;

/// Point layout definition, describing the schema the points in a view.
pub struct PointLayout<'pv>(pub(crate) &'pv pdal_sys::layout::PointLayout);

impl<'a> PointLayout<'a> {
    /// Get the size in bytes of a point in this layout.
    pub fn point_size(&self) -> usize {
        self.0.point_size()
    }

    pub fn dimension_count(&self) -> usize {
        self.0.dim_ids().count()
    }

    /// Returns the sequence of dimension identifiers used by the layout.
    pub fn dimension_ids(&self) -> impl Iterator<Item = DimTypeId> {
        self.0.dim_ids()
    }

    /// Returns the sequence of dimension types used by the layout.
    pub fn dimension_types(&self) -> impl Iterator<Item = LayoutDimension> {
        self.0.dim_ids().map(|id| LayoutDimension(self, id))
    }

    /// Lookup the dimension type by identifier.
    ///
    /// Returns `None` if identifier is not found.
    pub fn dimension_type(&self, id: DimTypeId) -> Option<LayoutDimension> {
        self.dimension_types().find(|dt| dt.id() == id)
    }

    /// Get the size in bytes of the given dimension in this layout.
    pub fn dimension_size(&self, id: DimTypeId) -> usize {
        self.0.dimSize(id)
    }

    /// Returns the byte offset of a dimension type with the given name.
    pub fn dimension_offset(&self, id: DimTypeId) -> usize {
        self.0.dimOffset(id)
    }
}

impl Debug for PointLayout<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dims = self.dimension_ids().collect::<Vec<_>>();
        f.debug_struct("PointLayout")
            .field("point_size", &self.point_size())
            .field("dimensions", &format_args!("{:?}", &dims))
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use crate::testkit::{read_test_file, TestResult};
    use crate::DimTypeId;

    #[test]
    fn test_layout() -> TestResult {
        let json = read_test_file("stats.json");
        let pipeline = crate::Pipeline::new(json)?;
        let result = pipeline.execute()?;
        let views = result.point_views()?;
        let view = views.first().ok_or("no point view")?;
        let layout = view.layout()?;

        assert_eq!(layout.point_size(), 56);
        assert_eq!(layout.dimension_offset(DimTypeId::X), 0);
        assert_eq!(layout.dimension_size(DimTypeId::X), 8);
        assert_eq!(layout.dimension_offset(DimTypeId::Y), 1);

        Ok(())
    }
}

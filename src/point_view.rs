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
use crate::utils::Elided;
use crate::{DimTypeId, PointLayout};
use std::fmt::{Debug, Formatter};

pub type PointId = pdal_sys::core::PointId;

pub use pdal_sys::core::PdalType;
pub use pdal_sys::core::PdalValue;

/// A point view is a collection of points with a common layout.
pub struct PointView(pub(crate) pdal_sys::point_view::PointViewPtr);

impl PointView {
    /// Point view ID.
    pub fn id(&self) -> i32 {
        self.0.id()
    }

    /// Number of points in the view.
    pub fn len(&self) -> usize {
        self.0.len() as usize
    }

    /// Iterator over the valid point IDs
    pub fn point_ids(&self) -> impl Iterator<Item = PointId> {
        (0..self.len()).map(|i| i as PointId)
    }

    /// Determine if the point view is empty
    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    /// Get the CRS as a Proj4 string.
    pub fn proj4(&self) -> Result<String> {
        Ok(self.0.proj4()?)
    }

    /// Get the CRS as a WKT string.
    pub fn wkt(&self) -> Result<String> {
        Ok(self.0.wkt()?)
    }

    /// Point view layout
    pub fn layout(&self) -> Result<PointLayout> {
        let pl = self.0.layout();
        Ok(PointLayout(pl))
    }

    /// Fetch the the dimension value of the point at the given index as the specified primitive type.
    pub fn point_value_as<T: PdalType>(&self, dim: DimTypeId, idx: PointId) -> Result<T> {
        Ok(self.0.point_value_as(dim, idx)?)
    }

    /// Fetch The the dimension value of the point at the given index as a wrapped [`PdalValue`].
    pub fn point_value(&self, dim: DimTypeId, idx: PointId) -> Result<PdalValue> {
        Ok(self.0.point_value(dim, idx)?)
    }
}

impl Debug for PointView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PointView")
            .field("id", &self.id())
            .field("len", &self.len())
            .field("proj4", &Elided(&self.proj4().unwrap_or_default()))
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use crate::testkit::{read_test_file, TestResult};
    use crate::{error::Result, DimTypeId, ExecutedPipeline, Pipeline};
    use pdal_sys::core::PdalValue;

    fn fixture() -> Result<ExecutedPipeline> {
        let json = read_test_file("copy.json");
        let pipeline = Pipeline::new(json)?;
        pipeline.execute()
    }

    #[test]
    fn test_iterator() -> TestResult {
        let result = fixture()?;
        let views = result.point_views()?;
        let view = views.first().ok_or("no point view")?;
        assert_eq!(view.len(), 110000);
        assert_eq!(view.proj4()?, "+proj=lcc +lat_0=41.75 +lon_0=-120.5 +lat_1=43 +lat_2=45.5 +x_0=400000 +y_0=0 +ellps=GRS80 +units=ft +no_defs");
        assert!(view
            .wkt()?
            .contains("NAD_1983_HARN_Lambert_Conformal_Conic"));
        Ok(())
    }

    #[test]
    fn test_point_values() -> TestResult {
        let result = fixture()?;
        let views = result.point_views()?;
        let view = views.first().ok_or("no point view")?;

        let total_intensity = view
            .point_ids()
            .filter_map(|pid| view.point_value(DimTypeId::Intensity, pid).ok())
            .map(PdalValue::to_f64)
            .sum::<f64>();

        let average_intensity = total_intensity / view.len() as f64;
        assert_eq!(average_intensity.floor(), 102.0);
        Ok(())
    }
}

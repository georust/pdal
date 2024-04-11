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

#[allow(clippy::needless_lifetimes)]
#[cxx::bridge(namespace = "pdal_sys")]
mod ffi {

    #[namespace = "pdal_sys::point_view_set"]
    unsafe extern "C++" {
        include!("pdal-sys/src/point_view/point_view.hpp");
        type PointViewSet;
        type PointViewSetIter<'pvs>;
        #[cxx_name = "size"]
        fn len(self: &PointViewSet) -> usize;
        fn iter<'pvs>(set: &'pvs PointViewSet) -> UniquePtr<PointViewSetIter<'pvs>>;
        fn hasNext(self: &PointViewSetIter) -> bool;
        fn next(self: Pin<&mut PointViewSetIter>) -> Result<SharedPtr<PointView>>;
    }

    #[namespace = "pdal_sys::point_view"]
    unsafe extern "C++" {
        type PointView;
        fn id(self: &PointView) -> i32;
        #[cxx_name = "size"]
        fn len(self: &PointView) -> u64;
        #[namespace = "pdal_sys::layout"]
        type PointLayout = crate::layout::PointLayout;
        fn layout(pv: &PointView) -> &PointLayout;
        fn proj4(pv: &PointView) -> Result<String>;
        fn wkt(pv: &PointView) -> Result<String>;
        #[namespace = "pdal_sys::core"]
        type DimTypeId = crate::core::DimTypeId;
        fn pointField_i8(pv: &PointView, dim: DimTypeId, idx: u64) -> Result<i8>;
        fn pointField_u8(pv: &PointView, dim: DimTypeId, idx: u64) -> Result<u8>;
        fn pointField_i16(pv: &PointView, dim: DimTypeId, idx: u64) -> Result<i16>;
        fn pointField_u16(pv: &PointView, dim: DimTypeId, idx: u64) -> Result<u16>;
        fn pointField_i32(pv: &PointView, dim: DimTypeId, idx: u64) -> Result<i32>;
        fn pointField_u32(pv: &PointView, dim: DimTypeId, idx: u64) -> Result<u32>;
        fn pointField_i64(pv: &PointView, dim: DimTypeId, idx: u64) -> Result<i64>;
        fn pointField_u64(pv: &PointView, dim: DimTypeId, idx: u64) -> Result<u64>;
        fn pointField_f32(pv: &PointView, dim: DimTypeId, idx: u64) -> Result<f32>;
        fn pointField_f64(pv: &PointView, dim: DimTypeId, idx: u64) -> Result<f64>;
    }

    // This triggers the generation of the C++ template backing this concrete type.
    // See: https://cxx.rs/extern-c++.html#explicit-shim-trait-impls
    impl Vec<DimTypeId> {}
}
pub use ffi::{PointView, PointViewSet, PointViewSetIter};

use crate::core::{pdal_sys_throw, DimTypeEncoding, DimTypeId, PdalType, PdalValue, PointId};
use cxx::{SharedPtr, UniquePtr};
use std::fmt::{Debug, Formatter};

pub type PointViewPtr = SharedPtr<PointView>;

impl PointView {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn proj4(&self) -> Result<String, cxx::Exception> {
        ffi::proj4(self)
    }
    #[inline]
    pub fn wkt(&self) -> Result<String, cxx::Exception> {
        ffi::wkt(self)
    }
    #[inline]
    pub fn layout(&self) -> &crate::layout::PointLayout {
        ffi::layout(self)
    }
    pub fn point_value_as<T: PdalType>(
        &self,
        dim: DimTypeId,
        idx: PointId,
    ) -> Result<T, cxx::Exception> {
        let r = match T::encoding() {
            DimTypeEncoding::Unsigned8 => T::static_cast(ffi::pointField_u8(self, dim, idx)?),
            DimTypeEncoding::Signed8 => T::static_cast(ffi::pointField_i8(self, dim, idx)?),
            DimTypeEncoding::Unsigned16 => T::static_cast(ffi::pointField_u16(self, dim, idx)?),
            DimTypeEncoding::Signed16 => T::static_cast(ffi::pointField_i16(self, dim, idx)?),
            DimTypeEncoding::Unsigned32 => T::static_cast(ffi::pointField_u32(self, dim, idx)?),
            DimTypeEncoding::Signed32 => T::static_cast(ffi::pointField_i32(self, dim, idx)?),
            DimTypeEncoding::Unsigned64 => T::static_cast(ffi::pointField_u64(self, dim, idx)?),
            DimTypeEncoding::Signed64 => T::static_cast(ffi::pointField_i64(self, dim, idx)?),
            DimTypeEncoding::Float => T::static_cast(ffi::pointField_f32(self, dim, idx)?),
            DimTypeEncoding::Double => T::static_cast(ffi::pointField_f64(self, dim, idx)?),
            // Normally we'd expect caller to use `point_value` for this case, but this rids us of an edge case.
            DimTypeEncoding::None => T::static_cast(self.point_value(dim, idx)?),
        };

        match r {
            Some(v) => Ok(v),
            None => Err(pdal_sys_throw(&format!(
                "Failed to convert value to type {:?}",
                T::encoding()
            ))
            .unwrap_err()),
        }
    }

    /// Get point dimension value as a discriminated union.
    pub fn point_value(&self, dim: DimTypeId, idx: PointId) -> Result<PdalValue, cxx::Exception> {
        match self.layout().dimEncoding(dim) {
            DimTypeEncoding::Unsigned8 => {
                Ok(PdalValue::Unsigned8(ffi::pointField_u8(self, dim, idx)?))
            }
            DimTypeEncoding::Signed8 => Ok(PdalValue::Signed8(ffi::pointField_i8(self, dim, idx)?)),
            DimTypeEncoding::Unsigned16 => {
                Ok(PdalValue::Unsigned16(ffi::pointField_u16(self, dim, idx)?))
            }
            DimTypeEncoding::Signed16 => {
                Ok(PdalValue::Signed16(ffi::pointField_i16(self, dim, idx)?))
            }
            DimTypeEncoding::Unsigned32 => {
                Ok(PdalValue::Unsigned32(ffi::pointField_u32(self, dim, idx)?))
            }
            DimTypeEncoding::Signed32 => {
                Ok(PdalValue::Signed32(ffi::pointField_i32(self, dim, idx)?))
            }
            DimTypeEncoding::Unsigned64 => {
                Ok(PdalValue::Unsigned64(ffi::pointField_u64(self, dim, idx)?))
            }
            DimTypeEncoding::Signed64 => {
                Ok(PdalValue::Signed64(ffi::pointField_i64(self, dim, idx)?))
            }
            DimTypeEncoding::Float => Ok(PdalValue::Float(ffi::pointField_f32(self, dim, idx)?)),
            DimTypeEncoding::Double => Ok(PdalValue::Double(ffi::pointField_f64(self, dim, idx)?)),
            _ => Err(pdal_sys_throw(&format!(
                "Failed to convert value to type {:?}",
                self.layout().dimEncoding(dim)
            ))
            .unwrap_err()),
        }
    }
}

impl Debug for PointView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PointView")
            .field("id", &self.id())
            .field("len", &self.len())
            .finish()
    }
}

/// Unordered set of [`PointView`] instances.
impl PointViewSet {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn iter(&self) -> impl Iterator<Item = PointViewPtr> + '_ {
        PointViewSetIterator(ffi::iter(self))
    }
}

pub struct PointViewSetIterator<'pvs>(UniquePtr<PointViewSetIter<'pvs>>);

impl Iterator for PointViewSetIterator<'_> {
    type Item = PointViewPtr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.hasNext() {
            self.0.pin_mut().next().ok()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{DimTypeId, PdalValue, PointId};
    use crate::pipeline_manager::createPipelineManager;
    use crate::testkit::*;
    use std::collections::HashMap;

    #[test]
    fn test_get_views() {
        std::env::set_current_dir(TEST_WD.to_path_buf()).unwrap();
        let mut mgr = createPipelineManager();
        mgr.pin_mut()
            .readPipelineFromFile(&data_file_path("info.json"))
            .unwrap();
        let _ = mgr.pin_mut().execute().unwrap();
        let r = mgr.views();
        assert!(r.is_ok());
        let vs = r.unwrap();
        assert_eq!(vs.len(), 1);

        let mut iter = vs.iter();
        let next = iter.next();
        assert!(next.is_some());

        let view = next.unwrap();
        assert_eq!(view.len(), 110000);
    }

    #[test]
    fn test_read_point() {
        std::env::set_current_dir(TEST_WD.to_path_buf()).unwrap();
        let mut mgr = createPipelineManager();
        mgr.pin_mut()
            .readPipelineFromFile(&data_file_path("info.json"))
            .unwrap();
        let _ = mgr.pin_mut().execute().unwrap();
        let r = mgr.views();
        let vs = r.unwrap();
        let mut iter = vs.iter();
        let next = iter.next();
        let view = next.unwrap();

        // Manually extracted point values.
        let example: PointId = 4;
        let expected = HashMap::from([
            (DimTypeId::X, PdalValue::Double(637174.33)),
            (DimTypeId::Y, PdalValue::Double(849407.37)),
            (DimTypeId::Z, PdalValue::Double(411.38)),
            (DimTypeId::Intensity, PdalValue::Unsigned16(15)),
            (DimTypeId::ReturnNumber, PdalValue::Unsigned8(1)),
            (DimTypeId::NumberOfReturns, PdalValue::Unsigned8(1)),
            (DimTypeId::ScanDirectionFlag, PdalValue::Unsigned8(1)),
            (DimTypeId::EdgeOfFlightLine, PdalValue::Unsigned8(0)),
            (DimTypeId::Classification, PdalValue::Unsigned8(1)),
            (DimTypeId::Synthetic, PdalValue::Unsigned8(0)),
            (DimTypeId::KeyPoint, PdalValue::Unsigned8(0)),
            (DimTypeId::Withheld, PdalValue::Unsigned8(0)),
            (DimTypeId::Overlap, PdalValue::Unsigned8(0)),
            (DimTypeId::ScanAngleRank, PdalValue::Float(-18.0)),
            (DimTypeId::UserData, PdalValue::Unsigned8(130)),
            (DimTypeId::PointSourceId, PdalValue::Unsigned16(7326)),
            (DimTypeId::GpsTime, PdalValue::Double(245379.4008614817)),
            (DimTypeId::Red, PdalValue::Unsigned16(78)),
            (DimTypeId::Green, PdalValue::Unsigned16(94)),
            (DimTypeId::Blue, PdalValue::Unsigned16(89)),
        ]);

        for (&dim, &expected_value) in &expected {
            let dim_val = view.point_value_as::<PdalValue>(dim, example);
            assert!(
                dim_val.is_ok(),
                "Failed to read dimension {dim:?} as PdalValue",
            );
            let pt_value = dim_val.unwrap();
            assert_eq!(
                pt_value, expected_value,
                "Unexpected value for dimension {dim:?}"
            );
        }

        assert_eq!(
            view.point_value_as::<f64>(DimTypeId::X, example)
                .expect("x coord"),
            expected[&DimTypeId::X].to_f64()
        );
    }
}

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

#![allow(dead_code)]

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

use crate::core::{pdal_sys_throw, DimTypeEncoding, DimTypeId, PdalType, PointId};
use cxx::{SharedPtr, UniquePtr};
use std::fmt::{Debug, Formatter};

pub type PointViewPtr = SharedPtr<PointView>;

impl PointView {
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
            _ => None,
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
}

impl Debug for PointView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PointView")
            .field("id", &self.id())
            .field("len", &self.len())
            .finish()
    }
}

impl PointViewSet {
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
    use crate::core::{DimTypeId, PointId};
    use crate::pipeline_manager::createPipelineManager;
    use crate::testkit::*;

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

        let dims = [
            DimTypeId::X,
            DimTypeId::Y,
            DimTypeId::Z,
            DimTypeId::Intensity,
            DimTypeId::ReturnNumber,
            DimTypeId::NumberOfReturns,
            DimTypeId::ScanDirectionFlag,
            DimTypeId::EdgeOfFlightLine,
            DimTypeId::Classification,
            DimTypeId::Synthetic,
            DimTypeId::KeyPoint,
            DimTypeId::Withheld,
            DimTypeId::Overlap,
            DimTypeId::ScanAngleRank,
            DimTypeId::UserData,
            DimTypeId::PointSourceId,
            DimTypeId::GpsTime,
            DimTypeId::Red,
            DimTypeId::Green,
            DimTypeId::Blue,
        ];

        let example: PointId = 4;

        for d in dims {
            let dim_val = view.point_value_as::<f64>(d, example);
            assert!(dim_val.is_ok());
            let dim_val = dim_val.unwrap();
            let typ = view.layout().dimEncoding(d);
            println!("Dim: {:?}, Value: {:?}, Type: {:?}", d, dim_val, typ);
        }
        // 637174.330,849407.370,411.380,15.000,1.000,1.000,1.000,0.000,1.000,0.000,0.000,0.000,0.000,-18.000,130.000,7326.000,245379.401,78.000,94.000,89.000
    }
}

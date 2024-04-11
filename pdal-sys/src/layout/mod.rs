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

#[allow(clippy::needless_lifetimes)]
#[cxx::bridge(namespace = "pdal_sys")]
mod ffi {
    #[namespace = "pdal_sys::layout"]
    unsafe extern "C++" {
        include!("pdal-sys/src/layout/layout.hpp");
        include!("pdal-sys/src/core/core.hpp");
        type PointLayout;
        #[cxx_name = "pointSize"]
        fn point_size(self: &PointLayout) -> usize;
        fn dimensionCount(pl: &PointLayout) -> usize;
        #[namespace = "pdal_sys::core"]
        type DimType = crate::core::DimType;
        #[namespace = "pdal_sys::core"]
        type DimTypeIter<'a> = crate::core::DimTypeIter<'a>;
        fn dimTypes<'pl>(pl: &'pl PointLayout) -> UniquePtr<DimTypeIter<'pl>>;

        #[namespace = "pdal_sys::core"]
        type DimTypeId = crate::core::DimTypeId;
        #[namespace = "pdal_sys::core"]
        type DimTypeEncoding = crate::core::DimTypeEncoding;
        fn dimOffset(self: &PointLayout, id: DimTypeId) -> usize;
        fn dimSize(self: &PointLayout, id: DimTypeId) -> usize;
        #[cxx_name = "dimType"]
        fn dimEncoding(self: &PointLayout, id: DimTypeId) -> DimTypeEncoding;

        #[namespace = "pdal_sys::core"]
        type DimIdIter = crate::core::DimIdIter;
        fn dimIds(pl: &PointLayout) -> UniquePtr<DimIdIter>;
    }
}
pub use ffi::PointLayout;

use crate::core::*;
use std::fmt::{Debug, Formatter};

impl PointLayout {
    #[inline]
    pub fn dim_types(&self) -> DimTypeIterator {
        DimTypeIterator::new(ffi::dimTypes(self))
    }

    #[inline]
    pub fn dim_ids(&self) -> DimIdIterator {
        DimIdIterator(ffi::dimIds(self))
    }
}

impl Debug for PointLayout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PointLayout")
            .field("point_size", &self.point_size())
            .field("dimensions", &self.dim_types().collect::<Vec<_>>())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
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
        let vs = r.unwrap();
        let mut iter = vs.iter();
        let next = iter.next();
        let view = next.unwrap();
        let layout = view.layout();
        assert_eq!(
            layout.point_size(),
            layout
                .dim_types()
                .map(|dt| dt.encoding().size_bytes())
                .sum()
        );
    }
}

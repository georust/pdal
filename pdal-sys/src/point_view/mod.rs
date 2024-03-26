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

#[cxx::bridge]
mod ffi {
    #[namespace = "pdal_sys::point_view_set"]
    unsafe extern "C++" {
        include!("pdal-sys/src/point_view/point_view.hpp");
        type PointViewSet;
        #[cxx_name = "size"]
        fn len(self: &PointViewSet) -> usize;
        fn iter(set: &PointViewSet) -> UniquePtr<PointViewIter>;
    }

    #[namespace = "pdal_sys::point_view_iter"]
    unsafe extern "C++" {
        type PointViewIter;
        fn hasNext(self: &PointViewIter) -> bool;
        fn next(self: Pin<&mut PointViewIter>) -> Result<SharedPtr<PointView>>;
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
    }
}

pub use ffi::*;
use std::fmt::{Debug, Formatter};

impl Debug for PointView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PointView")
            .field("id", &self.id())
            .field("len", &self.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::testkit::*;
    use crate::{createPipelineManager, layout};

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

        let mut iter = crate::point_view::ffi::iter(&vs);
        assert!(iter.hasNext());

        let view = iter.pin_mut().next().unwrap();
        assert_eq!(view.id(), 1);

        assert_eq!(layout(&view).pointSize(), 56);
    }
}

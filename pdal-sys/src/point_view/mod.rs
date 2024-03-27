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
    }
}

use cxx::{SharedPtr, UniquePtr};
pub use ffi::{PointView, PointViewSet, PointViewSetIter};
use std::fmt::{Debug, Formatter};

pub type PointViewPtr = SharedPtr<PointView>;

impl PointView {
    pub fn proj4(&self) -> Result<String, cxx::Exception> {
        ffi::proj4(self)
    }
    pub fn wkt(&self) -> Result<String, cxx::Exception> {
        ffi::wkt(self)
    }
    pub fn layout(&self) -> &ffi::PointLayout {
        ffi::layout(self)
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

        //assert_eq!(super::ffi::layout(&view).pointSize(), 56);
    }
}

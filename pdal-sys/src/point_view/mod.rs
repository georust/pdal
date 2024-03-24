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
    #[namespace = "pdal_sys"]
    unsafe extern "C++" {
        include!("pdal-sys/src/point_view/point_view.hpp");
        type PointViewSet;
        type PointViewIter;
        type PointView;
        fn len(set: &PointViewSet) -> usize;
        fn iter(set: &PointViewSet) -> UniquePtr<PointViewIter>;
        fn hasNext(self: &PointViewIter) -> bool;
        fn next(self: Pin<&mut PointViewIter>) -> Result<SharedPtr<PointView>>;
    }
}

pub use ffi::*;

// pub struct PointViewIter<'pipelne>(&'pipelne PointViewSet, usize);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_pipeline_manager;
    use crate::testkit::*;
    use std::any::Any;

    #[test]
    fn test_get_views() {
        std::env::set_current_dir(TEST_WD.to_path_buf()).unwrap();
        let mut mgr = create_pipeline_manager();
        mgr.pin_mut()
            .read_pipeline_from_file(&data_file_path("info.json"))
            .unwrap();
        let _ = mgr.pin_mut().execute().unwrap();
        let r = mgr.views();
        assert!(r.is_ok());
        let vs = r.unwrap();
        assert_eq!(crate::point_view::ffi::len(vs), 1);

        let mut iter = crate::point_view::ffi::iter(&vs);

        while iter.hasNext() {
            let view = iter.pin_mut().next().unwrap();
            println!("view: {:?}", view.type_id());
        }
    }
}

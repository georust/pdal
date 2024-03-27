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
    #[namespace = "pdal_sys::layout"]
    unsafe extern "C++" {
        include!("pdal-sys/src/layout/layout.hpp");
        include!("pdal-sys/src/core/core.hpp");
        type PointLayout;
        fn pointSize(self: &PointLayout) -> usize;
        #[namespace = "pdal_sys::core"]
        type DimType = crate::core::DimType;
        #[namespace = "pdal_sys::core"]
        type DimTypeIter<'a> = crate::core::DimTypeIter;
        fn dimTypes<'pl>(pl: &'pl PointLayout) -> UniquePtr<DimTypeIter<'pl>>;
    }
}

use cxx::UniquePtr;
pub use ffi::*;
use std::marker::PhantomData;
use std::mem;

impl PointLayout {
    pub fn dim_types(&self) -> DimTypeIterator {
        DimTypeIterator(ffi::dimTypes(self), PhantomData)
    }
}

pub struct DimTypeIterator<'a>(UniquePtr<DimTypeIter<'a>>, PhantomData<&'a ()>);

impl<'a> Iterator for DimTypeIterator<'a> {
    type Item = &'a DimType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.hasNext() {
            let v = self.0.pin_mut().next().ok()?;
            // TODO: Make this ⇩ go away
            unsafe { mem::transmute(v) }
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
        let vs = r.unwrap();
        let mut iter = vs.iter();
        let next = iter.next();
        let view = next.unwrap();
        let layout = view.layout();
        for d in layout.dim_types() {
            println!("{:?}", d);
        }

        //assert_eq!(super::ffi::layout(&view).pointSize(), 56);
    }
}

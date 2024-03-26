#![allow(dead_code)]

#[cxx::bridge]
mod ffi {
    #[namespace = "pdal_sys::layout"]
    unsafe extern "C++" {
        include!("pdal-sys/src/layout/layout.hpp");
        type PointLayout;
        fn pointSize(self: &PointLayout) -> usize;
    }
}

pub use ffi::*;

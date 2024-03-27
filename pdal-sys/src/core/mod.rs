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

mod enums;
pub use enums::DimTypeId;

#[cxx::bridge]
mod ffi {

    #[namespace = "pdal_sys::core"]
    unsafe extern "C++" {
        include!("pdal-sys/src/core/core.hpp");
        type DimType;
        type DimTypeId = super::DimTypeId;
        fn id(dt: &DimType) -> DimTypeId;
        //fn representation(dt: &DimType) -> DimTypeRepr;
        fn name(id: DimTypeId) -> String;
        fn description(id: DimTypeId) -> String;
        type DimTypeIter;
        fn hasNext(self: &DimTypeIter) -> bool;
        fn next(self: Pin<&mut DimTypeIter>) -> Result<&DimType>;
    }

    impl UniquePtr<DimTypeIter> {}
}

pub use ffi::{DimType, DimTypeIter};
use std::fmt::{Debug, Formatter};

impl Debug for DimType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DimType")
            .field("id", &ffi::id(self))
            .finish()
    }
}

impl DimTypeId {
    pub fn name(&self) -> String {
        ffi::name(*self)
    }

    pub fn description(&self) -> String {
        ffi::description(*self)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::DimTypeId;

    #[test]
    fn test_get_views() {
        assert_eq!(DimTypeId::Anisotropy.name(), "Anisotropy");
        assert!(DimTypeId::Anisotropy.description().contains("variance"));
    }
}

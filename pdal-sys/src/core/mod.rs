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

pub use enums::*;

#[cxx::bridge(namespace = "pdal_sys")]
mod ffi {

    #[namespace = "pdal_sys::core"]
    unsafe extern "C++" {
        include!("pdal-sys/src/core/core.hpp");
        // TODO: See github.com/dtolnay/cxx/issues/1332
        type DimType;
        type DimTypeId = super::enums::DimTypeId;
        fn id(dt: &DimType) -> DimTypeId;
        type DimTypeEncoding = super::enums::DimTypeEncoding;
        fn encoding(dt: &DimType) -> DimTypeEncoding;
        fn interpretationName(enc: DimTypeEncoding) -> String;
        fn encodingSizeBytes(enc: DimTypeEncoding) -> usize;
        fn encodingOrdinal(enc: DimTypeEncoding) -> i32;

        #[cxx_name = "name"]
        fn idName(id: DimTypeId) -> String;
        fn description(id: DimTypeId) -> String;
        type DimTypeIter<'a>;
        fn hasNext(self: &DimTypeIter) -> bool;
        fn next(self: Pin<&mut DimTypeIter>) -> Result<&DimType>;

        type DimIdIter;
        #[cxx_name = "hasNext"]
        fn hasNextId(self: &DimIdIter) -> bool;
        #[cxx_name = "next"]
        fn nextId(self: Pin<&mut DimIdIter>) -> Result<&DimTypeId>;

        fn pdal_sys_throw(msg: &str) -> Result<()>;
    }

    // These trigger the generation of the C++ template backing this concrete type.
    // See: https://cxx.rs/extern-c++.html#explicit-shim-trait-impls
    impl UniquePtr<DimTypeIter<'_>> {}
    impl UniquePtr<DimIdIter> {}
}

use cxx::UniquePtr;
pub(crate) use ffi::DimTypeIter;
pub use ffi::{pdal_sys_throw, DimIdIter, DimType, DimTypeEncoding};
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::mem;

/// Unique identifier for a point in a point view.
pub type PointId = u64;

impl DimType {
    #[inline]
    pub fn id(&self) -> DimTypeId {
        ffi::id(self)
    }
    #[inline]
    pub fn encoding(&self) -> DimTypeEncoding {
        ffi::encoding(self)
    }
}

impl Debug for DimType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DimType")
            .field("id", &self.id())
            .field("encoding", &self.encoding())
            .finish_non_exhaustive()
    }
}

impl DimTypeId {
    #[inline]
    pub fn name(&self) -> String {
        ffi::idName(*self)
    }
    #[inline]
    pub fn description(&self) -> String {
        ffi::description(*self)
    }
}

impl Debug for DimTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.debug_struct("DimTypeId")
                .field("name", &self.name())
                .field("description", &self.description())
                .finish()
        } else {
            f.write_str(&self.name())
        }
    }
}

impl Display for DimTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name())
    }
}

impl DimTypeEncoding {
    #[inline]
    pub fn name(&self) -> &'static str {
        match *self {
            DimTypeEncoding::None => "None",
            DimTypeEncoding::Unsigned8 => "Unsigned8",
            DimTypeEncoding::Signed8 => "Signed8",
            DimTypeEncoding::Unsigned16 => "Unsigned16",
            DimTypeEncoding::Signed16 => "Signed16",
            DimTypeEncoding::Unsigned32 => "Unsigned32",
            DimTypeEncoding::Signed32 => "Signed32",
            DimTypeEncoding::Unsigned64 => "Unsigned64",
            DimTypeEncoding::Signed64 => "Signed64",
            DimTypeEncoding::Float => "Float",
            DimTypeEncoding::Double => "Double",
        }
    }
    #[inline]
    pub fn interpretation(&self) -> String {
        ffi::interpretationName(*self)
    }
    #[inline]
    pub fn size_bytes(&self) -> usize {
        ffi::encodingSizeBytes(*self)
    }
}

impl Debug for DimTypeEncoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DimTypeEncoding")
            .field("name", &self.name())
            .field("interpretation", &self.interpretation())
            .field("size_bytes", &self.size_bytes())
            .finish()
    }
}

pub struct DimIdIterator(pub(crate) UniquePtr<DimIdIter>);

impl Iterator for DimIdIterator {
    type Item = DimTypeId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.hasNextId() {
            let v = self.0.pin_mut().nextId().ok()?;
            Some(*v)
        } else {
            None
        }
    }
}

pub struct DimTypeIterator<'a>(UniquePtr<DimTypeIter<'a>>, PhantomData<&'a ()>);

impl<'a> DimTypeIterator<'a> {
    pub fn new(iter: UniquePtr<DimTypeIter<'a>>) -> Self {
        Self(iter, PhantomData)
    }
}

impl<'a> Iterator for DimTypeIterator<'a> {
    type Item = &'a DimType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.hasNext() {
            let v = self.0.pin_mut().next().ok()?;
            // TODO: Make this ⇩ go away. Could't get the lifetime gymnastics to work...
            unsafe { mem::transmute(v) }
        } else {
            None
        }
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

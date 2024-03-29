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
        type DimTypeRepr;
        fn id(dt: &DimType) -> DimTypeId;
        fn repr(dt: &DimType) -> UniquePtr<DimTypeRepr>;
        #[cxx_name = "name"]
        fn repr_name(r: &DimTypeRepr) -> String;
        /// Type size.
        fn typeSizeBytes(dt: &DimTypeRepr) -> usize;

        #[cxx_name = "name"]
        fn id_name(id: DimTypeId) -> String;
        fn description(id: DimTypeId) -> String;
        type DimTypeIter<'a>;
        fn hasNext(self: &DimTypeIter) -> bool;
        fn next(self: Pin<&mut DimTypeIter>) -> Result<&DimType>;

        type DimIdIter;
        #[cxx_name = "hasNext"]
        fn hasNextId(self: &DimIdIter) -> bool;
        #[cxx_name = "next"]
        fn nextId(self: Pin<&mut DimIdIter>) -> Result<&DimTypeId>;
    }

    impl UniquePtr<DimTypeIter<'_>> {}
    impl UniquePtr<DimIdIter> {}
}

use cxx::UniquePtr;
pub use ffi::{DimIdIter, DimType, DimTypeIter, DimTypeRepr};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::mem;

pub type DimTypeReprPtr = UniquePtr<DimTypeRepr>;

impl DimType {
    #[inline]
    pub fn id(&self) -> DimTypeId {
        ffi::id(self)
    }
    #[inline]
    pub fn repr(&self) -> DimTypeReprPtr {
        ffi::repr(self)
    }
}

impl Debug for DimType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DimType")
            .field("id", &self.id())
            .field("representation", &self.repr())
            .finish_non_exhaustive()
    }
}

impl DimTypeId {
    #[inline]
    pub fn name(&self) -> String {
        ffi::id_name(*self)
    }
    #[inline]
    pub fn description(&self) -> String {
        ffi::description(*self)
    }
}

impl DimTypeRepr {
    #[inline]
    pub fn name(&self) -> String {
        ffi::repr_name(self)
    }
    #[inline]
    pub fn size_bytes(&self) -> usize {
        ffi::typeSizeBytes(self)
    }
}

impl Debug for DimTypeRepr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DimTypeRepr")
            .field("name", &self.name())
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

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

use crate::error::Result;
use crate::utils::{fetch_string_from_handle_with_buffer, Conv};
use pdal_sys::{size_t, PDALDimType, PDALDimTypeListPtr, PDALGetInvalidDimType};
use std::fmt::Debug;

/// Set of dimensions available in a point layout.
pub struct DimensionTypeList(PDALDimTypeListPtr);

impl DimensionTypeList {
    pub(crate) fn new(handle: PDALDimTypeListPtr) -> Self {
        Self(handle)
    }

    pub fn as_ptr(&self) -> PDALDimTypeListPtr {
        self.0
    }

    ///  Returns the number of elements in a dimension type list.
    pub fn len(&self) -> usize {
        unsafe { pdal_sys::PDALGetDimTypeListSize(self.as_ptr()) as usize }
    }

    /// Returns the number of bytes required to store data referenced by self.
    pub fn size_bytes(&self) -> usize {
        unsafe { pdal_sys::PDALGetDimTypeListByteCount(self.as_ptr()) as usize }
    }

    /// Get the dimension type at the given index.
    ///
    /// Returns DimensionType::invalid() if the index is out of range.
    pub fn index(&self, index: usize) -> DimensionType {
        DimensionType(unsafe { pdal_sys::PDALGetDimType(self.as_ptr(), index as size_t) })
    }

    /// Get an iterator over the dimension types in the list.
    pub fn iter(&self) -> impl Iterator<Item = DimensionType> + '_ {
        DimTypeListIter {
            list: self,
            index: 0,
        }
    }
}

impl Drop for DimensionTypeList {
    fn drop(&mut self) {
        unsafe { pdal_sys::PDALDisposeDimTypeList(self.0) }
    }
}

impl Debug for DimensionTypeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.iter().collect::<Vec<_>>().as_slice())
            .finish()
    }
}

#[derive(Clone, Copy)]
pub struct DimensionType(PDALDimType);

impl DimensionType {
    /// Returns the invalid dimension type. This dimension type has:
    ///   - An ID value of 0 that corresponds to `pdal::Dimension::Id::Unknown`
    ///   - An interpretation (data type) value of 0 that corresponds to `pdal::Dimension::Type::None`
    ///   - A scale value of 1.0
    ///   - An offset value of 0.0
    pub fn invalid() -> Self {
        Self(unsafe { PDALGetInvalidDimType() })
    }

    /// Numeric ID of the dimension type.
    pub fn id(&self) -> u32 {
        self.0.id
    }

    /// Gets the name of a dimension
    pub fn name(&self) -> Result<String> {
        let s =
            fetch_string_from_handle_with_buffer::<256, _>(self.0, pdal_sys::PDALGetDimTypeIdName)?;
        Ok(Conv(s).try_into()?)
    }

    // TODO: make enum for interpretation
    /// Gets the interpretation name of a dimension, which is its primitive storage type.
    pub fn interpretation(&self) -> Result<String> {
        let s = fetch_string_from_handle_with_buffer::<256, _>(
            self.0,
            pdal_sys::PDALGetDimTypeInterpretationName,
        )?;
        Ok(Conv(s).try_into()?)
    }

    /// Retrieves the byte count of a dimension type's interpretation, i.e., its data size.
    pub fn size_bytes(&self) -> usize {
        unsafe { pdal_sys::PDALGetDimTypeInterpretationByteCount(self.0) as usize }
    }

    /// Dimension scaling factor
    pub fn scale(&self) -> f64 {
        self.0.scale
    }

    /// Dimension offset value.
    pub fn offset(&self) -> f64 {
        self.0.offset
    }
}

impl Default for DimensionType {
    fn default() -> Self {
        Self::invalid()
    }
}

impl From<PDALDimType> for DimensionType {
    fn from(value: PDALDimType) -> Self {
        Self(value)
    }
}

impl Debug for DimensionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DimensionType")
            .field("id", &self.id())
            .field("name", &self.name().unwrap_or_default())
            .field("interpretation", &self.interpretation().unwrap_or_default())
            .field("size_bytes", &self.size_bytes())
            .field("scale", &self.scale())
            .field("offset", &self.offset())
            .finish()
    }
}

struct DimTypeListIter<'a> {
    list: &'a DimensionTypeList,
    index: usize,
}

impl Iterator for DimTypeListIter<'_> {
    type Item = DimensionType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.list.len() {
            let dt = self.list.index(self.index);
            self.index += 1;
            Some(dt)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testkit::{read_test_file, TestResult};

    #[test]
    fn test_get_layout() -> TestResult {
        let json = read_test_file("stats.json");
        let pipeline = crate::Pipeline::new(json)?;
        let result = pipeline.execute()?;
        let view = result.point_views()?.next().ok_or("no point view")?;
        let layout = view.layout()?;
        let types = layout.dimension_types()?;

        assert_eq!(types.len(), 20);

        let dim = types
            .iter()
            .find(|dt| dt.name().unwrap() == "Blue")
            .ok_or("Blue dimension not found")?;
        assert_eq!(dim.interpretation()?, "uint16_t");
        assert_eq!(dim.size_bytes(), 2);
        assert_eq!(dim.scale(), 1.0);
        assert_eq!(dim.offset(), 0.0);
        Ok(())
    }
}

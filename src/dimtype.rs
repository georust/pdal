use crate::error::Result;
use crate::utils::{fetch_string_from_handle_with_buffer, Conv};
use pdal_sys::{size_t, PDALDimType, PDALDimTypeListPtr, PDALGetInvalidDimType};

#[derive(Debug)]
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
    pub fn byte_count(&self) -> usize {
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

#[derive(Debug, Clone, Copy)]
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
    /// Gets the interpretation name of a dimension
    pub fn interpretation(&self) -> Result<String> {
        let s = fetch_string_from_handle_with_buffer::<256, _>(
            self.0,
            pdal_sys::PDALGetDimTypeInterpretationName,
        )?;
        Ok(Conv(s).try_into()?)
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

        for dt in types.iter() {
            println!("{} -> {}", dt.name()?, dt.interpretation()?);
        }

        Ok(())
    }
}

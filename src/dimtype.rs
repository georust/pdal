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

pub type DimType = pdal_sys::core::DimType;
pub type DimTypeId = pdal_sys::core::DimTypeId;
pub type DimTypeIter<'a> = pdal_sys::core::DimTypeIter<'a>;
pub type DimTypeRepr = pdal_sys::core::DimTypeRepr;

// #[cfg(test)]
// mod tests {
//     use crate::testkit::{read_test_file, TestResult};
//
//     #[test]
//     fn test_get_layout() -> TestResult {
//         let json = read_test_file("stats.json");
//         let pipeline = crate::Pipeline::new(json)?;
//         let result = pipeline.execute()?;
//         let view = result.point_views()?.next().ok_or("no point view")?;
//         let layout = view.layout()?;
//         let types = layout.dimension_types()?;
//
//         assert_eq!(types.len(), 20);
//
//         let dim = types
//             .iter()
//             .find(|dt| dt.name().unwrap() == "Blue")
//             .ok_or("Blue dimension not found")?;
//         assert_eq!(dim.interpretation()?, "uint16_t");
//         assert_eq!(dim.size_bytes(), 2);
//         assert_eq!(dim.scale(), 1.0);
//         assert_eq!(dim.offset(), 0.0);
//         Ok(())
//     }
// }

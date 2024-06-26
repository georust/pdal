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


#include "pdal-sys/src/core/core.hpp"
#include <pdal/pdal_types.hpp>

namespace pdal_sys {
    namespace core {
        void pdal_sys_throw(rust::Str msg) {
            throw pdal::pdal_error(static_cast<std::string>(msg));
        }

        DimTypeId id(const DimType &dt) {
            return dt.m_id;
        }
        rust::String description(DimTypeId id) {
            return rust::String { pdal::Dimension::description(id) };
        }
        rust::String name(DimTypeId id) {
            return rust::String { pdal::Dimension::name(id) };
        }

        DimTypeEncoding encoding(const DimType& id) {
            return id.m_type;
        }

        rust::String interpretationName(DimTypeEncoding enc) {
            return rust::String { pdal::Dimension::interpretationName(enc) };
        }

        std::size_t encodingSizeBytes(DimTypeEncoding enc) {
            return pdal::Dimension::size(enc);
        }

        int encodingOrdinal(DimTypeEncoding enc) {
            return (int) enc;
        }
    }
}
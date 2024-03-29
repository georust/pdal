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


#pragma once

#include <iterator>
#include "rust/cxx.h"
#include <pdal/pdal.hpp>

namespace pdal_sys {
    template <class T>
    class VecIterator {
    public:
        explicit VecIterator(const std::vector<T> &vec) {
            m_begin = vec.begin();
            m_end = vec.end();
        }

        bool hasNext() const {
            return m_begin != m_end;
        }

        const T& next() {
            return *m_begin++;
        }

    private:
        typename std::vector<T>::const_iterator m_begin;
        typename std::vector<T>::const_iterator m_end;
    };

    namespace core {
        using DimTypeEncoding = pdal::Dimension::Type;
        using DimType = pdal::DimType;
        using DimTypeId = pdal::Dimension::Id;
        typedef VecIterator<DimType> DimTypeIter;
        typedef VecIterator<DimTypeId> DimIdIter;

        DimTypeId id(const DimType &dt);
        rust::String description(DimTypeId id);
        rust::String name(DimTypeId id);

        using DimTypeRepr = pdal::Dimension::Type;
        DimTypeEncoding encoding(const DimType &dt);
        rust::String interpretationName(DimTypeEncoding enc);
        std::size_t encodingSizeBytes(DimTypeEncoding enc);
        int encodingOrdinal(DimTypeEncoding enc);
    }
}
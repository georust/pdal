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

#include <memory>
#include "pdal-sys/src/point_view/point_view.hpp"

namespace pdal_sys {
    namespace point_view_set {
        std::unique_ptr<PointViewSetIter> iter(const PointViewSet &set) {
            return std::make_unique<PointViewSetIter>(set);
        }

        PointViewSetIter::PointViewSetIter(const pdal::PointViewSet &views) :
                m_views(views), m_impl(m_views.cbegin()) {}

        bool PointViewSetIter::hasNext() const {
            return (m_impl != m_views.cend());
        }

        pdal::PointViewPtr PointViewSetIter::next() {
            if (!hasNext()) {
                throw std::out_of_range("No more elements in iterator");
            } else {
                return *(m_impl++);
            }
        }
    }

    namespace point_view {
        int id(const PointView& view) {
            return view.id();
        }

        rust::String proj4(const PointView& view) {
            auto sr = view.spatialReference();
            return sr.getProj4();
        }

        rust::String wkt(const PointView& view) {
            auto sr = view.spatialReference();
            return sr.getWKT();
        }

        const pdal::PointLayout& layout(const PointView& view) {
            // TODO: is this legit?
            return *view.layout();
        }
    }

}

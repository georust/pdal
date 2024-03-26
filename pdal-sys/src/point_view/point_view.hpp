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

#include "rust/cxx.h"
#include <pdal/pdal.hpp>
#include <utility>
#include "pdal-sys/src/layout/layout.hpp"

namespace pdal_sys {

    namespace point_view {
        using PointView = pdal::PointView;
        int id(const PointView &view);
        const pdal::PointLayout& layout(const PointView& view);
    }

    namespace point_view_set {
        using PointViewSet = pdal::PointViewSet;
    }

    namespace point_view_iter {
        class PointViewIter {
        public:
            explicit PointViewIter(const pdal::PointViewSet &views);

            bool hasNext() const;

            pdal::PointViewPtr next();

        private:
            const pdal::PointViewSet &m_views;
            pdal::PointViewSet::const_iterator m_impl;
        };

        std::unique_ptr<PointViewIter> iter(const point_view_set::PointViewSet &set);
    }

    namespace point_view_set {

        using pdal_sys::point_view_iter::PointViewIter;

        std::unique_ptr<PointViewIter> iter(const PointViewSet &set);
    }


}
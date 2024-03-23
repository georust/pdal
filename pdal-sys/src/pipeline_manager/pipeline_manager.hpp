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
#include "pdal-sys/src/point_view/point_view.hpp"

namespace pdal_sys {

class PipelineManager {
public:
    PipelineManager();
    void readPipeline(rust::Str json);
    void readPipelineFromFile(rust::Str path);
    bool pipelineStreamable() const;
    std::size_t execute();
    void executeStreamed();
    const pdal_sys::PointViewSet& views() const;
    rust::String metadata() const;
    rust::String schema() const;
    rust::String pipeline() const;

private:
    std::unique_ptr<pdal::PipelineManager> m_impl;
};

std::unique_ptr<PipelineManager> createPipelineManager();
}
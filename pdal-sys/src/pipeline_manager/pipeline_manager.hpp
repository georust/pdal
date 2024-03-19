#pragma once
#include "rust/cxx.h"
#include <pdal/pdal.hpp>

namespace pdal_sys {

class PipelineManager {
public:
    PipelineManager();
    void readPipeline(rust::Str json);
    void readPipelineFromFile(rust::Str path);
    bool pipelineStreamable() const;
    std::size_t execute();
    void executeStreamed();

private:
    std::unique_ptr<pdal::PipelineManager> m_impl;
};

std::unique_ptr<PipelineManager> createPipelineManager();
}
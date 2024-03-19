#include "pdal-sys/src/pipeline_manager/pipeline_manager.hpp"

#include <sstream>

namespace pdal_sys {

    PipelineManager::PipelineManager() : m_impl(std::unique_ptr<pdal::PipelineManager>(new pdal::PipelineManager)) {}

    void PipelineManager::readPipeline(rust::Str json) {
        std::string str = std::string(json);
        std::istringstream is(str);
        m_impl->readPipeline(is);
    }
    void PipelineManager::readPipelineFromFile(rust::Str path) {
        m_impl->readPipeline(std::string(path));
    }
    std::unique_ptr<PipelineManager> create_pipeline_manager() {
        return std::unique_ptr<PipelineManager>(new PipelineManager());
    }

}
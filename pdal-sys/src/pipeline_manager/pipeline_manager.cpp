#include "pdal-sys/src/pipeline_manager/pipeline_manager.hpp"

#include <sstream>

namespace pdal_sys {
    std::unique_ptr<PipelineManager> createPipelineManager() {
        return std::unique_ptr<PipelineManager>(new PipelineManager());
    }

    PipelineManager::PipelineManager() : m_impl(std::unique_ptr<pdal::PipelineManager>(new pdal::PipelineManager)) {}

    void PipelineManager::readPipeline(rust::Str json) {
        std::string str = std::string(json);
        std::istringstream is(str);
        m_impl->readPipeline(is);
    }
    void PipelineManager::readPipelineFromFile(rust::Str path) {
        m_impl->readPipeline(std::string(path));
    }

    bool PipelineManager::pipelineStreamable() const {
        return m_impl->pipelineStreamable();
    }

    std::size_t PipelineManager::execute() {
        return m_impl->execute();
    }

    void PipelineManager::executeStreamed() {
         m_impl->execute(pdal::ExecMode::PreferStream);
    }
}
#include "pdal-sys/src/options/options.hpp"

namespace pdal_sys {
    std::unique_ptr<Options> new_options() {
        return std::unique_ptr<Options>(new Options());
    }
    
    Options::Options() : m_options(std::unique_ptr<pdal::Options>(new pdal::Options())) {}
    
    void Options::add(rust::Str name, rust::Str value) const {
        m_options->add(std::string(name), std::string(value));
    }
}
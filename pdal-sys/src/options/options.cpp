#include "pdal-sys/src/options/options.hpp"

namespace pdal_sys {

    Options::Options() : m_impl(std::unique_ptr<pdal::Options>(new pdal::Options())) {}
    
    void Options::add(rust::Str name, rust::Str value) {
        m_impl->add(std::string(name), std::string(value));
    }

    // Rust constructor
    std::unique_ptr<Options> create_options() {
        return std::unique_ptr<Options>(new Options());
    }
}
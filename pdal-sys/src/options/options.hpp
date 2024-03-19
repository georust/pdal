#pragma once
#include "rust/cxx.h"
#include <pdal/Options.hpp>

namespace pdal_sys
{
    class Options {
    public:
        Options();
        void add(rust::Str name, rust::Str value) const;
    private:
        std::unique_ptr<pdal::Options> m_options;
    };
    
    std::unique_ptr<Options> new_options();
}
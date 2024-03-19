#pragma once
#include "rust/cxx.h"
#include <pdal/Options.hpp>

namespace pdal_sys
{
class Options {
public:
    Options();
    void add(rust::Str name, rust::Str value);
private:
    std::unique_ptr<pdal::Options> m_impl;
};

std::unique_ptr<Options> create_options();
}
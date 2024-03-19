#pragma once
#include "rust/cxx.h"
#include <pdal/pdal_config.hpp>

namespace pdal_sys {
    namespace Config {
        rust::String versionString();
        rust::String fullVersionString();
        rust::String sha1();
        rust::String debugInformation();
        rust::String pluginInstallPath();
    }
}

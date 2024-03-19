#include "pdal-sys/src/bridge.hpp"

namespace pdal_sys {
    namespace Config {
        rust::String versionString() {
            return pdal::Config::versionString();
        }
        rust::String fullVersionString() {
            return pdal::Config::fullVersionString();
        }
        rust::String sha1() {
            return pdal::Config::sha1();
        }
        rust::String debugInformation() {
            return pdal::Config::debugInformation();
        }
        rust::String pluginInstallPath() {
            return pdal::Config::pluginInstallPath();
        }
    }
}
# `pdal-sys` Notes

We currently vendor the [PDAL CAPI](https://github.com/PDAL/CAPI) source code in the `vendor` directory via a git submodule.
Part of this is due to it being a package not readily available in most package managers, 
and part of it is due to the fact that we're relying on tweaks to a fork that are not rolled into the official release yet.
One of those tweaks is to compile the CAPI as a static library, so as to be bundled inside with the final binary.

## Setup

Before building, you may need to this command first.

    git submodule update --init --recursive vendor

The `build.rs` file attempts to do this for you, but it may not work in all cases.

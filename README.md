# jpegxl-sys

`jpegxl-sys` is a wrapper over `libjxl` library. Check out the original library [here](https://github.com/libjxl/libjxl).

## Building

Building `libjxl` and statically linking is by default, requiring `git` command in `PATH`.

If you wish to use existing library and dynamic linking, then use the feature `system-jxl`. Set the custom include path
and lib path with `DEP_JXL_INCLUDE` and `DEP_JXL_LIB` respectively.

## Usage

Check out testing units in `src/lib.rs` for some examples.

### Multithread

Because `libjxl_threads` uses `std::thread`, if you build and statically link `libjxl`, you need to
link `libc++` or `libstdc++` standard library as well.
Using dynamic library doesn't need this requirement.

If you don't want the dependency, you can disable the `threads` feature.

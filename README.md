# jpegxl-sys

`jpegxl-sys` is a wrapper over `jpeg-xl` library. Check out the original library [here](https://gitlab.com/wg1/jpeg-xl).

## Building

Building `jpeg-xl` and statically linking is by default, requiring `git` command in `PATH`.

Since the library uses GCC extensions, you need to have a working GCC/Clang toolchain on Windows.

If you wish to use existing library and dynamic linking, then use the feature `system-jxl`. Set the custom include path
and lib path with `DEP_JXL_INCLUDE` and `DEP_JXL_LIB` respectively.

## Usage

Check out testing units in `src/lib.rs` for some examples.

### Multithread

Because `libjxl_threads` uses `std::thread`, if you build and statically link `jpeg-xl`, you need to
link `libc++` or `libstdc++` standard library as well.
Using dynamic library doesn't need this requirement.

If you don't want the dependency, you can disable the `threads` feature.

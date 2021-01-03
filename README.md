# jpegxl-sys

`jpegxl-sys` is a wrapper over `jpeg-xl` library. Check out the original library [here](https://gitlab.com/wg1/jpeg-xl).

## Building

Install the `jpeg-xl` library system-wide or specify `PKG_CONFIG_PATH` to search for needed paths. Optionally, you can
overwrite the include path and lib path with `DEP_JPEGXL_INCLUDE` and `DEP_JPEGXL_LIB` respectively.

If you want to build the library within cargo, enable `build-jpegxl` features in your `Cargo.toml`. You may need to
manually fetch submodules inside `jpeg-xl` source folder:

```bash
git submodule update --init --recursive
```

You need to have a working `llvm` environment. Note that this will link to `libc++` by default
(since you already use llvm). You can modify it by setting `DEP_JPEGXL_CXXLIB`.

## Usage

Check out testing units in `src/lib.rs` for some examples.

### Multithreading (WIP)

Since the reference multithread parallel runner needs statically linked `jpeg-xl`, it's only enabled
in `build-jpegxl` feature.

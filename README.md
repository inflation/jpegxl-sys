# jpegxl-sys

`jpegxl-sys` is a wrapper over `jpeg-xl` library. Check out the original library [here](https://gitlab.com/wg1/jpeg-xl).

## Building

Install the `jpeg-xl` library system-wide or specify `PKG_CONFIG_PATH` to search for needed paths. Optionally, you can
overwrite the include path and lib path with `DEP_JPEGXL_INCLUDE` and `DEP_JPEGXL_LIB` respectively.

If you want to build the library within cargo, enable `build-jpegxl` features in your `Cargo.toml`. You may need to
manually fetch submodules inside `jpeg-xl` source folder:

```bash
cd jpeg-xl; git submodule update --init
```

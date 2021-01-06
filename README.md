# jpegxl-sys

`jpegxl-sys` is a wrapper over `jpeg-xl` library. Check out the original library [here](https://gitlab.com/wg1/jpeg-xl).

## Building

Now building `jpeg-xl` and statically linking is by default. You may need to manually fetch dependents
inside `jpeg-xl` source folder:

```bash
./deps.sh
```

If you wish to use existing library and dynamically linking, then set the include path and lib path
with `DEP_JXL_INCLUDE` and `DEP_JXL_LIB` respectively, as well as with `--no-default-features`.

## Usage

Check out testing units in `src/lib.rs` for some examples.

### Multithreading

Fully working now.

Note: Because `jxl_threads` uses `std::thread`, if you build and statically link `jpeg-xl`, you need to
link to `libc++` standard library. Using dynamic library don't need this requirement.

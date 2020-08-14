use bindgen::builder;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=wrapper.hpp");

    let jpegxl_prefix = get_jpegxl_prefix();
    let include_dir =
        env::var("DEP_JPEGXL_INCLUDE").unwrap_or(format!("-I{}/include/jpegxl", jpegxl_prefix));
    let lib_dir = env::var("DEP_JPEGXL_LIB").unwrap_or(format!("{}/lib", jpegxl_prefix));
    println!("cargo:rustc-link-lib=jpegxl");
    println!("cargo:rustc-link-search=native={}", lib_dir);

    let bindings = builder()
        .header("wrapper.h")
        .clang_arg(&include_dir)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let cppbindings = builder()
        .header("wrapper.hpp")
        .clang_arg(&include_dir)
        .clang_arg("-xc++")
        .clang_arg("-std=c++17")
        .enable_cxx_namespaces()
        .whitelist_type("jpegxl::ThreadParallelRunner")
        .opaque_type("std::.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    cppbindings
        .write_to_file(out_path.join("cppbindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(not(feature = "build-jpegxl"))]
fn get_jpegxl_prefix() -> String {
    env::var("DEP_JPEGXL_PREFIX").unwrap_or("/usr/local/".to_string())
}

#[cfg(feature = "build-jpegxl")]
fn get_jpegxl_prefix() -> String {
    use cmake::Config;
    Config::new("jpeg-xl").build().display().to_string()
}

use bindgen::builder;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=wrapper.hpp");

    let (include_dir, lib_dir) = get_jpegxl_dir();
    println!("cargo:rustc-link-lib=jpegxl");
    println!("cargo:rustc-link-search=native={}", lib_dir);

    let bindings = builder()
        .header("wrapper.h")
        .clang_arg(format!("-I{}/jpegxl", &include_dir))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let cppbindings = builder()
        .header("wrapper.hpp")
        .clang_arg(format!("-I{}/jpegxl", &include_dir))
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
fn get_jpegxl_dir() -> (String, String) {
    let lib = pkg_config::Config::new().probe("libjpegxl").unwrap();
    (
        env::var("DEP_JPEGXL_INCLUDE")
            .unwrap_or(lib.include_paths.first().unwrap().display().to_string()),
        env::var("DEP_JPEGXL_LIB").unwrap_or(lib.link_paths.first().unwrap().display().to_string()),
    )
}

#[cfg(feature = "build-jpegxl")]
fn get_jpegxl_dir() -> (String, String) {
    let prefix = Config::new("jpeg-xl").build().display().to_string();
    (
        format!("{}/include/jpegxl", prefix),
        format!("{}/lib", prefix),
    )
}

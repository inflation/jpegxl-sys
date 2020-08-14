use bindgen::builder;
use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=wrapper.hpp");

    let lib_prefix = get_lib_prefix();
    let include_dir = format!("-I{}/include/jpegxl", lib_prefix);
    println!("cargo:rustc-link-lib=jpegxl");
    println!("cargo:rustc-link-search=native={}/lib", lib_prefix);

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

#[cfg(feature = "use-system-lib")]
fn get_lib_prefix() -> String {
    match env::var("JPEGXL_PREFIX") {
        Ok(path) => path,
        Err(_) => "/usr/local/".to_string(),
    }
}

#[cfg(not(feature = "use-system-lib"))]
fn get_lib_prefix() -> String {
    Config::new("jpeg-xl").build().display().to_string()
}

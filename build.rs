use bindgen::builder;
use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=wrapper.hpp");

    let lib_prefix = get_lib_prefix();

    let bindings = builder()
        .header("wrapper.h")
        .clang_arg(&lib_prefix)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let cppbindings = builder()
        .header("wrapper.hpp")
        .clang_arg(&lib_prefix)
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
    let lib_prefix = match env::var("JPEGXL_PREFIX_DIR") {
        Ok(path) => path,
        Err(_) => "/usr/local/".to_string(),
    };
    println!("cargo:rustc-link-lib=jpegxl");
    println!("cargo:rustc-link-search={}/lib", lib_prefix);

    format!("-I{}/include/jpegxl", lib_prefix)
}

#[cfg(not(feature = "use-system-lib"))]
fn get_lib_prefix() -> String {
    let dst = Config::new("jpeg-xl").build();

    println!("cargo:rustc-link-lib=static=libjpegxl");
    println!("cargo:rustc-link-search=native={}", dst.display());

    format!("-Isrc/jpeg-xl/include")
}

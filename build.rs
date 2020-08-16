use bindgen::builder;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    #[cfg(feature = "build-jpegxl")]
    println!("cargo:rerun-if-changed=wrapper.hpp");

    let include_dir = setup_jpegxl();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let bindings = builder()
        .header("wrapper.h")
        .clang_arg(format!("-I{}/jpegxl", &include_dir))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    #[cfg(feature = "build-jpegxl")]
    {
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
        cppbindings
            .write_to_file(out_path.join("cppbindings.rs"))
            .expect("Couldn't write bindings!");
    }
}

#[cfg(feature = "default")]
fn setup_jpegxl() -> String {
    let lib = pkg_config::Config::new().probe("libjpegxl").unwrap();
    let lib_path = env::var("DEP_JPEGXL_LIB")
        .unwrap_or_else(|_| lib.link_paths.first().unwrap().display().to_string());
    println!("cargo:rustc-link-lib=jpegxl");
    println!("cargo:rustc-link-search=native={}", lib_path);

    env::var("DEP_JPEGXL_INCLUDE")
        .unwrap_or_else(|_| lib.include_paths.first().unwrap().display().to_string())
}

#[cfg(feature = "build-jpegxl")]
fn setup_jpegxl() -> String {
    use cmake::Config;
    use std::process::Command;

    Command::new("git")
        .args(&["submodule", "update", "--init", "--recursive"])
        .output()
        .expect("Sync submodules failed!");

    let prefix = Config::new("jpeg-xl").build().display().to_string();
    let build_path = format!("{}/build", prefix);

    println!(
        "cargo:rustc-link-lib={}",
        env::var("DEP_JPEGXL_CXXLIB").unwrap_or("c++".to_string())
    );

    println!("cargo:rustc-link-lib=static=jpegxl-static");
    println!("cargo:rustc-link-lib=static=jpegxl_threads");
    println!("cargo:rustc-link-search=native={}", build_path);

    println!("cargo:rustc-link-lib=static=lcms2");
    println!("cargo:rustc-link-search=native={}/third_party", build_path);

    println!("cargo:rustc-link-lib=static=hwy");
    println!(
        "cargo:rustc-link-search=native={}/third_party/highway",
        build_path
    );

    println!("cargo:rustc-link-lib=static=brunslicommon-static");
    println!("cargo:rustc-link-lib=static=brunslidec-static");
    println!(
        "cargo:rustc-link-search=native={}/third_party/brunsli",
        build_path
    );

    println!("cargo:rustc-link-lib=static=brotlicommon-static");
    println!("cargo:rustc-link-lib=static=brotlidec-static");
    println!(
        "cargo:rustc-link-search=native={}/third_party/brotli",
        build_path
    );

    format!("{}/include", prefix)
}

#[cfg(feature = "docsrs")]
fn setup_jpegxl() -> String {
    String::from("include")
}

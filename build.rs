use bindgen::builder;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    #[cfg(feature = "build-jpegxl")]
    println!("cargo:rerun-if-changed=wrapper-thread.h");

    let include_dir = setup_jpegxl();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut b = builder()
        .header("wrapper.h");

    #[cfg(feature = "build-jpegxl")]
    {
        b = b.header("wrapper-thread.h");
    }

    let bindings = b
        .clang_arg(format!("-I{}", &include_dir))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(not(any(feature = "build-jpegxl", feature = "docsrs")))]
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
    libjxl_src::print_cargo_link();

    format!("{}/include", libjxl_src::out_dir())
}

#[cfg(feature = "docsrs")]
fn setup_jpegxl() -> String {
    String::from("include")
}

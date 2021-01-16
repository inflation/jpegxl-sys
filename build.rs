use bindgen::builder;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    #[cfg(not(feature = "without-threads"))]
    println!("cargo:rerun-if-changed=wrapper-threads.h");

    let include_dir = setup_jpegxl();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let header = if cfg!(not(feature = "without-threads")) {
        "wrapper-threads.h"
    } else {
        "wrapper.h"
    };

    let bindings = builder()
        .header(header)
        .clang_arg(format!("-I{}", &include_dir))
        .blacklist_function("strtold") // Returned long double becomes u128
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn setup_jpegxl() -> String {
    cfg_if::cfg_if! {
        if #[cfg(feature = "docsrs")] {
            String::from("include")
        } else if #[cfg(feature = "without-build")] {
            let lib_path = env::var("DEP_JXL_LIB").expect("Library path is not set!");
            println!("cargo:rustc-link-lib=jxl");

            #[cfg(not(feature = "without-threads"))]
            println!("cargo:rustc-link-lib=jxl_threads");

            println!("cargo:rustc-link-search=native={}", lib_path);

            env::var("DEP_JXL_INCLUDE").unwrap_or_else(|_| "include".to_owned())
        } else {
            use cmake::Config;
            use std::process::Command;

            let source = format!("{}/jpeg-xl", env::var("OUT_DIR").unwrap());

            Command::new("git")
                .args(&[
                    "clone",
                    "--depth=1",
                    "--branch=v0.2",
                    "https://gitlab.com/wg1/jpeg-xl",
                    &source,
                ])
                .status()
                .expect("Fetching source code failed!");
            Command::new("git")
                .args(&["-C", &source, "submodule", "init"])
                .status()
                .expect("Initializing submodule failed!");
            Command::new("git")
                .args(&["-C", &source, "submodule", "update", "--depth=1"])
                .status()
                .expect("Updating submodule failed!");

            let prefix = Config::new(&source).build().display().to_string();

            let lib_path = format!("{}/lib", prefix);

            println!("cargo:rustc-link-lib=static=jxl");

            #[cfg(not(feature = "without-threads"))]
            println!("cargo:rustc-link-lib=static=jxl_threads");

            println!("cargo:rustc-link-lib=static=hwy");
            println!("cargo:rustc-link-search=native={}", lib_path);

            println!("cargo:rustc-link-lib=static=skcms");
            println!(
                "cargo:rustc-link-search=native={}/build/third_party",
                prefix
            );

            #[cfg(not(feature = "without-threads"))]
            cfg_if::cfg_if! {
                if #[cfg(any(target_os = "macos", target_os = "ios"))] {
                    println!("cargo:rustc-link-lib=c++");
                } else {
                    println!("cargo:rustc-link-lib=stdc++");
                }
            }
            format!("{}/include", prefix)
        }
    }
}

use std::{
    env,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
    process::Output,
};

use bindgen::builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=wrapper.h");

    #[cfg(not(feature = "without-threads"))]
    println!("cargo:rerun-if-changed=wrapper-threads.h");

    let include_dir = setup_jpegxl()?;
    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    let header = if cfg!(not(feature = "without-threads")) {
        "wrapper-threads.h"
    } else {
        "wrapper.h"
    };

    let bindings = builder()
        .header(header)
        .clang_arg(format!("-I{}", &include_dir))
        .blacklist_function("strtold") // Returned long double becomes u128, which is not safe
        .blacklist_function("qecvt")
        .blacklist_function("qfcvt")
        .blacklist_function("qgcvt")
        .blacklist_function("qecvt_r")
        .blacklist_function("qfcvt_r")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .map_err(|_| "Unable to generate bindings!")?;
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}

fn setup_jpegxl() -> Result<String, Box<dyn std::error::Error>> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "docsrs")] {
            Ok(String::from("include"))
        } else if #[cfg(feature = "system-jpegxl")] {
            println!("cargo:rustc-link-lib=jxl");

            #[cfg(not(feature = "without-threads"))]
            println!("cargo:rustc-link-lib=jxl_threads");

            env::var("DEP_JXL_LIB").map(|l| {
                println!("cargo:rustc-link-search=native={}", l);
            }).ok();

            Ok(env::var("DEP_JXL_INCLUDE").unwrap_or_else(|_| "include".to_owned()))
        } else {
            build()
        }
    }
}

#[allow(dead_code)]
fn check_status(msg: &'static str) -> impl Fn(Output) -> Result<(), Error> {
    move |e| {
        e.status.success().then(|| ()).ok_or_else(|| {
            Error::new(
                ErrorKind::Other,
                format!("{}, stderr: {}", msg, String::from_utf8_lossy(&e.stderr)),
            )
        })
    }
}

#[allow(dead_code)]
fn build() -> Result<String, Box<dyn std::error::Error>> {
    use cmake::Config;
    use std::process::Command;

    let source = format!("{}/jpeg-xl", env::var("OUT_DIR")?);

    if Path::new(&source).exists() {
        Command::new("git")
            .args(&["-C", &source, "checkout", "v0.3.3"])
            .output()
            .and_then(check_status("Failed to checkout v0.3.3!"))?;
    } else {
        Command::new("git")
            .args(&[
                "clone",
                "--depth=1",
                "--branch=v0.3.3",
                "https://gitlab.com/wg1/jpeg-xl.git",
                &source,
            ])
            .output()
            .and_then(check_status("Failed to clone jpeg-xl!"))?;
    }
    Command::new("git")
        .args(&["-C", &source, "submodule", "init"])
        .output()
        .and_then(check_status("Failed to init submodule!"))?;
    Command::new("git")
        .args(&["-C", &source, "submodule", "update", "--depth=1"])
        .output()
        .and_then(check_status("Failed to update submodule!"))?;

    // Disable binary tools
    Command::new("sed")
        .args(&[
            "-i.bak",
            "61,118s/^/#/",
            &format!("{}/tools/CMakeLists.txt", &source),
        ])
        .output()
        .and_then(check_status("Disable binary failed!"))?;

    // macOS doesn't support `-static`, this comment out the flag
    #[cfg(target_os = "macos")]
    Command::new("sed")
        .args(&[
            "-i.bak",
            "152,153s/^/#/",
            &format!("{}/CMakeLists.txt", &source),
        ])
        .output()
        .and_then(check_status("Edit CMakeLists failed"))?;

    env::set_var("CMAKE_BUILD_PARALLEL_LEVEL", format!("{}", num_cpus::get()));

    let mut config = Config::new(&source);
    config
        .define("BUILD_GMOCK", "OFF")
        .define("BUILD_TESTING", "OFF")
        .define("INSTALL_GTEST", "OFF")
        .define("JPEGXL_ENABLE_BENCHMARK", "OFF")
        .define("JPEGXL_ENABLE_EXAMPLES", "OFF")
        .define("JPEGXL_ENABLE_OPENEXR", "OFF")
        .define("JPEGXL_STATIC", "ON");

    #[cfg(target_os = "windows")]
    config.target("x86_64-pc-windows-gnu");

    let prefix = config.build().display().to_string();

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

    println!("cargo:rustc-link-lib=static=brotlicommon-static");
    println!("cargo:rustc-link-lib=static=brotlienc-static");
    println!(
        "cargo:rustc-link-search=native={}/build/third_party/brotli",
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

    Ok(format!("{}/include", prefix))
}

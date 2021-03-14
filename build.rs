use std::{
    env,
    io::{Error, ErrorKind},
    path::PathBuf,
    process::Output,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_jpegxl()?;

    Ok(())
}

fn setup_jpegxl() -> Result<(), Box<dyn std::error::Error>> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "docsrs")] {
            Ok(())
        } else if #[cfg(feature = "system-jpegxl")] {
            println!("cargo:rustc-link-lib=jxl");

            #[cfg(not(feature = "without-threads"))]
            println!("cargo:rustc-link-lib=jxl_threads");

            env::var("DEP_JXL_LIB").map(|l| {
                println!("cargo:rustc-link-search=native={}", l);
            }).ok();

            Ok(())
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
fn build() -> Result<(), Box<dyn std::error::Error>> {
    use cmake::Config;
    use std::process::Command;

    let source: PathBuf = [&env::var("OUT_DIR")?, "jpeg-xl"].iter().collect();
    let source_str = source.to_str().ok_or("Source path is invalid UTF-8")?;

    if source.exists() {
        Command::new("git")
            .args(&["-C", source_str, "checkout", "v0.3.3"])
            .output()
            .and_then(check_status("Failed to checkout v0.3.3!"))?;
    } else {
        Command::new("git")
            .args(&[
                "clone",
                "--depth=1",
                "--branch=v0.3.3",
                "https://gitlab.com/wg1/jpeg-xl.git",
                source_str,
            ])
            .output()
            .and_then(check_status("Failed to clone jpeg-xl!"))?;
    }
    Command::new("git")
        .args(&["-C", source_str, "submodule", "init"])
        .output()
        .and_then(check_status("Failed to init submodule!"))?;
    Command::new("git")
        .args(&["-C", source_str, "submodule", "update", "--depth=1"])
        .output()
        .and_then(check_status("Failed to update submodule!"))?;

    // macOS(iOS) doesn't support `-static`, this comment out the flag
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    Command::new("sed")
        .args(&[
            "-i.bak",
            "152,153s/^/#/",
            source.join("CMakeLists.txt").to_str().unwrap(),
        ])
        .output()
        .and_then(check_status("Edit CMakeLists failed"))?;

    // Disable binary tools
    Command::new("sed")
        .args(&[
            "-i.bak",
            "61,118s/^/#/",
            source
                .join("tools")
                .join("CMakeLists.txt")
                .to_str()
                .unwrap(),
        ])
        .output()
        .and_then(check_status("Disable binary failed!"))?;

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

    let mut prefix = config.build();

    println!("cargo:rustc-link-lib=static=jxl");

    #[cfg(not(feature = "without-threads"))]
    println!("cargo:rustc-link-lib=static=jxl_threads");

    println!("cargo:rustc-link-lib=static=hwy");
    println!(
        "cargo:rustc-link-search=native={}",
        prefix.join("lib").display()
    );

    prefix.push("build");
    prefix.push("third_party");
    println!("cargo:rustc-link-lib=static=skcms");
    println!("cargo:rustc-link-search=native={}", prefix.display());

    println!("cargo:rustc-link-lib=static=brotlicommon-static");
    println!("cargo:rustc-link-lib=static=brotlienc-static");
    println!(
        "cargo:rustc-link-search=native={}",
        prefix.join("brotli").display()
    );

    #[cfg(not(feature = "without-threads"))]
    cfg_if::cfg_if! {
        if #[cfg(any(target_os = "macos", target_os = "ios"))] {
            println!("cargo:rustc-link-lib=c++");
        } else {
            println!("cargo:rustc-link-lib=stdc++");
        }
    }

    Ok(())
}

use std::{
    env,
    io::{Error, ErrorKind},
    path::PathBuf,
    process::Output,
};

const VERSION: &str = "v0.6";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_jpegxl()?;

    Ok(())
}

fn setup_jpegxl() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "system-jxl")]
    {
        println!("cargo:rustc-link-lib=jxl");

        #[cfg(feature = "threads")]
        println!("cargo:rustc-link-lib=jxl_threads");

        env::var("DEP_JXL_LIB")
            .map(|l| {
                println!("cargo:rustc-link-search=native={}", l);
            })
            .ok();

        Ok(())
    }

    #[cfg(not(feature = "system-jxl"))]
    build()
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

    let source: PathBuf = [&env::var("OUT_DIR")?, "libjxl"].iter().collect();
    let source_str = source.to_str().ok_or("Source path is invalid UTF-8")?;

    if source.exists() {
        Command::new("git")
            .args(&["-C", source_str, "checkout", VERSION])
            .output()
            .and_then(check_status("Failed to checkout the source code"))?;
    } else {
        Command::new("git")
            .args(&[
                "clone",
                "--depth=1",
                &format!("--branch={}", VERSION),
                "https://github.com/libjxl/libjxl.git",
                source_str,
            ])
            .output()
            .and_then(check_status("Failed to clone libjxl"))?;
    }
    Command::new("git")
        .args(&["-C", source_str, "submodule", "init"])
        .output()
        .and_then(check_status("Failed to init submodule!"))?;
    Command::new("git")
        .args(&["-C", source_str, "submodule", "update", "--depth=1"])
        .output()
        .and_then(check_status("Failed to update submodule!"))?;

    env::set_var("CMAKE_BUILD_PARALLEL_LEVEL", format!("{}", num_cpus::get()));

    let mut config = Config::new(&source);
    config
        .define("BUILD_GMOCK", "OFF")
        .define("BUILD_TESTING", "OFF")
        .define("INSTALL_GTEST", "OFF")
        .define("JPEGXL_ENABLE_TOOLS", "OFF")
        .define("JPEGXL_ENABLE_MANPAGES", "OFF")
        .define("JPEGXL_ENABLE_BENCHMARK", "OFF")
        .define("JPEGXL_ENABLE_EXAMPLES", "OFF")
        .define("JPEGXL_ENABLE_JNI", "OFF")
        .define("JPEGXL_ENABLE_OPENEXR", "OFF")
        .define("JPEGXL_STATIC", "ON");

    let mut prefix = config.build();

    println!("cargo:rustc-link-lib=static=jxl");

    #[cfg(feature = "threads")]
    println!("cargo:rustc-link-lib=static=jxl_threads");

    println!("cargo:rustc-link-lib=static=hwy");
    println!(
        "cargo:rustc-link-search=native={}",
        prefix.join("lib").display()
    );

    prefix.push("build");
    prefix.push("third_party");
    println!("cargo:rustc-link-search=native={}", prefix.display());

    println!("cargo:rustc-link-lib=static=brotlicommon-static");
    println!("cargo:rustc-link-lib=static=brotlidec-static");
    println!("cargo:rustc-link-lib=static=brotlienc-static");
    println!(
        "cargo:rustc-link-search=native={}",
        prefix.join("brotli").display()
    );

    #[cfg(feature = "threads")]
    {
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        println!("cargo:rustc-link-lib=c++");
        #[cfg(not(any(target_os = "macos", target_os = "ios")))]
        println!("cargo:rustc-link-lib=stdc++");
    }

    Ok(())
}

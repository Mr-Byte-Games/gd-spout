use std::path::PathBuf;

const SPOUT_DIR: &str = "deps/Spout2";

#[cfg(not(target_os = "windows"))]
fn main() {}

#[cfg(target_os = "windows")]
fn main() {
    let build_dir = build_spout();
    let lib_dir = build_dir.join("lib");
    let include_dir = build_dir.join("include");
    let spout_dx12_include_dir = include_dir.join("SpoutDX12");

    cxx_build::bridge("src/spout.rs")
        .include(spout_dx12_include_dir)
        .include("include/")
        .file("src/spout.cpp")
        .flag_if_supported("-std=c++14")
        .compile("spout");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=spout.rs");
    println!("cargo:rerun-if-changed=include/library.h");
    println!("cargo:rerun-if-changed=src/library.cpp");
    println!("cargo:rustc-link-lib=Spout");
    println!("cargo:rustc-link-lib=SpoutDX");
    println!("cargo:rustc-link-lib=SpoutDX12");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
}

#[cfg(target_os = "windows")]
fn build_spout() -> PathBuf {
    cmake::Config::new(SPOUT_DIR)
        .define("SKIP_INSTALL_ALL", "OFF")
        .define("SKIP_INSTALL_HEADERS", "OFF")
        .define("SKIP_INSTALL_LIBRARIES", "OFF")
        .define("SPOUT_BUILD_CMT", "OFF")
        .define("SPOUT_BUILD_LIBRARY", "ON")
        .define("SPOUT_BUILD_SPOUTDX", "ON")
        .define("SPOUT_BUILD_SPOUTDX_EXAMPLES", "OFF")
        .profile("Release")
        .build()
}

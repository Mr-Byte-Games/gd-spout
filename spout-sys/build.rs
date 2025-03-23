#[cfg(not(target_os = "windows"))]
fn main() {}

#[cfg(target_os = "windows")]
use std::path::Path;
#[cfg(target_os = "windows")]
use std::path::PathBuf;

#[cfg(target_os = "windows")]
const SPOUT_DIR: &str = "deps/Spout2";

#[cfg(target_os = "windows")]
fn main() {
    init_spout_submodule();

    let build_dir = build_spout();
    let lib_dir = build_dir.join("lib");
    let include_dir = build_dir.join("include");
    let spout_dx12_include_dir = include_dir.join("SpoutDX12");

    copy_spout_output(&build_dir);

    cxx_build::bridge("src/spout.rs")
        .include(spout_dx12_include_dir)
        .include("include/")
        .file("src/spout.cpp")
        .flag_if_supported("-std=c++14")
        .compile("spout");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=spout.rs");
    println!("cargo:rerun-if-changed=include/spout.h");
    println!("cargo:rerun-if-changed=src/spout.cpp");
    println!("cargo:rustc-link-lib=Spout");
    println!("cargo:rustc-link-lib=SpoutDX");
    println!("cargo:rustc-link-lib=SpoutDX12");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
}

#[cfg(target_os = "windows")]
fn init_spout_submodule() {
    let status = std::process::Command::new("git")
        .args(["submodule", "update", "--init"])
        .status()
        .unwrap();

    if !status.success() {
        panic!("failed to init spout submodule");
    }
}

#[cfg(target_os = "windows")]
fn copy_spout_output(build_dir: &Path) {
    let bin_dir = build_dir.join("bin");
    let target_dir = get_target_dir();

    let spout_dll_src = bin_dir.join("Spout.dll");
    let spout_dx_dll_src = bin_dir.join("SpoutDX.dll");
    let spout_dx12_dll_src = bin_dir.join("SpoutDX12.dll");

    let spout_dll_dst = target_dir.join("Spout.dll");
    let spout_dx_dll_dst = target_dir.join("SpoutDX.dll");
    let spout_dx12_dll_dst = target_dir.join("SpoutDX12.dll");

    std::fs::copy(dbg!(spout_dll_src), dbg!(spout_dll_dst)).unwrap();
    std::fs::copy(spout_dx_dll_src, spout_dx_dll_dst).unwrap();
    std::fs::copy(spout_dx12_dll_src, spout_dx12_dll_dst).unwrap();
}

#[cfg(target_os = "windows")]
fn get_target_dir() -> PathBuf {
    let manifest_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let profile = std::env::var("PROFILE").unwrap();

    manifest_path
        .join("..")
        .join("target")
        .join(profile)
        .canonicalize()
        .unwrap()
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

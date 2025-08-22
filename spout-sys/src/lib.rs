#[cfg(target_os = "windows")]
pub mod spout;

#[cfg(target_os = "windows")]
pub use spout::*;

#[cfg(not(target_os = "windows"))]
compile_error!("spout-sys only supports Windows");

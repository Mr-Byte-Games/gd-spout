#[cfg(target_os = "windows")]
pub mod spout;

#[cfg(target_os = "windows")]
pub use spout::*;

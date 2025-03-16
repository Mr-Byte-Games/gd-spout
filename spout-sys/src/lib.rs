#[cfg(target_os = "windows")]
mod spout;

#[cfg(target_os = "windows")]
pub use spout::*;
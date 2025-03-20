#[cfg(target_os = "windows")]
mod d3d12_util;

mod node;
mod resource;

use godot::prelude::*;
pub use node::*;
pub use resource::*;

pub struct GdSpoutExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GdSpoutExtension {}

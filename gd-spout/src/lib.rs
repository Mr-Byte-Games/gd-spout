mod d3d12_util;
mod sender;

use godot::prelude::*;
pub use sender::*;

pub struct GdSpoutExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GdSpoutExtension {}

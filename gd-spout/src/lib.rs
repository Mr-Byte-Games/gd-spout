mod node;
mod resource;
mod spout;

use godot::prelude::*;
pub use node::*;
pub use resource::*;

pub struct GdSpoutExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GdSpoutExtension {}

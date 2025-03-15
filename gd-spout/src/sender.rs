use crate::d3d12_util;
use godot::classes::{Node, RenderingServer, Texture2D};
use godot::prelude::*;
use spout_sys::SpoutDX12;

#[derive(GodotClass)]
#[class(init, base=Node)]
struct SpoutSender {
    #[export]
    pub name: GString,
    #[export]
    pub texture: Option<Gd<Texture2D>>,
    spout: Option<SpoutDX12>,
    base: Base<Node>,
}

#[godot_api]
impl INode for SpoutSender {
    fn ready(&mut self) {
        let Some(mut device) = d3d12_util::get_d3d12_device() else {
            godot_warn!("Unable to find ID3d12Device");
            return;
        };

        let callable = self.base().callable("on_post_draw");
        let mut spout = SpoutDX12::new();
        spout.open(&device);
        spout.set_sender_name("beach");

        self.spout = Some(spout);

        RenderingServer::singleton().connect("frame_post_draw", &callable);
    }
}

#[godot_api]
impl SpoutSender {
    #[func]
    fn on_post_draw(&mut self) {
        let Some(texture) = &self.texture else {
            return;
        };

        let Some(resource) = d3d12_util::get_d3d12_resource_from_texture(texture) else {
            return;
        };

        let desc = unsafe { resource.GetDesc() };
        godot_print!("Width: {}, Height: {}", desc.Width, desc.Height);

        let Some(spout) = &mut self.spout else {
            godot_warn!("No spout sender available.");
            return;
        };

        spout.send_texture(&resource);
    }
}

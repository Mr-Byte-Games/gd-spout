#[cfg(target_os = "windows")]
use crate::d3d12_util;
use godot::classes::{Node, RenderingServer, Texture2D};
use godot::prelude::*;
use std::sync::{Arc, LazyLock};

#[cfg(target_os = "windows")]
use spout_sys::SpoutDX12;

thread_local! {
    static RENDERING_DRIVER_D3D12: GString = "d3d12".into();
}

#[derive(GodotClass)]
#[class(init, base=Node)]
struct SpoutSender {
    #[export]
    pub name: GString,
    #[export]
    pub texture: Option<Gd<Texture2D>>,
    #[cfg(target_os = "windows")]
    spout: Option<SpoutDX12>,
    base: Base<Node>,
}

#[godot_api]
impl INode for SpoutSender {
    #[cfg(target_os = "windows")]
    fn exit_tree(&mut self) {
        let Some(spout) = &mut self.spout else {
            return;
        };

        spout.release_sender();
    }

    #[cfg(target_os = "windows")]
    fn ready(&mut self) {
        let driver_name = RenderingServer::singleton().get_current_rendering_driver_name();
        let is_d3d12 = RENDERING_DRIVER_D3D12.with(|driver_d3d12| *driver_d3d12 == driver_name);

        if !is_d3d12 {
            godot_warn!("Rendering driver is not configured to D3D12, SpoutSender is disabled");
            return;
        }

        let Some(device) = d3d12_util::get_d3d12_device() else {
            godot_warn!("Unable to find ID3D12Device");
            return;
        };

        let callable = self.base().callable("on_post_draw");
        let mut spout = SpoutDX12::new();
        spout.open(device);
        spout.set_sender_name(self.name.to_string());

        self.spout = Some(spout);

        RenderingServer::singleton().connect("frame_post_draw", &callable);
    }
}

#[cfg(target_os = "windows")]
#[godot_api]
impl SpoutSender {
    #[func]
    fn on_post_draw(&mut self) {
        let Some(spout) = &mut self.spout else {
            godot_warn!("No spout sender available.");
            return;
        };

        let Some(texture) = &self.texture else {
            godot_warn!("No texture available.");
            return;
        };

        let Some(resource) = d3d12_util::get_d3d12_resource_from_texture(texture) else {
            godot_warn!("Unable to obtain texture resource.");
            return;
        };

        spout.send_resource(resource);
    }
}

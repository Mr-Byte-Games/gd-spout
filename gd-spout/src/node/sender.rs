use godot::classes::{Engine, Node, RenderingServer, Texture2D};
use godot::prelude::*;

use crate::spout;
use crate::spout::sender::create_sender;

thread_local! {
    static RENDERING_DRIVER_D3D12: GString = "d3d12".into();
}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct SpoutSender {
    #[export]
    #[var(set = set_name)]
    name: GString,
    #[export]
    texture: Option<Gd<Texture2D>>,
    callback: Option<Callable>,
    spout: Option<Box<dyn spout::sender::SpoutSender>>,
    base: Base<Node>,
}

impl Drop for SpoutSender {
    fn drop(&mut self) {
        if let Some(callback) = self.callback.take() {
            RenderingServer::singleton().disconnect("frame_post_draw", &callback);
        }
    }
}

#[godot_api]
impl INode for SpoutSender {
    fn exit_tree(&mut self) {
        if let Some(callback) = self.callback.take() {
            RenderingServer::singleton().disconnect("frame_post_draw", &callback);
        }
    }

    fn ready(&mut self) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        if let Some(callback) = self.callback.take() {
            RenderingServer::singleton().disconnect("frame_post_draw", &callback);
        }

        let driver_name = RenderingServer::singleton()
            .get_current_rendering_driver_name()
            .to_string();

        let mut spout = create_sender(&driver_name);
        spout.set_sender_name(&self.name.to_string());
        self.spout = Some(spout);

        let callable = self.base().callable("on_post_draw");
        RenderingServer::singleton().connect("frame_post_draw", &callable);
        self.callback = Some(callable);
    }
}
#[godot_api]
impl SpoutSender {
    #[func]
    fn set_name(&mut self, name: GString) {
        if let Some(spout) = &mut self.spout {
            spout.set_sender_name(&name.to_string());
        }

        self.name = name;
    }

    #[func]
    fn on_post_draw(&mut self) {
        let Some(spout) = &mut self.spout else {
            godot_error!("No spout sender available.");
            return;
        };

        let Some(texture) = &self.texture else {
            godot_error!("No texture available.");
            return;
        };

        let source_rid = RenderingServer::singleton().texture_get_rd_texture(texture.get_rid());

        spout.send_resource(source_rid);
    }
}

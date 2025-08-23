use crate::spout::{SpoutReceiver, create_receiver};
use godot::classes::{ITexture2D, RenderingServer, Texture2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(tool, base=Texture2D)]
pub struct SpoutReceiverTexture {
    #[var(get = get_sender_name, set = set_sender_name)]
    #[export]
    sender_name: GString,
    spout_receiver: Box<dyn SpoutReceiver>,
    pre_draw_callback: Option<Callable>,
    base: Base<Texture2D>,
}

impl Drop for SpoutReceiverTexture {
    fn drop(&mut self) {
        if let Some(callback) = self.pre_draw_callback.take() {
            RenderingServer::singleton().disconnect("frame_pre_draw", &callback);
        }
    }
}

#[godot_api]
impl ITexture2D for SpoutReceiverTexture {
    fn init(base: Base<Texture2D>) -> SpoutReceiverTexture {
        let driver_name = RenderingServer::singleton()
            .get_current_rendering_driver_name()
            .to_string();

        Self {
            sender_name: GString::new(),
            spout_receiver: create_receiver(&driver_name),
            pre_draw_callback: None,
            base,
        }
    }

    fn get_width(&self) -> i32 {
        self.spout_receiver.width()
    }

    fn get_height(&self) -> i32 {
        self.spout_receiver.height()
    }

    fn get_rid(&self) -> Rid {
        self.spout_receiver.rid()
    }
}

#[godot_api]
impl SpoutReceiverTexture {
    #[func]
    fn get_sender_name(&self) -> GString {
        self.sender_name.clone()
    }

    #[func]
    fn set_sender_name(&mut self, sender_name: GString) {
        if let Some(callback) = self.pre_draw_callback.take() {
            RenderingServer::singleton().disconnect("frame_pre_draw", &callback);
        }

        let callable = self.base().callable("on_pre_draw");
        RenderingServer::singleton().connect("frame_pre_draw", &callable);

        self.spout_receiver.set_sender_name(&sender_name.to_string());
        self.sender_name = sender_name;
        self.pre_draw_callback = Some(callable);
        self.base_mut().emit_changed();
    }

    #[func]
    fn on_pre_draw(&mut self) {
        if self.spout_receiver.update_resource() {
            self.base_mut().emit_changed();
        }
    }
}

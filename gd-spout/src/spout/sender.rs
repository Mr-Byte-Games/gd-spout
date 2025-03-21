use godot::prelude::*;

#[cfg(target_os = "windows")]
mod dx12;

pub trait SpoutSender {
    fn set_sender_name(&mut self, name: &str);
    fn send_texture(&mut self, resource: Rid);
}
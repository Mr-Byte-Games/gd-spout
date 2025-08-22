use crate::spout::d3d12_util;
use crate::spout::d3d12_util::{get_d3d12_command_queue, get_d3d12_device};
use crate::spout::sender::SpoutSender;
use godot::prelude::*;
use spout_sys::SpoutDX12;

mod spout_sender_manager;

pub struct D3D12SpoutSender {
    spout: SpoutDX12,
}

impl Drop for D3D12SpoutSender {
    fn drop(&mut self) {
        self.spout.release_sender()
    }
}

impl D3D12SpoutSender {
    pub fn new() -> Result<Box<dyn SpoutSender>, Box<dyn std::error::Error>> {
        let Some(device) = get_d3d12_device() else {
            return Err("Unable to obtain D3D12 Device".into());
        };

        let Some(command_queue) = get_d3d12_command_queue() else {
            return Err("Unable to obtain D3D12 Command Queue".into());
        };

        let spout = SpoutDX12::new_with_queue(device, command_queue);

        Ok(Box::new(Self { spout }))
    }
}

impl SpoutSender for D3D12SpoutSender {
    fn set_sender_name(&mut self, name: &str) {
        if !self.spout.set_sender_name(name) {
            godot_error!("Unable to set sender name.");
        }
    }

    fn send_resource(&mut self, resource: Rid) {
        let Some(resource) = d3d12_util::get_d3d12_resource_from_texture(resource) else {
            godot_error!("Given RID returned invalid D3D12 resource.");
            return;
        };

        self.spout.send_resource(resource);
    }
}

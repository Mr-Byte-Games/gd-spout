use crate::spout::dx12::spout;
use crate::spout::sender::SpoutSender;
use godot::prelude::*;

pub struct D3D12SpoutSender {
    spout_sender: spout::Sender,
}

impl D3D12SpoutSender {
    pub fn new() -> Result<Box<dyn SpoutSender>, Box<dyn std::error::Error>> {
        let Some(device) = super::godot::get_d3d12_device() else {
            return Err("Unable to obtain D3D12 Device".into());
        };

        let Some(command_queue) = super::godot::get_d3d12_command_queue() else {
            return Err("Unable to obtain D3D12 Command Queue".into());
        };

        let spout_sender = spout::Sender::new(device, command_queue)?;

        Ok(Box::new(Self { spout_sender }))
    }
}

impl SpoutSender for D3D12SpoutSender {
    fn set_sender_name(&mut self, name: &str) {
        if !self.spout_sender.set_sender_name(name) {
            godot_error!("Unable to set sender name.");
        }
    }

    fn send_resource(&mut self, resource: Rid) {
        let Some(resource) = super::godot::get_d3d12_resource_from_texture(resource) else {
            godot_error!("Given RID returned invalid D3D12 resource.");
            return;
        };

        self.spout_sender.send_resource(resource);
    }
}

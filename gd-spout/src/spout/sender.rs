use godot::prelude::*;
use std::error::Error;

#[cfg(target_os = "windows")]
mod dx12;
mod no_op;

pub trait SpoutSender {
    fn set_sender_name(&mut self, name: &str);
    fn send_resource(&mut self, resource: Rid);
}

pub fn create_sender(driver_name: &str) -> Box<dyn SpoutSender> {
    let receiver = match driver_name {
        #[cfg(target_os = "windows")]
        "d3d12" => dx12::D3D12SpoutSender::new(),
        _ => Ok(no_op::NoOpSender::new()),
    };

    receiver.unwrap_or_else(|err: Box<dyn Error>| {
        godot_error!("{err}; Failed to create sender: {driver_name}; Falling back on no op implementation.");
        no_op::NoOpSender::new()
    })
}

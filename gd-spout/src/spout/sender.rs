use godot::prelude::*;
use no_op::NoOpSender;

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
        _ => Ok(NoOpSender::new()),
    };

    receiver.unwrap_or_else(|err| {
        godot_error!("{err}; Failed to create sender: {driver_name}; Falling back on no op implementation.");
        Box::new(NoOpSender)
    })
}

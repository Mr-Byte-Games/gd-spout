use crate::spout::{dx12, no_op};
use godot::prelude::*;
use std::error::Error;

pub trait SpoutReceiver {
    fn rid(&self) -> Rid;
    fn set_sender_name(&mut self, name: &str);
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn update_resource(&mut self) -> bool;
}

pub fn create_receiver(driver_name: &str) -> Box<dyn SpoutReceiver> {
    let receiver = match driver_name {
        #[cfg(target_os = "windows")]
        "d3d12" => dx12::D3D12SpoutReceiver::new(),
        _ => Ok(no_op::NoOpReceiver::new()),
    };

    receiver.unwrap_or_else(|err: Box<dyn Error>| {
        godot_error!("{err}; Failed to create receiver: {driver_name}; Falling back on no op implementation.");
        no_op::NoOpReceiver::new()
    })
}

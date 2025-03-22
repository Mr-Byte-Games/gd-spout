#[cfg(target_os = "windows")]
mod dx12;
mod no_op;

#[cfg(target_os = "windows")]
use dx12::D3D12SpoutReceiver;
use godot::builtin::Rid;
use godot::global::godot_error;
use no_op::NoOpReceiver;

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
        "d3d12" => D3D12SpoutReceiver::new(),
        _ => Ok(NoOpReceiver::new()),
    };

    receiver.unwrap_or_else(|err| {
        godot_error!("{err}");
        NoOpReceiver::new()
    })
}

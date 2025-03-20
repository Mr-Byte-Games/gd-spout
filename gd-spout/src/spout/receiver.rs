#[cfg(target_os = "windows")]
mod dx12;

#[cfg(target_os = "windows")]
use dx12::D3D12SpoutReceiver;
use godot::builtin::Rid;

pub trait SpoutReceiver {
    fn rid(&self) -> Rid;
    fn set_sender_name(&mut self, name: &str);
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn update_resource(&mut self) -> bool;
}

pub fn create_receiver(driver_name: &str) -> Box<dyn SpoutReceiver> {
    match driver_name {
        #[cfg(target_os = "windows")]
        "d3d12" => Box::new(D3D12SpoutReceiver::new()),
        _ => unimplemented!(),
    }
}

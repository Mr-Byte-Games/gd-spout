use godot::builtin::Rid;
use godot::classes::rendering_device::DriverResource;
use godot::classes::{RenderingServer, Texture2D};
use windows::Win32::Graphics::Direct3D12::{ID3D12Device, ID3D12Resource};

pub fn get_d3d12_resource_from_texture(texture: &Texture2D) -> Option<ID3D12Resource> {
    let texture_rid = texture.get_rid();
    let resource_id = RenderingServer::singleton().texture_get_native_handle(texture_rid);
    let resource = resource_id as *const ID3D12Resource;

    unsafe { resource.as_ref().cloned() }
}

pub fn get_d3d12_device() -> Option<ID3D12Device> {
    let mut device = RenderingServer::singleton().get_rendering_device()?;
    let logical_device_id =
        device.get_driver_resource(DriverResource::LOGICAL_DEVICE, Rid::Invalid, 0);

    unsafe { Some(std::mem::transmute(logical_device_id)) }
}


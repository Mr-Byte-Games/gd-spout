use godot::builtin::Rid;
use godot::classes::rendering_device::DriverResource;
use godot::classes::{RenderingServer, Texture2D};
use spout_sys::{ID3D12Device, ID3D12Resource};
use std::ptr::NonNull;

pub fn get_d3d12_resource_from_texture(texture: &Texture2D) -> Option<NonNull<ID3D12Resource>> {
    let texture_rid = texture.get_rid();
    let resource_id = RenderingServer::singleton().texture_get_native_handle(texture_rid);
    let resource = resource_id as *mut *mut ID3D12Resource;

    NonNull::new(resource)
        .map(|outer| unsafe { *outer.as_ptr() })
        .and_then(NonNull::new)
}

pub fn get_d3d12_device() -> Option<NonNull<ID3D12Device>> {
    let mut device = RenderingServer::singleton().get_rendering_device()?;
    let logical_device_id = device.get_driver_resource(DriverResource::LOGICAL_DEVICE, Rid::Invalid, 0);

    NonNull::new(logical_device_id as *mut ID3D12Device)
}

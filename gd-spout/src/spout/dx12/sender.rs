use crate::spout::dx12::fence::Fence;
use crate::spout::sender::SpoutSender;
use godot::prelude::*;
use spout_sys::{ID3D11Resource, Spout};
use std::ptr::NonNull;
use windows::Win32::Graphics::Direct3D12::{ID3D12CommandQueue, ID3D12Device, ID3D12Resource};
use windows::core::Interface;

pub struct D3D12SpoutSender {
    spout_sender: Sender,
}

impl D3D12SpoutSender {
    pub fn new() -> Result<Box<dyn SpoutSender>, Box<dyn std::error::Error>> {
        let Some(device) = super::godot::get_d3d12_device() else {
            return Err("Unable to obtain D3D12 Device".into());
        };

        let Some(command_queue) = super::godot::get_d3d12_command_queue() else {
            return Err("Unable to obtain D3D12 Command Queue".into());
        };

        let spout_sender = Sender::new(device, command_queue)?;

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
pub struct Sender {
    spout: Spout,
    fence: Fence,
    cached_resource: Option<NonNull<ID3D11Resource>>,
    sender_name: String,
}

impl Sender {
    pub fn new(device: ID3D12Device, command_queue: ID3D12CommandQueue) -> Result<Self, Box<dyn std::error::Error>> {
        let spout = unsafe { spout_sys::new(device.as_raw() as *mut spout_sys::ID3D12Device) };
        let fence =
            Fence::new(&device, command_queue).map_err(|e| format!("Failed to create sender fence manager: {}", e))?;

        Ok(Self {
            spout,
            fence,
            cached_resource: None,
            sender_name: String::new(),
        })
    }

    pub fn set_sender_name(&mut self, name: &str) -> bool {
        let success = self.spout.set_sender_name("");
        if success {
            self.sender_name = name.to_string();
        }
        success
    }

    pub fn send_resource(&mut self, dx12_resource: ID3D12Resource) -> bool {
        if self.fence.wait_for_gpu().is_err() {
            godot_error!("Failed to wait for GPU completion before sending");
            return false;
        }

        // TODO: Check if incoming resource has changed.

        if let Some(ref mut cached_resource) = self.cached_resource {
            return unsafe { self.spout.send_dx11_resource(cached_resource.as_mut()) };
        }

        let mut dx11_resource: *mut ID3D11Resource = std::ptr::null_mut();

        let success = unsafe {
            self.spout.wrap_dx12_resource(
                dx12_resource.as_raw() as *mut spout_sys::ID3D12Resource,
                &mut dx11_resource,
            )
        };

        if !success || dx11_resource.is_null() {
            godot_error!("Failed to wrap D3D12 resource for sending");
            return false;
        }

        let dx11_resource = unsafe { NonNull::new_unchecked(dx11_resource) };
        self.cached_resource = Some(dx11_resource);

        unsafe { self.spout.send_dx11_resource(dx11_resource.as_ptr()) }
    }

    pub fn release_sender(&mut self) {
        self.spout.release_sender();
        self.cached_resource = None;
        self.sender_name.clear();
    }
}

impl Drop for Sender {
    fn drop(&mut self) {
        self.release_sender();
    }
}

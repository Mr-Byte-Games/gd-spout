use crate::spout::dx12::fence::Fence;
use godot::prelude::*;
use spout_sys::{ID3D11Resource, Spout};
use std::ptr::NonNull;
use windows::Win32::Graphics::Direct3D12::ID3D12Resource;
use windows::{
    Win32::Graphics::Direct3D12::{ID3D12CommandQueue, ID3D12Device},
    core::Interface,
};

pub struct Sender {
    inner: Spout,
    cached_resource: Option<NonNull<ID3D11Resource>>,
    fence: Option<Fence>,
    sender_name: String,
}

impl Sender {
    pub fn new(device: ID3D12Device, command_queue: ID3D12CommandQueue) -> Result<Self, Box<dyn std::error::Error>> {
        let inner = unsafe { spout_sys::new(device.as_raw() as *mut spout_sys::ID3D12Device) };

        let fence =
            Fence::new(&device, command_queue).map_err(|e| format!("Failed to create sender fence manager: {}", e))?;

        Ok(Self {
            inner,
            cached_resource: None,
            fence: Some(fence),
            sender_name: String::new(),
        })
    }

    pub fn set_sender_name(&mut self, name: &str) -> bool {
        let success = self.inner.set_sender_name("");
        if success {
            self.sender_name = name.to_string();
        }
        success
    }

    pub fn get_sender_name(&self) -> &str {
        &self.sender_name
    }

    pub fn send_resource(&mut self, dx12_resource: ID3D12Resource) -> bool {
        if let Some(ref mut fence_manager) = self.fence {
            if fence_manager.wait_for_gpu().is_err() {
                godot_error!("Failed to wait for GPU completion before sending");
                return false;
            }
        }

        if let Some(ref mut cached_resource) = self.cached_resource {
            return unsafe { self.inner.send_dx11_resource(cached_resource.as_mut()) };
        }

        let mut dx11_resource: *mut ID3D11Resource = std::ptr::null_mut();

        let success = unsafe {
            self.inner.wrap_dx12_resource(
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

        unsafe { self.inner.send_dx11_resource(dx11_resource.as_ptr()) }
    }

    pub fn release_sender(&mut self) {
        self.inner.release_sender();
        self.cached_resource = None;
        self.sender_name.clear();
    }

    pub fn is_initialized(&self) -> bool {
        !self.sender_name.is_empty()
    }
}

impl Drop for Sender {
    fn drop(&mut self) {
        self.release_sender();
    }
}

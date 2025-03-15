use cxx::{let_cxx_string, UniquePtr};
use windows::Win32::Graphics::Direct3D12::{ID3D12Device, ID3D12Resource};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("spout-sys/include/spout.h");

        type SpoutDX12;
        type DeviceHandle;
        type TextureHandle;

        unsafe fn open(self: &SpoutDX12, device: *const DeviceHandle) -> bool;
        unsafe fn send_texture(self: &SpoutDX12, texture: *const TextureHandle) -> bool;
        unsafe fn set_sender_name(self: &SpoutDX12, name: &CxxString) -> bool;

        fn new_spout_dx12() -> UniquePtr<SpoutDX12>;
    }
}

pub struct SpoutDX12 {
    inner: UniquePtr<ffi::SpoutDX12>,
}

impl SpoutDX12 {
    pub fn new() -> Self {
        Self {
            inner: ffi::new_spout_dx12(),
        }
    }

    pub fn open(&mut self, device: &ID3D12Device) -> bool {
        unsafe {
            let device = std::mem::transmute(device);
            let result = self.inner.open(device);

            result
        }
    }

    pub fn set_sender_name(&mut self, name: &str) -> bool {
        let_cxx_string!(cxx_name = name);
        unsafe { self.inner.set_sender_name(&cxx_name) }
    }

    pub fn send_texture(&mut self, texture: &ID3D12Resource) -> bool {
        unsafe {
            let device = std::mem::transmute(texture);
             self.inner.send_texture(device)
        }
    }
}

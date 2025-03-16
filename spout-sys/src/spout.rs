#![cfg(target_os = "windows")]

use cxx::{UniquePtr, let_cxx_string};
use std::ptr::NonNull;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("spout-sys/include/spout.h");

        type SpoutDX12;
        type ID3D12Device;
        type ID3D12Resource;

        unsafe fn open(self: &SpoutDX12, device: *mut ID3D12Device) -> bool;
        unsafe fn send_resource(self: Pin<&mut SpoutDX12>, resource: *mut ID3D12Resource) -> bool;
        unsafe fn set_sender_name(self: &SpoutDX12, name: &CxxString) -> bool;
        fn release_sender(self: &SpoutDX12);

        fn new_spout_dx12() -> UniquePtr<SpoutDX12>;
    }
}

pub use ffi::ID3D12Device;
pub use ffi::ID3D12Resource;

pub struct SpoutDX12 {
    inner: UniquePtr<ffi::SpoutDX12>,
}

impl SpoutDX12 {
    pub fn new() -> Self {
        Self {
            inner: ffi::new_spout_dx12(),
        }
    }

    pub fn open(&mut self, device: NonNull<ID3D12Device>) -> bool {
        unsafe { self.inner.open(device.as_ptr()) }
    }

    pub fn set_sender_name(&mut self, name: impl AsRef<[u8]>) -> bool {
        let_cxx_string!(cxx_name = name);

        unsafe { self.inner.set_sender_name(&cxx_name) }
    }

    pub fn send_resource(&mut self, texture: NonNull<ID3D12Resource>) -> bool {
        let Some(inner) = self.inner.as_mut() else {
            return false;
        };

        unsafe { inner.send_resource(texture.as_ptr()) }
    }

    pub fn release_sender(&mut self) {
        self.inner.release_sender();
    }
}

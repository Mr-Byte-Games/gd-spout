#![cfg(target_os = "windows")]

use cxx::{UniquePtr, let_cxx_string};
use std::ptr::NonNull;

#[cxx::bridge]
mod ffi {
    #[derive(Debug, Copy, Clone)]
    enum DXGI_FORMAT {
        DXGI_FORMAT_UNKNOWN = 0x0,
        DXGI_FORMAT_R32G32B32A32_TYPELESS = 0x1,
        DXGI_FORMAT_R32G32B32A32_FLOAT = 0x2,
        DXGI_FORMAT_R32G32B32A32_UINT = 0x3,
        DXGI_FORMAT_R32G32B32A32_SINT = 0x4,
        DXGI_FORMAT_R32G32B32_TYPELESS = 0x5,
        DXGI_FORMAT_R32G32B32_FLOAT = 0x6,
        DXGI_FORMAT_R32G32B32_UINT = 0x7,
        DXGI_FORMAT_R32G32B32_SINT = 0x8,
        DXGI_FORMAT_R16G16B16A16_TYPELESS = 0x9,
        DXGI_FORMAT_R16G16B16A16_FLOAT = 0xa,
        DXGI_FORMAT_R16G16B16A16_UNORM = 0xb,
        DXGI_FORMAT_R16G16B16A16_UINT = 0xc,
        DXGI_FORMAT_R16G16B16A16_SNORM = 0xd,
        DXGI_FORMAT_R16G16B16A16_SINT = 0xe,
        DXGI_FORMAT_R32G32_TYPELESS = 0xf,
        DXGI_FORMAT_R32G32_FLOAT = 0x10,
        DXGI_FORMAT_R32G32_UINT = 0x11,
        DXGI_FORMAT_R32G32_SINT = 0x12,
        DXGI_FORMAT_R32G8X24_TYPELESS = 0x13,
        DXGI_FORMAT_D32_FLOAT_S8X24_UINT = 0x14,
        DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS = 0x15,
        DXGI_FORMAT_X32_TYPELESS_G8X24_UINT = 0x16,
        DXGI_FORMAT_R10G10B10A2_TYPELESS = 0x17,
        DXGI_FORMAT_R10G10B10A2_UNORM = 0x18,
        DXGI_FORMAT_R10G10B10A2_UINT = 0x19,
        DXGI_FORMAT_R11G11B10_FLOAT = 0x1a,
        DXGI_FORMAT_R8G8B8A8_TYPELESS = 0x1b,
        DXGI_FORMAT_R8G8B8A8_UNORM = 0x1c,
        DXGI_FORMAT_R8G8B8A8_UNORM_SRGB = 0x1d,
        DXGI_FORMAT_R8G8B8A8_UINT = 0x1e,
        DXGI_FORMAT_R8G8B8A8_SNORM = 0x1f,
        DXGI_FORMAT_R8G8B8A8_SINT = 0x20,
        DXGI_FORMAT_R16G16_TYPELESS = 0x21,
        DXGI_FORMAT_R16G16_FLOAT = 0x22,
        DXGI_FORMAT_R16G16_UNORM = 0x23,
        DXGI_FORMAT_R16G16_UINT = 0x24,
        DXGI_FORMAT_R16G16_SNORM = 0x25,
        DXGI_FORMAT_R16G16_SINT = 0x26,
        DXGI_FORMAT_R32_TYPELESS = 0x27,
        DXGI_FORMAT_D32_FLOAT = 0x28,
        DXGI_FORMAT_R32_FLOAT = 0x29,
        DXGI_FORMAT_R32_UINT = 0x2a,
        DXGI_FORMAT_R32_SINT = 0x2b,
        DXGI_FORMAT_R24G8_TYPELESS = 0x2c,
        DXGI_FORMAT_D24_UNORM_S8_UINT = 0x2d,
        DXGI_FORMAT_R24_UNORM_X8_TYPELESS = 0x2e,
        DXGI_FORMAT_X24_TYPELESS_G8_UINT = 0x2f,
        DXGI_FORMAT_R8G8_TYPELESS = 0x30,
        DXGI_FORMAT_R8G8_UNORM = 0x31,
        DXGI_FORMAT_R8G8_UINT = 0x32,
        DXGI_FORMAT_R8G8_SNORM = 0x33,
        DXGI_FORMAT_R8G8_SINT = 0x34,
        DXGI_FORMAT_R16_TYPELESS = 0x35,
        DXGI_FORMAT_R16_FLOAT = 0x36,
        DXGI_FORMAT_D16_UNORM = 0x37,
        DXGI_FORMAT_R16_UNORM = 0x38,
        DXGI_FORMAT_R16_UINT = 0x39,
        DXGI_FORMAT_R16_SNORM = 0x3a,
        DXGI_FORMAT_R16_SINT = 0x3b,
        DXGI_FORMAT_R8_TYPELESS = 0x3c,
        DXGI_FORMAT_R8_UNORM = 0x3d,
        DXGI_FORMAT_R8_UINT = 0x3e,
        DXGI_FORMAT_R8_SNORM = 0x3f,
        DXGI_FORMAT_R8_SINT = 0x40,
        DXGI_FORMAT_A8_UNORM = 0x41,
        DXGI_FORMAT_R1_UNORM = 0x42,
        DXGI_FORMAT_R9G9B9E5_SHAREDEXP = 0x43,
        DXGI_FORMAT_R8G8_B8G8_UNORM = 0x44,
        DXGI_FORMAT_G8R8_G8B8_UNORM = 0x45,
        DXGI_FORMAT_BC1_TYPELESS = 0x46,
        DXGI_FORMAT_BC1_UNORM = 0x47,
        DXGI_FORMAT_BC1_UNORM_SRGB = 0x48,
        DXGI_FORMAT_BC2_TYPELESS = 0x49,
        DXGI_FORMAT_BC2_UNORM = 0x4a,
        DXGI_FORMAT_BC2_UNORM_SRGB = 0x4b,
        DXGI_FORMAT_BC3_TYPELESS = 0x4c,
        DXGI_FORMAT_BC3_UNORM = 0x4d,
        DXGI_FORMAT_BC3_UNORM_SRGB = 0x4e,
        DXGI_FORMAT_BC4_TYPELESS = 0x4f,
        DXGI_FORMAT_BC4_UNORM = 0x50,
        DXGI_FORMAT_BC4_SNORM = 0x51,
        DXGI_FORMAT_BC5_TYPELESS = 0x52,
        DXGI_FORMAT_BC5_UNORM = 0x53,
        DXGI_FORMAT_BC5_SNORM = 0x54,
        DXGI_FORMAT_B5G6R5_UNORM = 0x55,
        DXGI_FORMAT_B5G5R5A1_UNORM = 0x56,
        DXGI_FORMAT_B8G8R8A8_UNORM = 0x57,
        DXGI_FORMAT_B8G8R8X8_UNORM = 0x58,
        DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM = 0x59,
        DXGI_FORMAT_B8G8R8A8_TYPELESS = 0x5a,
        DXGI_FORMAT_B8G8R8A8_UNORM_SRGB = 0x5b,
        DXGI_FORMAT_B8G8R8X8_TYPELESS = 0x5c,
        DXGI_FORMAT_B8G8R8X8_UNORM_SRGB = 0x5d,
        DXGI_FORMAT_BC6H_TYPELESS = 0x5e,
        DXGI_FORMAT_BC6H_UF16 = 0x5f,
        DXGI_FORMAT_BC6H_SF16 = 0x60,
        DXGI_FORMAT_BC7_TYPELESS = 0x61,
        DXGI_FORMAT_BC7_UNORM = 0x62,
        DXGI_FORMAT_BC7_UNORM_SRGB = 0x63,
        DXGI_FORMAT_AYUV = 0x64,
        DXGI_FORMAT_Y410 = 0x65,
        DXGI_FORMAT_Y416 = 0x66,
        DXGI_FORMAT_NV12 = 0x67,
        DXGI_FORMAT_P010 = 0x68,
        DXGI_FORMAT_P016 = 0x69,
        DXGI_FORMAT_420_OPAQUE = 0x6a,
        DXGI_FORMAT_YUY2 = 0x6b,
        DXGI_FORMAT_Y210 = 0x6c,
        DXGI_FORMAT_Y216 = 0x6d,
        DXGI_FORMAT_NV11 = 0x6e,
        DXGI_FORMAT_AI44 = 0x6f,
        DXGI_FORMAT_IA44 = 0x70,
        DXGI_FORMAT_P8 = 0x71,
        DXGI_FORMAT_A8P8 = 0x72,
        DXGI_FORMAT_B4G4R4A4_UNORM = 0x73,
        DXGI_FORMAT_P208 = 0x82,
        DXGI_FORMAT_V208 = 0x83,
        DXGI_FORMAT_V408 = 0x84,
        DXGI_FORMAT_FORCE_UINT = 0xffffffff,
    }

    unsafe extern "C++" {
        include!("spout-sys/include/spout.h");

        type SpoutDX12;
        type ID3D12Device;
        type ID3D12Resource;
        type DXGI_FORMAT;

        unsafe fn send_resource(self: Pin<&mut SpoutDX12>, resource: *mut ID3D12Resource) -> bool;
        unsafe fn receive_resource(self: &SpoutDX12, resource: *mut *mut ID3D12Resource) -> bool;
        unsafe fn create_receiver_resource(
            self: &SpoutDX12,
            device: *mut ID3D12Device,
            resource: *mut *mut ID3D12Resource,
        ) -> bool;
        fn set_sender_name(self: &SpoutDX12, name: &CxxString) -> bool;
        fn set_receiver_name(self: &SpoutDX12, name: &CxxString);
        fn release_sender(self: &SpoutDX12);
        fn release_receiver(self: &SpoutDX12);
        fn get_sender_width(self: &SpoutDX12) -> u32;
        fn get_sender_height(self: &SpoutDX12) -> u32;
        fn get_sender_format(self: &SpoutDX12) -> DXGI_FORMAT;
        fn is_updated(self: &SpoutDX12) -> bool;

        unsafe fn new_spout_dx12(device: *mut ID3D12Device) -> UniquePtr<SpoutDX12>;
    }
}

pub use ffi::DXGI_FORMAT;
pub use ffi::ID3D12Device;
pub use ffi::ID3D12Resource;

pub struct SpoutDX12 {
    inner: UniquePtr<ffi::SpoutDX12>,
}

impl SpoutDX12 {
    pub fn new(device: NonNull<ID3D12Device>) -> Self {
        Self {
            inner: unsafe { ffi::new_spout_dx12(device.as_ptr()) },
        }
    }

    pub fn set_sender_name(&mut self, name: impl AsRef<[u8]>) -> bool {
        let_cxx_string!(cxx_name = name);

        self.inner.set_sender_name(&cxx_name)
    }

    pub fn set_receiver_name(&mut self, name: impl AsRef<[u8]>) {
        let_cxx_string!(cxx_name = name);

        self.inner.set_receiver_name(&cxx_name)
    }

    pub fn send_resource(&mut self, texture: NonNull<ID3D12Resource>) -> bool {
        let Some(inner) = self.inner.as_mut() else {
            return false;
        };

        unsafe { inner.send_resource(texture.as_ptr()) }
    }

    pub fn receive_resource(&self, resource: &mut Option<NonNull<ID3D12Resource>>) -> bool {
        unsafe {
            let resource: *mut *mut ID3D12Resource = std::mem::transmute(resource);
            self.inner.receive_resource(resource)
        }
    }

    pub fn create_receiver_resource(
        &self,
        device: NonNull<ID3D12Device>,
        resource: &mut Option<NonNull<ID3D12Resource>>,
    ) -> bool {
        unsafe {
            let resource: *mut *mut ID3D12Resource = std::mem::transmute(resource);
            self.inner.create_receiver_resource(device.as_ptr(), resource)
        }
    }

    pub fn release_sender(&mut self) {
        self.inner.release_sender();
    }

    pub fn release_receiver(&mut self) {
        self.inner.release_receiver();
    }

    pub fn get_sender_width(&self) -> u32 {
        self.inner.get_sender_width()
    }

    pub fn get_sender_height(&self) -> u32 {
        self.inner.get_sender_height()
    }

    pub fn get_sender_format(&self) -> ffi::DXGI_FORMAT {
        self.inner.get_sender_format()
    }

    pub fn is_updated(&self) -> bool {
        self.inner.is_updated()
    }
}

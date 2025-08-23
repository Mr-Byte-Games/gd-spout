use crate::spout::dx12::godot::{convert_dxgi_to_rd_data_format, get_d3d12_device};
use crate::spout::receiver::SpoutReceiver;
use godot::classes::RenderingServer;
use godot::classes::rendering_device::{TextureSamples, TextureType, TextureUsageBits};
use godot::prelude::*;
use spout_sys::{ID3D12Resource, Spout};
use std::ptr::NonNull;
use windows::Win32::Graphics::Direct3D12::ID3D12Device;
use windows::core::Interface;

pub struct D3D12SpoutReceiver {
    spout: Spout,
    rd_texture_rid: Rid,
    rs_texture_rid: Rid,
    texture_resource: Option<NonNull<ID3D12Resource>>,
    device: ID3D12Device,
}

impl Drop for D3D12SpoutReceiver {
    fn drop(&mut self) {
        self.spout.release_receiver();
        self.free_godot_resources();
    }
}

impl D3D12SpoutReceiver {
    pub fn new() -> Result<Box<dyn SpoutReceiver>, Box<dyn std::error::Error>> {
        let Some(device) = get_d3d12_device() else {
            return Err("Unable to obtain D3D12 Device".into());
        };

        let spout = unsafe { spout_sys::new(device.as_raw() as *mut spout_sys::ID3D12Device) };
        let rs_texture_rid = RenderingServer::singleton().texture_2d_placeholder_create();

        Ok(Box::new(Self {
            spout,
            rs_texture_rid,
            rd_texture_rid: Rid::Invalid,
            texture_resource: None,
            device,
        }))
    }
}

impl SpoutReceiver for D3D12SpoutReceiver {
    fn rid(&self) -> Rid {
        self.rs_texture_rid
    }

    fn set_sender_name(&mut self, name: &str) {
        self.spout.set_receiver_name(name);
    }

    fn width(&self) -> i32 {
        self.spout.get_sender_width() as i32
    }

    fn height(&self) -> i32 {
        self.spout.get_sender_height() as i32
    }

    fn update_resource(&mut self) -> bool {
        let Some(resource) = self.update_spout_resource() else {
            return false;
        };

        self.free_godot_resources();
        self.update_godot_resources(resource);
        true
    }
}

impl D3D12SpoutReceiver {
    fn update_spout_resource(&mut self) -> Option<NonNull<ID3D12Resource>> {
        let resource: *mut *mut ID3D12Resource = unsafe { std::mem::transmute(&mut self.texture_resource) };
        let success = unsafe { self.spout.receive_dx12_resource(resource) };

        if !success || !self.spout.is_updated() {
            return None;
        }

        unsafe {
            self.spout
                .create_dx12_texture(self.device.as_raw() as *mut spout_sys::ID3D12Device, resource)
        };

        let Some(texture) = self.texture_resource else {
            godot_error!("Texture was null.");
            return None;
        };

        Some(texture)
    }

    fn update_godot_resources(&mut self, texture: NonNull<ID3D12Resource>) {
        let mut rendering_server = RenderingServer::singleton();
        let Some(mut rendering_device) = rendering_server.get_rendering_device() else {
            godot_error!("Rendering device was null.");
            return;
        };

        let data_format = convert_dxgi_to_rd_data_format(self.spout.get_sender_format());

        // TODO: Do I just copy from this image to one owned by Godot via a GPU only copy?
        self.rd_texture_rid = rendering_device.texture_create_from_extension(
            TextureType::TYPE_2D,
            data_format,
            TextureSamples::SAMPLES_1,
            TextureUsageBits::SAMPLING_BIT,
            texture.as_ptr() as u64,
            self.spout.get_sender_width() as u64,
            self.spout.get_sender_height() as u64,
            0,
            1,
        );
        self.rs_texture_rid = rendering_server.texture_rd_create(self.rd_texture_rid);
    }

    fn free_godot_resources(&mut self) {
        let mut rendering_server = RenderingServer::singleton();
        let Some(mut rendering_device) = rendering_server.get_rendering_device() else {
            godot_error!("Rendering device was null.");
            return;
        };

        if self.rs_texture_rid.is_valid() {
            rendering_server.free_rid(self.rs_texture_rid);
            self.rs_texture_rid = Rid::Invalid;
        }

        if self.rd_texture_rid.is_valid() {
            rendering_device.free_rid(self.rd_texture_rid);
            self.rd_texture_rid = Rid::Invalid;
        }
    }
}

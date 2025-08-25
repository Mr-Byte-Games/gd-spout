use crate::spout::dx12::godot::{
    convert_dxgi_to_rd_data_format, copy_rendering_device_texture, create_texture, get_d3d12_device,
};
use crate::spout::receiver::SpoutReceiver;
use godot::classes::RenderingServer;
use godot::classes::rendering_device::{TextureSamples, TextureType, TextureUsageBits};
use godot::classes::rendering_server::TextureLayeredType;
use godot::prelude::*;
use spout_sys::{ID3D12Resource, Spout};
use std::ptr::NonNull;
use windows::Win32::Graphics::Direct3D12::ID3D12Device;
use windows::core::Interface;

pub struct D3D12SpoutReceiver {
    spout: Spout,
    receiver_rd_rid: Rid,
    external_rd_rid: Rid,
    external_rs_rid: Rid,
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
            external_rs_rid: rs_texture_rid,
            external_rd_rid: Rid::Invalid,
            receiver_rd_rid: Rid::Invalid,
            texture_resource: None,
            device,
        }))
    }
}

impl SpoutReceiver for D3D12SpoutReceiver {
    fn rid(&self) -> Rid {
        self.external_rs_rid
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
        let rendering_server = RenderingServer::singleton();
        let Some(mut rendering_device) = rendering_server.get_rendering_device() else {
            godot_error!("Rendering device was null.");
            return false;
        };

        let Some(resource) = self.update_spout_resource() else {
            if let Err(err) =
                copy_rendering_device_texture(&mut rendering_device, self.receiver_rd_rid, self.external_rd_rid)
            {
                godot_error!("error copying rendering device texture: {}", err);
            }

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
        self.receiver_rd_rid = rendering_device.texture_create_from_extension(
            TextureType::TYPE_2D,
            data_format,
            TextureSamples::SAMPLES_1,
            TextureUsageBits::SAMPLING_BIT | TextureUsageBits::CAN_COPY_FROM_BIT,
            texture.as_ptr() as u64,
            self.spout.get_sender_width() as u64,
            self.spout.get_sender_height() as u64,
            1,
            1,
        );

        self.external_rd_rid = match create_texture(&mut rendering_device, self.receiver_rd_rid) {
            Ok(rid) => rid,
            Err(err) => {
                godot_error!("Error creating texture: {}", err);
                return;
            }
        };

        self.external_rs_rid = rendering_server
            .texture_rd_create_ex(self.external_rd_rid)
            .layer_type(TextureLayeredType::LAYERED_2D_ARRAY)
            .done();
    }

    fn free_godot_resources(&mut self) {
        let mut rendering_server = RenderingServer::singleton();
        let Some(mut rendering_device) = rendering_server.get_rendering_device() else {
            godot_error!("Rendering device was null.");
            return;
        };

        if self.external_rs_rid.is_valid() {
            rendering_server.free_rid(self.external_rs_rid);
            self.external_rs_rid = Rid::Invalid;
        }

        if self.external_rd_rid.is_valid() {
            rendering_server.free_rid(self.external_rd_rid);
            self.external_rd_rid = Rid::Invalid;
        }

        if self.receiver_rd_rid.is_valid() {
            rendering_device.free_rid(self.receiver_rd_rid);
            self.receiver_rd_rid = Rid::Invalid;
        }
    }
}

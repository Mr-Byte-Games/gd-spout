#[cfg(target_os = "windows")]
use crate::d3d12_util::get_d3d12_device;
use godot::classes::rendering_device::{DataFormat, TextureSamples, TextureType, TextureUsageBits};
use godot::classes::{ITexture2D, RenderingServer, Texture2D};
use godot::prelude::*;
#[cfg(target_os = "windows")]
use spout_sys::SpoutDX12;
#[cfg(target_os = "windows")]
use spout_sys::{ID3D12Resource, release_resource};
use std::ptr::NonNull;

#[derive(GodotClass)]
#[class(tool, base=Texture2D)]
pub struct SpoutReceiverTexture {
    #[var(get = get_receiver_name, set = set_receiver_name)]
    #[export]
    receiver_name: GString,
    #[cfg(target_os = "windows")]
    spout: SpoutDX12,
    rd_texture_rid: Rid,
    rs_texture_rid: Rid,
    #[cfg(target_os = "windows")]
    texture_resource: Option<NonNull<ID3D12Resource>>,
    #[cfg(target_os = "windows")]
    pre_draw_callback: Option<Callable>,
    base: Base<Texture2D>,
}

#[cfg(target_os = "windows")]
impl Drop for SpoutReceiverTexture {
    fn drop(&mut self) {
        release_resource(&mut self.texture_resource);
        self.spout.close();
        self.free_godot_resources();
    }
}

#[godot_api]
impl ITexture2D for SpoutReceiverTexture {
    fn init(base: Base<Texture2D>) -> SpoutReceiverTexture {
        #[cfg(target_os = "windows")]
        let spout = {
            let mut spout = SpoutDX12::new();

            if let Some(device) = get_d3d12_device() {
                spout.open(device);
            } else {
                godot_error!("Unable to obtain D3D12 Device");
            };

            spout
        };

        let rs_texture_rid = RenderingServer::singleton().texture_2d_placeholder_create();

        Self {
            receiver_name: GString::new(),
            #[cfg(target_os = "windows")]
            spout,
            rs_texture_rid,
            rd_texture_rid: Rid::Invalid,
            #[cfg(target_os = "windows")]
            texture_resource: None,
            #[cfg(target_os = "windows")]
            pre_draw_callback: None,
            base,
        }
    }

    #[cfg(target_os = "windows")]
    fn get_width(&self) -> i32 {
        self.spout.get_sender_width() as i32
    }

    #[cfg(target_os = "windows")]
    fn get_height(&self) -> i32 {
        self.spout.get_sender_height() as i32
    }

    fn get_rid(&self) -> Rid {
        self.rs_texture_rid
    }
}

#[godot_api]
impl SpoutReceiverTexture {
    #[func]
    fn get_receiver_name(&self) -> GString {
        self.receiver_name.clone()
    }

    #[func]
    fn set_receiver_name(&mut self, receiver_name: GString) {
        #[cfg(target_os = "windows")]
        {
            self.spout.set_receiver_name(receiver_name.to_string());

            if let Some(callback) = self.pre_draw_callback.take() {
                RenderingServer::singleton().disconnect("frame_pre_draw", &callback);
            }

            let callable = self.base().callable("on_pre_draw");
            RenderingServer::singleton().connect("frame_pre_draw", &callable);

            self.pre_draw_callback = Some(callable);
        }

        self.receiver_name = receiver_name;
        self.base_mut().emit_changed();
    }

    #[func]
    #[cfg(target_os = "windows")]
    fn on_pre_draw(&mut self) {
        let Some(resource) = self.update_spout_resource() else {
            return;
        };

        self.free_godot_resources();
        self.update_godot_resources(resource);
    }

    #[cfg(target_os = "windows")]
    fn update_spout_resource(&mut self) -> Option<*mut ID3D12Resource> {
        let success = self.spout.receive_resource(&mut self.texture_resource);

        if !success || !self.spout.is_updated() {
            return None;
        }

        release_resource(&mut self.texture_resource);

        let Some(device) = get_d3d12_device() else {
            godot_error!("Unable to obtain D3D12 Device");
            return None;
        };

        self.spout.create_receiver_resource(device, &mut self.texture_resource);

        let Some(texture) = &mut self.texture_resource else {
            godot_error!("Texture was null!");
            return None;
        };

        Some(texture.as_ptr())
    }

    #[cfg(target_os = "windows")]
    fn update_godot_resources(&mut self, texture: *mut ID3D12Resource) {
        let mut rendering_server = RenderingServer::singleton();
        let Some(mut rendering_device) = rendering_server.get_rendering_device() else {
            godot_error!("Rendering device was null!");
            return;
        };

        // TODO: Get texture format from the sender format.
        self.rd_texture_rid = rendering_device.texture_create_from_extension(
            TextureType::TYPE_2D,
            DataFormat::R8G8B8A8_UNORM,
            TextureSamples::SAMPLES_1,
            TextureUsageBits::SAMPLING_BIT,
            texture as u64,
            self.spout.get_sender_width() as u64,
            self.spout.get_sender_height() as u64,
            0,
            1,
        );
        self.rs_texture_rid = rendering_server.texture_rd_create(self.rd_texture_rid);

        self.base_mut().emit_changed();
    }

    #[cfg(target_os = "windows")]
    fn free_godot_resources(&mut self) {
        let mut rendering_server = RenderingServer::singleton();
        let Some(mut rendering_device) = rendering_server.get_rendering_device() else {
            godot_error!("Rendering device was null!");
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

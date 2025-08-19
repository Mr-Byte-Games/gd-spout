use godot::classes::rendering_device::TextureUsageBits;
use godot::classes::{Engine, Node, RenderingDevice, RenderingServer, Texture2D};
use godot::global::Error;
use godot::prelude::*;

use crate::spout;
use crate::spout::sender::create_sender;

thread_local! {
    static RENDERING_DRIVER_D3D12: GString = "d3d12".into();
}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct SpoutSender {
    #[export]
    #[var(set = set_name)]
    name: GString,
    #[export]
    texture: Option<Gd<Texture2D>>,
    callback: Option<Callable>,
    destination_texture: Option<Rid>,
    spout: Option<Box<dyn spout::sender::SpoutSender>>,
    base: Base<Node>,
}

impl Drop for SpoutSender {
    fn drop(&mut self) {
        if let Some(callback) = self.callback.take() {
            RenderingServer::singleton().disconnect("frame_post_draw", &callback);
        }
    }
}

#[godot_api]
impl INode for SpoutSender {
    fn exit_tree(&mut self) {
        if let Some(callback) = self.callback.take() {
            RenderingServer::singleton().disconnect("frame_post_draw", &callback);
        }
    }

    fn ready(&mut self) {
        if Engine::singleton().is_editor_hint() {
            return;
        }

        if let Some(callback) = self.callback.take() {
            RenderingServer::singleton().disconnect("frame_post_draw", &callback);
        }

        let driver_name = RenderingServer::singleton()
            .get_current_rendering_driver_name()
            .to_string();

        let mut spout = create_sender(&driver_name);
        spout.set_sender_name(&self.name.to_string());
        self.spout = Some(spout);

        let callable = self.base().callable("on_post_draw");
        RenderingServer::singleton().connect("frame_post_draw", &callable);
        self.callback = Some(callable);
    }
}
#[godot_api]
impl SpoutSender {
    #[func]
    fn set_name(&mut self, name: GString) {
        if let Some(spout) = &mut self.spout {
            spout.set_sender_name(&name.to_string());
        }

        self.name = name;
    }

    #[func]
    fn on_post_draw(&mut self) {
        let Some(spout) = &mut self.spout else {
            godot_error!("No spout sender available.");
            return;
        };

        let Some(texture) = &self.texture else {
            godot_error!("No texture available.");
            return;
        };

        let Some(mut rendering_device) = RenderingServer::singleton().get_rendering_device() else {
            return;
        };

        let source_rid = RenderingServer::singleton().texture_get_rd_texture(texture.get_rid());

        let result = copy_rendering_device_texture(source_rid, &mut self.destination_texture, &mut rendering_device);
        godot_print!("{result:?}");

        if let Some(rid) = self.destination_texture {
            spout.send_resource(rid);
        }
    }
}

pub fn copy_rendering_device_texture(
    source_texture_rid: Rid,
    destination_texture: &mut Option<Rid>,
    rendering_device: &mut RenderingDevice,
) -> Result<(), Error> {
    // Validate source texture
    if !rendering_device.texture_is_valid(source_texture_rid) {
        return Err(Error::ERR_INVALID_PARAMETER);
    }

    let source_format = rendering_device.texture_get_format(source_texture_rid).unwrap();
    let target_rid = if let Some(existing_rid) = destination_texture {
        // Validate existing destination texture
        if !rendering_device.texture_is_valid(*existing_rid) {
            return Err(Error::ERR_INVALID_PARAMETER);
        }

        // Optionally verify format compatibility
        let target_format = rendering_device.texture_get_format(*existing_rid).unwrap();
        if source_format.get_width() != target_format.get_width()
            || source_format.get_height() != target_format.get_height()
            || source_format.get_format() != target_format.get_format()
        {
            return Err(Error::ERR_INVALID_DATA);
        }

        *existing_rid
    } else {
        // Create new texture with same properties as source
        let mut texture_format = godot::classes::RdTextureFormat::new_gd();
        texture_format.set_format(source_format.get_format());
        texture_format.set_texture_type(source_format.get_texture_type());
        texture_format.set_width(source_format.get_width());
        texture_format.set_height(source_format.get_height());
        texture_format.set_depth(source_format.get_depth());
        texture_format.set_array_layers(source_format.get_array_layers());
        texture_format.set_mipmaps(source_format.get_mipmaps());

        // Set usage flags to allow copying and sampling
        let usage_flags =
            TextureUsageBits::CAN_COPY_FROM_BIT | TextureUsageBits::CAN_COPY_TO_BIT | TextureUsageBits::SAMPLING_BIT;

        texture_format.set_usage_bits(usage_flags);

        let texture_view = godot::classes::RdTextureView::new_gd();

        let new_rid = rendering_device.texture_create(&texture_format, &texture_view);
        if new_rid == Rid::Invalid {
            return Err(Error::ERR_CANT_CREATE);
        }

        *destination_texture = Some(new_rid);
        new_rid
    };

    let copy_result = rendering_device.texture_copy(
        source_texture_rid,
        target_rid,
        Vector3::new(0.0, 0.0, 0.0), // from_pos
        Vector3::new(0.0, 0.0, 0.0), // to_pos
        Vector3::new(
            source_format.get_width() as f32,
            source_format.get_height() as f32,
            source_format.get_depth() as f32,
        ), // size
        0,                           // src_mipmap
        0,                           // dst_mipmap
        0,                           // src_layer
        0,                           // dst_layer
    );

    match copy_result {
        Error::OK => Ok(()),
        error => Err(error),
    }
}

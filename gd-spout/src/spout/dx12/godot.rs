use godot::classes::RenderingDevice;
use godot::classes::rendering_device::TextureUsageBits;
use godot::{
    builtin::Rid,
    classes::RenderingServer,
    classes::rendering_device::{DataFormat, DriverResource},
    prelude::*,
};
use spout_sys::DXGI_FORMAT;
use std::error::Error;
use std::ffi;
use windows::{
    Win32::Graphics::Direct3D12::{ID3D12CommandQueue, ID3D12Device, ID3D12Resource},
    core::Interface,
};

pub fn get_d3d12_resource_from_texture(rid: Rid) -> Option<ID3D12Resource> {
    let mut device = RenderingServer::singleton().get_rendering_device()?;
    let resource_id = device.get_driver_resource(DriverResource::TEXTURE, rid, 0);
    let resource = resource_id as *mut *mut ffi::c_void;

    if resource.is_null() {
        return None;
    }

    unsafe { Interface::from_raw_borrowed(&*resource).cloned() }
}

pub fn get_d3d12_device() -> Option<ID3D12Device> {
    let mut device = RenderingServer::singleton().get_rendering_device()?;
    let device = device.get_driver_resource(DriverResource::LOGICAL_DEVICE, Rid::Invalid, 0) as *mut ffi::c_void;

    unsafe { Interface::from_raw_borrowed(&device).cloned() }
}

pub fn get_d3d12_command_queue() -> Option<ID3D12CommandQueue> {
    let mut device = RenderingServer::singleton().get_rendering_device()?;
    let command_queue_id = device.get_driver_resource(DriverResource::COMMAND_QUEUE, Rid::Invalid, 0);
    let resource = command_queue_id as *mut *mut ffi::c_void;

    unsafe { Interface::from_raw_borrowed(&*resource).cloned() }
}

pub fn convert_dxgi_to_rd_data_format(dxgi_input: DXGI_FORMAT) -> DataFormat {
    match dxgi_input {
        DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM => DataFormat::R8G8B8A8_UNORM,
        DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_SNORM => DataFormat::R8G8B8A8_SNORM,
        DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UINT => DataFormat::R8G8B8A8_UINT,
        DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_SINT => DataFormat::R8G8B8A8_SINT,
        DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_TYPELESS => DataFormat::R8G8B8A8_UNORM,
        DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM_SRGB => DataFormat::R8G8B8A8_SRGB,

        // TODO: Figure out all the other format mappings.
        // DXGI_FORMAT::DXGI_FORMAT_B4G4R4A4_UNORM => DataFormat::R4G4B4A4_UNORM_PACK16,
        // DXGI_FORMAT::DXGI_FORMAT_B5G6R5_UNORM => DataFormat::R5G6B5_UNORM_PACK16,
        // DXGI_FORMAT::DXGI_FORMAT_R8_TYPELESS => DataFormat::R8_UNORM,
        // DXGI_FORMAT::DXGI_FORMAT_R8G8_TYPELESS => DataFormat::R8G8_UNORM,
        // DXGI_FORMAT::DXGI_FORMAT_B8G8R8A8_TYPELESS => DataFormat::B8G8R8A8_UNORM,
        // DXGI_FORMAT::DXGI_FORMAT_R10G10B10A2_TYPELESS => DataFormat::A2R10G10B10_UNORM_PACK32,
        // DXGI_FORMAT::DXGI_FORMAT_R16_TYPELESS => DataFormat::R16_UNORM,
        // DXGI_FORMAT::DXGI_FORMAT_R16G16_TYPELESS => DataFormat::R16G16_UNORM,
        // DXGI_FORMAT::DXGI_FORMAT_R16G16B16A16_TYPELESS => DataFormat::R16G16B16A16_UNORM,
        // DXGI_FORMAT::DXGI_FORMAT_R32_TYPELESS => DataFormat::R32_UINT,
        // DXGI_FORMAT::DXGI_FORMAT_R32G32_TYPELESS => DataFormat::R32G32_UINT,
        // DXGI_FORMAT::DXGI_FORMAT_R32G32B32_TYPELESS => DataFormat::R32G32B32_UINT,
        // DXGI_FORMAT::DXGI_FORMAT_R32G32B32A32_TYPELESS => DataFormat::R32G32B32A32_UINT,
        // DXGI_FORMAT::DXGI_FORMAT_R11G11B10_FLOAT => DataFormat::B10G11R11_UFLOAT_PACK32,
        // DXGI_FORMAT::DXGI_FORMAT_R9G9B9E5_SHAREDEXP => DataFormat::E5B9G9R9_UFLOAT_PACK32,
        // DXGI_FORMAT::DXGI_FORMAT_R24G8_TYPELESS => DataFormat::X8_D24_UNORM_PACK32,
        // DXGI_FORMAT::DXGI_FORMAT_R32G8X24_TYPELESS => DataFormat::D32_SFLOAT_S8_UINT,
        // DXGI_FORMAT::DXGI_FORMAT_BC1_TYPELESS => DataFormat::BC1_RGB_UNORM_BLOCK,
        // DXGI_FORMAT::DXGI_FORMAT_BC2_TYPELESS => DataFormat::BC2_UNORM_BLOCK,
        // DXGI_FORMAT::DXGI_FORMAT_BC3_TYPELESS => DataFormat::BC3_UNORM_BLOCK,
        // DXGI_FORMAT::DXGI_FORMAT_BC4_TYPELESS => DataFormat::BC4_UNORM_BLOCK,
        // DXGI_FORMAT::DXGI_FORMAT_BC5_TYPELESS => DataFormat::BC5_UNORM_BLOCK,
        // DXGI_FORMAT::DXGI_FORMAT_BC6H_TYPELESS => DataFormat::BC6H_UFLOAT_BLOCK,
        // DXGI_FORMAT::DXGI_FORMAT_BC7_TYPELESS => DataFormat::BC7_UNORM_BLOCK,
        format => {
            godot_warn!("Unsupported DXGI format found {format:?}");

            DataFormat::MAX
        }
    }
}

pub fn copy_rendering_device_texture(
    rendering_device: &mut RenderingDevice,
    source_texture_rid: Rid,
    destination_texture: Rid,
) -> Result<(), Box<dyn Error>> {
    if !rendering_device.texture_is_valid(source_texture_rid) {
        Err("source texture is not valid")?;
    }

    if !rendering_device.texture_is_valid(destination_texture) {
        Err("destination texture is not valid")?;
    }

    let source_format = rendering_device.texture_get_format(source_texture_rid).unwrap();
    let target_format = rendering_device.texture_get_format(destination_texture).unwrap();
    if source_format.get_width() != target_format.get_width()
        || source_format.get_height() != target_format.get_height()
        || source_format.get_format() != target_format.get_format()
    {
        Err("source and destination format are incompatible")?;
    }

    let copy_result = rendering_device.texture_copy(
        source_texture_rid,
        destination_texture,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(
            source_format.get_width() as f32,
            source_format.get_height() as f32,
            source_format.get_depth() as f32,
        ),
        0,
        0,
        0,
        0,
    );

    match copy_result {
        godot::global::Error::OK => Ok(()),
        error => Err(format!("error copying texture {error:?}"))?,
    }
}

pub fn create_texture(rendering_device: &mut RenderingDevice, source: Rid) -> Result<Rid, Box<dyn Error>> {
    let source_format = rendering_device
        .texture_get_format(source)
        .ok_or("source texture format not found")?;

    let mut texture_format = godot::classes::RdTextureFormat::new_gd();
    texture_format.set_format(source_format.get_format());
    texture_format.set_texture_type(source_format.get_texture_type());
    texture_format.set_width(source_format.get_width());
    texture_format.set_height(source_format.get_height());
    texture_format.set_depth(source_format.get_depth());
    texture_format.set_array_layers(source_format.get_array_layers());
    texture_format.set_mipmaps(source_format.get_mipmaps());

    texture_format.set_usage_bits(
        TextureUsageBits::CAN_COPY_FROM_BIT | TextureUsageBits::CAN_COPY_TO_BIT | TextureUsageBits::SAMPLING_BIT,
    );

    let texture_view = godot::classes::RdTextureView::new_gd();

    let new_rid = rendering_device.texture_create(&texture_format, &texture_view);
    if new_rid == Rid::Invalid {
        Err("unable to create texture")?;
    }

    Ok(new_rid)
}

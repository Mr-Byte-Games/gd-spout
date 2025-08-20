use godot::builtin::Rid;
use godot::classes::RenderingServer;
use godot::classes::rendering_device::{DataFormat, DriverResource};
use godot::prelude::*;
use spout_sys::{DXGI_FORMAT, ID3D12Device, ID3D12Resource, ID3D12CommandQueue};
use std::ptr::NonNull;

pub fn get_d3d12_resource_from_texture(rid: Rid) -> Option<NonNull<ID3D12Resource>> {
    let mut device = RenderingServer::singleton().get_rendering_device()?;
    let resource_id = device.get_driver_resource(DriverResource::TEXTURE, rid, 0);
    let resource = resource_id as *mut *mut ID3D12Resource;

    NonNull::new(resource)
        .map(|outer| unsafe { *outer.as_ptr() })
        .and_then(NonNull::new)
}

pub fn get_d3d12_device() -> Option<NonNull<ID3D12Device>> {
    let mut device = RenderingServer::singleton().get_rendering_device()?;
    let logical_device_id = device.get_driver_resource(DriverResource::LOGICAL_DEVICE, Rid::Invalid, 0);

    NonNull::new(logical_device_id as *mut ID3D12Device)
}

pub fn get_d3d12_command_queue() -> Option<NonNull<ID3D12CommandQueue>> {
    let mut device = RenderingServer::singleton().get_rendering_device()?;
    let command_queue_id = device.get_driver_resource(DriverResource::COMMAND_QUEUE, Rid::Invalid, 0);
    let resource = command_queue_id as *mut *mut ID3D12CommandQueue;

    NonNull::new(resource)
        .map(|outer| unsafe { *outer.as_ptr() })
        .and_then(NonNull::new)
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

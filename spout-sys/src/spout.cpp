#include "spout.h"
#include "SpoutDX12.h"

struct DeviceHandle {
    ID3D12Device* device;
};

struct TextureHandle {
    ID3D12Resource* resource;
};


SpoutDX12::SpoutDX12() : spout(new spoutDX12()) {}

SpoutDX12::~SpoutDX12() {
    delete spout;
}

// TODO: Do I need the command queue?
bool SpoutDX12::open(const DeviceHandle* device) const {
    return spout->OpenDirectX12(device->device);
}

void SpoutDX12::close() const {
    spout->CloseDirectX12();
}

void SpoutDX12::release_sender() const {
    spout->ReleaseSender();
}

bool SpoutDX12::set_sender_name(const std::string &name) const {
    return spout->SetSenderName(name.c_str());
}

bool SpoutDX12::send_texture(const TextureHandle *texture) const {
    ID3D11Resource* d3d11_resource;

    if (!spout->WrapDX12Resource(texture->resource, &d3d11_resource, D3D12_RESOURCE_STATE_COPY_SOURCE)) {
        return false;
    }

    return spout->SendDX11Resource(d3d11_resource);
}


std::unique_ptr<SpoutDX12> new_spout_dx12() {
    return std::make_unique<SpoutDX12>();
}
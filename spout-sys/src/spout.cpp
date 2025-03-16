#include "spout.h"
#include "SpoutDX12.h"
#include <wrl.h>

struct DeviceHandle {
    ID3D12Device *device;
};

struct TextureHandle {
    ID3D12Resource *resource;
};


SpoutDX12::SpoutDX12() : _spout(new spoutDX12()),
                         _cachedD3D12Resource(nullptr),
                         _cachedD3D11Resource(nullptr) {
}

SpoutDX12::~SpoutDX12() {
    delete _spout;
}

// TODO: Do I need the command queue?
bool SpoutDX12::open(ID3D12Device *device) const {
    return _spout->OpenDirectX12(device);
}

void SpoutDX12::close() const {
    _spout->CloseDirectX12();
}

void SpoutDX12::release_sender() const {
    _spout->ReleaseSender();
}

bool SpoutDX12::set_sender_name(const std::string &name) const {
    return _spout->SetSenderName(name.c_str());
}

bool SpoutDX12::send_resource(ID3D12Resource *resource) {
    if (resource == nullptr) {
        return false;
    }

    if (_cachedD3D12Resource.Get() == resource) {
        return _spout->SendDX11Resource(_cachedD3D11Resource.Get());
    }

    _cachedD3D12Resource = resource;
    _cachedD3D11Resource = nullptr;

    ID3D11Resource *destination;

    if (!_spout->WrapDX12Resource(resource, &destination, D3D12_RESOURCE_STATE_RENDER_TARGET)) {
        return false;
    }

    _cachedD3D11Resource = destination;
    return true;
}

std::unique_ptr<SpoutDX12> new_spout_dx12() {
    return std::make_unique<SpoutDX12>();
}

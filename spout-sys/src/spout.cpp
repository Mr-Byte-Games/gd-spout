#include "spout.h"
#include "SpoutDX12.h"

SpoutDX12::SpoutDX12(ID3D12Device *device) : _spout(new spoutDX12()) {
    _spout->OpenDirectX12(device);
}

SpoutDX12::~SpoutDX12() {
    _spout->CloseDirectX12();
    delete _spout;
}

void SpoutDX12::release_sender() const {
    _spout->ReleaseSender();
}

void SpoutDX12::release_receiver() const {
    _spout->ReleaseReceiver();
}

bool SpoutDX12::set_sender_name(const std::string &name) const {
    return _spout->SetSenderName(name.c_str());
}

void SpoutDX12::set_receiver_name(const std::string &name) const {
    _spout->SetReceiverName(name.c_str());
}

bool SpoutDX12::send_dx11_resource(ID3D11Resource *resource) const {
    return _spout->SendDX11Resource(resource);
}

bool SpoutDX12::wrap_dx12_resource(ID3D12Resource *dx12_resource, ID3D11Resource **dx11_resource) const {
    return _spout->WrapDX12Resource(dx12_resource, dx11_resource, D3D12_RESOURCE_STATE_RENDER_TARGET);
}

unsigned int SpoutDX12::get_sender_height() const {
    return _spout->GetSenderHeight();
}

unsigned int SpoutDX12::get_sender_width() const {
    return _spout->GetSenderWidth();
}

DXGI_FORMAT SpoutDX12::get_sender_format() const {
    return _spout->GetSenderFormat();
}

bool SpoutDX12::is_updated() const {
    return _spout->IsUpdated();
}

bool SpoutDX12::receive_dx12_resource(ID3D12Resource **resource) const {
    return _spout->ReceiveDX12Resource(resource);
}

bool SpoutDX12::create_dx12_texture(ID3D12Device *device, unsigned int width, unsigned int height, ID3D12Resource **resource) const {
    return _spout->CreateDX12texture(
        device,
        width,
        height,
        D3D12_RESOURCE_STATE_COPY_DEST,
        _spout->GetSenderFormat(),
        resource
    );
}

std::unique_ptr<SpoutDX12> new_spout_dx12(ID3D12Device *device) {
    return std::make_unique<SpoutDX12>(device);
}

std::unique_ptr<SpoutDX12> new_spout_dx12_with_queue(ID3D12Device *device, ID3D12CommandQueue *commandQueue) {
    return std::make_unique<SpoutDX12>(device, commandQueue);
}

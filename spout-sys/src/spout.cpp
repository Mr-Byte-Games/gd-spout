#include "spout.h"
#include "SpoutDX12.h"
#include <wrl.h>

struct DeviceHandle {
    ID3D12Device *device;
};

struct TextureHandle {
    ID3D12Resource *resource;
};

SpoutDX12::SpoutDX12(ID3D12Device *device) : _spout(new spoutDX12()),
                                             _cachedD3D12Resource(nullptr),
                                             _cachedD3D11Resource(nullptr),
                                             _device(device),
                                             _commandQueue(nullptr),
                                             _fenceEvent(nullptr),
                                             _fenceValue(1) {
    _spout->OpenDirectX12(device);
}

SpoutDX12::SpoutDX12(ID3D12Device *device, ID3D12CommandQueue *commandQueue) : _spout(new spoutDX12()),
                                             _cachedD3D12Resource(nullptr),
                                             _cachedD3D11Resource(nullptr),
                                             _device(device),
                                             _commandQueue(commandQueue),
                                             _fenceEvent(nullptr),
                                             _fenceValue(1) {
    _spout->OpenDirectX12(device);
    
    // Create fence for synchronization with Godot's command queue
    if (_commandQueue) {
        device->CreateFence(0, D3D12_FENCE_FLAG_NONE, IID_PPV_ARGS(&_fence));
        _fenceEvent = CreateEvent(nullptr, FALSE, FALSE, nullptr);
    }
}

SpoutDX12::~SpoutDX12() {
    if (_fenceEvent) {
        CloseHandle(_fenceEvent);
    }
    
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

bool SpoutDX12::send_resource(ID3D12Resource *resource) {
    if (resource == nullptr) {
        return false;
    }

    if (_commandQueue && _fence) {
        _commandQueue->Signal(_fence.Get(), _fenceValue);

        if (_fence->GetCompletedValue() < _fenceValue) {
            _fence->SetEventOnCompletion(_fenceValue, _fenceEvent);
            WaitForSingleObject(_fenceEvent, INFINITE);
        }

        _fenceValue++;
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

bool SpoutDX12::receive_resource(ID3D12Resource **resource) const {
    return _spout->ReceiveDX12Resource(resource);
}

bool SpoutDX12::create_receiver_resource(ID3D12Device *device, ID3D12Resource **resource) const {
    return _spout->CreateDX12texture(
        device,
        _spout->GetSenderWidth(),
        _spout->GetSenderHeight(),
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

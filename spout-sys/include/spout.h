#pragma once

#include <memory>
#include <d3d12.h>
#include <d3d11.h>
#include "rust/cxx.h"

struct spoutDX12;

class SpoutDX12 {
public:
    explicit SpoutDX12(ID3D12Device *device);
    SpoutDX12(ID3D12Device *device, ID3D12CommandQueue *commandQueue);

    ~SpoutDX12();

    void release_sender() const;
    void release_receiver() const;
    bool set_sender_name(rust::Str name) const;
    void set_receiver_name(rust::Str name) const;
    
    // Simplified methods - no caching or fencing logic
    bool send_dx11_resource(ID3D11Resource *resource) const;
    bool wrap_dx12_resource(ID3D12Resource *dx12_resource, ID3D11Resource **dx11_resource) const;
    
    unsigned int get_sender_height() const;
    unsigned int get_sender_width() const;
    DXGI_FORMAT get_sender_format() const;
    bool is_updated() const;
    
    bool receive_dx12_resource(ID3D12Resource **resource) const;
    bool create_dx12_texture(ID3D12Device *device, ID3D12Resource **resource) const;

private:
    spoutDX12 *_spout;
};

std::unique_ptr<SpoutDX12> new_spout_dx12(ID3D12Device *device);

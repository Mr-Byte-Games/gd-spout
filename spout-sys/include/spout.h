#pragma once

#include <memory>
#include <string>
#include <d3d12.h>
#include <d3d11.h>
#include <wrl.h>

struct spoutDX12;

class SpoutDX12 {
public:
    SpoutDX12();
    ~SpoutDX12();

    bool open(ID3D12Device* device) const;
    void close() const;
    void release_sender() const;
    bool set_sender_name(const std::string &name) const;
    bool send_resource(ID3D12Resource *resource);

    void set_receiver_name(const std::string &name) const;
    unsigned int get_sender_height() const;
    unsigned int get_sender_width() const;

    DXGI_FORMAT get_sender_format() const;

    bool is_updated() const;

    bool receive_resource(ID3D12Resource **resource) const;

    bool create_receiver_resource(ID3D12Device *device, ID3D12Resource **resource) const;

private:
    spoutDX12* _spout;
    Microsoft::WRL::ComPtr<ID3D12Resource> _cachedD3D12Resource;
    Microsoft::WRL::ComPtr<ID3D11Resource> _cachedD3D11Resource;
};

std::unique_ptr<SpoutDX12> new_spout_dx12();
uint32_t safe_release(ID3D12Resource *target);
#pragma once

#include <memory>
#include <string>

struct spoutDX12;
struct DeviceHandle;
struct TextureHandle;

class SpoutDX12 {
public:
    SpoutDX12();
    ~SpoutDX12();

    bool open(const DeviceHandle* device) const;
    void close() const;
    void release_sender() const;
    bool set_sender_name(const std::string &name) const;
    bool send_texture(const TextureHandle *texture) const;

private:
    spoutDX12* spout;
};

std::unique_ptr<SpoutDX12> new_spout_dx12();
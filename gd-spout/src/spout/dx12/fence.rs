use godot::classes::gpu_particles_collision_sdf_3d::Resolution;
use windows::Win32::Foundation::{CloseHandle, HANDLE, WAIT_OBJECT_0};
use windows::Win32::Graphics::Direct3D12::{D3D12_FENCE_FLAG_NONE, ID3D12CommandQueue, ID3D12Device, ID3D12Fence};
use windows::Win32::System::Threading::{CreateEventW, INFINITE, WaitForSingleObject};
use windows::core::{HRESULT, Result};

pub struct Fence {
    fence: ID3D12Fence,
    fence_event: HANDLE,
    fence_value: u64,
    command_queue: ID3D12CommandQueue,
}

impl Fence {
    pub fn new(device: &ID3D12Device, command_queue: ID3D12CommandQueue) -> Result<Self> {
        let fence: ID3D12Fence = unsafe { device.CreateFence(0, D3D12_FENCE_FLAG_NONE)? };

        let fence_event = unsafe { CreateEventW(None, false, false, None)? };

        Ok(Self {
            fence,
            fence_event,
            fence_value: 1,
            command_queue,
        })
    }

    pub fn wait_for_gpu(&mut self) -> Result<()> {
        unsafe {
            self.command_queue.Signal(&self.fence, self.fence_value)?;
        }

        if unsafe { self.fence.GetCompletedValue() } < self.fence_value {
            unsafe {
                self.fence.SetEventOnCompletion(self.fence_value, self.fence_event)?;
                let wait_result = WaitForSingleObject(self.fence_event, INFINITE);

                if wait_result != WAIT_OBJECT_0 {
                    return Err(HRESULT(-1).into());
                }
            }
        }

        self.fence_value += 1;
        Ok(())
    }
}

impl Drop for Fence {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.fence_event);
        }
    }
}

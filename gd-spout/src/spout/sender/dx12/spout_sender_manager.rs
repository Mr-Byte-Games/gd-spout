use std::collections::HashMap;
use std::ptr::NonNull;
use std::sync::{Arc, Mutex};
use godot::prelude::*;
use spout_sys::{SpoutDX12, ID3D12Device, ID3D12Resource, ID3D11Resource, ID3D12CommandQueue};
use windows::{
    core::{Interface, Result as WinResult, HRESULT},
    Win32::{
        Foundation::{HANDLE, CloseHandle, CreateEventW},
        Graphics::Direct3D12::{
            ID3D12Device as WinID3D12Device,
            ID3D12CommandQueue as WinID3D12CommandQueue,
            ID3D12Fence,
            D3D12_FENCE_FLAG_NONE,
        },
        System::Threading::{WaitForSingleObject, INFINITE, WAIT_OBJECT_0},
    },
};

#[derive(Debug, Clone)]
struct ResourceCacheEntry {
    dx11_resource: NonNull<ID3D11Resource>,
}

#[derive(Debug)]
struct SenderFenceManager {
    fence: ID3D12Fence,
    fence_event: HANDLE,
    fence_value: u64,
    command_queue: WinID3D12CommandQueue,
}

impl SenderFenceManager {
    pub fn new(device: &WinID3D12Device, command_queue: WinID3D12CommandQueue) -> WinResult<Self> {
        let fence: ID3D12Fence = unsafe {
            device.CreateFence(0, D3D12_FENCE_FLAG_NONE)?
        };

        let fence_event = unsafe {
            CreateEventW(None, false, false, None)?
        };

        Ok(Self {
            fence,
            fence_event,
            fence_value: 1,
            command_queue,
        })
    }

    pub fn wait_for_gpu(&mut self) -> WinResult<()> {
        // Signal the fence
        unsafe {
            self.command_queue.Signal(&self.fence, self.fence_value)?;
        }

        // Wait for completion if needed
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

impl Drop for SenderFenceManager {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.fence_event);
        }
    }
}

pub struct SpoutSenderWrapper {
    inner: cxx::UniquePtr<SpoutDX12>,
    device: NonNull<ID3D12Device>,
    resource_cache: Arc<Mutex<HashMap<usize, ResourceCacheEntry>>>,
    fence_manager: Option<SenderFenceManager>,
    sender_name: String,
}

impl SpoutSenderWrapper {
    pub fn new(
        device: NonNull<ID3D12Device>, 
        command_queue: NonNull<ID3D12CommandQueue>
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let inner = unsafe { 
            spout_sys::new_spout_dx12_with_queue(device.as_ptr(), command_queue.as_ptr()) 
        };

        // Create Windows API wrappers for the raw pointers
        let win_device: WinID3D12Device = unsafe {
            WinID3D12Device::from_raw(device.as_ptr() as *mut _)
        };
        let win_command_queue: WinID3D12CommandQueue = unsafe {
            WinID3D12CommandQueue::from_raw(command_queue.as_ptr() as *mut _)
        };

        let fence_manager = SenderFenceManager::new(&win_device, win_command_queue)
            .map_err(|e| format!("Failed to create sender fence manager: {}", e))?;

        Ok(Self {
            inner,
            device,
            resource_cache: Arc::new(Mutex::new(HashMap::new())),
            fence_manager: Some(fence_manager),
            sender_name: String::new(),
        })
    }

    /// Set the sender name
    pub fn set_sender_name(&mut self, name: &str) -> bool {
        use cxx::let_cxx_string;
        let_cxx_string!(cxx_name = name);
        let success = self.inner.set_sender_name(&cxx_name);
        if success {
            self.sender_name = name.to_string();
        }
        success
    }

    pub fn get_sender_name(&self) -> &str {
        &self.sender_name
    }

    pub fn send_resource(&mut self, dx12_resource: NonNull<ID3D12Resource>) -> bool {
        if let Some(ref mut fence_manager) = self.fence_manager {
            if fence_manager.wait_for_gpu().is_err() {
                godot_error!("Failed to wait for GPU completion before sending");
                return false;
            }
        }

        let dx12_resource_ptr = dx12_resource.as_ptr() as usize;

        {
            let cache = self.resource_cache.lock().unwrap();
            if let Some(cache_entry) = cache.get(&dx12_resource_ptr) {
                return unsafe {
                    self.inner.send_dx11_resource(cache_entry.dx11_resource.as_ptr())
                };
            }
        }

        let mut dx11_resource: *mut ID3D11Resource = std::ptr::null_mut();
        
        let success = unsafe {
            self.inner.wrap_dx12_resource(dx12_resource.as_ptr(), &mut dx11_resource)
        };

        if !success || dx11_resource.is_null() {
            godot_error!("Failed to wrap D3D12 resource for sending");
            return false;
        }

        let dx11_resource = unsafe { NonNull::new_unchecked(dx11_resource) };

        {
            let mut cache = self.resource_cache.lock().unwrap();
            cache.insert(dx12_resource_ptr, ResourceCacheEntry {
                dx11_resource,
            });
        }

        unsafe {
            self.inner.send_dx11_resource(dx11_resource.as_ptr())
        }
    }

    pub fn release_sender(&mut self) {
        self.inner.release_sender();
        let mut cache = self.resource_cache.lock().unwrap();
        cache.clear();
        self.sender_name.clear();
    }

    pub fn is_initialized(&self) -> bool {
        !self.sender_name.is_empty()
    }
}

impl Drop for SpoutSenderWrapper {
    fn drop(&mut self) {
        self.release_sender();
    }
}

unsafe impl Send for SpoutSenderWrapper {}
unsafe impl Sync for SpoutSenderWrapper {}

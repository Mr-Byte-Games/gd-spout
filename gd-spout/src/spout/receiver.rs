#[cfg(target_os = "windows")]
mod dx12;

use std::error::Error;
#[cfg(target_os = "windows")]
use dx12::D3D12SpoutReceiver;
use godot::builtin::Rid;
use godot::classes::RenderingServer;

pub trait SpoutReceiver {
    fn rid(&self) -> Rid;
    fn set_sender_name(&mut self, name: &str);
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn update_resource(&mut self) -> bool;
}

pub fn create_receiver(driver_name: &str) -> Box<dyn SpoutReceiver> {
    let receiver =match driver_name {
        #[cfg(target_os = "windows")]
        "d3d12" => D3D12SpoutReceiver::new(),
        _ => Ok(NoOpReceiver::new()),
    };

    receiver.unwrap_or_else(|_| NoOpReceiver::new())
}


struct NoOpReceiver {
    placeholder: Rid
}

impl Drop for NoOpReceiver {
    fn drop(&mut self) {
        RenderingServer::singleton().free_rid(self.placeholder);
    }
}

impl NoOpReceiver {
    fn new() -> Box<dyn SpoutReceiver> {
        let receiver = NoOpReceiver {
            placeholder: RenderingServer::singleton().texture_2d_placeholder_create()
        };

        Box::new(receiver)
    }
}

impl SpoutReceiver for NoOpReceiver {
    fn rid(&self) -> Rid {
        self.placeholder
    }

    fn set_sender_name(&mut self, name: &str) {
        // No-op
    }

    fn width(&self) -> i32 {
        1
    }

    fn height(&self) -> i32 {
        1
    }

    fn update_resource(&mut self) -> bool {
        // No-op
        false
    }
}
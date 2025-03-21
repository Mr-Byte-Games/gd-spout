use godot::builtin::Rid;
use godot::classes::RenderingServer;
use crate::spout::receiver::SpoutReceiver;

pub struct NoOpReceiver {
    placeholder: Rid,
}

impl Drop for NoOpReceiver {
    fn drop(&mut self) {
        RenderingServer::singleton().free_rid(self.placeholder);
    }
}

impl NoOpReceiver {
    pub fn new() -> Box<dyn SpoutReceiver> {
        let receiver = NoOpReceiver {
            placeholder: RenderingServer::singleton().texture_2d_placeholder_create(),
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
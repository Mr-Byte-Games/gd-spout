use crate::spout::sender::SpoutSender;
use godot::builtin::Rid;

pub struct NoOpSender;

impl NoOpSender {
    pub fn new() -> Box<dyn SpoutSender> {
        Box::new(NoOpSender)
    }
}

impl SpoutSender for NoOpSender {
    fn set_sender_name(&mut self, _name: &str) {}

    fn send_resource(&mut self, _resource: Rid) {}
}

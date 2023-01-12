use std::collections::HashMap;

use crate::devices::Device;

pub struct Dispatcher {
    devices: HashMap<String, Box<dyn Device>>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, device: Box<dyn Device>) {
        self.devices.insert(device.name().to_string(), device);
    }
}

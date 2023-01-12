use std::collections::HashMap;

use crate::devices::Device;

//TODO: using for dispatch handles

pub struct Dispatcher {
    devices: HashMap<String, Box<dyn Device>>,
}

impl Default for Dispatcher {
    fn default() -> Self {
        Self::new()
    }
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

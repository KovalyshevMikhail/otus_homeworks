use crate::devices::Device;
use crate::services::ServiceDeviceManagement;

pub struct Home {
    name: String,
    rooms: Vec<Room>,
    manager_devices: ServiceDeviceManagement,
}

impl Home {
    pub fn new(name: String) -> Self {
        let manager_devices = ServiceDeviceManagement::new();

        Self {
            name,
            rooms: vec![],
            manager_devices,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn add_room(&mut self, room: Room) {
        self.manager_devices.add_room(&room).unwrap();
        self.rooms.push(room);
    }

    pub fn room(&self, name: &str) -> Option<&Room> {
        self.rooms.iter().find(|&room| room.name.as_str() == name)
    }

    pub fn rooms(&self) -> Vec<String> {
        self.rooms.iter().map(|room| room.name.clone()).collect()
    }

    pub fn add_device(&mut self, room_name: &str, device: Box<dyn Device>) -> Result<(), String> {
        self.manager_devices.add_device(room_name, device)
    }

    pub fn connect_device(&mut self, room_name: &str, device_connects_to: &str, device_connected: &str) -> Result<(), String> {
        self.manager_devices.connect_device(room_name, device_connects_to, device_connected)
    }

    pub fn device(&self, device_name: &str) -> Option<&Box<dyn Device>> {
        self.manager_devices.get_device(device_name)
    }

    pub fn devices(&self, room_name: &str) -> Vec<String> {
        self.manager_devices.get_devices(room_name).map(|name| name.clone()).collect()
    }

    pub fn print_report(&self) {
        self.manager_devices.print_report();
    }

    pub fn print_schema(&self) {
        self.manager_devices.print_schema();
    }
}

pub struct Room {
    name: String,
}

impl Room {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

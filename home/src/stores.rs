use std::{collections::HashMap, slice::Iter};

use crate::devices::Device;

pub const HOME_NAME: &str = "HOME";

/// Structure to store Device
///
/// Contains only property devices, which store <{Device}>
pub struct StoreDevices {
    devices: Vec<Box<dyn Device>>,
}

impl Default for StoreDevices {
    fn default() -> Self {
        Self::new()
    }
}

impl StoreDevices {
    /// Method create new Store
    pub fn new() -> StoreDevices {
        StoreDevices { devices: vec![] }
    }

    /// Method check contains device in the store
    pub fn contains(&self, device: &dyn Device) -> bool {
        self.devices
            .iter()
            .any(|d| d.name() == device.name() && d.info() == device.info())
    }

    /// Method add device to the store
    pub fn add_device(&mut self, device: Box<dyn Device>) {
        self.devices.push(device)
    }

    pub fn remove_device(&mut self, device_name: &str) -> Result<(), String> {
        match self.devices.iter().position(|device| device.name() == device_name) {
            Some(index) => {
                self.devices.remove(index);
                Ok(())
            },
            None => {
                let message = format!("Not found device with name [{}] in Devices Store", device_name);
                Err(message)
            }
        }
    }

    /// Method return Iter to the all devices
    pub fn iter(&self) -> Iter<Box<dyn Device>> {
        self.devices.iter()
    }
}

/// Struct of store schema of the Home
pub struct StoreDeviceLinks {
    links: HashMap<String, Vec<String>>,
}

impl Default for StoreDeviceLinks {
    fn default() -> Self {
        Self::new()
    }
}

impl StoreDeviceLinks {
    /// Method create new store
    pub fn new() -> Self {
        let mut links = HashMap::new();
        links.insert(String::from(HOME_NAME), Vec::<String>::new());

        StoreDeviceLinks { links }
    }

    /// Method check contains room in keys
    pub fn contains_room(&self, room_name: &str) -> bool {
        self.links.contains_key(&String::from(room_name))
    }

    /// Method add room to store
    ///
    /// If room is unique - its Ok
    /// If not - panic
    pub fn add_room(&mut self, room_name: &str) {
        match self.links.get_mut(&String::from(HOME_NAME)) {
            Some(home) => home.push(String::from(room_name)),
            None => panic!("Create StoreDeviceLinks without root HOME"),
        }
        self.links.insert(String::from(room_name), vec![]);
    }

    /// Method remove room from store
    ///
    /// If room exists - its OK
    /// If not - panic
    pub fn remove_room(&mut self, room_name: &str) -> Result<(), String> {
        match self.links.get_mut(&String::from(HOME_NAME)) {
            Some(home) => {
                match home.iter().position(|room| room.as_str() == room_name) {
                    Some(index) => {
                        home.remove(index);
                    },
                    None => {
                        let message = format!("Room with name [{}] not found in Schemas store", room_name);
                        return Err(message)
                    }
                }
            },
            None => panic!("Create StoreDeviceLinks without root HOME"),
        }

        self.links.remove(room_name);

        Ok(())
    }

    pub fn contains_device(&self, device_name: &str) -> bool {
        self.links.contains_key(&String::from(device_name))
    }

    /// Method check contains device in the room
    pub fn contains_device_in_room(&self, room_name: &str, device_name: &str) -> bool {
        let contains_in_room = match self.links.get(&String::from(room_name)) {
            None => false,
            Some(room_devices) => room_devices.contains(&String::from(device_name)),
        };
        let contains_in_list = match self.links.get(&String::from(device_name)) {
            None => false,
            Some(_) => true,
        };

        contains_in_list && contains_in_room
    }

    /// Method add device to the room
    /// If room contains - OK
    /// If not - panic
    pub fn add_device(&mut self, room_name: &str, device_name: &str) {
        match self.links.get_mut(&String::from(room_name)) {
            None => {
                panic!("Something wrong, room must be exists")
            }
            Some(room_devices) => room_devices.push(String::from(device_name)),
        }
        self.links.insert(String::from(device_name), vec![]);
    }

    pub fn remove_device(&mut self, device_name: &str) -> Result<(), String> {
        match self.links.remove(device_name) {
            Some(_) => {},
            None => {
                let message = format!("No found device name [{}] at schema", device_name);
                return Err(message);
            }
        }

        for (_, entities) in self.links.iter_mut() {
            match entities.iter().position(|name| name == device_name) {
                None => {},
                Some(index) => {
                    entities.remove(index);
                    return Ok(())
                }
            }
        }

        Ok(())
    }

    /// Method check contains connected devices
    /// TODO: rewrite mechanism
    pub fn contains_connected_device(&self, device_to: &str, device_from: &str) -> bool {
        match self.links.get(&String::from(device_to)) {
            None => false,
            Some(connected_devices) => connected_devices.contains(&String::from(device_from)),
        }
    }

    /// Method connect devices
    /// TODO: rewrite mechanism
    pub fn connect_device(&mut self, device_name_to: &str, device_name_from: &str) {
        match self.links.get_mut(&String::from(device_name_to)) {
            None => {
                panic!("Device[{}] doesn't exists in schema store", device_name_to);
            }
            Some(connected_devices) => {
                if connected_devices.contains(&String::from(device_name_from)) {
                    panic!(
                        "In device[{}] already connected another device[{}]",
                        device_name_to, device_name_from
                    );
                } else {
                    connected_devices.push(String::from(device_name_from));
                }
            }
        }
    }

    /// Method return connections of entity (Room or Device)
    pub fn entities(&self, name: &str) -> Iter<String> {
        match self.links.get(name) {
            None => [].iter(),
            Some(entities) => entities.iter(),
        }
    }
}

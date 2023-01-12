use std::{collections::HashMap, slice::Iter};

use crate::devices::Device;

pub const HOME_NAME: &str = "HOME";

/// Structure to store Device
///
/// Contains only property devices, which store <{name}, {Device}>
pub struct StoreDevices {
    devices: Vec<Box<dyn Device>>,
}

impl StoreDevices {
    pub fn new() -> StoreDevices {
        StoreDevices { devices: vec![] }
    }

    pub fn contains(&self, device: &Box<dyn Device>) -> bool {
        match self
            .devices
            .iter()
            .find(|d| d.name() == device.name() && d.info() == device.info())
        {
            Some(_) => true,
            None => false,
        }
    }

    pub fn add_device(&mut self, device: Box<dyn Device>) {
        self.devices.push(device)
    }

    pub fn iter(&self) -> Iter<Box<dyn Device>> {
        let tmp = self.devices.iter();

        tmp
    }
}

pub struct StoreDeviceLinks {
    links: HashMap<String, Vec<String>>,
}

impl StoreDeviceLinks {
    pub fn new() -> Self {
        let mut links = HashMap::new();
        links.insert(String::from(HOME_NAME), Vec::<String>::new());

        StoreDeviceLinks { links }
    }

    pub fn contains_room(&self, room_name: &str) -> bool {
        self.links.contains_key(&String::from(room_name))
    }

    pub fn add_room(&mut self, room_name: &str) {
        match self.links.get_mut(&String::from(HOME_NAME)) {
            Some(home) => home.push(String::from(room_name)),
            None => panic!("Create StoreDeviceLinks without root HOME"),
        }
        self.links.insert(String::from(room_name), vec![]);
    }

    pub fn contains_device(&self, room_name: &str, device_name: &str) -> bool {
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

    pub fn add_device(&mut self, room_name: &str, device_name: &str) {
        match self.links.get_mut(&String::from(room_name)) {
            None => {
                panic!("Something wrong, room must be exists")
            }
            Some(room_devices) => room_devices.push(String::from(device_name)),
        }
        self.links.insert(String::from(device_name), vec![]);
    }

    pub fn contains_connected_device(&self, device_to: &str, device_from: &str) -> bool {
        match self.links.get(&String::from(device_to)) {
            None => false,
            Some(connected_devices) => connected_devices.contains(&String::from(device_from)),
        }
    }

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

    pub fn entities(&self, name: &str) -> Iter<String> {
        match self.links.get(name) {
            None => [].iter(),
            Some(entities) => entities.iter(),
        }
    }
}

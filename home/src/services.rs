use crate::stores::HOME_NAME;
use crate::{
    devices::Device,
    stores::{StoreDeviceLinks, StoreDevices},
};
use std::slice::Iter;

/// Struct to manage store of device of the home
///
/// Struct contains only store with all devices
///
pub struct ServiceDevices {
    store_devices: StoreDevices,
}

impl ServiceDevices {
    /// Method create new service
    pub fn new(store_devices: StoreDevices) -> ServiceDevices {
        ServiceDevices { store_devices }
    }

    /// Method add device to store
    ///
    /// If device is unique - good
    /// If not - return error
    ///
    pub fn add_device(&mut self, device: Box<dyn Device>) -> Result<(), String> {
        if self.store_devices.contains(device.as_ref()) {
            let message = format!("Devices store already contains device [{}]", device.name());
            Err(message)
        } else {
            self.store_devices.add_device(device);
            Ok(())
        }
    }

    pub fn remove_device(&mut self, device_name: &str) -> Result<(), String> {
        self.store_devices.remove_device(device_name)
    }

    /// Method return device by its name
    ///
    /// If device name founds - return Some
    /// If not - return None
    pub fn get_device(&self, device_name: &str) -> Option<&dyn Device> {
        self.store_devices
            .iter()
            .find(|&device| device.name() == device_name)
            .map(|found| found.as_ref())
    }

    pub fn get_devices(&self) -> Vec<String> {
        self.store_devices.iter().map(|device| device.name().to_string()).collect()
    }

    pub fn _remove_device(&self, _device: Box<dyn Device>) -> Result<(), String> {
        todo!()
    }

    /// Method collect all information about each device
    ///
    /// Returns report about devices
    pub fn collect_data_for_report(&self) -> String {
        let mut info = String::from("");
        for device in self.store_devices.iter() {
            info.push_str(device.info().as_str());
            info.push_str("\n\n");
        }
        info
    }
}

/// Struct to manage store of the schema of the Home
pub struct ServiceSchemaDevices {
    store_schema: StoreDeviceLinks,
}

impl ServiceSchemaDevices {
    /// Method create new service
    pub fn new(store_schema: StoreDeviceLinks) -> Self {
        ServiceSchemaDevices { store_schema }
    }

    /// Method add room to the schema
    ///
    /// If room name is unique - return Ok
    /// If room already exists - return Error
    pub fn add_room(&mut self, room_name: &str) -> Result<(), String> {
        if self.store_schema.contains_room(room_name) {
            let message = format!("Room [{}] already contains in home", room_name);
            Err(message)
        } else {
            self.store_schema.add_room(room_name);
            Ok(())
        }
    }

    pub fn remove_room(&mut self, room_name: &str) -> Result<(), String> {
        if !self.store_schema.contains_room(room_name) {
            let message = format!("Room [{}] not contains in home", room_name);
            Err(message)
        } else {
            self.store_schema.remove_room(room_name)?;
            Ok(())
        }
    }

    pub fn rooms(&self) -> Vec<String> {
        self.store_schema.rooms()
    }

    /// Method add device to the room
    ///
    /// If room and device is unique - return Ok
    /// If room OR device already exists - return Error
    pub fn add_device(&mut self, room_name: &str, device_name: &str) -> Result<(), String> {
        if !self.store_schema.contains_room(room_name) {
            let message = format!("Home not contains room[{}]", room_name);
            return Err(message);
        }
        if self.store_schema.contains_device_in_room(room_name, device_name) {
            let message = format!(
                "Room[{}] already contains device[{}]",
                room_name, device_name
            );
            return Err(message);
        }

        self.store_schema.add_device(room_name, device_name);

        Ok(())
    }

    pub fn remove_device(&mut self, device_name: &str) -> Result<(), String> {
        self.store_schema.remove_device(device_name)
    }

    /// Method connect one device to another
    /// TODO: rewrite mechanism
    pub fn connect_device(
        &mut self,
        room_name: &str,
        device_connects_to: &str,
        device_connected: &str,
    ) -> Result<(), String> {
        if !self
            .store_schema
            .contains_device_in_room(room_name, device_connected)
        {
            let message = format!(
                "Room[{}] not contains device[{}]",
                room_name, device_connected
            );
            return Err(message);
        }
        if !self
            .store_schema
            .contains_device_in_room(room_name, device_connects_to)
        {
            let message = format!(
                "Room[{}] not contains device[{}]",
                room_name, device_connects_to
            );
            return Err(message);
        }

        self.store_schema
            .connect_device(device_connects_to, device_connected);

        Ok(())
    }

    /// Method return devices in the room
    pub fn room_devices(&self, room_name: &str) -> Iter<String> {
        self.store_schema.entities(room_name)
    }

    /// Method return schema of the Home
    pub fn collect_schema(&self) -> String {
        let mut result = String::from("Schema of home:\n");

        for room in self.store_schema.entities(HOME_NAME) {
            let room_devices = self.collect_devices_inner(room.as_str(), 1);
            result.push_str(format!("[ROOM] {}\n", room).as_str());
            result.push_str(room_devices.as_str());
        }
        result.push_str("In development");

        result
    }

    /// Recursive method return information about entity (Room or Device) and his connections
    fn collect_devices_inner(&self, from_name: &str, tabs: usize) -> String {
        let mut result = String::from("");
        let tabs_str = "\t".repeat(tabs);

        for device in self.store_schema.entities(from_name) {
            let devices_inner = self.collect_devices_inner(device.as_str(), tabs + 1);
            result.push_str(format!("{}- {}\n", tabs_str, device).as_str());
            result.push_str(devices_inner.as_str());
        }

        result
    }
}

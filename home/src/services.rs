use std::slice::Iter;
use crate::{
    devices::Device,
    places::Room,
    stores::{StoreDeviceLinks, StoreDevices},
};
use crate::stores::HOME_NAME;

pub struct ServiceDeviceManagement {
    service_devices: ServiceDevices,
    service_schema: ServiceSchemaDevices,
}

impl ServiceDeviceManagement {
    pub fn new() -> Self {
        let service_devices = ServiceDevices::new();
        let service_schema = ServiceSchemaDevices::new();
        ServiceDeviceManagement {
            service_devices,
            service_schema,
        }
    }

    pub fn add_room(&mut self, room: &Room) -> Result<(), String> {
        self.service_schema.add_room(room.name())
    }

    pub fn add_device(&mut self, room_name: &str, device: Box<dyn Device>) -> Result<(), String> {
        let device_name = String::from(device.name()).clone();
        self.service_devices.add_device(device)?;
        self.service_schema
            .add_device(room_name, device_name.as_str())?;
        Ok(())
    }

    pub fn connect_device(
        &mut self,
        room_name: &str,
        device_connects_to: &str,
        device_connected: &str,
    ) -> Result<(), String> {
        self.service_schema
            .connect_device(
                room_name,
                device_connects_to,
                device_connected,
            )
            .unwrap();

        Ok(())
    }

    pub fn get_device(&self, device_name: &str) -> Option<&Box<dyn Device>> {
        self.service_devices.get_device(device_name)
    }

    pub fn get_devices(&self, room_name: &str) -> Iter<String> {
        self.service_schema.room_devices(room_name)
    }

    pub fn print_report(&self) {
        let report = self.service_devices.collect_data_for_report();
        println!("Generated report about all devices:\n{}", report);
    }

    pub fn print_schema(&self) {
        let report = self.service_schema.collect_schema();
        println!("{}", report);
    }
}

struct ServiceDevices {
    store_devices: StoreDevices,
}

impl ServiceDevices {
    pub fn new() -> ServiceDevices {
        let store_devices = StoreDevices::new();
        let tmp = ServiceDevices { store_devices };

        tmp
    }

    pub fn add_device(&mut self, device: Box<dyn Device>) -> Result<(), String> {
        if self.store_devices.contains(&device) {
            let message = format!("Devices store already contains device [{}]", device.name());
            Err(message)
        } else {
            self.store_devices.add_device(device);
            Ok(())
        }
    }

    pub fn get_device(&self, device_name: &str) -> Option<&Box<dyn Device>> {
        self.store_devices.iter().find(|&device| device.name() == device_name)
    }

    pub fn _remove_device(&self, _device: Box<dyn Device>) -> Result<(), String> {
        todo!()
    }

    pub fn collect_data_for_report(&self) -> String {
        let mut info = String::from("");
        for device in self.store_devices.iter() {
            info.push_str(device.info().as_str());
            info.push_str("\n\n");
        }
        info
    }
}

struct ServiceSchemaDevices {
    store_schema: StoreDeviceLinks,
}

impl ServiceSchemaDevices {
    pub fn new() -> Self {
        let store_schema = StoreDeviceLinks::new();
        ServiceSchemaDevices { store_schema }
    }

    pub fn add_room(&mut self, room_name: &str) -> Result<(), String> {
        if self.store_schema.contains_room(room_name) {
            let message = format!("Room [{}] already contains in home", room_name);
            Err(message)
        } else {
            self.store_schema.add_room(room_name);
            Ok(())
        }
    }

    pub fn add_device(&mut self, room_name: &str, device_name: &str) -> Result<(), String> {
        if !self.store_schema.contains_room(room_name) {
            let message = format!("Home not contains room[{}]", room_name);
            return Err(message);
        }
        if self.store_schema.contains_device(room_name, device_name) {
            let message = format!(
                "Room[{}] already contains device[{}]",
                room_name, device_name
            );
            return Err(message);
        }

        self.store_schema.add_device(room_name, device_name);

        Ok(())
    }

    pub fn connect_device(
        &mut self,
        room_name: &str,
        device_connects_to: &str,
        device_connected: &str,
    ) -> Result<(), String> {
        if !self.store_schema.contains_device(room_name, device_connected) {
            let message = format!(
                "Room[{}] not contains device[{}]",
                room_name, device_connected
            );
            return Err(message);
        }
        if !self
            .store_schema
            .contains_device(room_name, device_connects_to)
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

    pub fn room_devices(&self, room_name: &str) -> Iter<String> {
        self.store_schema.entities(room_name)
    }

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

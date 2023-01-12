use crate::stores::HOME_NAME;
use crate::{
    devices::Device,
    places::Room,
    stores::{StoreDeviceLinks, StoreDevices},
};
use std::slice::Iter;

/// Struct to store services to manage home
///
/// Struct contains two services:
/// 1. Service which manage devices store
/// 2. Service which manage schema of the home
pub struct ServiceDeviceManagement {
    service_devices: ServiceDevices,
    service_schema: ServiceSchemaDevices,
}

impl Default for ServiceDeviceManagement {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceDeviceManagement {
    /// Method create new struct
    ///
    /// Example:
    /// ```
    /// use crate::home::services::ServiceDeviceManagement;
    ///
    /// let service = ServiceDeviceManagement::new();
    /// ```
    pub fn new() -> Self {
        let service_devices = ServiceDevices::new();
        let service_schema = ServiceSchemaDevices::new();
        ServiceDeviceManagement {
            service_devices,
            service_schema,
        }
    }

    /// Method add new room to services
    ///
    /// Now, service add room only to schema service
    ///
    /// Example:
    /// ```
    /// use crate::home::places::Room;
    /// use crate::home::services::ServiceDeviceManagement;
    ///
    /// let room = Room::new("My room");
    /// let mut service = ServiceDeviceManagement::new();
    /// let result: Result<(), String> = service.add_room(&room);
    ///
    /// # assert!(result.is_ok()); // normal add is OK
    /// # assert!(service.add_room(&room).is_err()); // duplicate add is KO
    /// ```
    pub fn add_room(&mut self, room: &Room) -> Result<(), String> {
        self.service_schema.add_room(room.name())
    }

    /// Method add device to specific room to services
    ///
    /// Example:
    /// ```
    /// use crate::home::devices::socket::Socket;
    /// use crate::home::places::Room;
    /// use crate::home::services::ServiceDeviceManagement;
    ///
    /// let room = Room::new("My room");
    /// let mut service = ServiceDeviceManagement::new();
    /// service.add_room(&room).unwrap();
    ///
    /// let device = Socket::new();
    /// let result: Result<(), String> = service.add_device(room.name(), Box::new(device));
    ///
    /// # assert!(result.is_ok()); // normal add is OK
    /// # assert!(service.add_device(room.name(), Box::new(Socket::new())).is_err()); // duplicate add is KO
    /// ```
    pub fn add_device(&mut self, room_name: &str, device: Box<dyn Device>) -> Result<(), String> {
        let device_name = String::from(device.name());
        self.service_devices.add_device(device)?;
        self.service_schema
            .add_device(room_name, device_name.as_str())?;
        Ok(())
    }

    /// Method connect devices
    /// TODO: rewrite method
    ///
    pub fn connect_device(
        &mut self,
        room_name: &str,
        device_connects_to: &str,
        device_connected: &str,
    ) -> Result<(), String> {
        self.service_schema
            .connect_device(room_name, device_connects_to, device_connected)
            .unwrap();

        Ok(())
    }

    /// Method return device from store devices by specific name
    ///
    /// Example:
    /// ```
    /// use crate::home::devices::Device;
    /// use crate::home::devices::socket::Socket;
    /// use crate::home::places::Room;
    /// use crate::home::services::ServiceDeviceManagement;
    ///
    /// let room = Room::new("My room");
    /// let mut service = ServiceDeviceManagement::new();
    /// service.add_room(&room).unwrap();
    ///
    /// let device_name = "My device";
    /// let device = Socket::from(device_name, "Description", 1000.0);
    /// service.add_device(room.name(), Box::new(device)).unwrap();
    ///
    /// let device_opt: Option<&Box<dyn Device>> = service.get_device(device_name);
    ///
    /// # assert!(device_opt.is_some()); // normal get device return some is OK
    /// # assert_eq!(device_opt.unwrap().name(), device_name); // normal get device is OK
    /// # assert!(service.get_device("Unknown device").is_none()); // unknown get device is KO
    /// ```
    pub fn get_device(&self, device_name: &str) -> Option<&dyn Device> {
        self.service_devices.get_device(device_name)
    }

    /// Method return iter with list of devices names in the room
    ///
    /// Example:
    /// ```
    /// use crate::home::devices::socket::Socket;
    /// use crate::home::places::Room;
    /// use crate::home::services::ServiceDeviceManagement;
    ///
    /// let room_name = "My room";
    /// let room = Room::new(room_name);
    /// let mut service = ServiceDeviceManagement::new();
    /// service.add_room(&room).unwrap();
    ///
    /// let device_name = "My device";
    /// let device = Socket::from(device_name, "Description", 1000.0);
    /// service.add_device(room.name(), Box::new(device)).unwrap();
    ///
    /// let devices = service.get_devices(room_name);
    /// # assert!(!devices.collect::<Vec<&String>>().is_empty()); // normal get devices is OK
    /// # assert!(service.get_devices("Unknown room").collect::<Vec<&String>>().is_empty()); // unknown room get devices is KO
    /// ```
    pub fn get_devices(&self, room_name: &str) -> Iter<String> {
        self.service_schema.room_devices(room_name)
    }

    /// Method collect report and print it to stdout
    ///
    /// Example:
    /// ```
    /// use crate::home::devices::socket::Socket;
    /// use crate::home::places::Room;
    /// use crate::home::services::ServiceDeviceManagement;
    ///
    /// let room_name = "My room";
    /// let room = Room::new(room_name);
    /// let mut service = ServiceDeviceManagement::new();
    /// service.add_room(&room).unwrap();
    ///
    /// let device_name = "My device";
    /// let device = Socket::from(device_name, "Description", 1000.0);
    /// service.add_device(room.name(), Box::new(device)).unwrap();
    ///
    /// service.print_report();
    /// ```
    pub fn print_report(&self) {
        let report = self.service_devices.collect_data_for_report();
        println!("Generated report about all devices:\n{}", report);
    }

    /// Method collect schema information and print it to stdout
    ///
    /// Example:
    /// ```
    /// use crate::home::devices::socket::Socket;
    /// use crate::home::places::Room;
    /// use crate::home::services::ServiceDeviceManagement;
    ///
    /// let room_name = "My room";
    /// let room = Room::new(room_name);
    /// let mut service = ServiceDeviceManagement::new();
    /// service.add_room(&room).unwrap();
    ///
    /// let device_name = "My device";
    /// let device = Socket::from(device_name, "Description", 1000.0);
    /// service.add_device(room.name(), Box::new(device)).unwrap();
    ///
    /// service.print_schema();
    /// ```
    pub fn print_schema(&self) {
        let report = self.service_schema.collect_schema();
        println!("{}", report);
    }
}

/// Struct to manage store of device of the home
///
/// Struct contains only store with all devices
///
struct ServiceDevices {
    store_devices: StoreDevices,
}

impl ServiceDevices {
    /// Method create new service
    pub fn new() -> ServiceDevices {
        let store_devices = StoreDevices::new();
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
struct ServiceSchemaDevices {
    store_schema: StoreDeviceLinks,
}

impl ServiceSchemaDevices {
    /// Method create new service
    pub fn new() -> Self {
        let store_schema = StoreDeviceLinks::new();
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

    /// Method add device to the room
    ///
    /// If room and device is unique - return Ok
    /// If room OR device already exists - return Error
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
            .contains_device(room_name, device_connected)
        {
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

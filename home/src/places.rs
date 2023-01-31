use std::net::TcpListener;
use std::thread;
use crate::devices::Device;
use crate::network::HOME_PORT;
use crate::services::{ServiceDevices, ServiceSchemaDevices};
use crate::stores::{StoreDevices, StoreDeviceLinks};

/// Home structure
///
/// Home contains some architecture of items.
/// On first level can placed rooms and unconnected devices.
/// Next levels are for connected devices
///
pub struct Home {
    name: String,
    service_devices: ServiceDevices,
    service_schema: ServiceSchemaDevices,
    tcp_listener: TcpListener
}

impl Home {
    /// Method create new example of Home with specific name
    ///
    /// Example:
    /// ```
    /// use crate::home::places::Home;
    ///
    /// let home = Home::new("MY best Home");
    /// ```
    pub fn new(name: &str) -> Self {
        let store_devices = StoreDevices::new();
        let store_schema = StoreDeviceLinks::new();

        let service_devices = ServiceDevices::new(store_devices);
        let service_schema = ServiceSchemaDevices::new(store_schema);

        let mut address = String::from("127.0.0.1:");
        address.push_str(HOME_PORT);
        let tcp_listener = TcpListener::bind(address).unwrap();

        Self {
            name: String::from(name),
            service_devices,
            service_schema,
            tcp_listener
        }
    }

    /// Method create new example of Home with specific name and created all services
    ///
    /// Example:
    /// ```
    /// use crate::home::places::Home;
    /// use crate::home::services::{ServiceDevices, ServiceSchemaDevices};
    /// use crate::home::stores::{StoreDevices, StoreDeviceLinks};
    ///
    /// let store_devices = StoreDevices::new();
    /// let store_schema = StoreDeviceLinks::new();
    ///
    /// let service_devices = ServiceDevices::new(store_devices);
    /// let service_schema = ServiceSchemaDevices::new(store_schema);
    ///
    /// let home = Home::from("MY best Home", service_devices, service_schema);
    /// ```
    pub fn from(name: &str, service_devices: ServiceDevices, service_schema: ServiceSchemaDevices) -> Self {
        let mut address = String::from("127.0.0.1:");
        address.push_str(HOME_PORT);
        let tcp_listener = TcpListener::bind(address).unwrap();

        Self {
            name: String::from(name),
            service_devices,
            service_schema,
            tcp_listener
        }
    }

    /// Method return name of the Home
    ///
    /// Example:
    /// ```
    /// use crate::home::places::Home;
    ///
    /// let home = Home::new("MY best Home");
    /// println!("{}", home.name());
    ///
    /// # assert_eq!(home.name(), "MY best Home")
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Method add new room to the Home
    ///
    /// Example:
    /// ```
    /// use crate::home::places::{Home, Room};
    ///
    /// let mut home = Home::new("MY best Home");
    ///
    /// home.add_room("Kitchen").unwrap();
    ///
    /// # assert!(home.rooms().contains(&String::from("Kitchen")));
    /// # assert!(home.add_room("Kitchen").is_err())
    /// ```
    pub fn add_room(&mut self, room: &str) -> Result<(), String> {
        self.service_schema.add_room(room)
    }

    /// Method remove room from home
    ///
    /// Example:
    /// ```
    /// use crate::home::places::{Home, Room};
    ///
    /// let mut home = Home::new("MY best Home");
    /// let room_name = "Kitchen";
    ///
    /// home.add_room("Kitchen").unwrap();
    /// let result = home.remove_room(room_name);
    ///
    /// # assert!(!home.rooms().contains(&String::from(room_name))); // normal remove is OK
    /// # assert!(home.remove_room(room_name).is_err()) // second remove is KO
    /// ```
    pub fn remove_room(&mut self, room_name: &str) -> Result<(), String> {
        self.service_schema.remove_room(room_name)
    }

    /// Method return struct Room by specific name
    ///
    /// Example:
    /// ```
    /// use crate::home::places::Home;
    ///
    /// let mut home = Home::new("MY best Home");
    ///
    /// home.add_room("Kitchen").unwrap();
    ///
    /// let room_opt: Option<String> = home.room("Kitchen");
    ///
    /// match room_opt {
    ///     Some(room_found) => println!("Found room: {}", room_found.name()),
    ///     None => println!("Not found room!")
    /// }
    ///
    /// # assert_eq!(home.room("Kitchen").unwrap().name(), "Kitchen")
    /// ```
    pub fn room(&self, name: &str) -> Option<String> {
        self
            .service_schema
            .rooms()
            .iter()
            .find(|&room_name| room_name.as_str() == name)
            .map(|room| room.clone())
    }

    /// Method return all rooms names
    ///
    /// Example:
    /// ```
    /// use crate::home::places::Home;
    ///
    /// let mut home = Home::new("MY best Home");
    ///
    /// home.add_room("Kitchen").unwrap();
    ///
    /// for room in home.rooms() {
    ///     println!("{}", room);
    /// }
    ///
    /// # assert!(!home.rooms().is_empty())
    /// ```
    pub fn rooms(&self) -> Vec<String> {
        self.service_schema.rooms()
    }

    /// Method add new device to the room
    ///
    /// Example:
    /// ```
    /// use crate::home::places::Home;
    /// use crate::home::devices::socket::Socket;
    ///
    /// let mut home = Home::new("MY best Home");
    /// let room_name = "Kitchen";
    /// home.add_room(room_name).unwrap();
    ///
    /// let device = Socket::new();
    /// let result: Result<(), String> = home.add_device(room_name, Box::new(device));
    ///
    /// # assert!(result.is_ok()); // normal add is OK
    /// # assert!(home.add_device(room_name, Box::new(Socket::new())).is_err()); // duplicate add is KO
    /// # assert!(home.add_device("Unknown room", Box::new(Socket::new())).is_err()) // add to the unknown room is KO
    /// ```
    pub fn add_device(&mut self, room_name: &str, device: Box<dyn Device>) -> Result<(), String> {
        let device_name = String::from(device.name());
        self.service_devices.add_device(device)?;
        self.service_schema
            .add_device(room_name, device_name.as_str())?;
        Ok(())
    }

    /// Method remove device from home
    ///
    /// Example:
    /// ```
    /// use crate::home::places::Home;
    /// use crate::home::devices::socket::Socket;
    ///
    /// let mut home = Home::new("MY best Home");
    /// let room_name = "Kitchen";
    /// home.add_room(room_name).unwrap();
    ///
    /// home.add_device(room_name, Box::new(Socket::from("S01", "S01 Description", 1000.0))).unwrap();
    /// home.add_device(room_name, Box::new(Socket::from("S02", "S02 Description", 1000.0))).unwrap();
    /// home.add_device(room_name, Box::new(Socket::from("S03", "S03 Description", 1000.0))).unwrap();
    /// home.add_device(room_name, Box::new(Socket::from("S04", "S04 Description", 1000.0))).unwrap();
    ///
    /// let result = home.remove_device("S03");
    /// let devices = home.devices_in_room(room_name);
    ///
    /// # assert!(result.is_ok());
    /// # assert!(!devices.contains(&String::from("S03")));
    /// # assert_eq!(devices.len(), 3);
    /// ```
    pub fn remove_device(&mut self, device_name: &str) -> Result<(), String> {
        self.service_schema.remove_device(device_name)?;
        self.service_devices.remove_device(device_name)
    }

    /// Method connects one device to another
    ///
    /// Method is not tested
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

    /// Method find device by its name
    ///
    /// Example:
    /// ```
    /// use crate::home::places::Home;
    /// use crate::home::devices::socket::Socket;
    ///
    /// let mut home = Home::new("MY best Home");
    /// let room_name = "Kitchen";
    /// home.add_room(room_name).unwrap();
    ///
    /// let device = Socket::from("Socket", "Description of Socket", 1000.0);
    /// home.add_device(room_name, Box::new(device)).unwrap();
    ///
    /// let device_opt = home.device("Socket");
    /// match device_opt {
    ///     Some(device_found) => println!("Found device with name {}", device_found.name()),
    ///     None => panic!("Not found device!")
    /// }
    ///
    /// # assert!(home.device("Socket").is_some()); // normal find is OK
    /// # assert!(home.device("Unknown device").is_none()); // find of unknown device is KO
    /// ```
    pub fn device(&self, device_name: &str) -> Option<&dyn Device> {
        self.service_devices.get_device(device_name)
    }

    /// Method return list of all devices names in the specific room
    ///
    /// Example:
    /// ```
    /// use crate::home::places::Home;
    /// use crate::home::devices::socket::Socket;
    ///
    /// let mut home = Home::new("MY best Home");
    /// let room_name = "Kitchen";
    /// home.add_room(room_name).unwrap();
    ///
    /// let device = Socket::from("Socket", "Description of Socket", 1000.0);
    /// home.add_device(room_name, Box::new(device)).unwrap();
    ///
    /// let devices_names = home.devices_in_room(room_name);
    ///
    /// for device_name in devices_names {
    ///     println!("{}", device_name);
    /// }
    ///
    /// # assert!(!home.devices_in_room(room_name).is_empty()); // get list of normal room is OK
    /// # assert!(home.devices_in_room("Unknown room").is_empty()); // get list of unknown room is KO
    /// ```
    pub fn devices_in_room(&self, room_name: &str) -> Vec<String> {
        let result: Vec<String> = self
            .service_schema
            .room_devices(room_name)
            .cloned()
            .collect();

        result
    }

    pub fn devices(&self) -> Vec<String> {
        self.service_devices.get_devices()
    }

    pub fn start_process(&self) {
        let listener = &self.tcp_listener;

        thread::scope(move |_| {
            for stream in listener.incoming() {
                let stream = stream.unwrap();

                let _ = thread::spawn(move || {
                    println!("Connection created! {:?}", stream);
                });

            }
        })
    }

    /// Method print report about all devices of the home
    ///
    /// Example:
    /// ```
    /// use crate::home::places::Home;
    /// use crate::home::devices::socket::Socket;
    ///
    /// let mut home = Home::new("MY best Home");
    /// let room_name = "Kitchen";
    /// home.add_room(room_name).unwrap();
    ///
    /// let device = Socket::from("Socket", "Description of Socket", 1000.0);
    /// home.add_device(room_name, Box::new(device)).unwrap();
    ///
    /// home.print_report(); // prints information about only 1 device - Socket
    /// ```
    pub fn print_report(&self) {
        let report = self.service_devices.collect_data_for_report();
        println!("Generated report about all devices:\n{}", report);
    }

    /// Method print schema connections of the home
    ///
    /// Example:
    /// ```
    /// use crate::home::places::Home;
    /// use crate::home::devices::socket::Socket;
    ///
    /// let mut home = Home::new("MY best Home");
    /// let room_name = "Kitchen";
    /// home.add_room(room_name).unwrap();
    ///
    /// let device = Socket::from("Socket", "Description of Socket", 1000.0);
    /// home.add_device(room_name, Box::new(device)).unwrap();
    ///
    /// home.print_schema(); // prints room connected to the home and device connected to the home
    /// ```
    pub fn print_schema(&self) {
        let report = self.service_schema.collect_schema();
        println!("{}", report);
    }
}

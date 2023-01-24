use crate::devices::Device;
use crate::services::{ServiceDevices, ServiceSchemaDevices};

/// Home structure
///
/// Home contains some architecture of items.
/// On first level can placed rooms and unconnected devices.
/// Next levels are for connected devices
///
pub struct Home {
    name: String,
    rooms: Vec<Room>,
    service_devices: ServiceDevices,
    service_schema: ServiceSchemaDevices,
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
        let service_devices = ServiceDevices::new();
        let service_schema = ServiceSchemaDevices::new();

        Self {
            name: String::from(name),
            rooms: vec![],
            service_devices,
            service_schema,
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
    /// let room = Room::new("Kitchen");
    ///
    /// home.add_room(room).unwrap();
    ///
    /// # assert!(home.rooms().contains(&String::from("Kitchen")));
    /// # assert!(home.add_room(Room::new("Kitchen")).is_err())
    /// ```
    pub fn add_room(&mut self, room: Room) -> Result<(), String> {
        self.service_schema.add_room(room.name())
    }

    /// Method remove room from home
    ///
    /// Example:
    /// ```
    /// use crate::home::places::{Home, Room};
    ///
    /// let mut home = Home::new("MY best Home");
    /// let room_name = "Kitchen";
    /// let room = Room::new(room_name);
    ///
    /// home.add_room(room).unwrap();
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
    /// use crate::home::places::{Home, Room};
    ///
    /// let mut home = Home::new("MY best Home");
    /// let room = Room::new("Kitchen");
    ///
    /// home.add_room(room).unwrap();
    ///
    /// let room_opt: Option<&Room> = home.room("Kitchen");
    ///
    /// match room_opt {
    ///     Some(room_found) => println!("Found room: {}", room_found.name()),
    ///     None => println!("Not found room!")
    /// }
    ///
    /// # assert_eq!(home.room("Kitchen").unwrap().name(), "Kitchen")
    /// ```
    pub fn room(&self, name: &str) -> Option<&Room> {
        self.rooms.iter().find(|&room| room.name.as_str() == name)
    }

    /// Method return all rooms names
    ///
    /// Example:
    /// ```
    /// use crate::home::places::{Home, Room};
    ///
    /// let mut home = Home::new("MY best Home");
    /// let room = Room::new("Kitchen");
    ///
    /// home.add_room(room).unwrap();
    ///
    /// for room in home.rooms() {
    ///     println!("{}", room);
    /// }
    ///
    /// # assert!(!home.rooms().is_empty())
    /// ```
    pub fn rooms(&self) -> Vec<String> {
        self.rooms.iter().map(|room| room.name.clone()).collect()
    }

    /// Method add new device to the room
    ///
    /// Example:
    /// ```
    /// use crate::home::places::{Home, Room};
    /// use crate::home::devices::socket::Socket;
    ///
    /// let mut home = Home::new("MY best Home");
    /// let room_name = "Kitchen";
    /// let room = Room::new(room_name);
    /// home.add_room(room).unwrap();
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
    /// use crate::home::places::{Home, Room};
    /// use crate::home::devices::socket::Socket;
    ///
    /// let mut home = Home::new("MY best Home");
    /// let room_name = "Kitchen";
    /// let room = Room::new(room_name);
    /// home.add_room(room).unwrap();
    ///
    /// home.add_device(room_name, Box::new(Socket::from("S01", "S01 Description", 1000.0))).unwrap();
    /// home.add_device(room_name, Box::new(Socket::from("S02", "S02 Description", 1000.0))).unwrap();
    /// home.add_device(room_name, Box::new(Socket::from("S03", "S03 Description", 1000.0))).unwrap();
    /// home.add_device(room_name, Box::new(Socket::from("S04", "S04 Description", 1000.0))).unwrap();
    ///
    /// let result = home.remove_device("S03");
    /// let devices = home.devices(room_name);
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
    /// use crate::home::places::{Home, Room};
    /// use crate::home::devices::socket::Socket;
    ///
    /// let mut home = Home::new("MY best Home");
    /// let room_name = "Kitchen";
    /// let room = Room::new(room_name);
    /// home.add_room(room).unwrap();
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
    /// use crate::home::places::{Home, Room};
    /// use crate::home::devices::socket::Socket;
    ///
    /// let mut home = Home::new("MY best Home");
    /// let room_name = "Kitchen";
    /// let room = Room::new(room_name);
    /// home.add_room(room).unwrap();
    ///
    /// let device = Socket::from("Socket", "Description of Socket", 1000.0);
    /// home.add_device(room_name, Box::new(device)).unwrap();
    ///
    /// let devices_names = home.devices(room_name);
    ///
    /// for device_name in devices_names {
    ///     println!("{}", device_name);
    /// }
    ///
    /// # assert!(!home.devices(room_name).is_empty()); // get list of normal room is OK
    /// # assert!(home.devices("Unknown room").is_empty()); // get list of unknown room is KO
    /// ```
    pub fn devices(&self, room_name: &str) -> Vec<String> {
        let result: Vec<String> = self
            .service_schema
            .room_devices(room_name)
            .cloned()
            .collect();

        result
    }

    /// Method print report about all devices of the home
    ///
    /// Example:
    /// ```
    /// use crate::home::places::{Home, Room};
    /// use crate::home::devices::socket::Socket;
    ///
    /// let mut home = Home::new("MY best Home");
    /// let room_name = "Kitchen";
    /// let room = Room::new(room_name);
    /// home.add_room(room).unwrap();
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
    /// use crate::home::places::{Home, Room};
    /// use crate::home::devices::socket::Socket;
    ///
    /// let mut home = Home::new("MY best Home");
    /// let room_name = "Kitchen";
    /// let room = Room::new(room_name);
    /// home.add_room(room).unwrap();
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

/// Struct to store Room information
///
/// Struct contains only name
pub struct Room {
    name: String,
}

impl Room {
    /// Method create new room by specific name
    ///
    /// Example:
    /// ```
    /// use crate::home::places::Room;
    ///
    /// let room = Room::new("My best Room");
    /// # assert_eq!(room.name(), "My best Room");
    /// ```
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }

    /// Method return name of the room
    ///
    /// Example:
    /// ```
    /// use crate::home::places::Room;
    ///
    /// let room = Room::new("My best Room");
    /// let room_name = room.name();
    ///
    /// # assert_eq!(room_name, "My best Room");
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

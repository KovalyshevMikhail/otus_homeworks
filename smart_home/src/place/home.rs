use std::collections::{HashMap, VecDeque};
use std::io::Read;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use regex::Regex;
use crate::devices::Device;
use crate::network::{HOME_PORT, recv_string};
use crate::network::tcp_command::TcpCommand;
use crate::network::tcp_entity::{TcpCommunication, TcpConnection, TcpNode};
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
    tcp_node: Arc<TcpNode>,
    pub tcp_clients: Arc<Mutex<HashMap<String, TcpConnection>>>,
    pub commands: Arc<Mutex<VecDeque<TcpCommand>>>,
}

impl Home {
    /// Method create new example of Home with specific name
    ///
    /// Example:
    /// ```
    /// use crate::home::place::home::Home;
    ///
    /// let home = Home::new("MY best Home");
    /// ```
    pub fn new(name: &str) -> Self {
        let store_devices = StoreDevices::new();
        let store_schema = StoreDeviceLinks::new();

        let service_devices = ServiceDevices::new(store_devices);
        let service_schema = ServiceSchemaDevices::new(store_schema);

        let tcp_node = Arc::new(TcpNode::new(HOME_PORT));
        let tcp_clients = Arc::new(Mutex::new(HashMap::new()));

        let commands = Arc::new(Mutex::new(VecDeque::new()));

        Self {
            name: String::from(name),
            service_devices,
            service_schema,
            tcp_node,
            tcp_clients,
            commands
        }
    }

    /// Method create new example of Home with specific name and created all services
    ///
    /// Example:
    /// ```
    /// use crate::home::place::home::Home;
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
        let tcp_node = Arc::new(TcpNode::new(HOME_PORT));
        let tcp_clients = Arc::new(Mutex::new(HashMap::new()));
        let commands = Arc::new(Mutex::new(VecDeque::new()));

        Self {
            name: String::from(name),
            service_devices,
            service_schema,
            tcp_node,
            tcp_clients,
            commands
        }
    }

    /// Method return name of the Home
    ///
    /// Example:
    /// ```
    /// use crate::home::place::home::Home;
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
    /// use crate::home::place::home::Home;
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

    /// Method remove room from smart_home
    ///
    /// Example:
    /// ```
    /// use crate::home::place::home::Home;
    ///
    /// let mut smart_home = Home::new("MY best Home");
    /// let room_name = "Kitchen";
    ///
    /// smart_home.add_room("Kitchen").unwrap();
    /// let result = smart_home.remove_room(room_name);
    ///
    /// # assert!(!smart_home.rooms().contains(&String::from(room_name))); // normal remove is OK
    /// # assert!(smart_home.remove_room(room_name).is_err()) // second remove is KO
    /// ```
    pub fn remove_room(&mut self, room_name: &str) -> Result<(), String> {
        self.service_schema.remove_room(room_name)
    }

    /// Method return struct Room by specific name
    ///
    /// Example:
    /// ```
    /// use crate::home::place::home::Home;
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
    /// use crate::home::place::home::Home;
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
    /// use crate::home::place::home::Home;
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
    pub fn add_device(&mut self, room_name: &str, device: Box<dyn Device + Send + Sync>) -> Result<(), String> {
        let device_name = String::from(device.name());
        self.service_devices.add_device(device)?;
        self.service_schema
            .add_device(room_name, device_name.as_str())?;
        Ok(())
    }

    /// Method remove device from smart_home
    ///
    /// Example:
    /// ```
    /// use crate::home::place::home::Home;
    /// use crate::home::devices::socket::Socket;
    ///
    /// let mut smart_home = Home::new("MY best Home");
    /// let room_name = "Kitchen";
    /// smart_home.add_room(room_name).unwrap();
    ///
    /// smart_home.add_device(room_name, Box::new(Socket::from("S01", "S01 Description", 1000.0))).unwrap();
    /// smart_home.add_device(room_name, Box::new(Socket::from("S02", "S02 Description", 1000.0))).unwrap();
    /// smart_home.add_device(room_name, Box::new(Socket::from("S03", "S03 Description", 1000.0))).unwrap();
    /// smart_home.add_device(room_name, Box::new(Socket::from("S04", "S04 Description", 1000.0))).unwrap();
    ///
    /// let result = smart_home.remove_device("S03");
    /// let devices = smart_home.devices_in_room(room_name);
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
    /// use crate::home::place::home::Home;
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
    pub fn device(&self, device_name: &str) -> Option<&Box<dyn Device + Send + Sync>> {
        self.service_devices.get_device(device_name)
    }

    /// Method return list of all devices names in the specific room
    ///
    /// Example:
    /// ```
    /// use crate::home::place::home::Home;
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

    /// Method print report about all devices of the smart_home
    ///
    /// Example:
    /// ```
    /// use crate::home::place::home::Home;
    /// use crate::home::devices::socket::Socket;
    ///
    /// let mut smart_home = Home::new("MY best Home");
    /// let room_name = "Kitchen";
    /// smart_home.add_room(room_name).unwrap();
    ///
    /// let device = Socket::from("Socket", "Description of Socket", 1000.0);
    /// smart_home.add_device(room_name, Box::new(device)).unwrap();
    ///
    /// smart_home.print_report(); // prints information about only 1 device - Socket
    /// ```
    pub fn print_report(&self) {
        let report = self.service_devices.collect_data_for_report();
        println!("Generated report about all devices:\n{}", report);
    }

    /// Method print schema connections of the smart_home
    ///
    /// Example:
    /// ```
    /// use crate::home::place::home::Home;
    /// use crate::home::devices::socket::Socket;
    ///
    /// let mut smart_home = Home::new("MY best Home");
    /// let room_name = "Kitchen";
    /// smart_home.add_room(room_name).unwrap();
    ///
    /// let device = Socket::from("Socket", "Description of Socket", 1000.0);
    /// smart_home.add_device(room_name, Box::new(device)).unwrap();
    ///
    /// smart_home.print_schema(); // prints room connected to the smart_home and device connected to the smart_home
    /// ```
    pub fn print_schema(&self) {
        let report = self.service_schema.collect_schema();
        println!("{}", report);
    }
}

impl TcpCommunication for Home {
    fn start_receive(&mut self) {
        let tcp_node = self.tcp_node.clone();
        let tcp_clients = self.tcp_clients.clone();
        let commands = self.commands.clone();

        thread::spawn(move || {
            let receive_iter = tcp_node.incoming();

            for tcp_connection_r in receive_iter {
                match tcp_connection_r {
                    Ok(tcp_connection) => {
                        let addr = tcp_connection.peer_addr().unwrap();
                        // try to register
                        println!("HOME. Connection is OK {:?}", addr);

                        //self.tcp_connection.add_connection("custom", stream).unwrap();
                        let tcp_commands: Arc<Mutex<VecDeque<TcpCommand>>> = commands.clone();
                        let tcp_clients_cloned: Arc<Mutex<HashMap<String, TcpConnection>>> = tcp_clients.clone();

                        thread::spawn(move || {
                            let addr = tcp_connection.peer_addr().unwrap().to_string();
                            tcp_clients_cloned.lock().unwrap().insert(addr.clone(), tcp_connection).unwrap();

                            if handle_function(addr.clone(), tcp_commands, tcp_clients_cloned).is_err() {
                                println!("HOME. Connection [{}] is closed", addr);
                            }
                        });
                    }
                    Err(err) => {
                        println!("HOME. error in Socket connects {}", err);
                    }
                }
            }
        });
    }

    fn try_register(&mut self) -> Result<(), String>{
        Ok(())
    }

    fn save_tcp_client(&mut self, connection: TcpConnection, name: &str) -> Result<(), String> {
        if !self.tcp_clients.lock().unwrap().contains_key(name) {
            self.tcp_clients.lock().unwrap().insert(name.to_string(), connection);
            Ok(())
        } else {
            let message = format!("HOME. tcp client with name [{}] already exists", name);
            Err(message)
        }
    }

    fn get_tcp_client(&self, name: &str) -> Option<&TcpConnection> {
        todo!()
    }
}

fn handle_function(addr: String, tcp_commands: Arc<Mutex<VecDeque<TcpCommand>>>, tcp_clients: Arc<Mutex<HashMap<String, TcpConnection>>>) -> Result<(), String> {
    let mut rounds = 0;
    let mut clients = tcp_clients.lock();
    let connection = clients.as_mut().unwrap().get_mut(addr.as_str()).unwrap();

    loop {
        println!("HOME. read round {}", rounds);
        let request = connection.receive_request()?;
        let re = Regex::new(r"\[(\w+):\s*\[(\w+)\]\]").unwrap();

        println!("HOME. receive request {}", request);

        let caps = re.captures(request.as_str()).unwrap();
        let command = caps.get(1).unwrap().as_str();
        let args = caps.get(2).unwrap().as_str();
        let addr = connection.peer_addr().unwrap().to_string();
        let tcp_command = TcpCommand::new(command, args, addr.as_str());

        println!("HOME. receive command {:?}", tcp_command);

        tcp_commands.lock().unwrap().push_front(tcp_command);

        rounds += 1;
    }
}

pub struct HomeRequestHandler {
    home: Arc<Mutex<Home>>,
    tcp_clients: Arc<Mutex<HashMap<String, TcpConnection>>>,
    commands_in: Arc<Mutex<VecDeque<TcpCommand>>>,
    commands_out: Arc<Mutex<VecDeque<TcpCommand>>>
}

impl HomeRequestHandler {
    pub fn new(home: Arc<Mutex<Home>>, commands: Arc<Mutex<VecDeque<TcpCommand>>>, tcp_clients: Arc<Mutex<HashMap<String, TcpConnection>>>) -> HomeRequestHandler {
        let commands_out = Arc::new(Mutex::new(VecDeque::new()));

        Self {
            home,
            tcp_clients,
            commands_in: commands,
            commands_out
        }
    }

    pub fn job_analyze_commands(&mut self) {
        let home_clone = self.home.clone();
        let commands_in_clone = self.commands_in.clone();
        let commands_out_clone = self.commands_out.clone();

        thread::spawn(move || {
            let mut rounds = 0;

            loop {
                println!("HOME. job_analyze_commands. round {}", rounds);

                while let Some(command_in) = commands_in_clone.lock().unwrap().pop_back() {
                    println!("HOME. job_analyze_commands. analyze command [{:?}]", command_in);

                    if command_in.command == "register" {
                        commands_out_clone.lock().unwrap().push_front(TcpCommand::new("OK", "", command_in.sender.as_str()));
                        commands_out_clone.lock().unwrap().push_front(TcpCommand::new("info", "", command_in.sender.as_str()));
                    }
                }

                thread::sleep(Duration::from_secs(1));
                rounds += 1;
            }
        });
    }

    pub fn job_send_commands_out(&mut self) {
        let home_clone = self.home.clone();
        let tcp_clients = self.tcp_clients.clone();
        let commands_out_clone = self.commands_out.clone();

        thread::spawn(move || {
            let mut rounds = 0;

            loop {
                println!("HOME. job_send_commands_out. round {}", rounds);

                while let Some(command_out) = commands_out_clone.lock().unwrap().pop_back() {
                    println!("HOME. job_send_commands_out. send out command [{:?}]", command_out);
                    let mut clients = tcp_clients.lock();
                    let connection = clients.as_mut().unwrap().get_mut(command_out.sender.as_str()).unwrap();
                    crate::network::send_string(command_out.to_string(), &mut connection.stream).unwrap();
                }

                thread::sleep(Duration::from_secs(1));
                rounds += 1;
            }
        });
    }
}
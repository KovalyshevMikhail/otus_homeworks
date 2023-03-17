use std::collections::{HashMap, VecDeque};
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use regex::Regex;
use crate::network::HOME_PORT;
use crate::network::tcp_command::TcpCommand;
use crate::network::tcp_entity::{TcpCommunication, TcpConnection, TcpNode};
use super::Device;

/// Socket of smart smart_home
///
/// Example
/// ```
/// use crate::home::devices::socket::Socket;
///
/// // create socket with default parameters
/// let socket1 = Socket::new();
///
/// // create socket with custom parameters
/// let socket2 = Socket::from("S01", "description of SW01", 1000.0);
/// ```
pub struct Socket {
    name: String,
    description: String,
    power_max: f32,
    power_consumption: f32,
    enabled: bool,
    tcp_node: Arc<TcpNode>,
    pub tcp_clients: Arc<Mutex<HashMap<String, TcpConnection>>>,
    pub commands: Arc<Mutex<VecDeque<TcpCommand>>>
}

impl Default for Socket {
    fn default() -> Self {
        Self::new()
    }
}

impl Socket {
    /// Function create Socket with default parameters
    /// name - default
    /// description - default description
    /// power_max - 100.0
    ///
    /// Example
    /// ```
    /// use crate::home::devices::socket::Socket;
    ///
    /// let socket1 = Socket::new();
    /// ```
    pub fn new() -> Self {
        let tcp_node = Arc::new(TcpNode::new(53_001));
        let tcp_clients = Arc::new(Mutex::new(HashMap::new()));
        let commands = Arc::new(Mutex::new(VecDeque::new()));

        Socket {
            name: String::from("default"),
            description: String::from("default description"),
            power_max: 100.0,
            power_consumption: 0.0,
            enabled: false,
            tcp_node,
            tcp_clients,
            commands
        }
    }

    /// Function create Socket with custom parameters
    ///
    /// Example:
    /// ```
    /// use crate::home::devices::socket::Socket;
    ///
    /// let socket2 = Socket::from("S01", "description of SW01", 1000.0);
    /// ```
    pub fn from(name: &str, description: &str, power_max: f32) -> Self {
        let tcp_node = Arc::new(TcpNode::new(55_001));
        let tcp_clients = Arc::new(Mutex::new(HashMap::new()));
        let commands = Arc::new(Mutex::new(VecDeque::new()));

        Socket {
            name: String::from(name),
            description: String::from(description),
            power_max,
            power_consumption: 0.0,
            enabled: false,
            tcp_node,
            tcp_clients,
            commands
        }
    }

    /// Method enable power of Socket
    ///
    /// Example:
    /// ```
    /// use crate::home::devices::socket::Socket;
    ///
    /// let mut socket = Socket::from("S01", "description of SW01", 1000.0);
    /// socket.power_on();
    /// ```
    pub fn power_on(&mut self) {
        self.enabled = true;
    }

    /// Method disable power of Socket
    ///
    /// Example:
    /// ```
    /// use crate::home::devices::socket::Socket;
    ///
    /// let mut socket = Socket::from("S01", "description of SW01", 1000.0);
    /// socket._power_off();
    /// ```
    pub fn _power_off(&mut self) {
        self.enabled = false;
    }
}

impl Device for Socket {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn status(&self) -> String {
        String::from("asd")
    }

    /// Method generate info about Socket
    ///
    /// Example:
    /// ```
    /// use crate::home::devices::Device;
    /// use crate::home::devices::socket::Socket;
    ///
    /// let mut socket = Socket::from("S01", "description of SW01", 1000.0);
    /// println!("{}", socket.info());
    /// ```
    fn info(&self) -> String {
        format!(
            "[SOCKET] {} [power is on={}]\n{}\nParameters: [\n\tMax power = {}\n\tPower consumption = {}\n]",
            self.name, self.enabled, self.description, self.power_max, self.power_consumption
        )
    }
}

impl TcpCommunication for Socket {
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
                        println!("SOCKET. Connection is OK {:?}", addr);

                        //self.tcp_connection.add_connection("custom", stream).unwrap();
                        let tcp_commands: Arc<Mutex<VecDeque<TcpCommand>>> = commands.clone();
                        let tcp_clients_cloned: Arc<Mutex<HashMap<String, TcpConnection>>> = tcp_clients.clone();

                        thread::spawn(move || {
                            let addr = tcp_connection.peer_addr().unwrap().to_string();
                            tcp_clients_cloned.lock().unwrap().insert(addr.clone(), tcp_connection).unwrap();

                            if handle_function(addr.clone(), tcp_commands, tcp_clients_cloned).is_err() {
                                println!("SOCKET. Connection [{}] is closed", addr);
                            }
                        });
                    }
                    Err(err) => {
                        println!("SOCKET. error in Socket connects {}", err);
                    }
                }
            }
        });
    }

    fn try_register(&mut self) -> Result<(), String> {
        let home_addr = format!("127.0.0.1:{}", HOME_PORT);
        let mut stream = TcpStream::connect(home_addr).unwrap();

        let command = format!("[register: [{}]]", self.name);
        crate::network::send_string(command.as_str(), &mut stream).unwrap();

        println!("SOCKET. read from HOME");
        let result = crate::network::recv_string(&mut stream).unwrap();

        println!("SOCKET. parse response");
        if result != "[OK: []]" {
            let msg = format!("SOCKET. try register error. received {:?}", result);
            Err(msg)
        } else {
            self.save_tcp_client(TcpConnection { stream }, "home").unwrap();
            Ok(())
        }
    }

    fn save_tcp_client(&mut self, connection: TcpConnection, name: &str) -> Result<(), String> {
        if !self.tcp_clients.lock().unwrap().contains_key(name) {
            self.tcp_clients.lock().unwrap().insert(name.to_string(), connection);
            Ok(())
        } else {
            let message = format!("SOCKET. tcp client with name [{}] already exists", name);
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
        println!("SOCKET. read round {}", rounds);
        let request = connection.receive_request()?;
        let re = Regex::new(r"\[(\w+):\s*\[(\w+)\]\]").unwrap();

        println!("SOCKET. receive request {}", request);

        let caps = re.captures(request.as_str()).unwrap();
        let command = caps.get(1).unwrap().as_str();
        let args = caps.get(2).unwrap().as_str();
        let addr = connection.peer_addr().unwrap().to_string();
        let tcp_command = TcpCommand::new(command, args, addr.as_str());

        println!("SOCKET. receive command {:?}", tcp_command);

        tcp_commands.lock().unwrap().push_front(tcp_command);

        rounds += 1;
    }
}

pub struct SocketRequestHandler {
    socket: Arc<Mutex<Socket>>,
    tcp_clients: Arc<Mutex<HashMap<String, TcpConnection>>>,
    commands_in: Arc<Mutex<VecDeque<TcpCommand>>>,
    commands_out: Arc<Mutex<VecDeque<TcpCommand>>>
}

impl SocketRequestHandler {
    pub fn new(home: Arc<Mutex<Socket>>, commands: Arc<Mutex<VecDeque<TcpCommand>>>, tcp_clients: Arc<Mutex<HashMap<String, TcpConnection>>>) -> SocketRequestHandler {
        let commands_out = Arc::new(Mutex::new(VecDeque::new()));

        Self {
            socket: home,
            tcp_clients,
            commands_in: commands,
            commands_out
        }
    }

    pub fn job_analyze_commands(&mut self) {
        let socket_clone = self.socket.clone();
        let commands_in_clone = self.commands_in.clone();
        let commands_out_clone = self.commands_out.clone();

        thread::spawn(move || {
            let mut rounds = 0;

            loop {
                println!("SOCKET. job_analyze_commands. round {}", rounds);

                while let Some(command_in) = commands_in_clone.lock().unwrap().pop_back() {
                    println!("SOCKET. job_analyze_commands. analyze command [{:?}]", command_in);

                    if command_in.command == "info" {
                        let info = socket_clone.lock().unwrap().info();
                        let tcp_command = TcpCommand::new("device_info", info.as_str(), command_in.sender.as_str());
                        commands_out_clone.lock().unwrap().push_front(tcp_command);
                    }
                }

                thread::sleep(Duration::from_secs(1));
                rounds += 1;
            }
        });
    }

    pub fn job_send_commands_out(&mut self) {
        let socket_clone = self.socket.clone();
        let commands_out_clone = self.commands_out.clone();
        let tcp_clients_clone = self.tcp_clients.clone();

        thread::spawn(move || {
            let mut rounds = 0;

            loop {
                println!("SOCKET. job_send_commands_out. round {}", rounds);

                while let Some(command_out) = commands_out_clone.lock().unwrap().pop_back() {
                    println!("SOCKET. job_send_commands_out. send out command [{:?}]", command_out);
                    let mut clients = tcp_clients_clone.lock();
                    let connection = clients.as_mut().unwrap().get_mut(command_out.sender.as_str()).unwrap();
                    crate::network::send_string(command_out.to_string(), &mut connection.stream).unwrap();
                }

                thread::sleep(Duration::from_secs(1));
                rounds += 1;
            }
        });
    }
}
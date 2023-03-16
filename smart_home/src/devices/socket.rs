use std::collections::VecDeque;
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
    commands: Arc<Mutex<Vec<TcpCommand>>>
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
        let tcp_node = Arc::new(TcpNode::new(55_001));
        let commands = Arc::new(Mutex::new(Vec::new()));

        Socket {
            name: String::from("default"),
            description: String::from("default description"),
            power_max: 100.0,
            power_consumption: 0.0,
            enabled: false,
            tcp_node,
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
        let commands = Arc::new(Mutex::new(Vec::new()));

        Socket {
            name: String::from(name),
            description: String::from(description),
            power_max,
            power_consumption: 0.0,
            enabled: false,
            tcp_node,
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
                        let tcp_commands: Arc<Mutex<Vec<TcpCommand>>> = commands.clone();

                        thread::spawn(move || {
                            if handle_function(tcp_connection, tcp_commands).is_err() {
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
            Ok(())
        }
    }

    fn save_tcp_client(&mut self, connection: TcpStream, name: &str) -> Result<(), String> {
        Ok(())
    }

    fn get_tcp_client(&self, name: &str) -> Result<TcpStream, String> {
        todo!()
    }
}

fn handle_function(mut connection: TcpConnection, tcp_commands: Arc<Mutex<Vec<TcpCommand>>>) -> Result<(), String> {
    loop {
        let request = connection.receive_request()?;
        let re = Regex::new(r"\[(\w): \[(\w+)\]\]").unwrap();

        for parts in re.captures_iter(request.as_str()) {
            let command = &parts[1];
            let args = &parts[2];
            let tcp_command = TcpCommand::new(command, args, connection.stream.try_clone().unwrap());

            println!("SOCKET. receive command {:?}", tcp_command);

            tcp_commands.lock().unwrap().push(tcp_command);
        }
    }
}

pub struct SocketRequestHandler {
    socket: Arc<Mutex<Socket>>,
    commands_in: Arc<Mutex<VecDeque<TcpCommand>>>,
    commands_out: Arc<Mutex<VecDeque<TcpCommand>>>
}

impl SocketRequestHandler {
    pub fn new(home: Arc<Mutex<Socket>>, commands: Arc<Mutex<VecDeque<TcpCommand>>>) -> SocketRequestHandler {
        let commands_out = Arc::new(Mutex::new(VecDeque::new()));

        Self {
            socket: home,
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
                println!("job_analyze_commands. round {}", rounds);

                while let Some(command_in) = commands_in_clone.lock().unwrap().pop_back() {
                    println!("job_analyze_commands. analyze command [{:?}]", command_in);

                    if command_in.command == "info" {
                        let info = socket_clone.lock().unwrap().info();
                        let tcp_command = TcpCommand::new("device_info", info.as_str(), command_in.sender.try_clone().unwrap());
                        commands_out_clone.lock().unwrap().push_front(tcp_command);
                    }
                }

                thread::sleep(Duration::from_secs(1));
                rounds += 1;
            }
        });
    }

    pub fn job_send_commands_out(&mut self) {
        let commands_out_clone = self.commands_out.clone();

        thread::spawn(move || {
            let mut rounds = 0;

            loop {
                println!("job_send_commands_out. round {}", rounds);

                while let Some(command_out) = commands_out_clone.lock().unwrap().pop_back() {
                    println!("job_send_commands_out. send out command [{:?}]", command_out);
                    crate::network::send_string(command_out.to_string(), command_out.sender.try_clone().unwrap()).unwrap();
                }

                thread::sleep(Duration::from_secs(1));
                rounds += 1;
            }
        });
    }
}
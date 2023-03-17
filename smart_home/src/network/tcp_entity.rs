use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};


pub enum NodeType {
    Client,
    Server,
    All
}

pub struct TcpNode
{
    listener: TcpListener,
    pub is_end: bool
}

impl TcpNode
{
    pub fn new(port: u32) -> Self {
        println!("TCP_NODE. Try create listener on port {}", port);
        let listener =
            match TcpListener::bind(format!("127.0.0.1:{}", port)) {
                Ok(listener) => { listener },
                Err(err) => {
                    let message = format!("Binding has errors with port [{}]: {:?}", port, err);
                    panic!("{}", message);
                }
            };

        Self {
            listener,
            is_end: false
        }
    }

    pub fn incoming(&self) -> impl Iterator<Item = Result<TcpConnection, String>> + '_ {
        self.listener.incoming().map(|stream_r| {
            match stream_r {
                Ok(stream) => {
                    let tcp_connection = TcpConnection { stream };
                    Ok(tcp_connection)
                },
                Err(err) => {
                    let msg = format!("Error in receive stream => {}", err);
                    Err(msg)
                }
            }
        })
    }

    pub fn send_command(&self, _command: &str) {
        // crate::network::send_string(command, &mut self.client.unwrap());
    }
}

impl Drop for TcpNode {
    fn drop(&mut self) {
        self.is_end = true
    }
}

pub struct TcpNodeConnection {
    clients: HashMap<String, TcpStream>
}

impl TcpNodeConnection {
    pub fn new() -> TcpNodeConnection {
        let clients = HashMap::new();

        Self {
            clients
        }
    }

    pub fn add_connection(&mut self, name: &str, connection: TcpStream) -> Result<(), String> {
        if self.clients.contains_key(name) {
            let message = format!("TcpNodeConnection. Client already have [{}]", name);
            Err(message)
        } else {
            self.clients.insert(name.to_string(), connection);
            Ok(())
        }
    }
}

trait TcpNodeHandler {
    fn handle(&mut self, message: &str) -> Result<(), String>;
}

pub trait TcpCommunication {
    fn start_receive(&mut self);
    fn try_register(&mut self) -> Result<(), String>;
    fn save_tcp_client(&mut self, connection: TcpConnection, name: &str) -> Result<(), String>;
    fn get_tcp_client(&self, name: &str) -> Option<&TcpConnection>;
}

pub struct TcpConnection {
    pub stream: TcpStream
}

impl TcpConnection {
    pub fn send_response(&mut self, response: &str) -> Result<(), String> {
        match crate::network::send_string(response, &mut self.stream) {
            Ok(_) => {
                Ok(())
            }
            Err(err) => {
                let message = format!("TcpConnection [{:?}]. Error send response [{}]", self.stream, err);
                Err(message)
            }
        }
    }

    pub fn receive_request(&mut self) -> Result<String, String> {
        match crate::network::recv_string(&mut self.stream) {
            Ok(request) => {
                Ok(request)
            }
            Err(err) => {
                let message = format!("TcpConnection [{:?}]. Error receive request [{}]", self.stream, err);
                Err(message)
            }
        }
    }

    pub fn peer_addr(&self) -> Result<SocketAddr, String> {
        match self.stream.peer_addr() {
            Ok(addr) => {
                Ok(addr)
            }
            Err(err) => {
                let message = format!("TcpConnection [{:?}]. Error peer address command [{}]", self.stream, err);
                Err(message)
            }
        }
    }
}
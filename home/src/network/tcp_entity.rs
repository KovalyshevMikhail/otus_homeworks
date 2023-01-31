use std::net::{TcpListener, TcpStream};
use crate::network::HOME_PORT;


pub enum NodeType {
    Client,
    Server,
    All
}

struct TcpNode {
    listener: Option<TcpListener>,
    client: Option<TcpStream>
}

impl TcpNode {
    pub fn new(node_type: NodeType, port: u32) -> Self {
        let listener = Some(TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap());
        let client = {
            let stream = TcpStream::connect(format!("127.0.0.1:{}", HOME_PORT)).unwrap();
            Some(stream)
        };

        match node_type {
            NodeType::Client => {
                Self {
                    listener: None,
                    client
                }
            },
            NodeType::Server => {
                Self {
                    listener,
                    client: None
                }
            },
            NodeType::All => {
                Self {
                    listener,
                    client
                }
            }
        }
    }

    pub fn send_command(&self, _command: &str) {
        // crate::network::send_string(command, &mut self.client.unwrap());
    }

    pub fn start_receive(&mut self) {

        // thread::scope(move |_| {
        //     for stream in self.listener.incoming() {
        //         let stream = stream.unwrap();
        //
        //         thread::spawn(move || {
        //             println!("Connection created! {:?}", stream);
        //         })
        //     }
        // })
    }
}

struct TcpConnection();

struct TcpClient();
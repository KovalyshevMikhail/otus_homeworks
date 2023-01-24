use std::collections::HashMap;
use std::net::TcpListener;

//TODO: using for dispatch handles

pub struct DispatchInfo {
    name: String,
}

pub struct Dispatcher {
    listener: TcpListener,
    entities: HashMap<usize, DispatchInfo>
}

impl Default for Dispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Dispatcher {
    pub fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1::10000").unwrap();

        Self {
            entities: HashMap::new(),
            listener
        }
    }

    pub fn start(&self) -> Result<(), String> {
        for connection in self.listener.incoming() {
            let connection = connection.unwrap();

            println!("{:?}", connection);
        }

        Ok(())
    }
}

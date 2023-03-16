use std::net::TcpStream;

#[derive(Debug)]
pub struct TcpCommand {
    pub command: String,
    pub args: String,
    pub sender: TcpStream
}

impl TcpCommand {
    pub fn new(command: &str, args: &str, sender: TcpStream) -> TcpCommand {
        Self {
            command: String::from(command),
            args: String::from(args),
            sender
        }
    }
}

impl ToString for TcpCommand {
    fn to_string(&self) -> String {
        format!("[{}: [{}]]", self.command, self.args)
    }
}
#[derive(Debug)]
pub struct TcpCommand {
    pub command: String,
    pub args: String,
    pub sender: String
}

impl TcpCommand {
    pub fn new(command: &str, args: &str, sender: &str) -> TcpCommand {
        Self {
            command: String::from(command),
            args: String::from(args),
            sender: String::from(sender)
        }
    }
}

impl ToString for TcpCommand {
    fn to_string(&self) -> String {
        format!("[{}: [{}]]", self.command, self.args)
    }
}
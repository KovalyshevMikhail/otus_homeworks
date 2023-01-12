use super::Device;

/// Socket of smart home
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
        Socket {
            name: String::from("default"),
            description: String::from("default description"),
            power_max: 100.0,
            power_consumption: 0.0,
            enabled: false,
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
        Socket {
            name: String::from(name),
            description: String::from(description),
            power_max,
            power_consumption: 0.0,
            enabled: false,
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

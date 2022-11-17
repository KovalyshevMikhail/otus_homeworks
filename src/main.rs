/// Socket of smart home
///
/// Example
/// ```
/// // create socket with default parameters
/// let socket1 = Socket::new();
///
/// // create socket with custom parameters
/// let socket2 = Socket::from("S01", "description of SW01", 1000.0);
/// ```
struct Socket {
    name: String,
    description: String,
    power_max: f32,
    power_consumption: f32,
    enabled: bool,
}

impl Socket {
    /// Function create Socket with default parameters
    /// name - default
    /// description - default description
    /// power_max - 100.0
    ///
    /// Example
    /// ```
    /// let socket1 = Socket::new();
    /// ```
    fn new() -> Self {
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
    /// let socket2 = Socket::from("S01", "description of SW01", 1000.0);
    /// ```
    fn from(name: &str, description: &str, power_max: f32) -> Self {
        Socket {
            name: String::from(name),
            description: String::from(description),
            power_max: power_max,
            power_consumption: 0.0,
            enabled: false,
        }
    }

    /// Method enable power of Socket
    ///
    /// Example:
    /// ```
    /// let mut socket = Socket::from("S01", "description of SW01", 1000.0);
    /// socket.power_on();
    /// ```
    fn power_on(&mut self) {
        self.enabled = true;
    }

    /// Method disable power of Socket
    ///
    /// Example:
    /// ```
    /// let mut socket = Socket::from("S01", "description of SW01", 1000.0);
    /// socket.power_off();
    /// ```
    fn _power_off(&mut self) {
        self.enabled = false;
    }

    /// Method generate info about Socket
    ///
    /// Example:
    /// ```
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

/// Termometer is a device of smart home
///
/// Termometer contains temperature from sensor
///
/// Example:
/// ```
/// // create termometer with default parameters
/// let socket1 = Termometer::new();
///
/// // create termometer with custom parameters
/// let socket2 = Termometer::from("T01", "description of T01");
/// ```
struct Termometer {
    name: String,
    description: String,
    current_temperature: f32,
}

impl Termometer {
    /// Function create new Termometer with default parameters:
    /// name - default
    /// description - default description
    ///
    /// Example:
    /// ```
    /// let term = Termometer::new();
    /// ```
    fn new() -> Self {
        Termometer {
            name: String::from("default_name"),
            description: String::from("default_description"),
            current_temperature: 0.0,
        }
    }

    /// Function create new Termometer with custom parameters
    ///
    /// Example:
    /// ```
    /// let term = Termometer::from("T01", "Description of T01");
    /// ```
    fn from(name: &str, description: &str) -> Self {
        Termometer {
            name: String::from(name),
            description: String::from(description),
            current_temperature: 0.0,
        }
    }

    /// Method generate info about Termometer
    ///
    /// Example:
    /// ```
    /// let mut term = Termometer::from("T01", "Description of T01");
    /// println!("{}", term.info());
    /// ```
    fn info(&self) -> String {
        format!(
            "[TERMOMETER] {}\n{}\nParameters: [\n\tCurrent temperature = {}\n]",
            self.name, self.description, self.current_temperature
        )
    }
}

fn main() {
    let mut socket1 = Socket::new();
    socket1.power_on();
    println!("\n{}", socket1.info());

    let socket2 = Socket::from("S01", "description of SW01", 1000.0);
    println!("\n{}", socket2.info());

    let term1 = Termometer::new();
    println!("\n{}", term1.info());

    let term2 = Termometer::from("T01", "Description of T01");
    println!("\n{}", term2.info());
}

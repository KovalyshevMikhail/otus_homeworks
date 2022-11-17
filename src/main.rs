struct Socket {
    name: String,
    description: String,
    power_max: f32,
    power_consumption: f32,
    enabled: bool,
}

impl Socket {
    fn new() -> Self {
        Socket {
            name: String::from("default"),
            description: String::from("default description"),
            power_max: 100.0,
            power_consumption: 0.0,
            enabled: false,
        }
    }

    fn from(name: &str, description: &str, power_max: f32) -> Self {
        Socket {
            name: String::from(name),
            description: String::from(description),
            power_max: power_max,
            power_consumption: 0.0,
            enabled: false,
        }
    }

    fn power_on(&mut self) {
        self.enabled = true;
    }

    fn _power_off(&mut self) {
        self.enabled = false;
    }

    fn info(&self) -> String {
        format!(
            "[SOCKET] {} [power is on={}]\n{}\nParameters: [\n\tMax power = {}\n\tPower consumption = {}\n]",
            self.name, self.enabled, self.description, self.power_max, self.power_consumption
        )
    }
}

struct Termometer {
    name: String,
    description: String,
    current_temperature: f32,
}

impl Termometer {
    fn new() -> Self {
        Termometer {
            name: String::from("default_name"),
            description: String::from("default_description"),
            current_temperature: 0.0,
        }
    }

    fn from(name: &str, description: &str) -> Self {
        Termometer {
            name: String::from(name),
            description: String::from(description),
            current_temperature: 0.0,
        }
    }

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

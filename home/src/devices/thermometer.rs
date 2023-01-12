use super::Measurable;
use crate::devices::{Connectable, Device};

/// Thermometer is a device of smart home
///
/// Thermometer contains temperature from sensor
///
/// Example:
/// ```
/// use crate::home::devices::thermometer::Thermometer;
///
/// // create thermometer with default parameters
/// let socket1 = Thermometer::new();
///
/// // create thermometer with custom parameters
/// let socket2 = Thermometer::from("T01", "description of T01");
/// ```
pub struct Thermometer {
    name: String,
    description: String,
    current_temperature: f32,
}

impl Default for Thermometer {
    fn default() -> Self {
        Self::new()
    }
}

impl Thermometer {
    /// Function create new Thermometer with default parameters:
    /// name - default
    /// description - default description
    ///
    /// Example:
    /// ```
    /// use crate::home::devices::thermometer::Thermometer;
    ///
    /// let term = Thermometer::new();
    /// ```
    pub fn new() -> Self {
        Thermometer {
            name: String::from("default_name"),
            description: String::from("default_description"),
            current_temperature: 0.0,
        }
    }

    /// Function create new Thermometer with custom parameters
    ///
    /// Example:
    /// ```
    /// use crate::home::devices::thermometer::Thermometer;
    ///
    /// let term = Thermometer::from("T01", "Description of T01");
    /// ```
    pub fn from(name: &str, description: &str) -> Self {
        Thermometer {
            name: String::from(name),
            description: String::from(description),
            current_temperature: 0.0,
        }
    }

    /// Method generate info about Thermometer
    ///
    /// Example:
    /// ```
    /// use crate::home::devices::thermometer::Thermometer;
    ///
    /// let mut term = Thermometer::from("T01", "Description of T01");
    /// println!("{}", term.info());
    /// ```
    pub fn info(&self) -> String {
        format!(
            "[THERMOMETER] {}\n{}\nParameters: [\n\tCurrent temperature = {}\n]",
            self.name, self.description, self.current_temperature
        )
    }
}

impl Device for Thermometer {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn status(&self) -> String {
        String::from("OK")
    }

    fn info(&self) -> String {
        format!("[THERMOMETER] {} \n NO PARAMETERS", self.name)
    }
}

impl Measurable for Thermometer {
    fn data(&self) -> &str {
        "sample info"
    }
}

impl Connectable for Thermometer {}

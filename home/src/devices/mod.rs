pub mod socket;
pub mod thermometer;

/// General trait for all Devices
///
/// Trait must be to implement only three things in a String form:
/// - get name of device
/// - get description of device
/// - get status of device
pub trait Device {
    fn name(&self) -> &str;
    fn status(&self) -> String;
    fn info(&self) -> String;
}

pub trait Connectable {}

pub trait Measurable {
    fn data(&self) -> &str;
}

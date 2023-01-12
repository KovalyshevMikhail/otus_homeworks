use home::devices::{socket::Socket, thermometer::Thermometer, Device};
use home::places::{Home, Room};

fn main() {
    let mut socket1 = Socket::new();
    socket1.power_on();
    println!("\n{}", socket1.info());

    let socket2 = Socket::from("S01", "description of SW01", 1000.0);
    println!("\n{}", socket2.info());

    let term1 = Thermometer::new();
    println!("\n{}", term1.info());

    let term2 = Thermometer::from("T01", "Description of T01");
    println!("\n{}", term2.info());

    let mut home = Home::new("home".to_string());
    let room_name_01 = "R01";
    let room_01 = Room::new(room_name_01.to_string());
    home.add_room(room_01);

    let device_01 = Socket::new();
    home.add_device(room_name_01, Box::new(device_01)).unwrap();

    home.print_report();
}
